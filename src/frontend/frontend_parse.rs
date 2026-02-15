/// Parse Solidity source into [`ContractIR`] values.
pub fn parse_source(source: &str) -> Result<Vec<ContractIR>, FrontendError> {
    let (source_unit, comments) = parse(source, 0)
        .map_err(|diags| FrontendError::Parse(format_diagnostics(source, &diags)))?;

    // Build a map of end positions to preceding doc comments
    let comment_map = build_comment_map(&comments, source);

    let mut contracts = Vec::new();
    // Collect file-level `type X is Y` definitions so they can be injected into all contracts.
    let mut file_level_type_aliases: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut file_level_structs: Vec<StructIR> = Vec::new();
    let mut file_level_enums: Vec<EnumIR> = Vec::new();

    for part in source_unit.0.into_iter() {
        match part {
            SourceUnitPart::PragmaDirective(pragma) => {
                enforce_supported_pragma(&pragma)?;
            }
            SourceUnitPart::ContractDefinition(contract) => {
                contracts.push(convert_contract(*contract, &comment_map));
            }
            SourceUnitPart::TypeDefinition(td) => {
                let underlying = format!("{}", td.ty);
                file_level_type_aliases.insert(td.name.name, underlying);
            }
            SourceUnitPart::StructDefinition(def) => {
                file_level_structs.push(convert_struct(*def));
            }
            SourceUnitPart::EnumDefinition(def) => {
                file_level_enums.push(convert_enum(*def));
            }
            _ => {}
        }
    }

    // Inject file-level type aliases into every contract in the file.
    if !file_level_type_aliases.is_empty() {
        for contract in &mut contracts {
            for (name, underlying) in &file_level_type_aliases {
                contract
                    .type_aliases
                    .entry(name.clone())
                    .or_insert_with(|| underlying.clone());
            }
        }
    }

    if !file_level_structs.is_empty() {
        for contract in &mut contracts {
            for file_struct in &file_level_structs {
                if !contract
                    .structs
                    .iter()
                    .any(|existing| existing.name == file_struct.name)
                {
                    contract.structs.push(file_struct.clone());
                }
            }
        }
    }

    if !file_level_enums.is_empty() {
        for contract in &mut contracts {
            for file_enum in &file_level_enums {
                if !contract
                    .enums
                    .iter()
                    .any(|existing| existing.name == file_enum.name)
                {
                    contract.enums.push(file_enum.clone());
                }
            }
        }
    }

    Ok(contracts)
}

fn enforce_supported_pragma(
    pragma: &solang_parser::pt::PragmaDirective,
) -> Result<(), FrontendError> {
    use solang_parser::pt::PragmaDirective;

    let PragmaDirective::Version(_, ident, comparators) = pragma else {
        return Ok(());
    };

    if ident.name != "solidity" {
        return Ok(());
    }

    let spec = comparators
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");

    // Compiler compatibility targets mainstream modern Solidity ranges used by
    // upstream protocols. We accept pragmas that intersect 0.5.x through 0.8.x.
    if pragma_supports_neo_solidity(spec.as_str()) {
        Ok(())
    } else {
        Err(FrontendError::UnsupportedVersion(spec))
    }
}

fn pragma_supports_neo_solidity(spec: &str) -> bool {
    let normalized = spec.replace(' ', "").to_lowercase();

    if normalized.is_empty() {
        return true;
    }

    // Accept if any OR-branch can include a supported compiler range.
    normalized
        .split("||")
        .any(branch_supports_neo_solidity)
}

fn branch_supports_neo_solidity(branch: &str) -> bool {
    if branch.is_empty() {
        return false;
    }

    let comparators = split_comparators(branch);
    if comparators.is_empty() {
        return false;
    }

    let mut lower = Bound::Unbounded;
    let mut upper = Bound::Unbounded;

    for comparator in comparators {
        if comparator == "*" {
            continue;
        }

        if let Some((start, end)) = parse_hyphen_range(&comparator) {
            lower = lower.max(Bound::Inclusive(start));
            upper = upper.min(Bound::Inclusive(end));
            continue;
        }

        if let Some((version, level)) = parse_caret(&comparator) {
            let upper_version = match level {
                0 => Version {
                    major: version.major.saturating_add(1),
                    minor: 0,
                    patch: 0,
                },
                _ => Version {
                    major: version.major,
                    minor: version.minor.saturating_add(1),
                    patch: 0,
                },
            };
            lower = lower.max(Bound::Inclusive(version));
            upper = upper.min(Bound::Exclusive(upper_version));
            continue;
        }

        if let Some(version) = parse_tilde(&comparator) {
            let upper_version = Version {
                major: version.major,
                minor: version.minor.saturating_add(1),
                patch: 0,
            };
            lower = lower.max(Bound::Inclusive(version));
            upper = upper.min(Bound::Exclusive(upper_version));
            continue;
        }

        if let Some((op, version)) = parse_operator_version(&comparator) {
            match op {
                ComparatorOp::Greater => lower = lower.max(Bound::Exclusive(version)),
                ComparatorOp::GreaterEq => lower = lower.max(Bound::Inclusive(version)),
                ComparatorOp::Less => upper = upper.min(Bound::Exclusive(version)),
                ComparatorOp::LessEq => upper = upper.min(Bound::Inclusive(version)),
                ComparatorOp::Exact => {
                    lower = lower.max(Bound::Inclusive(version));
                    upper = upper.min(Bound::Inclusive(version));
                }
            }
            continue;
        }

        if let Some(version) = parse_plain_version(&comparator) {
            lower = lower.max(Bound::Inclusive(version));
            upper = upper.min(Bound::Inclusive(version));
            continue;
        }

        // Unknown comparator format: reject conservatively.
        return false;
    }

    intersects_supported_neo_range(lower, upper)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

#[derive(Clone, Copy, Debug)]
enum Bound {
    Unbounded,
    Inclusive(Version),
    Exclusive(Version),
}

impl Bound {
    fn max(self, other: Self) -> Self {
        use Bound::{Exclusive, Inclusive, Unbounded};

        match (self, other) {
            (Unbounded, x) | (x, Unbounded) => x,
            (Inclusive(a), Inclusive(b)) => {
                if a >= b {
                    Inclusive(a)
                } else {
                    Inclusive(b)
                }
            }
            (Exclusive(a), Exclusive(b)) => {
                if a >= b {
                    Exclusive(a)
                } else {
                    Exclusive(b)
                }
            }
            (Inclusive(a), Exclusive(b)) => {
                if a > b {
                    Inclusive(a)
                } else if b > a {
                    Exclusive(b)
                } else {
                    Exclusive(a)
                }
            }
            (Exclusive(a), Inclusive(b)) => {
                if a > b {
                    Exclusive(a)
                } else if b > a {
                    Inclusive(b)
                } else {
                    Exclusive(a)
                }
            }
        }
    }

    fn min(self, other: Self) -> Self {
        use Bound::{Exclusive, Inclusive, Unbounded};

        match (self, other) {
            (Unbounded, x) | (x, Unbounded) => x,
            (Inclusive(a), Inclusive(b)) => {
                if a <= b {
                    Inclusive(a)
                } else {
                    Inclusive(b)
                }
            }
            (Exclusive(a), Exclusive(b)) => {
                if a <= b {
                    Exclusive(a)
                } else {
                    Exclusive(b)
                }
            }
            (Inclusive(a), Exclusive(b)) => {
                if a < b {
                    Inclusive(a)
                } else if b < a {
                    Exclusive(b)
                } else {
                    Exclusive(a)
                }
            }
            (Exclusive(a), Inclusive(b)) => {
                if a < b {
                    Exclusive(a)
                } else if b < a {
                    Inclusive(b)
                } else {
                    Exclusive(a)
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum ComparatorOp {
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Exact,
}

fn split_comparators(branch: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = branch.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        if ch == ',' {
            i += 1;
            continue;
        }

        if ch == '^' || ch == '~' {
            let mut token = String::new();
            token.push(ch);
            i += 1;
            while i < chars.len() {
                let c = chars[i];
                if c == ',' || c == '^' || c == '~' || c == '<' || c == '>' || c == '=' {
                    break;
                }
                token.push(c);
                i += 1;
            }
            tokens.push(token);
            continue;
        }

        if ch == '<' || ch == '>' || ch == '=' {
            let mut token = String::new();
            token.push(ch);
            i += 1;
            if i < chars.len() && chars[i] == '=' {
                token.push('=');
                i += 1;
            }
            while i < chars.len() {
                let c = chars[i];
                if c == ',' || c == '^' || c == '~' || c == '<' || c == '>' || c == '=' {
                    break;
                }
                token.push(c);
                i += 1;
            }
            tokens.push(token);
            continue;
        }

        // Plain version or hyphen range.
        let mut token = String::new();
        while i < chars.len() {
            let c = chars[i];
            if c == ',' || c == '^' || c == '~' || c == '<' || c == '>' || c == '=' {
                break;
            }
            token.push(c);
            i += 1;
        }
        if !token.is_empty() {
            tokens.push(token);
        }
    }

    tokens
}

fn parse_hyphen_range(comparator: &str) -> Option<(Version, Version)> {
    let (left, right) = comparator.split_once('-')?;
    let start = parse_plain_version(left)?;
    let end = parse_plain_version(right)?;
    Some((start, end))
}

fn parse_caret(comparator: &str) -> Option<(Version, u8)> {
    let raw = comparator.strip_prefix('^')?;
    let dots = raw.matches('.').count() as u8;
    let version = parse_plain_version(raw)?;
    Some((version, dots))
}

fn parse_tilde(comparator: &str) -> Option<Version> {
    let raw = comparator.strip_prefix('~')?;
    parse_plain_version(raw)
}

fn parse_operator_version(comparator: &str) -> Option<(ComparatorOp, Version)> {
    if let Some(raw) = comparator.strip_prefix(">=") {
        return parse_plain_version(raw).map(|v| (ComparatorOp::GreaterEq, v));
    }
    if let Some(raw) = comparator.strip_prefix("<=") {
        return parse_plain_version(raw).map(|v| (ComparatorOp::LessEq, v));
    }
    if let Some(raw) = comparator.strip_prefix('>') {
        return parse_plain_version(raw).map(|v| (ComparatorOp::Greater, v));
    }
    if let Some(raw) = comparator.strip_prefix('<') {
        return parse_plain_version(raw).map(|v| (ComparatorOp::Less, v));
    }
    if let Some(raw) = comparator.strip_prefix('=') {
        return parse_plain_version(raw).map(|v| (ComparatorOp::Exact, v));
    }
    None
}

fn parse_plain_version(raw: &str) -> Option<Version> {
    if raw.is_empty() || raw == "*" {
        return None;
    }

    let mut parts = raw.split('.');
    let major_raw = parts.next()?;
    let minor_raw = parts.next().unwrap_or("0");
    let patch_raw = parts.next().unwrap_or("0");

    if parts.next().is_some() {
        return None;
    }

    // Allow wildcard suffixes like 0.8.* or 0.8.x as "any patch".
    let major = major_raw.parse::<u64>().ok()?;
    let minor = if minor_raw == "*" || minor_raw == "x" {
        0
    } else {
        minor_raw.parse::<u64>().ok()?
    };
    let patch = if patch_raw == "*" || patch_raw == "x" {
        0
    } else {
        patch_raw.parse::<u64>().ok()?
    };

    Some(Version {
        major,
        minor,
        patch,
    })
}

fn intersects_supported_neo_range(lower: Bound, upper: Bound) -> bool {
    // Supported upstream Solidity ranges for this compiler compatibility layer.
    (5u64..=8).any(|minor| {
        intersects_semver_window(
            lower,
            upper,
            Version {
                major: 0,
                minor,
                patch: 0,
            },
            Version {
                major: 0,
                minor: minor + 1,
                patch: 0,
            },
        )
    })
}

fn intersects_semver_window(
    lower: Bound,
    upper: Bound,
    target_start: Version,
    target_end_exclusive: Version,
) -> bool {

    let effective_start = match lower {
        Bound::Unbounded => target_start,
        Bound::Inclusive(v) => v,
        Bound::Exclusive(v) => next_patch(v),
    };

    let effective_end_exclusive = match upper {
        Bound::Unbounded => target_end_exclusive,
        Bound::Inclusive(v) => next_patch(v),
        Bound::Exclusive(v) => v,
    };

    let range_start = if effective_start > target_start {
        effective_start
    } else {
        target_start
    };
    let range_end = if effective_end_exclusive < target_end_exclusive {
        effective_end_exclusive
    } else {
        target_end_exclusive
    };

    range_start < range_end
}

fn next_patch(version: Version) -> Version {
    Version {
        major: version.major,
        minor: version.minor,
        patch: version.patch.saturating_add(1),
    }
}

/// Build a map from source positions to their preceding Natspec comments.
fn build_comment_map(comments: &[Comment], _source: &str) -> HashMap<usize, NatspecDocIR> {
    let mut map = HashMap::new();
    let mut last_doc_comment: Option<(usize, String)> = None;

    for comment in comments {
        match comment {
            Comment::DocLine(loc, text) | Comment::DocBlock(loc, text) => {
                if let Loc::File(_, _, end) = loc {
                    // Accumulate doc comments - update end position to latest
                    let clean_text = clean_doc_comment(text);
                    if let Some((ref mut end_pos, ref mut existing)) = last_doc_comment {
                        *end_pos = *end; // Update to latest end position
                        existing.push('\n');
                        existing.push_str(&clean_text);
                    } else {
                        last_doc_comment = Some((*end, clean_text));
                    }
                }
            }
            Comment::Line(_loc, _) | Comment::Block(_loc, _) => {
                // Regular comments break doc comment sequences
                if let Some((end_pos, doc_text)) = last_doc_comment.take() {
                    map.insert(end_pos, parse_natspec(&doc_text));
                }
            }
        }
    }

    // Handle trailing doc comment
    if let Some((end_pos, doc_text)) = last_doc_comment {
        map.insert(end_pos, parse_natspec(&doc_text));
    }

    map
}

/// Remove comment delimiters and leading asterisks/slashes
fn clean_doc_comment(text: &str) -> String {
    text.lines()
        .map(|line| {
            let trimmed = line.trim();
            // Remove /// prefix from line doc comments
            if let Some(rest) = trimmed.strip_prefix("///") {
                rest.trim().to_string()
            // Remove /** and */ from block doc comments
            } else if let Some(rest) = trimmed.strip_prefix("/**") {
                rest.trim_end_matches("*/").trim().to_string()
            } else if let Some(rest) = trimmed.strip_suffix("*/") {
                rest.trim().to_string()
            // Remove leading * from block comment lines
            } else if let Some(rest) = trimmed.strip_prefix('*') {
                rest.trim().to_string()
            } else {
                trimmed.to_string()
            }
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parse Natspec tags from a documentation comment
fn parse_natspec(text: &str) -> NatspecDocIR {
    let mut doc = NatspecDocIR::default();
    let mut current_tag: Option<&str> = None;
    let mut current_content = String::new();

    for line in text.lines() {
        let trimmed = line.trim();

        // Check for tag at start of line
        if trimmed.starts_with('@') {
            // Save previous tag content
            if let Some(tag) = current_tag {
                save_tag_content(&mut doc, tag, &current_content);
            }

            // Parse new tag
            let parts: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();
            current_tag = Some(parts[0]);
            current_content = parts
                .get(1)
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
        } else if current_tag.is_some() {
            // Continue previous tag content
            if !current_content.is_empty() {
                current_content.push(' ');
            }
            current_content.push_str(trimmed);
        } else {
            // No tag yet - treat as @notice
            if doc.notice.is_none() && !trimmed.is_empty() {
                doc.notice = Some(trimmed.to_string());
            } else if let Some(ref mut notice) = doc.notice {
                notice.push(' ');
                notice.push_str(trimmed);
            }
        }
    }

    // Save final tag
    if let Some(tag) = current_tag {
        save_tag_content(&mut doc, tag, &current_content);
    }

    doc
}

fn save_tag_content(doc: &mut NatspecDocIR, tag: &str, content: &str) {
    let content = content.trim().to_string();
    if content.is_empty() {
        return;
    }

    match tag {
        "@title" => doc.title = Some(content),
        "@author" => doc.author = Some(content),
        "@notice" => doc.notice = Some(content),
        "@dev" => doc.dev = Some(content),
        "@param" => {
            // Format: @param name description
            let parts: Vec<&str> = content.splitn(2, char::is_whitespace).collect();
            if parts.len() >= 2 {
                doc.params
                    .push((parts[0].to_string(), parts[1].trim().to_string()));
            } else if !parts.is_empty() {
                doc.params.push((parts[0].to_string(), String::new()));
            }
        }
        "@return" => doc.returns.push(content),
        tag if tag.starts_with("@custom:") => {
            let custom_tag = tag.strip_prefix("@custom:").unwrap_or("");
            doc.custom.push((custom_tag.to_string(), content));
        }
        _ => {} // Ignore unknown tags
    }
}

/// Find the doc comment that precedes a given source location
fn find_preceding_doc(loc: &Loc, comment_map: &HashMap<usize, NatspecDocIR>) -> NatspecDocIR {
    if let Loc::File(_, start, _) = loc {
        // Look for a doc comment that ends near this start position
        // Allow some whitespace between comment end and definition start
        for offset in 0..100 {
            if let Some(pos) = start.checked_sub(offset) {
                if let Some(doc) = comment_map.get(&pos) {
                    return doc.clone();
                }
            } else {
                break;
            }
        }
    }
    NatspecDocIR::default()
}

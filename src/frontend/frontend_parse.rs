/// Parse Solidity source into [`ContractIR`] values.
pub fn parse_source(source: &str) -> Result<Vec<ContractIR>, FrontendError> {
    let (source_unit, comments) = parse_solidity_guarded(source)
        .map_err(|diags| FrontendError::ParseDiagnostics(collect_parse_diagnostics(source, &diags)))?;

    // Build a map of end positions to preceding doc comments
    let comment_map = build_comment_map(&comments, source);

    let mut contracts = Vec::new();
    // Collect file-level `type X is Y` definitions so they can be injected into all contracts.
    let mut file_level_type_aliases: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut file_level_structs: Vec<StructIR> = Vec::new();
    let mut file_level_enums: Vec<EnumIR> = Vec::new();
    // File-scope custom `error` declarations (Solidity 0.8.4+). Merged into
    // every contract so revert-site lowering can resolve the declared
    // signature regardless of where the error was declared.
    let mut file_level_errors: Vec<ErrorIR> = Vec::new();
    // Task #187 — collect file-scope free functions (Solidity 0.7+). A free
    // function like `function helper(uint a, uint b) pure returns (uint) { ... }`
    // declared outside any contract is conceptually internal to every contract
    // in the source unit; merging it into each primary contract's function
    // table lets call-site dispatch (`ctx.function_names.contains(...)`)
    // resolve the reference as a regular internal call.
    let mut file_level_free_functions: Vec<FunctionIR> = Vec::new();
    // Task #188 — Solidity 0.8.13+ file-level `using { L.f1, L.f2 } for T;`
    // (and `using L for T global;`) attach directives apply to every contract
    // declared in the same source unit. Collect them here and merge into each
    // converted ContractIR below, symmetric with the file-level type-alias /
    // struct / enum / free-function injection passes. Without this, the IR
    // lowering stage only sees contract-scope `using` directives and the
    // member-style call resolver hard-errors on `x.double()` for any
    // attachment declared at file scope.
    let mut file_level_usings: Vec<Using> = Vec::new();

    // Track the declared pragma's minimum Solidity version so we can reject
    // features that were introduced later (solc-compatible behavior).
    let mut pragma_min_version: Option<Version> = None;

    for part in source_unit.0 {
        match part {
            SourceUnitPart::PragmaDirective(pragma) => {
                if let Some(min) = enforce_supported_pragma(&pragma)? {
                    // The combined source unit's effective minimum compiler
                    // version is the *intersection* of every file's pragma
                    // range. Since each file's pragma gives a lower bound on
                    // the version that file accepts, the chosen compiler
                    // version must be `>= max(file_mins)`. Earlier versions
                    // tracked the MIN here, which incorrectly lowered the
                    // effective version when one imported file declared
                    // `>=0.4.16` (a broad lower bound used by ENS / Aave / some
                    // OZ utility files); that caused legitimate uses of
                    // `string.concat` / `bytes.concat` in the entry contract
                    // to fail the feature-version gate.
                    pragma_min_version = match pragma_min_version {
                        Some(existing) if existing >= min => Some(existing),
                        _ => Some(min),
                    };
                }
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
            SourceUnitPart::ErrorDefinition(def) => {
                file_level_errors.push(convert_error(*def));
            }
            SourceUnitPart::FunctionDefinition(def) => {
                // Task #187 — file-scope free function. Free functions are
                // implicitly internal (Solidity rejects `public`/`external`
                // at file scope). Normalize visibility to `Internal` so the
                // merged function behaves like any other internal helper in
                // the consuming contract, and mark the type as `Function`
                // regardless of what solang-parser surfaces.
                let mut fn_ir = convert_function(*def, &comment_map);
                fn_ir.visibility = VisibilityKind::Internal;
                fn_ir.ty = FunctionTy::Function;
                file_level_free_functions.push(fn_ir);
            }
            SourceUnitPart::Using(using) => {
                // Task #188 — capture file-level `using` directives; merged
                // into each contract below once all parts have been parsed.
                file_level_usings.push(*using);
            }
            SourceUnitPart::ImportDirective(_) => {
                // Intentionally ignored. Import resolution happens in a
                // separate pass that scans source for import directives
                // independently and merges imported files into the source
                // unit before/around this function. Any ImportDirective
                // node still present here is therefore a no-op.
            }
            SourceUnitPart::StraySemicolon(_) => {
                // Parser artifact (a stray `;` at file scope) — drop silently.
            }
            SourceUnitPart::EventDefinition(_)
            | SourceUnitPart::VariableDefinition(_)
            | SourceUnitPart::Annotation(_) => {
                // File-level events / constants / annotations are not yet
                // lowered into the IR. Listed explicitly — never via `_` —
                // so a future solang-parser grammar addition cannot silently
                // fall through and produce an empty contract (audit L-FE1).
            }
            // L-FE1 safety net — any variant not explicitly handled above is
            // a construct this compiler does not know about. Unreachable for
            // the parser version pinned in Cargo.lock (hence the allow below);
            // if it fires after a parser upgrade, the user sees a clear "file
            // a bug" message instead of a silent empty contract. Keep this arm
            // even though it is currently unreachable — it is the forward-
            // compatibility guard L-FE1 requires.
            #[allow(unreachable_patterns)]
            other => {
                return Err(FrontendError::UnsupportedConstruct(format!(
                    "{other:?}"
                )));
            }
        }
    }

    // Enforce per-feature pragma gates (solc emits a hard error when a feature
    // is used outside its declared minimum version). POC: `string.concat` /
    // `bytes.concat`. See `FEATURE_*_MIN` constants for the registry.
    enforce_feature_version_gates(source, pragma_min_version)?;

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

    // Inject file-level custom errors into every contract in the file.
    // Contract-scope declarations shadow same-named file-scope ones.
    if !file_level_errors.is_empty() {
        for contract in &mut contracts {
            for file_error in &file_level_errors {
                if !contract
                    .errors
                    .iter()
                    .any(|existing| existing.name == file_error.name)
                {
                    contract.errors.push(file_error.clone());
                }
            }
        }
    }

    // Task #187 — inject file-scope free functions into every contract in the
    // source unit. Mirrors how the library-merge pass in `analyse_all_sources`
    // pulls sibling library bodies into primary contracts so the IR lowering
    // stage (`function_names` symbol table) can dispatch free-function calls
    // as regular internal calls instead of falling through to the
    // unresolved-call compatibility path that silently drops arguments and
    // pushes a zero return value. Contracts where a same-named method already
    // exists keep their own definition (contract-scope wins).
    if !file_level_free_functions.is_empty() {
        for contract in &mut contracts {
            for free_fn in &file_level_free_functions {
                if !contract
                    .functions
                    .iter()
                    .any(|existing| existing.name == free_fn.name)
                {
                    contract.functions.push(free_fn.clone());
                }
            }
        }
    }

    // Task #188 — merge every file-level `using` directive into each contract
    // in the source unit. The IR-lowering stage consumes `ContractIR`'s
    // `using_directives` / `using_for_libraries` / `has_using_function_list`
    // fields to build the `using_target_types`, `using_function_list_targets`,
    // and `using_function_list_scope_targets` symbol tables that drive
    // `ctx.has_using_directives()` and the member-style call resolver. Without
    // this merge, the file-level form `using { L.f1, L.f2 } for T;` is
    // completely invisible to lowering (both library-form `using L for T;`
    // and function-list form are affected). Libraries don't participate in
    // `using`-for dispatch, so skip them here — mirroring the
    // `normalize_library_for_neo` treatment downstream.
    if !file_level_usings.is_empty() {
        for contract in &mut contracts {
            if matches!(contract.kind, ContractKind::Library) {
                continue;
            }
            for using in &file_level_usings {
                apply_file_level_using(contract, using);
            }
        }
    }

    Ok(contracts)
}

fn enforce_supported_pragma(
    pragma: &solang_parser::pt::PragmaDirective,
) -> Result<Option<Version>, FrontendError> {
    use solang_parser::pt::PragmaDirective;

    let PragmaDirective::Version(_, ident, comparators) = pragma else {
        return Ok(None);
    };

    if ident.name != "solidity" {
        return Ok(None);
    }

    let spec = comparators
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");

    // Compiler compatibility targets mainstream modern Solidity ranges used by
    // upstream protocols. We accept pragmas that intersect 0.5.x through 0.8.x.
    if pragma_supports_neo_devpack_solidity(spec.as_str()) {
        Ok(pragma_min_version(spec.as_str()))
    } else {
        Err(FrontendError::UnsupportedVersion(spec))
    }
}

/// Compute the pragma's lowest-allowed concrete Solidity version.
///
/// Used to enforce per-feature version gates (e.g. `string.concat` requires
/// `>= 0.8.12`). When the pragma admits multiple OR-branches, we pick the
/// smallest lower bound since any of those versions may be used at compile
/// time. Returns `None` for unbounded or unparseable ranges.
fn pragma_min_version(spec: &str) -> Option<Version> {
    let normalized = spec.replace(' ', "").to_lowercase();
    if normalized.is_empty() {
        return None;
    }

    let mut best: Option<Version> = None;
    for branch in normalized.split("||") {
        let Some(v) = branch_min_version(branch) else {
            continue;
        };
        best = match best {
            Some(existing) if existing <= v => Some(existing),
            _ => Some(v),
        };
    }
    best
}

fn branch_min_version(branch: &str) -> Option<Version> {
    let comparators = split_comparators(branch);
    let mut lower: Option<Version> = None;
    let mut update = |candidate: Version| {
        lower = match lower {
            Some(existing) if existing >= candidate => Some(existing),
            _ => Some(candidate),
        };
    };

    for comparator in comparators {
        if comparator == "*" {
            continue;
        }
        if let Some((start, _)) = parse_hyphen_range(&comparator) {
            update(start);
            continue;
        }
        if let Some((version, _)) = parse_caret(&comparator) {
            update(version);
            continue;
        }
        if let Some(version) = parse_tilde(&comparator) {
            update(version);
            continue;
        }
        if let Some((op, version)) = parse_operator_version(&comparator) {
            match op {
                ComparatorOp::Greater => update(next_patch(version)),
                ComparatorOp::GreaterEq | ComparatorOp::Exact => update(version),
                _ => {}
            }
            continue;
        }
        if let Some(version) = parse_plain_version(&comparator) {
            update(version);
        }
    }
    lower
}

/// Feature registry: features that require a minimum Solidity version.
///
/// POC covers `string.concat`/`bytes.concat` (both introduced in 0.8.12 /
/// 0.8.4 respectively). Extend this as more features are gated.
const FEATURE_STRING_CONCAT_MIN: Version = Version {
    major: 0,
    minor: 8,
    patch: 12,
};
const FEATURE_BYTES_CONCAT_MIN: Version = Version {
    major: 0,
    minor: 8,
    patch: 4,
};

/// Reject feature uses that predate the pragma's declared minimum version.
///
/// Scans `source` for `string.concat(` / `bytes.concat(` tokens (outside of
/// comments and string literals) and errors when `pragma_min` is below the
/// feature's introduction version. Matches solc's "feature unavailable in
/// declared pragma range" behavior for the most common gap.
fn enforce_feature_version_gates(
    source: &str,
    pragma_min: Option<Version>,
) -> Result<(), FrontendError> {
    let Some(min) = pragma_min else {
        return Ok(());
    };

    let stripped = strip_comments_and_strings(source);

    if min < FEATURE_STRING_CONCAT_MIN && contains_builtin_call(&stripped, "string.concat(") {
        return Err(FrontendError::Parse(format!(
            "feature `string.concat` requires pragma >= 0.8.12; declared pragma allows {}.{}.{}",
            min.major, min.minor, min.patch
        )));
    }
    if min < FEATURE_BYTES_CONCAT_MIN && contains_builtin_call(&stripped, "bytes.concat(") {
        return Err(FrontendError::Parse(format!(
            "feature `bytes.concat` requires pragma >= 0.8.4; declared pragma allows {}.{}.{}",
            min.major, min.minor, min.patch
        )));
    }
    Ok(())
}

/// True when `needle` (e.g. `"string.concat("`) appears as a real builtin call
/// rather than as a suffix of a larger identifier — i.e. the character
/// immediately before it is not part of an identifier (`a-zA-Z0-9_`). Without
/// this a user variable like `myString.concat(...)` would falsely trip the
/// pragma-feature gate.
fn contains_builtin_call(haystack: &str, needle: &str) -> bool {
    let bytes = haystack.as_bytes();
    let mut start = 0usize;
    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;
        let boundary_ok = abs == 0
            || {
                let prev = bytes[abs - 1];
                !(prev.is_ascii_alphanumeric() || prev == b'_')
            };
        if boundary_ok {
            return true;
        }
        start = abs + 1;
    }
    false
}

/// Lightweight lexer-aware strip: replaces string-literal and comment bodies
/// with spaces so they cannot produce false positives for feature scans.
/// Handles `//` line comments, `/* */` block comments, `"..."` and `'...'`
/// string literals with backslash escapes. Adequate for identifier-level
/// feature probing (not a full Solidity lexer).
fn strip_comments_and_strings(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut out = String::with_capacity(source.len());
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            while i < bytes.len() && bytes[i] != b'\n' {
                out.push(' ');
                i += 1;
            }
            continue;
        }
        if c == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
            out.push_str("  ");
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                out.push(' ');
                i += 1;
            }
            if i + 1 < bytes.len() {
                out.push_str("  ");
                i += 2;
            }
            continue;
        }
        if c == b'"' || c == b'\'' {
            let quote = c;
            out.push(' ');
            i += 1;
            while i < bytes.len() && bytes[i] != quote {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    out.push_str("  ");
                    i += 2;
                    continue;
                }
                out.push(' ');
                i += 1;
            }
            if i < bytes.len() {
                out.push(' ');
                i += 1;
            }
            continue;
        }
        out.push(c as char);
        i += 1;
    }
    out
}

fn pragma_supports_neo_devpack_solidity(spec: &str) -> bool {
    let normalized = spec.replace(' ', "").to_lowercase();

    if normalized.is_empty() {
        return true;
    }

    // Accept if any OR-branch can include a supported compiler range.
    normalized
        .split("||")
        .any(branch_supports_neo_devpack_solidity)
}

fn branch_supports_neo_devpack_solidity(branch: &str) -> bool {
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

/// Advance `pos` over ASCII whitespace in `bytes`, returning the index of the
/// next non-whitespace byte (or the end of input).
fn skip_whitespace_forward(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
        pos += 1;
    }
    pos
}

/// Build a map from a DECLARATION's start position to the Natspec block that
/// immediately precedes it. Each accumulated doc block is keyed at the first
/// non-whitespace byte AFTER it (i.e. the start of the declaration it
/// documents), so attachment is an exact lookup at the declaration's `start`
/// — independent of distance, and never bleeding across an intervening
/// declaration. A doc run is also broken when a non-whitespace token sits
/// between two doc comments (that earlier doc belongs to the intervening
/// declaration, not the later block).
fn build_comment_map(comments: &[Comment], source: &str) -> HashMap<usize, NatspecDocIR> {
    let mut map = HashMap::new();
    let bytes = source.as_bytes();
    // (end_of_last_doc_line, accumulated_text)
    let mut pending: Option<(usize, String)> = None;

    for comment in comments {
        match comment {
            Comment::DocLine(loc, text) | Comment::DocBlock(loc, text) => {
                if let Loc::File(_, start, end) = loc {
                    let clean_text = clean_doc_comment(text);
                    let continues = match &pending {
                        Some((prev_end, _)) => bytes
                            .get(*prev_end..*start)
                            .is_some_and(|gap| gap.iter().all(u8::is_ascii_whitespace)),
                        None => false,
                    };
                    if continues {
                        if let Some((prev_end, existing)) = pending.as_mut() {
                            *prev_end = *end;
                            existing.push('\n');
                            existing.push_str(&clean_text);
                        }
                    } else {
                        if let Some((prev_end, doc_text)) = pending.take() {
                            map.insert(
                                skip_whitespace_forward(bytes, prev_end),
                                parse_natspec(&doc_text),
                            );
                        }
                        pending = Some((*end, clean_text));
                    }
                }
            }
            Comment::Line(_loc, _) | Comment::Block(_loc, _) => {
                // Regular comments break doc comment sequences.
                if let Some((prev_end, doc_text)) = pending.take() {
                    map.insert(
                        skip_whitespace_forward(bytes, prev_end),
                        parse_natspec(&doc_text),
                    );
                }
            }
        }
    }

    if let Some((prev_end, doc_text)) = pending.take() {
        map.insert(
            skip_whitespace_forward(bytes, prev_end),
            parse_natspec(&doc_text),
        );
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

/// Find the doc comment that precedes a given declaration. `build_comment_map`
/// keys each doc block at the start of the declaration it documents, so this is
/// an exact lookup at the declaration's `start` — no fixed-distance backward
/// scan (which mis-attached across intervening tokens or missed docs separated
/// by more than 100 bytes of whitespace).
fn find_preceding_doc(loc: &Loc, comment_map: &HashMap<usize, NatspecDocIR>) -> NatspecDocIR {
    if let Loc::File(_, start, _) = loc {
        if let Some(doc) = comment_map.get(start) {
            return doc.clone();
        }
    }
    NatspecDocIR::default()
}

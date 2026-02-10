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

    for part in source_unit.0.into_iter() {
        match part {
            SourceUnitPart::ContractDefinition(contract) => {
                contracts.push(convert_contract(*contract, &comment_map));
            }
            SourceUnitPart::TypeDefinition(td) => {
                let underlying = format!("{}", td.ty);
                file_level_type_aliases.insert(td.name.name, underlying);
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

    Ok(contracts)
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


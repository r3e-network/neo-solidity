fn format_diagnostics(source: &str, diagnostics: &[Diagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diag| {
            if let solang_parser::pt::Loc::File(_, start, _) = diag.loc {
                let (line, column) = offset_to_line_column(source, start);
                format!("{}:{}: {}", line, column, diag.message)
            } else {
                diag.message.clone()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn offset_to_line_column(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut column = 1usize;
    let mut current = 0usize;

    for ch in source.chars() {
        if current >= offset {
            break;
        }

        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }

        current += ch.len_utf8();
    }

    (line, column)
}


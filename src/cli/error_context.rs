pub fn print_detailed_error(source: &str, error_msg: &str) {
    eprintln!("{}", error_msg);

    if let Some((line, col)) = extract_line_column(error_msg) {
        print_source_context(source, line, col);
    }
}

pub fn extract_line_column(error_msg: &str) -> Option<(usize, usize)> {
    if let Some(line_start) = error_msg.find("line ")
        && let Some(line_end) = error_msg[line_start..].find(',')
    {
        let line_str = &error_msg[line_start + 5..line_start + line_end];
        if let Ok(line) = line_str.trim().parse::<usize>()
            && let Some(col_start) = error_msg.find("column ")
        {
            let col_str = &error_msg[col_start + 7..];
            let col_end = col_str
                .find(|c: char| !c.is_numeric())
                .unwrap_or(col_str.len());
            if let Ok(col) = col_str[..col_end].trim().parse::<usize>() {
                return Some((line, col));
            }
        }
    }
    None
}

pub fn print_source_context(source: &str, error_line: usize, error_col: usize) {
    let lines: Vec<&str> = source.lines().collect();

    if error_line == 0 || error_line > lines.len() {
        return;
    }

    eprintln!();
    eprintln!("源代码位置:");

    if error_line > 1 {
        eprintln!("{:4} | {}", error_line - 1, lines[error_line - 2]);
    }

    eprintln!("{:4} | {}", error_line, lines[error_line - 1]);

    let indent = format!("{:4} | ", error_line);
    let pointer = " ".repeat(error_col.saturating_sub(1)) + "^";
    eprintln!("{}{}", indent, pointer);

    if error_line < lines.len() {
        eprintln!("{:4} | {}", error_line + 1, lines[error_line]);
    }
    eprintln!();
}

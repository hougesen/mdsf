pub mod table_of_contents;

pub fn update_markdown_section(readme: &str, key: &str, value: &str) -> String {
    let start = format!("<!-- START_SECTION:{key} -->");
    let end = format!("<!-- END_SECTION:{key} -->");

    let mut lines = Vec::new();

    let mut inside = false;
    let mut start_seen = false;
    let mut end_seen = false;

    for line in readme.lines() {
        if inside {
            if line == end {
                lines.push(line.to_string());

                inside = false;
                end_seen = true;
            }
        } else if line == start {
            lines.push(line.to_string());

            lines.push(format!("\n{value}\n"));

            inside = true;
            start_seen = true;
        } else {
            lines.push(line.to_string());
        }
    }

    if start_seen && end_seen {
        lines.join("\n")
    } else {
        value.to_string()
    }
}

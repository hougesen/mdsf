pub mod table_of_contents;

pub fn update_markdown_section(
    readme: &str,
    key: &str,
    value: &str,
) -> Result<String, regex::Error> {
    let start = format!("<!-- START_SECTION:{key} -->");
    let end = format!("<!-- END_SECTION:{key} -->");

    let re = regex::RegexBuilder::new(format!(r"({start})[^{{}}]*?{end}").as_str())
        .multi_line(true)
        .build()?;

    let first_value = format!("{start}{end}");

    let updated = re.replace(readme, &first_value);

    let update = format!("{start}\n\n{value}\n\n{end}");

    Ok(updated.replace(&first_value, &update))
}

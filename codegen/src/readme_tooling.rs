use std::str::FromStr;

use anyhow::{Ok, Result};
use regex::RegexBuilder;

use crate::cargo::get_package_version;

#[derive(serde::Deserialize)]
struct Tool {
    description: String,
    r#enum: Vec<String>,
}

#[derive(serde::Deserialize)]
struct SchemaTooling {
    #[serde(rename = "oneOf")]
    one_of: Vec<Tool>,
}

#[derive(serde::Deserialize)]
struct OpenApiSchemaDefinitions {
    #[serde(rename = "Tooling")]
    tooling: SchemaTooling,
}

#[derive(serde::Deserialize)]
struct OpenApiSchema {
    definitions: OpenApiSchemaDefinitions,
}

fn load_schema(package_version: &str) -> Result<OpenApiSchema> {
    let p =
        std::path::PathBuf::from_str(&format!("./schemas/v{package_version}/mdsf.schema.json"))?;

    let file = std::fs::read_to_string(p)?;

    serde_json::from_str::<OpenApiSchema>(&file).map_err(anyhow::Error::from)
}

fn create_table(schema: Vec<Tool>) -> String {
    let formatter_heading = "Formatter";
    let mut formatter_width = formatter_heading.len();

    let description_heading = "Description";
    let mut description_width = description_heading.len();

    let mut tools = std::collections::HashMap::<String, String>::new();

    for tool in schema {
        let formatter = tool.r#enum.first().unwrap();

        formatter_width = formatter_width.max(formatter.len());

        let description = tool.description.trim().to_string();

        description_width = description_width.max(description.len());

        tools.insert(formatter.to_owned(), description);
    }

    let tool_count = tools.len();

    let mut lines = Vec::new();

    for (mut key, mut value) in tools {
        while key.len() < formatter_width {
            key.push(' ');
        }

        while value.len() < description_width {
            value.push(' ');
        }

        lines.push(format!("| {key} | {value} |"));
    }

    lines.sort_unstable();

    let mut filler = "| ".to_string();

    for _ in 0..formatter_width {
        filler.push('-');
    }

    filler.push_str(" | ");

    for _ in 0..description_width {
        filler.push('-');
    }

    filler.push_str(" |");

    lines.insert(0, filler);

    let mut heading = format!("| {formatter_heading}");

    for _ in 0..formatter_width - formatter_heading.len() {
        heading.push(' ');
    }

    heading.push_str(&format!(" | {description_heading}"));

    for _ in 0..description_width - description_heading.len() {
        heading.push(' ');
    }

    heading.push_str(" |");

    lines.insert(0, heading);

    lines.insert(
        0,
        format!("`mdsf` currently supports {tool_count} tools. Feel free to open an issue/pull-request if your favorite tool is missing! 😃\n"),
    );

    lines.join("\n")
}

pub fn update_readme(key: &str, value: &str) -> Result<()> {
    let readme = std::fs::read_to_string("./README.md")?;

    let update = format!("<!-- START_SECTION:{key} -->\n\n{value}\n\n<!-- END_SECTION:{key} -->");

    let re = RegexBuilder::new(
        format!(r"(<!-- START_SECTION:{key} -->)[^{{}}]*<!-- END_SECTION:{key} -->",).as_str(),
    )
    .multi_line(true)
    .build()?;

    let updated = re.replace(&readme, update);

    std::fs::write("./README.md", updated.to_string())?;

    Ok(())
}

pub fn generate() -> Result<()> {
    println!("generate readme tooling");

    let package_version = get_package_version()?;

    let schema = load_schema(&package_version)?;

    let table = create_table(schema.definitions.tooling.one_of);

    update_readme("supported-tools", &table)?;

    Ok(())
}

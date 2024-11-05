use regex::RegexBuilder;

use crate::tools::{GeneratedCommand, Tool};

mod command_help;
mod command_table;

pub fn pad_right(mut input: String, len: usize, update: char) -> String {
    while input.len() < len {
        input.push(update);
    }

    input
}

pub fn update_readme(readme: &str, key: &str, value: &str) -> Result<String, regex::Error> {
    let update = format!("<!-- START_SECTION:{key} -->\n\n{value}\n\n<!-- END_SECTION:{key} -->");

    let re = RegexBuilder::new(
        format!(r"(<!-- START_SECTION:{key} -->)[^{{}}]*?<!-- END_SECTION:{key} -->").as_str(),
    )
    .multi_line(true)
    .build()?;

    let updated = re.replace(&readme, update);

    Ok(updated.to_string())
}

pub fn generate(_plugins: Vec<Tool>, commands: Vec<GeneratedCommand>) -> anyhow::Result<()> {
    let mut readme = std::fs::read_to_string("./README.md")?;

    {
        let content = command_table::generate_command_table(commands);

        readme = update_readme(&readme, "supported-tools", &content)?;

        readme = readme.replace("PATH", "$PATH");
    };

    {
        readme = command_help::generate(readme)?;
    }

    std::fs::write("./README.md", readme)?;

    Ok(())
}

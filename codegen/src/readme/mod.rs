use regex::RegexBuilder;

use crate::tools::{GeneratedCommand, Tool};

mod command_help;
mod command_table;
mod table_of_contents;
mod tool_table;

pub fn pad_right(mut input: String, len: usize, update: char) -> String {
    while input.len() < len {
        input.push(update);
    }

    input
}

pub fn update_readme(readme: &str, key: &str, value: &str) -> Result<String, regex::Error> {
    let start = format!("<!-- START_SECTION:{key} -->");
    let end = format!("<!-- END_SECTION:{key} -->");

    let re = RegexBuilder::new(format!(r"({start})[^{{}}]*?{end}").as_str())
        .multi_line(true)
        .build()?;

    let first_value = format!("{start}{end}");

    let updated = re.replace(readme, &first_value);

    let update = format!("{start}\n\n{value}\n\n{end}");

    Ok(updated.replace(&first_value, &update))
}

pub fn generate(plugins: &[Tool], commands: Vec<GeneratedCommand>) -> anyhow::Result<()> {
    let mut readme = std::fs::read_to_string("./README.md")?;

    {
        let content = command_table::generate_command_table(commands);

        readme = update_readme(&readme, "supported-commands", &content)?;
    };

    {
        readme = command_help::generate(readme)?;
    }

    {
        let tool_table = tool_table::generate(plugins);

        readme = update_readme(&readme, "supported-tools", &tool_table)?;
    }

    std::fs::write("./README.md", &readme)?;

    {
        let t = table_of_contents::generate()?;

        readme = update_readme(&readme, "toc", &t)?;
    }

    std::fs::write("./README.md", readme)?;

    Ok(())
}

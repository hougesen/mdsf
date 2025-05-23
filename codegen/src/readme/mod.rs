use crate::{
    error::CodegenError,
    markdown::{table_of_contents, update_markdown_section},
    tools::{GeneratedCommand, Tool},
};

mod command_help;
mod command_table;
mod tool_table;

pub fn pad_right(mut input: String, len: usize, update: char) -> String {
    while input.len() < len {
        input.push(update);
    }

    input
}

pub fn generate(plugins: &[Tool], commands: Vec<GeneratedCommand>) -> Result<(), CodegenError> {
    let path = std::path::Path::new("./README.md");

    let mut contents = std::fs::read_to_string(path)?;

    {
        let content = command_table::generate_command_table(commands);

        contents = update_markdown_section(&contents, "supported-commands", &content);
    };

    {
        contents = command_help::generate(contents)?;
    }

    {
        let tool_table = tool_table::generate(plugins);

        contents = update_markdown_section(&contents, "supported-tools", &tool_table);
    }

    std::fs::write(path, &contents)?;

    {
        let t = table_of_contents::generate(path)?;

        contents = update_markdown_section(&contents, "toc", &t);
    }

    let mut contents = contents.trim().to_owned();
    contents.push('\n');

    std::fs::write(path, contents)?;

    Ok(())
}

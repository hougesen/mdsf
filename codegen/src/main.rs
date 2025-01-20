use anyhow::{Ok, Result};
use tools::Tool;

mod cargo;
mod language_to_filetype;
mod readme;
mod schema;
mod tools;

fn get_plugin_files() -> Vec<Tool> {
    let tool_folder = "tools";

    let _ = std::fs::create_dir_all(tool_folder);

    let walker = ignore::WalkBuilder::new(tool_folder).build().flatten();

    walker
        .filter_map(|entry| {
            if entry.file_name() == "plugin.json" {
                println!("{}", entry.path().display());

                let content = std::fs::read_to_string(entry.path()).unwrap();

                let plugin = serde_json::from_str::<Tool>(&content).unwrap();

                Some(plugin)
            } else {
                None
            }
        })
        .collect()
}

fn main() -> Result<()> {
    let plugins = get_plugin_files();

    let generated_commands = tools::generate(&plugins)?;

    schema::generate()?;

    language_to_filetype::generate()?;

    readme::generate(plugins, generated_commands)?;

    Ok(())
}

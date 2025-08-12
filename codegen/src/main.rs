use error::CodegenError;
use tools::Tool;

mod actions;
mod contributing;
mod error;
mod language_to_filetype;
pub mod markdown;
mod readme;
mod schema;
mod tools;

pub const GENERATED_FILE_COMMENT: &str =
    "//!\n//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY\n//!";

fn normalize_homepage(s: String) -> String {
    if s.starts_with("https://github.com/") || s.starts_with("https://gitlab.com/") {
        s.to_lowercase()
    } else {
        s
    }
}

fn normalize_description(s: &str) -> String {
    s.trim().to_string()
}

fn normalize_plugin(mut plugin: Tool) -> Tool {
    plugin.homepage = normalize_homepage(plugin.homepage);
    plugin.description = normalize_description(&plugin.description);

    for info in plugin.commands.values_mut() {
        info.homepage = info.homepage.clone().map(normalize_homepage);
        info.description = info.description.as_ref().map(|d| normalize_description(d));

        info.tests.sort_by(|a, b| {
            if a.language != b.language {
                a.language.cmp(&b.language)
            } else if a.test_input != b.test_input {
                a.test_input.cmp(&b.test_input)
            } else if a.test_output != b.test_output {
                a.test_output.cmp(&b.test_output)
            } else if a.disabled {
                core::cmp::Ordering::Greater
            } else if b.disabled {
                core::cmp::Ordering::Less
            } else {
                core::cmp::Ordering::Equal
            }
        });
    }

    plugin.languages = plugin
        .languages
        .into_iter()
        .map(|l| l.trim().to_lowercase())
        .collect();

    plugin.categories = plugin
        .categories
        .into_iter()
        .map(|l| l.trim().to_lowercase())
        .collect();

    plugin
}

fn get_plugin_files() -> Vec<Tool> {
    let tool_folder = "tools";

    let _ = std::fs::create_dir_all(tool_folder);

    let walker = ignore::WalkBuilder::new(tool_folder).build().flatten();

    let mut tools = walker
        .filter_map(|entry| {
            if entry.file_name() == "plugin.json" {
                println!("{}", entry.path().display());

                let content = std::fs::read_to_string(entry.path()).unwrap();

                let plugin = normalize_plugin(serde_json::from_str::<Tool>(&content).unwrap());

                std::fs::write(entry.path(), serde_json::to_string_pretty(&plugin).unwrap())
                    .unwrap();

                Some(plugin)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    tools.sort_unstable_by(|a, b| a.binary.cmp(&b.binary));

    tools
}

fn main() -> Result<(), CodegenError> {
    let plugins = get_plugin_files();

    let generated_commands = tools::generate(&plugins)?;

    schema::generate()?;

    language_to_filetype::generate()?;

    readme::generate(&plugins, generated_commands)?;

    actions::generate(&plugins)?;

    contributing::generate()?;

    Ok(())
}

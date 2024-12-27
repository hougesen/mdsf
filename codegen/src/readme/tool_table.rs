use super::pad_right;
use crate::tools::Tool;

pub fn generate(plugins: &[Tool]) -> String {
    let tool_count = plugins.len();

    let mut rows = Vec::new();

    let name_header = "Name";
    let mut name_row_length = name_header.len();

    let description_header = "Description";
    let mut description_row_length = description_header.len();

    let category_header = "Categories";
    let mut category_row_length = category_header.len();

    let language_header = "Languages";
    let mut language_row_length = language_header.len();

    for plugin in plugins {
        let name = plugin
            .name
            .as_ref()
            .unwrap_or(&plugin.binary)
            .trim()
            .to_owned();
        let name = if plugin.homepage.is_empty() {
            name
        } else {
            format!("[{}]({})", name, plugin.homepage)
        };
        name_row_length = name_row_length.max(name.len());

        let description = plugin.description.trim().to_owned();
        description_row_length = description_row_length.max(description.len());

        let mut categories = plugin
            .categories
            .iter()
            .map(|s| format!("`{}`", s.trim()))
            .collect::<Vec<_>>();
        categories.sort_unstable();
        let categories = categories.join(", ");
        category_row_length = category_row_length.max(categories.len());

        let mut languages = plugin
            .languages
            .iter()
            .map(|s| format!("`{}`", s.trim()))
            .collect::<Vec<_>>();
        languages.sort_unstable();
        let languages = languages.join(", ");
        language_row_length = language_row_length.max(languages.len());

        rows.push((name, description, categories, languages));
    }

    let mut lines = Vec::new();

    for (name, description, categories, languages) in rows {
        lines.push(format!(
            "| {} | {} | {} | {} |",
            pad_right(name, name_row_length, ' '),
            pad_right(description, description_row_length, ' '),
            pad_right(categories, category_row_length, ' '),
            pad_right(languages, language_row_length, ' '),
        ));
    }

    lines.sort_unstable();

    lines.insert(
        0,
        format!(
            "| {} | {} | {} | {} |",
            pad_right("-".to_string(), name_row_length, '-'),
            pad_right("-".to_string(), description_row_length, '-'),
            pad_right("-".to_string(), category_row_length, '-'),
            pad_right("-".to_string(), language_row_length, '-'),
        ),
    );

    lines.insert(
        0,
        format!(
            "| {} | {} | {} | {} |",
            pad_right(name_header.to_string(), name_row_length, ' '),
            pad_right(description_header.to_string(), description_row_length, ' '),
            pad_right(category_header.to_string(), category_row_length, ' '),
            pad_right(language_header.to_string(), language_row_length, ' '),
        ),
    );

    lines.insert(0, String::new());

    lines.insert(
        0,
        format!("`mdsf` currently supports {tool_count} tools. Feel free to open an issue/pull-request if your favorite tool/command is missing! 😃" ),
    );

    lines.join("\n")
}

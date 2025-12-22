mod error;
mod tools;

#[derive(serde::Deserialize, Debug)]
pub struct ToolFileCommand {
    #[serde(default)]
    pub homepage: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub stdin: bool,

    pub arguments: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ToolFile {
    pub binary: String,

    pub description: String,

    pub languages: Vec<String>,

    pub homepage: String,

    pub categories: Vec<String>,

    pub commands: std::collections::BTreeMap<String, ToolFileCommand>,

    #[serde(default)]
    pub name: String,
}

fn find_tools() -> Result<Vec<ToolFile>, crate::error::Error> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir("tools")? {
        let entry = entry?;

        let path = entry.path();

        if path.is_dir() {
            let f = std::fs::read_to_string(path.join("plugin.json"))?;

            files.push(serde_json::from_str::<ToolFile>(&f)?);
        }
    }

    files.sort_unstable_by(|a, b| a.binary.cmp(&b.binary));

    Ok(files)
}

fn setup_liquid_parser() -> Result<liquid::Parser, liquid::Error> {
    liquid::ParserBuilder::with_stdlib().build()
}

fn main() -> Result<(), crate::error::Error> {
    let parser = setup_liquid_parser()?;

    let path = std::path::Path::new("docs/www");

    let _ = std::fs::remove_dir_all(path);
    let _ = std::fs::create_dir_all(path);

    let tools = find_tools()?;

    let tools_dir = path.join("tools");

    let _ = std::fs::create_dir_all(&tools_dir);
    std::fs::write(
        tools_dir.join("index.html"),
        tools::generate_tool_list(&parser, &tools)?,
    )?;

    for tool in &tools {
        let tool_dir = tools_dir.join(if tool.name.is_empty() {
            tool.binary.clone()
        } else {
            tool.name.clone()
        });

        let _ = std::fs::create_dir_all(&tool_dir);

        std::fs::write(
            tool_dir.join("index.html"),
            tools::generate_tool_site(&parser, tool)?,
        )?;
    }

    Ok(())
}

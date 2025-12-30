mod tools;

#[derive(serde::Deserialize, Debug)]
pub struct ToolFileCommand {
    pub arguments: Vec<String>,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub homepage: String,

    #[serde(default)]
    pub stdin: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct ToolFile {
    pub binary: String,

    pub categories: Vec<String>,

    pub commands: std::collections::BTreeMap<String, ToolFileCommand>,

    pub description: String,

    pub homepage: String,

    pub languages: Vec<String>,

    #[serde(default)]
    pub name: String,
}

fn find_tools() -> anyhow::Result<Vec<ToolFile>> {
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

fn main() -> anyhow::Result<()> {
    let parser = setup_liquid_parser()?;

    let path = std::path::Path::new("docs/www");

    let _ = std::fs::remove_dir_all(path);
    let _ = std::fs::create_dir_all(path);

    let tools = find_tools()?;

    std::fs::write(
        path.join("index.html"),
        tools::generate_tool_list(&parser, &tools)?,
    )?;

    let tools_dir = path.join("tools");
    let _ = std::fs::create_dir_all(&tools_dir);

    for tool in &tools {
        let dir = tools_dir.join(if tool.name.is_empty() {
            tool.binary.clone()
        } else {
            tool.name.clone()
        });

        let _ = std::fs::create_dir_all(&dir);

        std::fs::write(
            dir.join("index.html"),
            tools::generate_tool_site(&parser, tool)?,
        )?;
    }

    Ok(())
}

use crate::ToolFile;

pub fn generate_tool_list(
    parser: &liquid::Parser,
    tools: &[ToolFile],
) -> Result<String, liquid::Error> {
    let tools = tools
        .iter()
        .map(|p| {
            liquid::object!({
                "binary": p.binary,
                "categories": p.categories,
                "description": p.description,
                "languages": p.languages,
                "name": p.name,
                "commands": p.commands.len(),
            })
        })
        .collect::<Vec<_>>();

    let globals = liquid::object!({
        "tools": tools,
    });

    parser
        .parse_file("docs/templates/tools/index.liquid")?
        .render(&globals)
}

pub fn generate_tool_site(
    parser: &liquid::Parser,
    tool: &ToolFile,
) -> Result<String, liquid::Error> {
    let commands = tool
        .commands
        .iter()
        .map(|(k, v)| {
            let name = if k.is_empty() {
                tool.binary.clone()
            } else {
                format!("{}:{k}", tool.binary)
            };

            liquid::object!({
                "arguments": v.arguments,
                "description": v.description,
                "homepage": v.homepage,
                "name": name,
            })
        })
        .collect::<Vec<_>>();

    let globals = liquid::object!({
        "binary": tool.binary,
        "categories": tool.categories,
        "commands": commands,
        "description": tool.description,
        "homepage": tool.homepage,
        "languages": tool.languages,
        "name": tool.name,
    });

    parser
        .parse_file("docs/templates/tools/[slug]/index.liquid")?
        .render(&globals)
}

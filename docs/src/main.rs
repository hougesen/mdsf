mod error;
mod packages;

#[derive(serde::Deserialize, Debug)]
pub struct PluginFileCommand {
    pub arguments: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct PluginFile {
    pub binary: String,

    pub description: String,

    pub languages: Vec<String>,

    pub categories: Vec<String>,

    pub commands: std::collections::BTreeMap<String, PluginFileCommand>,
}

fn find_plugins() -> Result<Vec<PluginFile>, crate::error::Error> {
    let mut files = Vec::new();

    for entry in std::fs::read_dir("tools")? {
        let entry = entry?;

        let path = entry.path();

        if path.is_dir() {
            let f = std::fs::read_to_string(path.join("plugin.json"))?;

            files.push(serde_json::from_str(&f)?);
        }
    }

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

    let plugins = find_plugins()?;

    let _ = std::fs::create_dir_all(path.join("packages"));
    std::fs::write(
        path.join("packages/index.html"),
        packages::generate_package_list(&parser, &plugins)?,
    )?;

    Ok(())
}

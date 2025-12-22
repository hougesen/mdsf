use crate::PluginFile;

pub fn generate_package_list(
    parser: &liquid::Parser,
    plugins: &[PluginFile],
) -> Result<String, liquid::Error> {
    let globals = liquid::object!({
        "plugins": plugins.iter().map(|p| liquid::object!({
            "binary": p.binary.clone(),
            "description": p.description.clone(),
            "categories": p.categories.clone(),
            "languages": p.languages.clone(),
        })).collect::<Vec<_>>()
    });

    parser
        .parse_file("docs/templates/packages.liquid")?
        .render(&globals)
}

pub fn generate_package_site(
    parser: &liquid::Parser,
    plugin: &PluginFile,
) -> Result<String, liquid::Error> {
    let globals = liquid::object!({
        "binary": plugin.binary.clone(),
        "description": plugin.description.clone(),
        "categories": plugin.categories.clone(),
        "languages": plugin.languages.clone(),
    });

    parser
        .parse_file("docs/templates/packages.liquid")?
        .render(&globals)
}

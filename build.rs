#[derive(serde::Deserialize)]
struct LinguishLanguage {
    extensions: Option<Vec<String>>,
    aliases: Option<Vec<String>>,
}

fn get_linguish_languages() -> std::collections::HashMap<String, LinguishLanguage> {
    let url = "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/languages.yml";

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    serde_yaml::from_str::<std::collections::HashMap<String, LinguishLanguage>>(&body).unwrap()
}

const WHITESPACE: &str = "    ";

fn build_mapping(languages: std::collections::HashMap<String, LinguishLanguage>) -> String {
    let mut primary = std::collections::HashMap::<String, String>::new();
    let mut secondary = std::collections::HashMap::<String, String>::new();

    for (language, context) in languages {
        if let Some(extensions) = context.extensions {
            if let Some(extension) = extensions.first() {
                primary.insert(language.trim().to_lowercase(), extension.to_owned());

                if let Some(aliases) = context.aliases {
                    for alias in aliases {
                        secondary.insert(alias.trim().to_lowercase(), extension.to_owned());
                    }
                }
            }
        }
    }

    assert!(!primary.is_empty());
    assert!(!secondary.is_empty());

    let mut mappings: Vec<String> = Vec::new();

    let mut seen_languages = std::collections::HashSet::<String>::new();

    for (language, ext) in primary.into_iter().chain(secondary.into_iter()) {
        if !seen_languages.contains(&language) {
            mappings.push(format!(
                "{WHITESPACE}{WHITESPACE}\"{language}\" => \"{ext}\",",
            ));
            seen_languages.insert(language);
        }
    }

    mappings.sort_unstable();

    mappings.push(format!("{WHITESPACE}{WHITESPACE}_ => \"\","));

    format!(
        "pub fn language_to_ext(language: &str) -> String {{
{WHITESPACE}match language.to_lowercase().as_str() {{
{}
{WHITESPACE}}}
{WHITESPACE}.to_string()
}}
",
        mappings.join("\n")
    )
}

fn main() {
    println!("hello from build.rs");

    let languages = get_linguish_languages();

    let result = build_mapping(languages);

    std::fs::write("src/generated.rs", result).unwrap();
}

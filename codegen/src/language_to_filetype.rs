use crate::{GENERATED_FILE_COMMENT, error::CodegenError};

#[derive(serde::Deserialize)]
struct LinguishLanguage {
    extensions: Option<Vec<String>>,
    aliases: Option<Vec<String>>,
}

fn get_linguish_languages()
-> Result<std::collections::HashMap<String, LinguishLanguage>, CodegenError> {
    let url = "https://raw.githubusercontent.com/github-linguist/linguist/master/lib/linguist/languages.yml";

    let body = ureq::get(url).call()?.body_mut().read_to_string()?;

    serde_yaml::from_str::<std::collections::HashMap<String, LinguishLanguage>>(&body)
        .map_err(CodegenError::from)
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

    let mut mappings: Vec<String> = Vec::new();

    let mut seen_languages = std::collections::HashSet::<String>::new();

    for (language, ext) in primary.into_iter().chain(secondary.into_iter()) {
        #[allow(clippy::set_contains_or_insert)]
        if !seen_languages.contains(&language) {
            mappings.push(format!(
                "{WHITESPACE}{WHITESPACE}\"{language}\" => Some(\"{ext}\"),",
            ));
            seen_languages.insert(language);
        }
    }

    mappings.sort_unstable();

    mappings.push(format!("{WHITESPACE}{WHITESPACE}_ => None,"));

    format!(
        "{GENERATED_FILE_COMMENT}

#[allow(clippy::too_many_lines)]
#[inline]
pub fn language_to_ext(language: &str) -> Option<&'static str> {{
{WHITESPACE}#[allow(clippy::match_same_arms)]
{WHITESPACE}match language.to_lowercase().as_str() {{
{}
{WHITESPACE}}}
}}
",
        mappings.join("\n")
    )
}

pub fn generate() -> Result<(), CodegenError> {
    println!("generate language_to_ft");

    let languages = get_linguish_languages()?;

    let result = build_mapping(languages);

    std::fs::write("./mdsf/src/filetype/generated_file_types.rs", result)?;

    Ok(())
}

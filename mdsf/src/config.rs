use json_comments::{CommentSettings, StripComments};

use crate::{
    error::MdsfError, execution::MdsfFormatter, languages::default_tools,
    runners::JavaScriptRuntime, terminal::print_config_not_found, tools::Tooling,
};

#[inline]
const fn is_false(b: &bool) -> bool {
    !(*b)
}

#[derive(serde::Serialize, serde::Deserialize, Hash)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct MdsfConfig {
    #[serde(rename = "$schema", default = "default_schema_location")]
    pub schema: String,

    /// Used for settings custom file extensions for a given language.
    /// ```json
    /// {
    ///   "custom_file_extensions": {
    ///     "rust": ".rust"
    ///   }
    /// }
    /// ```
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub custom_file_extensions: std::collections::BTreeMap<String, String>,

    /// Format the processed document with the selected markdown formatter.
    #[serde(default, skip_serializing_if = "is_false")]
    pub format_finished_document: bool,

    /// Set npm script runner runtime.
    ///
    /// Should be considered experimental since not all tools support being run using Bun/Deno.
    ///
    /// `node -> npx`
    ///
    /// `bun -> bunx`
    ///
    /// `deno -> deno run`
    #[serde(default, skip_serializing_if = "JavaScriptRuntime::is_default")]
    pub javascript_runtime: JavaScriptRuntime,

    /// Aliases for tools
    ///
    /// ```json
    /// {
    ///   "language_aliases": {
    ///     "language": "is_alias_of"
    ///   }
    /// }
    /// ```
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub language_aliases: std::collections::BTreeMap<String, String>,

    ///  Defines which formatter is used by the language.
    /// ```json
    /// {
    ///   "languages": {
    ///     "rust": "rustfmt",
    ///     "js": "prettier"
    ///   }
    /// }
    /// ```
    #[serde(default)]
    pub languages: std::collections::BTreeMap<String, MdsfFormatter<Tooling>>,
}

impl Default for MdsfConfig {
    #[inline]
    fn default() -> Self {
        Self {
            schema: default_schema_location(),
            custom_file_extensions: std::collections::BTreeMap::default(),
            format_finished_document: false,
            javascript_runtime: JavaScriptRuntime::default(),
            language_aliases: std::collections::BTreeMap::default(),
            languages: default_tools(),
        }
    }
}

impl MdsfConfig {
    #[inline]
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, MdsfError> {
        match std::fs::read_to_string(path.as_ref()) {
            Ok(raw_config) => Self::parse(&raw_config)
                .map_err(|_serde_error| MdsfError::ConfigParse(path.as_ref().to_path_buf())),
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound {
                    print_config_not_found(path.as_ref());
                }

                Err(MdsfError::from(error))
            }
        }
    }

    #[inline]
    pub fn parse(input: &str) -> serde_json::Result<Self> {
        let stripped = StripComments::with_settings(CommentSettings::c_style(), input.as_bytes());

        serde_json::from_reader(stripped)
    }

    #[inline]
    pub fn setup_language_aliases(&mut self) -> Result<(), MdsfError> {
        // TODO: reduce the amount of cloning

        if !self.language_aliases.is_empty() {
            let mut seen_languages: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();

            for (language, alias) in &self.language_aliases {
                if let Some(already_set_by) = seen_languages.get(language) {
                    return Err(MdsfError::LanguageAliasClash(
                        language.to_owned(),
                        alias.to_owned(),
                        already_set_by.to_owned(),
                    ));
                }

                if self.languages.contains_key(language) {
                    return Err(MdsfError::LanguageAliasLanguagesContainsLanguage(
                        language.to_owned(),
                    ));
                }

                if let Some(tools) = self.languages.get(alias) {
                    self.languages.insert(language.to_owned(), tools.clone());

                    seen_languages.insert(language.to_owned(), alias.to_owned());
                } else {
                    return Err(MdsfError::LanguageAliasMissingTools(alias.to_owned()));
                }
            }
        }

        Ok(())
    }
}

#[inline]
fn default_schema_location() -> String {
    let package_version = env!("CARGO_PKG_VERSION");

    format!(
        "https://raw.githubusercontent.com/hougesen/mdsf/main/schemas/v{package_version}/mdsf.schema.json"
    )
}

#[cfg(test)]
mod test_config {
    use super::MdsfConfig;

    #[test]
    fn schema_should_be_serializable() {
        let config = MdsfConfig::default();

        let json = serde_json::to_string_pretty(&config).expect("it to be serializable");

        let loaded = serde_json::from_str::<MdsfConfig>(&json).expect("it to be parsed");

        assert_eq!(config, loaded);
    }

    #[test]
    #[cfg(feature = "json-schema")]
    fn json_schema_should_be_serializable() {
        serde_json::to_string_pretty(&schemars::schema_for!(MdsfConfig))
            .expect("it to be serializable");
    }

    #[test]
    fn it_should_ignore_comments() {
        let r = r#"{
    // this is a slash comment
    "javascript":  ["prettier"],
    "rust": "rustfmt",
    /* this is a multiline comment
    "roc": {
        "enabled": false
    }
    */
    "go": "gofmt"         // hello world

}"#;

        MdsfConfig::parse(r).expect("it to parse the config");
    }
}

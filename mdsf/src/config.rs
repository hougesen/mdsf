use json_comments::{CommentSettings, StripComments};

use crate::{
    cli::{OnMissingLanguageDefinition, OnMissingToolBinary},
    error::MdsfError,
    execution::MdsfFormatter,
    languages::default_tools,
    tools::Tooling,
};

#[allow(clippy::trivially_copy_pass_by_ref)]
#[inline]
const fn is_false(b: &bool) -> bool {
    !(*b)
}

#[allow(clippy::struct_excessive_bools)]
#[derive(serde::Serialize, serde::Deserialize, Hash, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct MdsfConfigRunners {
    /// Whether to support running npm packages using `bunx $PACKAGE_NAME`
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub bunx: bool,

    /// Whether to support running npm packages using `deno run -A npm:$PACKAGE_NAME`
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub deno: bool,

    /// Whether to support running dub packages using `dotnet $PACKAGE_NAME`
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub dotnet: bool,

    /// Whether to support running dub packages using `dub run $PACKAGE_NAME`
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub dub: bool,

    /// Whether to support running ruby packages using `gem exec $PACKAGE_NAME`
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub gem_exec: bool,

    /// Whether to support running npm packages using `npx $PACKAGE_NAME`
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub npx: bool,

    /// Whether to support running pypi packages using `pipx run $PACKAGE_NAME`
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub pipx: bool,

    /// Whether to support running npm packages using `pnpm dlx $PACKAGE_NAME`
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub pnpm: bool,

    /// Whether to support running pypi packages using `uv tool run $PACKAGE_NAME`
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub uv: bool,

    /// Whether to support running npm packages using `yarn dlx $PACKAGE_NAME`
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub yarn: bool,
}

impl MdsfConfigRunners {
    #[inline]
    fn is_default(&self) -> bool {
        *self == Self::default()
    }

    #[inline]
    pub const fn all() -> Self {
        Self {
            bunx: true,
            deno: true,
            dotnet: true,
            dub: true,
            gem_exec: true,
            npx: true,
            pipx: true,
            pnpm: true,
            uv: true,
            yarn: true,
        }
    }
}

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize, Hash, Default,
)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum Newline {
    #[default]
    #[serde(rename = "lf")]
    Lf,
    #[serde(rename = "cr")]
    Cr,
    #[serde(rename = "crlf")]
    CrLf,
}

pub const LF_NEWLINE_CHAR: char = '\n';

impl Newline {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Lf => "\n",
            Self::Cr => "\r",
            Self::CrLf => "\r\n",
        }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[inline]
    fn is_default(&self) -> bool {
        *self == Self::default()
    }

    #[inline]
    pub fn normalize(self, input: String) -> String {
        // We could most likely optimize this, but I am not sure if the added complexity is worth it

        if self == Self::Lf && !input.contains('\r') {
            input
        } else {
            input.lines().collect::<Vec<_>>().join(self.as_str())
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Hash, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct MdsfConfig {
    #[serde(rename = "$schema", default = "default_schema_location")]
    pub schema: String,

    /// Used for settings custom file extensions for a given language.
    ///
    /// ```json
    /// {
    ///   "custom_file_extensions": {
    ///     "rust": ".rust"
    ///   }
    /// }
    /// ```
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub custom_file_extensions: std::collections::BTreeMap<String, String>,

    /// Run the selected markdown tools on the finished output.
    ///
    /// Default: `false`
    #[serde(default, skip_serializing_if = "is_false")]
    pub format_finished_document: bool,

    /// Aliases for tools.
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

    ///  Defines which tools are used by the language.
    ///
    /// ```json
    /// {
    ///     "languages": {
    ///       // Only run `ruff` on Python snippets,
    ///       "python": "ruff:format",
    ///       // Run `usort` on file and then `black`
    ///       "python": ["usort", "black"],
    ///       // Run `usort`, if that fails run `isort`, finally run `black`
    ///       "python": [["usort", "isort"], "black"],
    ///
    ///       // Tools listed under "*" will be run on any snippet.
    ///       "*": ["typos"],
    ///
    ///       // Tools  listed under "_" will only be run when there is not tool configured for the file type OR globally ("*").
    ///       "_": "prettier"
    ///     }
    /// }
    /// ```
    #[serde(default)]
    pub languages: std::collections::BTreeMap<String, MdsfFormatter<Tooling>>,

    /// The newline used for the output.
    ///
    /// Default: `lf`
    #[serde(default, skip_serializing_if = "Newline::is_default")]
    pub newline: Newline,

    /// What to do when a codeblock language has no tools defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_missing_language_definition: Option<OnMissingLanguageDefinition>,

    /// What to do when the binary of a tool cannot be found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_missing_tool_binary: Option<OnMissingToolBinary>,

    /// List of package registry script runners that should be enabled.
    ///
    /// Should be considered experimental since not all tools support being run that way.
    #[serde(default, skip_serializing_if = "MdsfConfigRunners::is_default")]
    pub runners: MdsfConfigRunners,
}

impl Default for MdsfConfig {
    #[inline]
    fn default() -> Self {
        Self {
            schema: default_schema_location(),
            custom_file_extensions: std::collections::BTreeMap::default(),
            format_finished_document: false,
            language_aliases: std::collections::BTreeMap::default(),
            languages: default_tools(),
            newline: Newline::default(),
            on_missing_language_definition: None,
            on_missing_tool_binary: None,
            runners: MdsfConfigRunners::default(),
        }
    }
}

impl MdsfConfig {
    #[inline]
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, MdsfError> {
        let path = path.as_ref();

        match std::fs::read_to_string(path) {
            Ok(raw_config) => Self::parse(&raw_config)
                .map_err(|_serde_error| MdsfError::ConfigParse(path.to_path_buf())),
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound {
                    return Err(MdsfError::ConfigNotFound(path.to_path_buf()));
                }

                Err(MdsfError::Io(error))
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

                let tools = self
                    .languages
                    .get(alias)
                    .ok_or_else(|| MdsfError::LanguageAliasMissingTools(alias.to_owned()))?;

                self.languages.insert(language.to_owned(), tools.clone());

                seen_languages.insert(language.to_owned(), alias.to_owned());
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
    use crate::{error::MdsfError, execution::setup_snippet};

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

    #[test]
    fn test_config_load_works() -> Result<(), Box<dyn core::error::Error>> {
        let f = tempfile::Builder::new().rand_bytes(24).tempfile()?;

        let default_config = MdsfConfig::default();

        std::fs::write(f.path(), serde_json::to_string(&default_config)?)?;

        let loaded = MdsfConfig::load(f.path()).expect("it to return the config");

        assert_eq!(default_config, loaded);

        Ok(())
    }

    #[test]
    fn test_config_load_return_error_if_not_found() {
        let before = MdsfConfig::load(std::path::Path::new(
            "ifthispathexiststhereissomethingwrong",
        ))
        .expect_err("Expect it to return file not found");

        assert!(matches!(before, MdsfError::ConfigNotFound(_)));
    }

    #[test]
    fn it_should_error_on_broken_config() -> std::io::Result<()> {
        let input = "{thisisnotvalidjson}";

        let file = setup_snippet(input, ".json")?;

        let output = MdsfConfig::load(file.path()).expect_err("it should return an error");

        assert!(matches!(output, MdsfError::ConfigParse(_)));

        Ok(())
    }
}

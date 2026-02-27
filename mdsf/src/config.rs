use crate::{config::files::MdsfConfigFiles, error::MdsfError, terminal::print_config_not_found};

mod files;

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

#[derive(Debug, serde::Serialize, serde::Deserialize, Hash, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum MdsfTool {
    Preset(crate::tools::Tooling),

    Custom(crate::custom::CustomTool),
}

impl core::fmt::Display for MdsfTool {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Preset(t) => t.fmt(f),
            Self::Custom(t) => t.fmt(f),
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

    #[serde(default, skip_serializing_if = "MdsfConfigFiles::is_default")]
    pub files: MdsfConfigFiles,

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
    pub languages: std::collections::BTreeMap<String, crate::execution::MdsfToolWrapper<MdsfTool>>,

    /// The newline used for the output.
    ///
    /// Default: `lf`
    #[serde(default, skip_serializing_if = "Newline::is_default")]
    pub newline: Newline,

    /// What to do when a codeblock language has no tools defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_missing_language_definition: Option<crate::cli::OnMissingLanguageDefinition>,

    /// What to do when the binary of a tool cannot be found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_missing_tool_binary: Option<crate::cli::OnMissingToolBinary>,

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
            files: MdsfConfigFiles::default(),
            format_finished_document: false,
            language_aliases: std::collections::BTreeMap::default(),
            languages: crate::languages::default_tools(),
            newline: Newline::default(),
            on_missing_language_definition: None,
            on_missing_tool_binary: None,
            runners: MdsfConfigRunners::default(),
        }
    }
}

impl MdsfConfig {
    #[inline]
    pub fn auto_load() -> Result<Self, MdsfError> {
        let dir = std::env::current_dir()?;

        for name in [
            "mdsf.json",
            ".mdsf.json",
            "mdsf.jsonc",
            ".mdsf.jsonc",
            "mdsf.json5",
            ".mdsf.json5",
            "mdsf.toml",
            ".mdsf.toml",
        ] {
            let path = dir.join(name);

            let c = Self::load(path);

            if let Err(MdsfError::ConfigNotFound(_)) = c {
                continue;
            }

            return c;
        }

        print_config_not_found(&dir);

        Ok(Self::default())
    }

    #[inline]
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Self, MdsfError> {
        let path = path.as_ref();

        let contents = std::fs::read_to_string(path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                MdsfError::ConfigNotFound(path.to_path_buf())
            } else {
                MdsfError::Io(error)
            }
        })?;

        Self::parse(&contents, path)
    }

    #[inline]
    fn parse(input: &str, path: &std::path::Path) -> Result<Self, MdsfError> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json" | "jsonc" | "json5") => Self::parse_json(input)
                .map_err(|err| MdsfError::ConfigParseJson((path.to_path_buf(), err))),
            Some("toml") => Self::parse_toml(input)
                .map_err(|err| MdsfError::ConfigParseToml((path.to_path_buf(), err))),
            _ => Self::parse_json(input)
                .map_err(|_| Self::parse_toml(input))
                .map_err(|_| MdsfError::ConfigParseUnknownFormat(path.to_path_buf())),
        }
    }

    #[inline]
    fn parse_json(input: &str) -> Result<Self, json5::Error> {
        json5::from_str(input)
    }

    #[inline]
    fn parse_toml(input: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(input)
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

    #[inline]
    pub fn parse_schema_version(&self) -> Option<(&str, bool)> {
        let package_version = env!("CARGO_PKG_VERSION");

        if self.schema == default_schema_location() {
            return Some((package_version, true));
        }

        let start = "https://raw.githubusercontent.com/hougesen/mdsf/main/schemas/";
        let end = "/mdsf.schema.json";

        // TODO: make this pretty
        if self.schema.starts_with(start)
            && self.schema.ends_with(end)
            && let Some((_, remaining)) = self.schema.split_once(start)
            && !remaining.is_empty()
            && let Some((version, _)) = remaining.rsplit_once(end)
            && !version.is_empty()
        {
            return Some((version, version == package_version));
        }

        None
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
    fn schema_should_be_serializable() -> Result<(), serde_json::Error> {
        let config = MdsfConfig::default();

        let json = serde_json::to_string_pretty(&config)?;

        let loaded = serde_json::from_str::<MdsfConfig>(&json)?;

        assert_eq!(config, loaded);

        Ok(())
    }

    #[test]
    #[cfg(feature = "json-schema")]
    fn json_schema_should_be_serializable() -> Result<(), serde_json::Error> {
        serde_json::to_string_pretty(&schemars::schema_for!(MdsfConfig))?;

        Ok(())
    }

    #[test]
    fn it_should_ignore_comments() -> Result<(), json5::Error> {
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

        MdsfConfig::parse_json(r)?;

        Ok(())
    }

    #[test]
    fn test_config_load_works() -> Result<(), Box<dyn core::error::Error>> {
        let f = tempfile::Builder::new().rand_bytes(24).tempfile()?;

        let default_config = MdsfConfig::default();

        std::fs::write(f.path(), serde_json::to_string(&default_config)?)?;

        let loaded = MdsfConfig::load(f.path())?;

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
    fn it_should_error_on_broken_config() -> Result<(), std::io::Error> {
        let input = "{thisisnotvalidjson}";

        let file = setup_snippet(input, ".json")?;

        let output = MdsfConfig::load(file.path()).expect_err("it should return an error");

        assert!(matches!(output, MdsfError::ConfigParseJson((_, _))));

        Ok(())
    }
}

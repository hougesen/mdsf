use json_comments::{CommentSettings, StripComments};
use schemars::JsonSchema;

use crate::{
    error::MdsfError,
    formatters::{MdsfFormatter, Tooling},
    languages::default_tools,
    runners::JavaScriptRuntime,
    terminal::print_config_not_found,
};

#[derive(serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[schemars(deny_unknown_fields)]
pub struct MdsfConfig {
    #[serde(rename = "$schema", default = "default_schema_location")]
    pub schema: String,

    /// Format the processed document with the selected markdown formatter.
    #[serde(default)]
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
    #[serde(default)]
    pub javascript_runtime: JavaScriptRuntime,

    #[serde(default)]
    pub languages: std::collections::BTreeMap<String, MdsfFormatter<Tooling>>,
}

impl Default for MdsfConfig {
    #[inline]
    fn default() -> Self {
        Self {
            schema: default_schema_location(),
            format_finished_document: false,
            javascript_runtime: JavaScriptRuntime::default(),

            languages: default_tools(),
        }
    }
}

impl MdsfConfig {
    #[inline]
    pub fn load() -> Result<Self, MdsfError> {
        let dir = std::env::current_dir()?;

        let path = dir.join("mdsf.json");

        match std::fs::read_to_string(&path) {
            Ok(raw_config) => {
                Self::parse(&raw_config).map_err(|_serde_error| MdsfError::ConfigParse(path))
            }
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound {
                    print_config_not_found();
                    Ok(Self::default())
                } else {
                    Err(MdsfError::from(error))
                }
            }
        }
    }

    #[inline]
    pub fn parse(input: &str) -> serde_json::Result<Self> {
        let stripped = StripComments::with_settings(CommentSettings::c_style(), input.as_bytes());

        serde_json::from_reader(stripped)
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

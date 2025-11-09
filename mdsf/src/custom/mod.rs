#[derive(serde::Serialize, serde::Deserialize, Hash, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct CustomTool {
    pub binary: String,

    pub arguments: Vec<String>,

    #[serde[default]]
    pub stdin: bool,
}

impl core::fmt::Display for CustomTool {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (custom)", self.binary)
    }
}

impl CustomTool {
    #[inline]
    pub fn tool_name(&self) -> &str {
        &self.binary
    }

    #[inline]
    pub fn format_snippet(
        &self,
        _snippet_path: &std::path::Path,
        _timeout: u64,
        _debug_enabled: bool,
        _config_runners: &crate::config::MdsfConfigRunners,
    ) -> Result<(bool, Option<String>), crate::error::MdsfError> {
        todo!()
    }
}

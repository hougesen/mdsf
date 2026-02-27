#[derive(serde::Serialize, serde::Deserialize, Hash, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct MdsfConfigFiles {
    /// File extensions that should be formatted using `mdsf`
    pub extensions: std::collections::BTreeSet<String>,
}

impl Default for MdsfConfigFiles {
    #[inline]
    fn default() -> Self {
        Self {
            extensions: default_extensions(),
        }
    }
}

impl MdsfConfigFiles {
    #[inline]
    pub(crate) fn is_default(&self) -> bool {
        self.extensions == default_extensions()
    }

    #[inline]
    pub fn is_enabled_file_extension(&self, extension: &str) -> bool {
        self.extensions.contains(extension)
    }
}

#[inline]
fn default_extensions() -> std::collections::BTreeSet<String> {
    std::collections::BTreeSet::from_iter(["md".to_string()])
}

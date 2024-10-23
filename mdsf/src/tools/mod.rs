pub mod alejandra;
pub mod prettier;

#[derive(serde::Serialize, serde::Deserialize, Hash)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum Tooling {
    #[serde(rename = "alejandra")]
    #[doc = "The Uncompromising Nix Code Formatter - [https://github.com/kamadorueda/alejandra](https://github.com/kamadorueda/alejandra)"]
    Alejandra,
    #[serde(rename = "prettier")]
    #[doc = "Prettier is an opinionated code formatter. - [https://github.com/prettier/prettier](https://github.com/prettier/prettier)"]
    Prettier,
}

impl Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    pub fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), crate::error::MdsfError> {
        match self {
            Self::Alejandra => alejandra::run_alejandra(snippet_path),
            Self::Prettier => prettier::run_prettier(snippet_path),
        }
    }
}

impl AsRef<str> for Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    fn as_ref(&self) -> &str {
        match self {
            Self::Alejandra => "alejandra",
            Self::Prettier => "prettier",
        }
    }
}

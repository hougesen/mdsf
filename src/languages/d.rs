use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{dfmt::format_using_dfmt, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum D {
    #[default]
    #[serde(rename = "dfmt")]
    DFmt,
}

impl Default for Lang<D> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<D>::default(),
        }
    }
}

impl Default for MdsfFormatter<D> {
    #[inline]
    fn default() -> Self {
        Self::Single(D::DFmt)
    }
}

impl LanguageFormatter for D {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::DFmt => format_using_dfmt(snippet_path),
        }
    }
}

impl core::fmt::Display for D {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DFmt => write!(f, "dfmt"),
        }
    }
}

#[cfg(test)]
mod test_elm {
    use super::D;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = r#"import   Html       exposing   (text)


main =
      text              "Hello!"


  "#;

    const EXTENSION: &str = crate::languages::Language::D.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<D>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<D> {
            enabled: false,
            formatter: MdsfFormatter::Single(D::default())
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }
}

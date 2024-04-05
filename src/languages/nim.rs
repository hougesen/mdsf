use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{nimpretty::format_using_nimpretty, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Nim {
    #[default]
    #[serde(rename = "nimpretty")]
    Nimpretty,
}

impl Default for Lang<Nim> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Nim>::default(),
        }
    }
}

impl Default for MdsfFormatter<Nim> {
    #[inline]
    fn default() -> Self {
        Self::Single(Nim::Nimpretty)
    }
}

impl LanguageFormatter for Nim {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Nimpretty => format_using_nimpretty(snippet_path),
        }
    }
}

impl core::fmt::Display for Nim {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Nimpretty => write!(f, "nimpretty"),
        }
    }
}

#[cfg(test)]
mod test_nim {
    use super::Nim;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "proc           add( a         :int , b:int )        : int =
  return a +          b  ";

    const EXTENSION: &str = crate::languages::Language::Nim.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Nim>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Nim> {
            enabled: false,
            formatter: MdsfFormatter::Single(Nim::default()),
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(nimpretty)]
    #[test]
    fn test_nimpretty() {
        let l = Lang::<Nim> {
            enabled: true,
            formatter: MdsfFormatter::Single(Nim::Nimpretty),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "proc add(a: int, b: int): int =
  return a + b
";

        assert_eq!(output, expected_output);
    }
}

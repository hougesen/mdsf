use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{gleam_format::format_using_gleam_format, MdsfFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Gleam {
    #[default]
    #[serde(rename = "gleam_format")]
    GleamFormat,
}

impl Default for Lang<Gleam> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Gleam>::default(),
        }
    }
}

impl Default for MdsfFormatter<Gleam> {
    #[inline]
    fn default() -> Self {
        Self::Single(Gleam::GleamFormat)
    }
}

impl LanguageFormatter for Gleam {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::GleamFormat => format_using_gleam_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_gleam {
    use super::Gleam;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    const INPUT: &str = "pub fn add(a:Int,b:Int)->Int{a+b}";

    const EXTENSION: &str = crate::languages::Language::Gleam.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Gleam>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Gleam> {
            enabled: false,
            formatter: MdsfFormatter::Single(Gleam::default())
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(gleam)]
    #[test]
    fn test_gleam_format() {
        let l = Lang::<Gleam> {
            enabled: true,
            formatter: MdsfFormatter::Single(Gleam::GleamFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "pub fn add(a: Int, b: Int) -> Int {
  a + b
}
";

        assert_eq!(output, expected_output);
    }
}

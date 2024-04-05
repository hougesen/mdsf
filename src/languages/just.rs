use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{just_fmt::format_using_just_fmt, MdsfFormatter};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Just {
    #[default]
    #[serde(rename = "just_fmt")]
    JustFmt,
}

impl Default for Lang<Just> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Just>::default(),
        }
    }
}

impl Default for MdsfFormatter<Just> {
    #[inline]
    fn default() -> Self {
        Self::Single(Just::JustFmt)
    }
}

impl LanguageFormatter for Just {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::JustFmt => format_using_just_fmt(snippet_path),
        }
    }
}

impl core::fmt::Display for Just {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::JustFmt => write!(f, "just_fmt"),
        }
    }
}

#[cfg(test)]
mod test_just {
    use super::Just;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "


build:
                cargo build
                cargo build --release
            ";
    const EXTENSION: &str = crate::languages::Language::Just.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Just>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Just> {
            enabled: false,
            formatter: MdsfFormatter::Single(Just::default())
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(just)]
    #[test]
    fn test_just_fmt() {
        let l = Lang::<Just> {
            enabled: true,
            formatter: MdsfFormatter::Single(Just::JustFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "build:
    cargo build
    cargo build --release
";

        assert_eq!(output, expected_output);
    }
}

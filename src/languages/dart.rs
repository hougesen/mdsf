use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{dart_format::format_using_dart_format, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Dart {
    #[default]
    #[serde(rename = "dart_format")]
    DartFormat,
}

impl Default for Lang<Dart> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Dart>::default(),
        }
    }
}

impl Default for MdsfFormatter<Dart> {
    #[inline]
    fn default() -> Self {
        Self::Single(Dart::DartFormat)
    }
}

impl LanguageFormatter for Dart {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::DartFormat => format_using_dart_format(snippet_path),
        }
    }
}

impl core::fmt::Display for Dart {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DartFormat => write!(f, "dart_format"),
        }
    }
}

#[cfg(test)]
mod test_dart {
    use super::Dart;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "class Adder {   int add(int a, int b) {     return a + b;   } }    ";

    const EXTENSION: &str = crate::languages::Language::Dart.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Dart>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Dart> {
            enabled: false,
            formatter: MdsfFormatter::Single(Dart::default(),)
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(dart)]
    #[test]
    fn test_dart_format() {
        let l = Lang::<Dart> {
            enabled: true,
            formatter: MdsfFormatter::Single(Dart::DartFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "class Adder {
  int add(int a, int b) {
    return a + b;
  }
}
";

        assert_eq!(output, expected_output);
    }
}

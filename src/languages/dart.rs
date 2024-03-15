use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{dart_format::format_using_dart_format, format_multiple, MdsfFormatter},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum DartFormatter {
    #[default]
    #[serde(rename = "dart_format")]
    DartFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Dart {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<DartFormatter>,
}

impl Default for Dart {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<DartFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<DartFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(DartFormatter::DartFormat)
    }
}

impl LanguageFormatter<DartFormatter> for Dart {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        format_multiple(&self.formatter, snippet_path, &Self::format_single)
            .map(|(_should_continue, output)| output)
    }

    #[inline]
    fn format_single(
        formatter: &DartFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            DartFormatter::DartFormat => format_using_dart_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_dart {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

    use super::{Dart, DartFormatter};

    const INPUT: &str = "class Adder {   int add(int a, int b) {     return a + b;   } }    ";

    const EXTENSION: &str = crate::languages::Language::Dart.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Dart::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Dart {
            enabled: false,
            formatter: MdsfFormatter::Single(DartFormatter::default(),)
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_dart_format() {
        let l = Dart {
            enabled: true,
            formatter: MdsfFormatter::Single(DartFormatter::DartFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
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

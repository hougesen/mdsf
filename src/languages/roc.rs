use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::roc_format::format_using_roc_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum RocFormatter {
    #[default]
    #[serde(rename = "roc_format")]
    RocFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Roc {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: RocFormatter,
}

impl Default for Roc {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: RocFormatter::default(),
        }
    }
}

impl LanguageFormatter for Roc {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            RocFormatter::RocFormat => format_using_roc_format(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test_roc {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Roc, RocFormatter};

    const INPUT: &str = r#"app "helloWorld"
    packages { pf: "https://github.com/roc-lang/" }
    imports [pf.Stdout]
    provides [main] to pf






main =
    Stdout.line "Hello, World!"


    "#;

    const EXTENSION: &str = crate::languages::Language::Roc.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Roc::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Roc {
            enabled: false,
            formatter: RocFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_roc_format() {
        let l = Roc {
            enabled: true,
            formatter: RocFormatter::RocFormat,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = r#"app "helloWorld"
    packages { pf: "https://github.com/roc-lang/" }
    imports [pf.Stdout]
    provides [main] to pf

main =
    Stdout.line "Hello, World!"

"#;

        assert_eq!(output, expected_output);
    }
}

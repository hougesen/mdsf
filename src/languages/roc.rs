use schemars::JsonSchema;

use crate::formatters::{roc_format::format_using_roc_format, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Roc {
    #[default]
    #[serde(rename = "roc_format")]
    RocFormat,
}

impl Default for Lang<Roc> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Roc>::default(),
        }
    }
}

impl Default for MdsfFormatter<Roc> {
    #[inline]
    fn default() -> Self {
        Self::Single(Roc::RocFormat)
    }
}

impl LanguageFormatter for Roc {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::RocFormat => format_using_roc_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_roc {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Roc;

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
        assert!(Lang::<Roc>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Roc> {
            enabled: false,
            formatter: MdsfFormatter::Single(Roc::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(roc)]
    #[test]
    fn test_roc_format() {
        let l = Lang::<Roc> {
            enabled: true,
            formatter: MdsfFormatter::Single(Roc::RocFormat),
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

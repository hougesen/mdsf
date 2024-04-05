use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{roc_format::format_using_roc_format, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
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
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::RocFormat => format_using_roc_format(snippet_path),
        }
    }
}

impl core::fmt::Display for Roc {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RocFormat => write!(f, "roc_format"),
        }
    }
}

#[cfg(test)]
mod test_roc {
    use super::Roc;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

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
        .format(snippet_path, &LineInfo::fake())
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
            .format(snippet_path, &LineInfo::fake())
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

use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{ktfmt::format_using_ktfmt, ktlint::format_using_ktlint, MdsfFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Kotlin {
    #[default]
    #[serde(rename = "ktlint")]
    Ktlint,
    #[serde(rename = "ktfmt")]
    Ktfmt,
}

impl Default for Lang<Kotlin> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Kotlin>::default(),
        }
    }
}

impl Default for MdsfFormatter<Kotlin> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Kotlin::Ktlint),
            Self::Single(Kotlin::Ktfmt),
        ])])
    }
}

impl LanguageFormatter for Kotlin {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Ktlint => format_using_ktlint(snippet_path),
            Self::Ktfmt => format_using_ktfmt(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_kotlin {
    use super::Kotlin;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    const INPUT: &str = "  fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            ";

    const EXTENSION: &str = crate::languages::Language::Kotlin.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Kotlin>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Kotlin> {
            enabled: false,
            formatter: MdsfFormatter::Single(Kotlin::default())
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(ktlint)]
    #[test]
    fn test_ktlint() {
        let l = Lang::<Kotlin> {
            enabled: true,
            formatter: MdsfFormatter::Single(Kotlin::Ktlint),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "

fun add(
    a: Int,
    b: Int,
): Int {
    return a + b
}
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(ktfmt)]
    #[test]
    fn test_ktfmt() {
        let l = Lang::<Kotlin> {
            enabled: true,
            formatter: MdsfFormatter::Single(Kotlin::Ktfmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "fun add(a: Int, b: Int): Int {
    return a + b
}
";

        assert_eq!(output, expected_output);
    }
}

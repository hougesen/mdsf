use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{npm_groovy_lint::format_using_npm_groovy_lint, MdsfFormatter};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Groovy {
    #[default]
    #[serde(rename = "npm-groovy-lint")]
    NpmGroovyLint,
}

impl Default for Lang<Groovy> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Groovy>::default(),
        }
    }
}

impl Default for MdsfFormatter<Groovy> {
    #[inline]
    fn default() -> Self {
        Self::Single(Groovy::NpmGroovyLint)
    }
}

impl LanguageFormatter for Groovy {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::NpmGroovyLint => format_using_npm_groovy_lint(snippet_path),
        }
    }
}

impl core::fmt::Display for Groovy {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NpmGroovyLint => write!(f, "npm-groovy-lint"),
        }
    }
}

#[cfg(test)]
mod test_groovy {
    use super::Groovy;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "        def add(a, b) {
            return a + b
        }

        assert add(1,2) == 3            ";

    const EXTENSION: &str = crate::languages::Language::Groovy.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Groovy>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Lang::<Groovy> {
            enabled: false,
            formatter: MdsfFormatter::Single(Groovy::NpmGroovyLint),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .is_none());
    }

    #[test]
    fn test_npm_groovy_lint() {
        let l = Lang::<Groovy> {
            enabled: true,
            formatter: MdsfFormatter::Single(Groovy::NpmGroovyLint),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "def add(a, b) {
    return a + b
}

assert add(1, 2) == 3
";

        assert_eq!(output, expected_output);
    }
}

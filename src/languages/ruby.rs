use schemars::JsonSchema;

use crate::formatters::{
    rubocop::format_using_rubocop, rubyfmt::format_using_rubyfmt, rufo::format_using_rufo,
    standardrb::format_using_standardrb, MdsfFormatter,
};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Ruby {
    #[default]
    #[serde(rename = "rubyfmt")]
    RubyFmt,
    #[serde(rename = "rubocop")]
    RuboCop,
    #[serde(rename = "rufo")]
    Rufo,
    #[serde(rename = "standardrb")]
    Standardrb,
}

impl Default for Lang<Ruby> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Ruby>::default(),
        }
    }
}

impl Default for MdsfFormatter<Ruby> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Ruby::RuboCop),
            Self::Single(Ruby::Rufo),
            Self::Single(Ruby::RubyFmt),
            Self::Single(Ruby::Standardrb),
        ])])
    }
}

impl LanguageFormatter for Ruby {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::RuboCop => format_using_rubocop(snippet_path),
            Self::Rufo => format_using_rufo(snippet_path),
            Self::RubyFmt => format_using_rubyfmt(snippet_path),
            Self::Standardrb => format_using_standardrb(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_ruby {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Ruby;

    const INPUT: &str =
        "def   add(  a ,                                                          b )
                        return a + b
                end";

    const EXTENSION: &str = crate::languages::Language::Ruby.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Ruby>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Ruby> {
            enabled: false,
            formatter: MdsfFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(rubocop)]
    #[test]
    fn test_rubocop() {
        let expected_output = "def add(a, b)
  return a + b
end
";

        let l = Lang::<Ruby> {
            enabled: true,
            formatter: MdsfFormatter::Single(Ruby::RuboCop),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(rufo)]
    #[test]
    fn test_rufo() {
        let expected_output = "def add(a, b)
  return a + b
end
";

        let l = Lang::<Ruby> {
            enabled: true,
            formatter: MdsfFormatter::Single(Ruby::Rufo),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(standardrb)]
    #[test]
    fn test_standardrb() {
        let expected_output = "def add(a, b)
  a + b
end
";

        let l = Lang::<Ruby> {
            enabled: true,
            formatter: MdsfFormatter::Single(Ruby::Standardrb),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}

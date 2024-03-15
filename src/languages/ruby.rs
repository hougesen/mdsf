use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{format_multiple, rubocop::format_using_rubocop, MdsfFormatter},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum RubyFormatter {
    #[default]
    #[serde(rename = "rubocop")]
    RuboCop,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Ruby {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<RubyFormatter>,
}

impl Default for Ruby {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<RubyFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<RubyFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(RubyFormatter::RuboCop)
    }
}

impl LanguageFormatter<RubyFormatter> for Ruby {
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
        formatter: &RubyFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            RubyFormatter::RuboCop => format_using_rubocop(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_ruby {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

    use super::{Ruby, RubyFormatter};

    const INPUT: &str =
        "def   add(  a ,                                                          b )
                        return a + b
                end";

    const EXTENSION: &str = crate::languages::Language::Ruby.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Ruby::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Ruby {
            enabled: false,
            formatter: MdsfFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_rubocop() {
        let expected_output = "def add(a, b)
  return a + b
end
";

        let l = Ruby {
            enabled: true,
            formatter: MdsfFormatter::Single(RubyFormatter::RuboCop),
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

use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::mix_format::format_using_mix_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum ElixirFormatter {
    #[default]
    #[serde(rename = "mix_format")]
    MixFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Elixir {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: ElixirFormatter,
}

impl Default for Elixir {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: ElixirFormatter::default(),
        }
    }
}

impl LanguageFormatter for Elixir {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            ElixirFormatter::MixFormat => format_using_mix_format(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Elixir, ElixirFormatter};

    const INPUT: &str = "
        def              add(a  ,      b   )   do    a   +   b                 end

";

    const EXTENSION: &str = crate::languages::Language::Elixir.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Elixir::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Elixir {
            enabled: false,
            formatter: ElixirFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_mix_format() {
        let l = Elixir {
            enabled: true,
            formatter: ElixirFormatter::MixFormat,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "def add(a, b) do
  a + b
end
";

        assert_eq!(output, expected_output);
    }
}

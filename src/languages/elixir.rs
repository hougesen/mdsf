use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{format_multiple, mix_format::format_using_mix_format, MdsfFormatter},
};

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
    pub formatter: MdsfFormatter<ElixirFormatter>,
}

impl Default for Elixir {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<ElixirFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<ElixirFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(ElixirFormatter::MixFormat)
    }
}

impl LanguageFormatter<ElixirFormatter> for Elixir {
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
        formatter: &ElixirFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            ElixirFormatter::MixFormat => format_using_mix_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_elixir {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

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
            formatter: MdsfFormatter::Single(ElixirFormatter::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_mix_format() {
        let l = Elixir {
            enabled: true,
            formatter: MdsfFormatter::Single(ElixirFormatter::MixFormat),
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

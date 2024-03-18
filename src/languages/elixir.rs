use schemars::JsonSchema;

use crate::formatters::{mix_format::format_using_mix_format, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Elixir {
    #[default]
    #[serde(rename = "mix_format")]
    MixFormat,
}

impl Default for Lang<Elixir> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Elixir>::default(),
        }
    }
}

impl Default for MdsfFormatter<Elixir> {
    #[inline]
    fn default() -> Self {
        Self::Single(Elixir::MixFormat)
    }
}

impl LanguageFormatter for Elixir {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::MixFormat => format_using_mix_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_elixir {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Elixir;

    const INPUT: &str = "
        def              add(a  ,      b   )   do    a   +   b                 end

";

    const EXTENSION: &str = crate::languages::Language::Elixir.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Elixir>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Elixir> {
            enabled: false,
            formatter: MdsfFormatter::Single(Elixir::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(mix)]
    #[test]
    fn test_mix_format() {
        let l = Lang::<Elixir> {
            enabled: true,
            formatter: MdsfFormatter::Single(Elixir::MixFormat),
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

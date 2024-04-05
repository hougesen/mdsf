use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{efmt::format_using_efmt, erlfmt::format_using_erlfmt, MdsfFormatter};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Erlang {
    #[default]
    #[serde(rename = "erlfmt")]
    Erlfmt,
    #[serde(rename = "efmt")]
    Efmt,
}

impl Default for Lang<Erlang> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Erlang>::default(),
        }
    }
}

impl Default for MdsfFormatter<Erlang> {
    #[inline]
    fn default() -> Self {
        Self::Single(Erlang::Erlfmt)
    }
}

impl LanguageFormatter for Erlang {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Erlfmt => format_using_erlfmt(snippet_path),
            Self::Efmt => format_using_efmt(snippet_path),
        }
    }
}

impl core::fmt::Display for Erlang {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Efmt => write!(f, "efmt"),
            Self::Erlfmt => write!(f, "erlfmt"),
        }
    }
}

#[cfg(test)]
mod test_erlang {
    use super::Erlang;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
.";

    const EXTENSION: &str = crate::languages::Language::Gleam.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Erlang>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Erlang> {
            enabled: false,
            formatter: MdsfFormatter::Single(Erlang::default())
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(erlfmt)]
    #[test]
    fn test_erlfmt() {
        let l = Lang::<Erlang> {
            enabled: true,
            formatter: MdsfFormatter::Single(Erlang::Erlfmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "what_is(Erlang) ->
    case Erlang of
        movie -> [hello(mike, joe, robert), credits];
        language -> no_more_formatting_arguments
    end.";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(efmt)]
    #[test]
    fn test_efmt() {
        let input = "what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
.";

        let l = Lang::<Erlang> {
            enabled: true,
            formatter: MdsfFormatter::Single(Erlang::Efmt),
        };

        let snippet = setup_snippet(input, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "what_is(Erlang) ->
    case Erlang of movie -> [hello(mike, joe, robert), credits]; language -> formatting_arguments end.
"
;

        assert_eq!(output, expected_output);
    }
}

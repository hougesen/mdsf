use schemars::JsonSchema;

use crate::formatters::{elm_format::format_using_elm_format, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Elm {
    #[default]
    #[serde(rename = "elm-format")]
    ElmFormat,
}

impl Default for Lang<Elm> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Elm>::default(),
        }
    }
}

impl Default for MdsfFormatter<Elm> {
    #[inline]
    fn default() -> Self {
        Self::Single(Elm::ElmFormat)
    }
}

impl LanguageFormatter for Elm {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::ElmFormat => format_using_elm_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_elm {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Elm;

    const INPUT: &str = r#"import   Html       exposing   (text)


main =
      text              "Hello!"


  "#;

    const EXTENSION: &str = crate::languages::Language::Elm.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Elm>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Elm> {
            enabled: false,
            formatter: MdsfFormatter::Single(Elm::default())
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_elm_format() {
        let l = Lang::<Elm> {
            enabled: true,
            formatter: MdsfFormatter::Single(Elm::ElmFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
        let expected_output = r#"module Main exposing (main)

import Html exposing (text)


main =
    text "Hello!"
"#;

        assert_eq!(output, expected_output);
    }
}

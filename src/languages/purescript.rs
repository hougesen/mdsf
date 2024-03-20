use schemars::JsonSchema;

use crate::formatters::{purs_tidy::format_using_purs_tidy, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum PureScript {
    #[default]
    #[serde(rename = "purs-tidy")]
    PursTidy,
}

impl Default for Lang<PureScript> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<PureScript>::default(),
        }
    }
}

impl Default for MdsfFormatter<PureScript> {
    #[inline]
    fn default() -> Self {
        Self::Single(PureScript::PursTidy)
    }
}

impl LanguageFormatter for PureScript {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::PursTidy => format_using_purs_tidy(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_purescript {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::PureScript;

    const INPUT: &str = r#"module       Test.Main   where

import Prelude

import   Effect   (Effect)
import                  Effect.Class.Console  (log)

main     ::   Effect Unit
main   =    do
  log          "You should add some tests.""#;

    const EXTENSION: &str = crate::languages::Language::PureScript.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<PureScript>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<PureScript> {
            enabled: false,
            formatter: MdsfFormatter::Single(PureScript::default())
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_purs_tidy() {
        let l = Lang::<PureScript> {
            enabled: true,
            formatter: MdsfFormatter::Single(PureScript::PursTidy),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = r#"module Test.Main where

import Prelude

import Effect (Effect)
import Effect.Class.Console (log)

main :: Effect Unit
main = do
  log "You should add some tests.""#;

        assert_eq!(output, expected_output);
    }
}

use schemars::JsonSchema;

use crate::formatters::{
    fourmolu::format_using_fourmolu, hindent::format_using_hindent, ormolu::format_using_ormolu,
    MdsfFormatter,
};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Haskell {
    #[serde(rename = "fourmolu")]
    Fourmolu,
    #[default]
    #[serde(rename = "ormolu")]
    Ormolu,
    #[serde(rename = "hindent")]
    HIndent,
}

impl Default for Lang<Haskell> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Haskell>::default(),
        }
    }
}

impl Default for MdsfFormatter<Haskell> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Haskell::Fourmolu),
            Self::Single(Haskell::Ormolu),
            Self::Single(Haskell::HIndent),
        ])])
    }
}

impl LanguageFormatter for Haskell {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Fourmolu => format_using_fourmolu(snippet_path),
            Self::Ormolu => format_using_ormolu(snippet_path),
            Self::HIndent => format_using_hindent(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_haskell {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Haskell;

    const INPUT: &str = "
addNumbers::Int->Int->Int
addNumbers a b = do 
        a + b 
        ";

    const EXTENSION: &str = crate::languages::Language::Haskell.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Haskell>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Haskell> {
            enabled: false,
            formatter: MdsfFormatter::Single(Haskell::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(fourmolu)]
    #[test]
    fn test_fourmolu() {
        let l = Lang::<Haskell> {
            enabled: true,
            formatter: MdsfFormatter::Single(Haskell::Fourmolu),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "addNumbers :: Int -> Int -> Int
addNumbers a b = do
    a + b
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(ormolu)]
    #[test]
    fn test_ormolu() {
        let l = Lang::<Haskell> {
            enabled: true,
            formatter: MdsfFormatter::Single(Haskell::Ormolu),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "addNumbers :: Int -> Int -> Int
addNumbers a b = do
  a + b
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(hindent)]
    #[test]
    fn test_hindent() {
        let l = Lang::<Haskell> {
            enabled: true,
            formatter: MdsfFormatter::Single(Haskell::HIndent),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "addNumbers :: Int -> Int -> Int
addNumbers a b = do
  a + b
";

        assert_eq!(output, expected_output);
    }
}

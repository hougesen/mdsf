use schemars::JsonSchema;

use crate::formatters::{fourmolu::format_using_fourmolu, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Haskell {
    #[default]
    #[serde(rename = "fourmolu")]
    Fourmolu,
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
        Self::Single(Haskell::Fourmolu)
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
}

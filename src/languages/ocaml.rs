use schemars::JsonSchema;

use crate::formatters::{ocamlformat::format_using_ocamlformat, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum OCaml {
    #[default]
    #[serde(rename = "ocamlformat")]
    OCamlFormat,
}

impl Default for Lang<OCaml> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<OCaml>::default(),
        }
    }
}

impl Default for MdsfFormatter<OCaml> {
    #[inline]
    fn default() -> Self {
        Self::Single(OCaml::OCamlFormat)
    }
}

impl LanguageFormatter for OCaml {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::OCamlFormat => format_using_ocamlformat(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_ocaml {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::OCaml;

    const INPUT: &str = "
let add a b  =  a +  b
            ";

    const EXTENSION: &str = crate::languages::Language::OCaml.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<OCaml>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<OCaml> {
            enabled: false,
            formatter: MdsfFormatter::Single(OCaml::default())
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    /// NOTE: this is ignored since the ocaml runtime takes a long time to installed
    #[test_with::no_env(GITHUB_ACTIONS)]
    #[test]
    fn test_ocamlformat() {
        let l = Lang::<OCaml> {
            enabled: true,
            formatter: MdsfFormatter::Single(OCaml::OCamlFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "let add a b = a + b
";

        assert_eq!(output, expected_output);
    }
}

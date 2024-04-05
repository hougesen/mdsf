use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{
        ocamlformat::format_using_ocamlformat, ocp_indent::format_using_ocp_indent, MdsfFormatter,
    },
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum OCaml {
    #[default]
    #[serde(rename = "ocamlformat")]
    OCamlFormat,
    #[serde(rename = "ocp-indent")]
    OcpIndent,
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
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(OCaml::OCamlFormat),
            Self::Single(OCaml::OcpIndent),
        ])])
    }
}

impl LanguageFormatter for OCaml {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::OCamlFormat => format_using_ocamlformat(snippet_path),
            Self::OcpIndent => format_using_ocp_indent(snippet_path),
        }
    }
}

impl core::fmt::Display for OCaml {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OCamlFormat => write!(f, "ocamlformat"),
            Self::OcpIndent => write!(f, "ocpindent"),
        }
    }
}

#[cfg(test)]
mod test_ocaml {
    use super::OCaml;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

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
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(ocamlformat)]
    #[test]
    fn test_ocamlformat() {
        let l = Lang::<OCaml> {
            enabled: true,
            formatter: MdsfFormatter::Single(OCaml::OCamlFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "let add a b = a + b
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(ocp-indent)]
    #[test]
    fn test_ocp_indent() {
        let input = "
    let add a b
                                 = a + b
                ";
        let expected_output = "
let add a b
  = a + b
";

        let l = Lang::<OCaml> {
            enabled: true,
            formatter: MdsfFormatter::Single(OCaml::OcpIndent),
        };

        let snippet = setup_snippet(input, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}

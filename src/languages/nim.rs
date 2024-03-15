use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{format_multiple, nimpretty::format_using_nimpretty, MdsfFormatter},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum NimFormatter {
    #[default]
    #[serde(rename = "nimpretty")]
    Nimpretty,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Nim {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<NimFormatter>,
}

impl Default for Nim {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<NimFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<NimFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(NimFormatter::Nimpretty)
    }
}

impl LanguageFormatter<NimFormatter> for Nim {
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
        formatter: &NimFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            NimFormatter::Nimpretty => format_using_nimpretty(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_nim {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

    use super::{Nim, NimFormatter};

    const INPUT: &str = "proc           add( a         :int , b:int )        : int =
  return a +          b  ";

    const EXTENSION: &str = crate::languages::Language::Nim.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Nim::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Nim {
            enabled: false,
            formatter: MdsfFormatter::Single(NimFormatter::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_clang_format() {
        let l = Nim {
            enabled: true,
            formatter: MdsfFormatter::Single(NimFormatter::Nimpretty),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "proc add(a: int, b: int): int =
  return a + b
";

        assert_eq!(output, expected_output);
    }
}

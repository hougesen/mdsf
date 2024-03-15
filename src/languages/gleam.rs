use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{format_multiple, gleam_format::format_using_gleam_format, MdsfFormatter},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum GleamFormatter {
    #[default]
    #[serde(rename = "gleam_format")]
    GleamFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Gleam {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<GleamFormatter>,
}

impl Default for Gleam {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<GleamFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<GleamFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(GleamFormatter::GleamFormat)
    }
}

impl LanguageFormatter<GleamFormatter> for Gleam {
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
        formatter: &GleamFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            GleamFormatter::GleamFormat => format_using_gleam_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_gleam {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

    use super::{Gleam, GleamFormatter};

    const INPUT: &str = "pub fn add(a:Int,b:Int)->Int{a+b}";

    const EXTENSION: &str = crate::languages::Language::Gleam.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Gleam::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Gleam {
            enabled: false,
            formatter: MdsfFormatter::Single(GleamFormatter::default())
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_gleam_format() {
        let l = Gleam {
            enabled: true,
            formatter: MdsfFormatter::Single(GleamFormatter::GleamFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "pub fn add(a: Int, b: Int) -> Int {
  a + b
}
";

        assert_eq!(output, expected_output);
    }
}

use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{format_multiple, rustfmt::format_using_rustfmt, MdsfFormatter},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum RustFormatter {
    #[default]
    #[serde(rename = "rustfmt")]
    RustFmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Rust {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<RustFormatter>,
}

impl Default for Rust {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<RustFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<RustFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(RustFormatter::RustFmt)
    }
}

impl LanguageFormatter<RustFormatter> for Rust {
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
        formatter: &RustFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            RustFormatter::RustFmt => format_using_rustfmt(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_rust {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

    use super::{Rust, RustFormatter};

    const INPUT: &str = "pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    ";

    const EXTENSION: &str = crate::languages::Language::Rust.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Rust::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Rust {
            enabled: false,
            formatter: MdsfFormatter::Single(RustFormatter::RustFmt),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_rustfmt() {
        let expected_output = "pub async fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";

        let l = Rust {
            enabled: true,
            formatter: MdsfFormatter::Single(RustFormatter::RustFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}

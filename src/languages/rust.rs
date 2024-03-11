use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::rustfmt::format_using_rustfmt};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum RustFormatter {
    #[default]
    #[serde(rename = "rustfmt")]
    RustFmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Rust {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: RustFormatter,
}

impl Default for Rust {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: RustFormatter::default(),
        }
    }
}

impl LanguageFormatter for Rust {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            RustFormatter::RustFmt => format_using_rustfmt(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        formatters::setup_snippet,
        languages::{Language, LanguageFormatter},
    };

    use super::{Rust, RustFormatter};

    const INPUT: &str = "";

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet =
            setup_snippet(INPUT, Language::Rust.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        Rust::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet =
            setup_snippet(INPUT, Language::Rust.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Rust {
            enabled: false,
            formatter: RustFormatter::RustFmt,
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}

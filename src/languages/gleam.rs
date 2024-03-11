use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::gleam_format::format_using_gleam_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum GleamFormatter {
    #[default]
    #[serde(rename = "gleam_format")]
    GleamFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Gleam {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: GleamFormatter,
}

impl Default for Gleam {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: GleamFormatter::default(),
        }
    }
}

impl LanguageFormatter for Gleam {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            GleamFormatter::GleamFormat => format_using_gleam_format(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        formatters::setup_snippet,
        languages::{Language, LanguageFormatter},
    };

    use super::{Gleam, GleamFormatter};

    const INPUT: &str = "";

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet =
            setup_snippet(INPUT, Language::Gleam.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        Gleam::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet =
            setup_snippet(INPUT, Language::Gleam.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Gleam {
            enabled: false,
            formatter: GleamFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}

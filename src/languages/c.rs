use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::clang_format::format_using_clang_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum CFormatter {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct C {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: CFormatter,
}

impl Default for C {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: CFormatter::default(),
        }
    }
}

impl LanguageFormatter for C {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            CFormatter::ClangFormat => format_using_clang_format(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        formatters::setup_snippet,
        languages::{Language, LanguageFormatter},
    };

    use super::{CFormatter, C};

    const INPUT: &str = "";

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet = setup_snippet(INPUT, Language::C.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        C::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, Language::C.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(C {
            enabled: false,
            formatter: CFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}

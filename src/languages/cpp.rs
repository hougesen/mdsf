use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::clang_format::format_using_clang_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum CppFormatter {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Cpp {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: CppFormatter,
}

impl Default for Cpp {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: CppFormatter::default(),
        }
    }
}

impl LanguageFormatter for Cpp {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            CppFormatter::ClangFormat => format_using_clang_format(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        formatters::setup_snippet,
        languages::{Language, LanguageFormatter},
    };

    use super::{Cpp, CppFormatter};

    const INPUT: &str = "";

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet =
            setup_snippet(INPUT, Language::Cpp.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        Cpp::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet =
            setup_snippet(INPUT, Language::Cpp.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Cpp {
            enabled: false,
            formatter: CppFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}

use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{
        autopep8::format_using_autopep8, black::format_using_black, blue::format_using_blue,
        ruff::format_using_ruff, yapf::format_using_yapf,
    },
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum PythonFormatter {
    #[default]
    #[serde(rename = "ruff")]
    Ruff,
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "yapf")]
    Yapf,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "autopep8")]
    Autopep8,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Python {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: PythonFormatter,
}

impl Default for Python {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: PythonFormatter::default(),
        }
    }
}

impl LanguageFormatter for Python {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            PythonFormatter::Autopep8 => format_using_autopep8(snippet_path).map(|res| res.1),
            PythonFormatter::Black => format_using_black(snippet_path).map(|res| res.1),
            PythonFormatter::Blue => format_using_blue(snippet_path).map(|res| res.1),
            PythonFormatter::Ruff => format_using_ruff(snippet_path).map(|res| res.1),
            PythonFormatter::Yapf => format_using_yapf(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        formatters::setup_snippet,
        languages::{Language, LanguageFormatter},
    };

    use super::{Python, PythonFormatter};

    const INPUT: &str = "";

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet =
            setup_snippet(INPUT, Language::Python.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        Python::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet =
            setup_snippet(INPUT, Language::Python.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Python {
            enabled: false,
            formatter: PythonFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}

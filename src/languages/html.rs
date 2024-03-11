use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::prettier::format_using_prettier};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum HtmlFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Html {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: HtmlFormatter,
}

impl Default for Html {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: HtmlFormatter::default(),
        }
    }
}

impl LanguageFormatter for Html {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            HtmlFormatter::Prettier => format_using_prettier(snippet_path, true).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        formatters::setup_snippet,
        languages::{Language, LanguageFormatter},
    };

    use super::{Html, HtmlFormatter};

    const INPUT: &str = "";

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet =
            setup_snippet(INPUT, Language::Html.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        Html::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet =
            setup_snippet(INPUT, Language::Html.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Html {
            enabled: false,
            formatter: HtmlFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }
}

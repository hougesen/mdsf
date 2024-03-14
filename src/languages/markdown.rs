use schemars::JsonSchema;

use crate::formatters::prettier::format_using_prettier;

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum MarkdownFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Markdown {
    #[serde(default = "bool::default")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MarkdownFormatter,
}

impl Default for Markdown {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: false,
            formatter: MarkdownFormatter::default(),
        }
    }
}

impl LanguageFormatter for Markdown {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            MarkdownFormatter::Prettier => format_using_prettier(snippet_path, false),
        }
        .map(|res| res.1)
    }
}

#[cfg(test)]
mod test_markdown {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Markdown, MarkdownFormatter};

    const INPUT: &str = "
# mads    was here


  the    whitespace    should    be removed
";

    const EXTENSION: &str = crate::languages::Language::Markdown.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(!Markdown::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Markdown {
            enabled: false,
            formatter: MarkdownFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_prettier() {
        let expected_output = "# mads was here

the whitespace should be removed
";

        let l = Markdown {
            enabled: true,
            formatter: MarkdownFormatter::Prettier,
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

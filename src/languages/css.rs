use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::prettier::format_using_prettier};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum CssFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Css {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: CssFormatter,
}

impl Default for Css {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: CssFormatter::default(),
        }
    }
}

impl LanguageFormatter for Css {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            CssFormatter::Prettier => format_using_prettier(snippet_path, true),
        }
        .map(|res| res.1)
    }
}

#[cfg(test)]
mod test_css {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Css, CssFormatter};

    const INPUT: &str = " h1   {color: blue;} p    {color: red;} ";

    const EXTENSION: &str = crate::languages::Language::Css.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Css::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Css {
            enabled: false,
            formatter: CssFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Css {
            enabled: true,
            formatter: CssFormatter::Prettier,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "h1 {
  color: blue;
}
p {
  color: red;
}
";

        assert_eq!(output, expected_output);
    }
}

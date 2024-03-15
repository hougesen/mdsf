use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{format_multiple, prettier::format_using_prettier, MdsfFormatter},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum HtmlFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Html {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<HtmlFormatter>,
}

impl Default for Html {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<HtmlFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<HtmlFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(HtmlFormatter::Prettier)
    }
}

impl LanguageFormatter<HtmlFormatter> for Html {
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
        formatter: &HtmlFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            HtmlFormatter::Prettier => format_using_prettier(snippet_path, true),
        }
    }
}

#[cfg(test)]
mod test_html {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

    use super::{Html, HtmlFormatter};

    const INPUT: &str =  " <!doctype html> <html> <head> <style> body {background-color: powderblue;} h1   {color: blue;} p    {color: red;} </style> </head> <body>  <h1>This is a heading</h1> <p>This is a paragraph.</p>  </body> </html> ";

    const EXTENSION: &str = crate::languages::Language::Html.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Html::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Html {
            enabled: false,
            formatter: MdsfFormatter::Single(HtmlFormatter::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_prettier() {
        let expected_output = "<!doctype html>
<html>
  <head>
    <style>
      body {
        background-color: powderblue;
      }
      h1 {
        color: blue;
      }
      p {
        color: red;
      }
    </style>
  </head>
  <body>
    <h1>This is a heading</h1>
    <p>This is a paragraph.</p>
  </body>
</html>
";

        let l = Html {
            enabled: true,
            formatter: MdsfFormatter::Single(HtmlFormatter::Prettier),
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

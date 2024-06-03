use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{
        djlint::format_using_djlint, htmlbeautifier::format_using_htmlbeautifier,
        prettier::format_using_prettier, MdsfFormatter,
    },
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Html {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "djlint")]
    DjLint,
    #[serde(rename = "htmlbeautifier")]
    Htmlbeautifier,
}

impl Default for Lang<Html> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Html>::default(),
        }
    }
}

impl Default for MdsfFormatter<Html> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Html::Prettier),
            Self::Single(Html::DjLint),
        ])])
    }
}

impl LanguageFormatter for Html {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Prettier => format_using_prettier(snippet_path),
            Self::DjLint => format_using_djlint(snippet_path),
            Self::Htmlbeautifier => format_using_htmlbeautifier(snippet_path),
        }
    }
}

impl core::fmt::Display for Html {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Prettier => write!(f, "prettier"),
            Self::DjLint => write!(f, "djlint"),
            Self::Htmlbeautifier => write!(f, "htmlbeautifier"),
        }
    }
}

#[cfg(test)]
mod test_html {
    use super::Html;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str =  " <!doctype html> <html> <head> <style> body {background-color: powderblue;} h1   {color: blue;} p    {color: red;} </style> </head> <body>  <h1>This is a heading</h1> <p>This is a paragraph.</p>  </body> </html> ";

    const EXTENSION: &str = crate::languages::Language::Html.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Html>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Html> {
            enabled: false,
            formatter: MdsfFormatter::Single(Html::default()),
        }
        .format(snippet_path, &LineInfo::fake())
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

        let l = Lang::<Html> {
            enabled: true,
            formatter: MdsfFormatter::Single(Html::Prettier),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}

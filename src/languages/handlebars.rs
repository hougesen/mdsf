use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{djlint::format_using_djlint, prettier::format_using_prettier, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Handlebars {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "djlint")]
    DjLint,
}

impl Default for Lang<Handlebars> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Handlebars>::default(),
        }
    }
}

impl Default for MdsfFormatter<Handlebars> {
    #[inline]
    fn default() -> Self {
        Self::Single(Handlebars::Prettier)
    }
}

impl LanguageFormatter for Handlebars {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Prettier => format_using_prettier(snippet_path),
            Self::DjLint => format_using_djlint(snippet_path),
        }
    }
}

impl core::fmt::Display for Handlebars {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Prettier => write!(f, "prettier"),
            Self::DjLint => write!(f, "djlint"),
        }
    }
}

#[cfg(test)]
mod test_handlebars {
    use super::Handlebars;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str =  " <!doctype html> <html> <head> <style> body {background-color: powderblue;} h1   {color: blue;} p    {color: red;} </style> </head> <body>  <h1>This is a heading</h1> <p>This is a paragraph.</p>  </body> </html> ";

    const EXTENSION: &str = crate::languages::Language::Handlebars.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Handlebars>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Handlebars> {
            enabled: false,
            formatter: MdsfFormatter::Single(Handlebars::default()),
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }
}

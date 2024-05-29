use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{
        prettier::format_using_prettier, stylelint::format_using_stylelint, MdsfFormatter,
    },
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Css {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "stylelint")]
    StyleLint,
}

impl Default for Lang<Css> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Css>::default(),
        }
    }
}

impl Default for MdsfFormatter<Css> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Css::Prettier),
            Self::Single(Css::StyleLint),
        ])])
    }
}

impl LanguageFormatter for Css {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Prettier => format_using_prettier(snippet_path),
            Self::StyleLint => format_using_stylelint(snippet_path),
        }
    }
}

impl core::fmt::Display for Css {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Prettier => write!(f, "prettier"),
            Self::StyleLint => write!(f, "stylelint"),
        }
    }
}

#[cfg(test)]
mod test_css {
    use super::Css;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::{CssFlavor, Lang},
        LineInfo,
    };

    const INPUT: &str = " h1   {color: blue;} p    {color: red;} ";

    const EXTENSION: &str = crate::languages::Language::Css(CssFlavor::Css).to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Css>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Css> {
            enabled: false,
            formatter: MdsfFormatter::Single(Css::default()),
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Lang::<Css> {
            enabled: true,
            formatter: MdsfFormatter::Single(Css::Prettier),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
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

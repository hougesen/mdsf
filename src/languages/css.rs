use schemars::JsonSchema;

use crate::formatters::{prettier::format_using_prettier, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Css {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
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
        Self::Single(Css::Prettier)
    }
}

impl LanguageFormatter for Css {
    #[inline]
    fn format_single(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Prettier => format_using_prettier(snippet_path, true),
        }
    }
}

#[cfg(test)]
mod test_css {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Css;

    const INPUT: &str = " h1   {color: blue;} p    {color: red;} ";

    const EXTENSION: &str = crate::languages::Language::Css.to_file_ext();

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
        .format(snippet_path)
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

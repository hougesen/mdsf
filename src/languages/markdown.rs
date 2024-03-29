use schemars::JsonSchema;

use crate::formatters::{prettier::format_using_prettier, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Markdown {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

impl Default for Lang<Markdown> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: false,
            formatter: MdsfFormatter::<Markdown>::default(),
        }
    }
}

impl Default for MdsfFormatter<Markdown> {
    #[inline]
    fn default() -> Self {
        Self::Single(Markdown::Prettier)
    }
}

impl LanguageFormatter for Markdown {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Prettier => format_using_prettier(snippet_path, false),
        }
    }
}

#[cfg(test)]
mod test_markdown {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Markdown;

    const INPUT: &str = "
# mads    was here


  the    whitespace    should    be removed
";

    const EXTENSION: &str = crate::languages::Language::Markdown.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(!Lang::<Markdown>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Markdown> {
            enabled: false,
            formatter: MdsfFormatter::<Markdown>::default(),
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

        let l = Lang::<Markdown> {
            enabled: true,
            formatter: MdsfFormatter::Single(Markdown::Prettier),
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

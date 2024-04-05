use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{prettier::format_using_prettier, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum GraphQL {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

impl Default for Lang<GraphQL> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<GraphQL>::default(),
        }
    }
}

impl Default for MdsfFormatter<GraphQL> {
    #[inline]
    fn default() -> Self {
        Self::Single(GraphQL::Prettier)
    }
}

impl LanguageFormatter for GraphQL {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Prettier => format_using_prettier(snippet_path, true),
        }
    }
}

impl core::fmt::Display for GraphQL {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Prettier => write!(f, "prettier"),
        }
    }
}

#[cfg(test)]
mod test_grapql {
    use super::GraphQL;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "{   hero {     name
                # Queries can have comments!
         friends {       name     }   } }";

    const EXTENSION: &str = crate::languages::Language::GraphQL.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<GraphQL>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<GraphQL> {
            enabled: false,
            formatter: MdsfFormatter::Single(GraphQL::default()),
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_prettier() {
        let expected_output = "{
  hero {
    name
    # Queries can have comments!
    friends {
      name
    }
  }
}
";

        let l = Lang::<GraphQL> {
            enabled: true,
            formatter: MdsfFormatter::Single(GraphQL::Prettier),
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

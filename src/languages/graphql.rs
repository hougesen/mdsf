use schemars::JsonSchema;

use crate::formatters::{prettier::format_using_prettier, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
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
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Prettier => format_using_prettier(snippet_path, true),
        }
    }
}

#[cfg(test)]
mod test_grapql {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::GraphQL;

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
        .format(snippet_path)
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
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }
}

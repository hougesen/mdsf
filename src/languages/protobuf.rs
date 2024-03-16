use schemars::JsonSchema;

use crate::formatters::{clang_format::format_using_clang_format, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Protobuf {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

impl Default for Lang<Protobuf> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Protobuf>::default(),
        }
    }
}

impl Default for MdsfFormatter<Protobuf> {
    #[inline]
    fn default() -> Self {
        Self::Single(Protobuf::ClangFormat)
    }
}

impl LanguageFormatter for Protobuf {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::ClangFormat => format_using_clang_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_protobuf {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Protobuf;

    const INPUT: &str = "service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }";

    const EXTENSION: &str = crate::languages::Language::Protobuf.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Protobuf>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Protobuf> {
            enabled: false,
            formatter: MdsfFormatter::Single(Protobuf::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_clang_format() {
        let expected_output =
            "service SearchService { rpc Search(SearchRequest) returns (SearchResponse); }";

        let l = Lang::<Protobuf> {
            enabled: true,
            formatter: MdsfFormatter::Single(Protobuf::ClangFormat),
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

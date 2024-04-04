use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{
    buf::format_using_buf, clang_format::format_using_clang_format, MdsfFormatter,
};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Protobuf {
    #[default]
    #[serde(rename = "buf")]
    Buf,
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
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Protobuf::Buf),
            Self::Single(Protobuf::ClangFormat),
        ])])
    }
}

impl LanguageFormatter for Protobuf {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Buf => format_using_buf(snippet_path),
            Self::ClangFormat => format_using_clang_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_protobuf {
    use super::Protobuf;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

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

    #[test_with::executable(buf)]
    #[test]
    fn test_buf() {
        let expected_output = "service SearchService {
  rpc Search(SearchRequest) returns (SearchResponse);
}
";

        let l = Lang::<Protobuf> {
            enabled: true,
            formatter: MdsfFormatter::Single(Protobuf::Buf),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(clang-format)]
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

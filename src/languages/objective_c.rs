use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{clang_format::format_using_clang_format, MdsfFormatter};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum ObjectiveC {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

impl Default for Lang<ObjectiveC> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<ObjectiveC>::default(),
        }
    }
}

impl Default for MdsfFormatter<ObjectiveC> {
    #[inline]
    fn default() -> Self {
        Self::Single(ObjectiveC::ClangFormat)
    }
}

impl LanguageFormatter for ObjectiveC {
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

impl core::fmt::Display for ObjectiveC {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ClangFormat => write!(f, "clang-format"),
        }
    }
}

#[cfg(test)]
mod test_objective_c {
    use super::ObjectiveC;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "int add(int a,int b){
            a - a ;
       return a + b;
    }";

    const EXTENSION: &str = crate::languages::Language::ObjectiveC.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<ObjectiveC>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<ObjectiveC> {
            enabled: false,
            formatter: MdsfFormatter::Single(ObjectiveC::ClangFormat),
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn test_clang_format() {
        let expected_output = "int add(int a, int b) {
  a - a;
  return a + b;
}";

        let l = Lang::<ObjectiveC> {
            enabled: true,
            formatter: MdsfFormatter::Single(ObjectiveC::ClangFormat),
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

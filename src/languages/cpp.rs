use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{clang_format::format_using_clang_format, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Cpp {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

impl Default for Lang<Cpp> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Cpp>::default(),
        }
    }
}

impl Default for MdsfFormatter<Cpp> {
    #[inline]
    fn default() -> Self {
        Self::Single(Cpp::ClangFormat)
    }
}

impl LanguageFormatter for Cpp {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::ClangFormat => format_using_clang_format(snippet_path),
        }
    }
}

impl core::fmt::Display for Cpp {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ClangFormat => write!(f, "clang-format"),
        }
    }
}

#[cfg(test)]
mod test_cpp {
    use super::Cpp;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

    const INPUT: &str = "int add(int a,int b){
                 a-b;
       return a + b;
    }";

    const EXTENSION: &str = crate::languages::Language::Cpp.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Cpp>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Cpp> {
            enabled: false,
            formatter: MdsfFormatter::Single(Cpp::default())
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn test_clang_format() {
        let l = Lang::<Cpp> {
            enabled: true,
            formatter: MdsfFormatter::Single(Cpp::ClangFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "int add(int a, int b) {
  a - b;
  return a + b;
}";

        assert_eq!(output, expected_output);
    }
}

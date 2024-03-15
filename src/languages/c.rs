use schemars::JsonSchema;

use crate::formatters::{clang_format::format_using_clang_format, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum C {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

impl Default for Lang<C> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<C>::default(),
        }
    }
}

impl Default for MdsfFormatter<C> {
    #[inline]
    fn default() -> Self {
        Self::Single(C::ClangFormat)
    }
}

impl LanguageFormatter for C {
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
mod test_c {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::C;

    const INPUT: &str = "int add(int a,int b){
                a-b;
       return a + b;
    }";

    const EXTENSION: &str = crate::languages::Language::C.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<C>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<C> {
            enabled: false,
            formatter: MdsfFormatter::Single(C::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_clang_format() {
        let l = Lang::<C> {
            enabled: true,
            formatter: MdsfFormatter::Single(C::ClangFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "int add(int a, int b) {
  a - b;
  return a + b;
}";

        assert_eq!(output, expected_output);
    }
}

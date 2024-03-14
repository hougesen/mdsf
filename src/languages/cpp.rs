use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::clang_format::format_using_clang_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum CppFormatter {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Cpp {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: CppFormatter,
}

impl Default for Cpp {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: CppFormatter::default(),
        }
    }
}

impl LanguageFormatter for Cpp {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            CppFormatter::ClangFormat => format_using_clang_format(snippet_path),
        }
        .map(|res| res.1)
    }
}

#[cfg(test)]
mod test_cpp {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Cpp, CppFormatter};

    const INPUT: &str = "int add(int a,int b){
                 a-b;
       return a + b;
    }";

    const EXTENSION: &str = crate::languages::Language::Cpp.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Cpp::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Cpp {
            enabled: false,
            formatter: CppFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_clang_format() {
        let l = Cpp {
            enabled: true,
            formatter: CppFormatter::ClangFormat,
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

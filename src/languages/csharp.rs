use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::clang_format::format_using_clang_format};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CSharpFormatter {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub struct CSharp {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: CSharpFormatter,
}

impl Default for CSharp {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: CSharpFormatter::default(),
        }
    }
}

impl LanguageFormatter for CSharp {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            CSharpFormatter::ClangFormat => {
                format_using_clang_format(snippet_path).map(|res| res.1)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{CSharp, CSharpFormatter};

    const INPUT: &str = "namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                a-b ;
                                                        return a + b;
                                                    }
                                                 }
                                                 } ";

    const EXTENSION: &str = crate::languages::Language::CSharp.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        CSharp::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(CSharp {
            enabled: false,
            formatter: CSharpFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_clang_format() {
        let l = CSharp {
            enabled: true,
            formatter: CSharpFormatter::ClangFormat,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "namespace Mdsf {
class Adder {
  public static int add(int a, int b) {
    a - b;
    return a + b;
  }
}
}";

        assert_eq!(output, expected_output);
    }
}

use schemars::JsonSchema;

use crate::formatters::{clang_format::format_using_clang_format, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Java {
    #[default]
    #[serde(rename = "clang-format")]
    ClangFormat,
}

impl Default for Lang<Java> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Java>::default(),
        }
    }
}

impl Default for MdsfFormatter<Java> {
    #[inline]
    fn default() -> Self {
        Self::Single(Java::ClangFormat)
    }
}

impl LanguageFormatter for Java {
    #[inline]
    fn format_single(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::ClangFormat => format_using_clang_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_java {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Java;

    const INPUT: &str = "class HelloWorld {
    public static void main(String[] args) {
                System.out.println(\"Hello\");
                System.out.println(\"World!\");
                 }
}";

    const EXTENSION: &str = crate::languages::Language::Java.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Java>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Java> {
            enabled: false,
            formatter: MdsfFormatter::Single(Java::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_clang_format() {
        let expected_output = "class HelloWorld {
  public static void main(String[] args) {
    System.out.println(\"Hello\");
    System.out.println(\"World!\");
  }
}";
        let l = Lang::<Java> {
            enabled: true,
            formatter: MdsfFormatter::Single(Java::ClangFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(expected_output, output);
    }
}

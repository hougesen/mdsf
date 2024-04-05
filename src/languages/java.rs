use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{
    clang_format::format_using_clang_format, google_java_format::format_using_google_java_format,
    MdsfFormatter,
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Java {
    #[default]
    #[serde(rename = "google-java-format")]
    GoogleJavaFormat,
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
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Java::GoogleJavaFormat),
            Self::Single(Java::ClangFormat),
        ])])
    }
}

impl LanguageFormatter for Java {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::ClangFormat => format_using_clang_format(snippet_path),
            Self::GoogleJavaFormat => format_using_google_java_format(snippet_path),
        }
    }
}

impl core::fmt::Display for Java {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::GoogleJavaFormat => write!(f, "google-java-format"),
            Self::ClangFormat => write!(f, "clang-format"),
        }
    }
}

#[cfg(test)]
mod test_java {
    use super::Java;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

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
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(clang-format)]
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
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(google-java-format)]
    #[test]
    fn test_google_java_format() {
        let expected_output = "class HelloWorld {
  public static void main(String[] args) {
    System.out.println(\"Hello\");
    System.out.println(\"World!\");
  }
}
";
        let l = Lang::<Java> {
            enabled: true,
            formatter: MdsfFormatter::Single(Java::GoogleJavaFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(expected_output, output);
    }
}

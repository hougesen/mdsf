use schemars::JsonSchema;

use crate::formatters::{
    clang_format::format_using_clang_format, csharpier::format_using_csharpier, MdsfFormatter,
};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum CSharp {
    #[default]
    #[serde(rename = "csharpier")]
    CSharpier,
    #[serde(rename = "clang-format")]
    ClangFormat,
}

impl Default for Lang<CSharp> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<CSharp>::default(),
        }
    }
}

impl Default for MdsfFormatter<CSharp> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(CSharp::CSharpier),
            Self::Single(CSharp::ClangFormat),
        ])])
    }
}

impl LanguageFormatter for CSharp {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::CSharpier => format_using_csharpier(snippet_path),
            Self::ClangFormat => format_using_clang_format(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_csharp {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::CSharp;

    const INPUT: &str = "namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                var c         = a+b ;
                                                        return c;
                                                    }
                                                 }
                                                 } ";

    const EXTENSION: &str = crate::languages::Language::CSharp.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<CSharp>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<CSharp> {
            enabled: false,
            formatter: MdsfFormatter::<CSharp>::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn test_clang_format() {
        let l = Lang::<CSharp> {
            enabled: true,
            formatter: MdsfFormatter::Single(CSharp::ClangFormat),
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
    var c = a + b;
    return c;
  }
}
}";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(dotnet)]
    #[test]
    fn test_csharpier() {
        let l = Lang::<CSharp> {
            enabled: true,
            formatter: MdsfFormatter::Single(CSharp::CSharpier),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "namespace Mdsf
{
    class Adder
    {
        public static int add(int a, int b)
        {
            var c = a + b;
            return c;
        }
    }
}
";

        assert_eq!(output, expected_output);
    }
}

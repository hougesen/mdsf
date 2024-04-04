use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{swiftformat::format_using_swiftformat, MdsfFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Swift {
    #[default]
    #[serde(rename = "swiftformat")]
    SwiftFormat,
}

impl Default for Lang<Swift> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Swift>::default(),
        }
    }
}

impl Default for MdsfFormatter<Swift> {
    #[inline]
    fn default() -> Self {
        Self::Single(Swift::SwiftFormat)
    }
}

impl LanguageFormatter for Swift {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::SwiftFormat => format_using_swiftformat(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_swift {
    use super::Swift;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    const INPUT: &str = " func add(a:Int ,b:Int)->Int {
    return a + b     
    }";

    const EXTENSION: &str = crate::languages::Language::Swift.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Swift>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Swift> {
            enabled: false,
            formatter: MdsfFormatter::Single(Swift::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(swiftformat)]
    #[test]
    fn test_swiftformat() {
        let l = Lang::<Swift> {
            enabled: true,
            formatter: MdsfFormatter::Single(Swift::SwiftFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "func add(a: Int, b: Int) -> Int {
    return a + b
}
";

        assert_eq!(output, expected_output);
    }
}

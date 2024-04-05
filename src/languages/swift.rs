use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{
    swift_format::format_using_swift_format, swiftformat::format_using_swiftformat, MdsfFormatter,
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Swift {
    #[default]
    #[serde(rename = "swift-format")]
    AppleSwiftFormat,
    #[serde(rename = "swiftformat")]
    NicklockwoodSwiftFormat,
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
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Swift::AppleSwiftFormat),
            Self::Single(Swift::NicklockwoodSwiftFormat),
        ])])
    }
}

impl LanguageFormatter for Swift {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::AppleSwiftFormat => format_using_swift_format(snippet_path),
            Self::NicklockwoodSwiftFormat => format_using_swiftformat(snippet_path),
        }
    }
}

impl core::fmt::Display for Swift {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AppleSwiftFormat => write!(f, "swift-format"),
            Self::NicklockwoodSwiftFormat => write!(f, "swiftformat"),
        }
    }
}

#[cfg(test)]
mod test_swift {
    use super::Swift;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
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
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(swiftformat)]
    #[test]
    fn test_swiftformat() {
        let l = Lang::<Swift> {
            enabled: true,
            formatter: MdsfFormatter::Single(Swift::NicklockwoodSwiftFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "func add(a: Int, b: Int) -> Int {
    return a + b
}
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(swift-format)]
    #[test]
    fn test_swift_format() {
        let l = Lang::<Swift> {
            enabled: true,
            formatter: MdsfFormatter::Single(Swift::AppleSwiftFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "func add(a: Int, b: Int) -> Int {
    return a + b
}
";

        assert_eq!(output, expected_output);
    }
}

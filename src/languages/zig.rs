use schemars::JsonSchema;

use crate::formatters::{zigfmt::format_using_zigfmt, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Zig {
    #[default]
    #[serde(rename = "zigfmt")]
    ZigFmt,
}

impl Default for Lang<Zig> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Zig>::default(),
        }
    }
}

impl Default for MdsfFormatter<Zig> {
    #[inline]
    fn default() -> Self {
        Self::Single(Zig::ZigFmt)
    }
}

impl LanguageFormatter for Zig {
    #[inline]
    fn format_single(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::ZigFmt => format_using_zigfmt(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_zig {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Zig;

    const INPUT: &str = "
    fn     add   (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }
    ";

    const EXTENSION: &str = crate::languages::Language::Zig.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Zig>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Lang::<Zig> {
            enabled: false,
            formatter: MdsfFormatter::Single(Zig::ZigFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l.format(snippet_path).expect("it to not fail").is_none());
    }

    #[test]
    fn test_zigfmt() {
        let l = Lang::<Zig> {
            enabled: true,
            formatter: MdsfFormatter::Single(Zig::ZigFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "fn add(a: i32, b: i32) i32 {
    return a + b;
}
";

        assert_eq!(output, expected_output);
    }
}

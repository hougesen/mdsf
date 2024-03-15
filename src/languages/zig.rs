use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{format_multiple, zigfmt::format_using_zigfmt, MdsfFormatter},
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum ZigFormatter {
    #[default]
    #[serde(rename = "zigfmt")]
    ZigFmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Zig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<ZigFormatter>,
}

impl Default for Zig {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<ZigFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<ZigFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Single(ZigFormatter::ZigFmt)
    }
}

impl LanguageFormatter<ZigFormatter> for Zig {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        format_multiple(&self.formatter, snippet_path, &Self::format_single)
            .map(|(_should_continue, output)| output)
    }

    #[inline]
    fn format_single(
        formatter: &ZigFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            ZigFormatter::ZigFmt => format_using_zigfmt(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_zig {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

    use super::{Zig, ZigFormatter};

    const INPUT: &str = "
    fn     add   (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }
    ";

    const EXTENSION: &str = crate::languages::Language::Zig.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Zig::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Zig {
            enabled: false,
            formatter: MdsfFormatter::Single(ZigFormatter::ZigFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l.format(snippet_path).expect("it to not fail").is_none());
    }

    #[test]
    fn test_zigfmt() {
        let l = Zig {
            enabled: true,
            formatter: MdsfFormatter::Single(ZigFormatter::ZigFmt),
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

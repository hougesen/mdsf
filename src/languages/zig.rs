use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::zigfmt::format_using_zigfmt};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum ZigFormatter {
    #[default]
    #[serde(rename = "zigfmt")]
    ZigFmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Zig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: ZigFormatter,
}

impl Default for Zig {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: ZigFormatter::default(),
        }
    }
}

impl LanguageFormatter for Zig {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            ZigFormatter::ZigFmt => format_using_zigfmt(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test_zig {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Zig, ZigFormatter};

    const INPUT: &str = "
    fn     add   (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }
    ";

    const EXTENSION: &str = crate::languages::Language::Zig.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        Zig::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Zig {
            enabled: false,
            formatter: ZigFormatter::ZigFmt,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l.format(snippet_path).expect("it to not fail").is_none());
    }

    #[test]
    fn test_zigfmt() {
        let l = Zig {
            enabled: true,
            formatter: ZigFormatter::ZigFmt,
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

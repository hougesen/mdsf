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
    use crate::{
        formatters::setup_snippet,
        languages::{Language, LanguageFormatter},
    };

    use super::{Zig, ZigFormatter};

    const INPUT: &str = "
    fn     add   (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }
    ";

    const EXPECTED_OUTPUT: &str = "fn add(a: i32, b: i32) i32 {
    return a + b;
}
";

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Zig {
            enabled: false,
            formatter: ZigFormatter::ZigFmt,
        };

        let snippet =
            setup_snippet(INPUT, Language::Zig.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l.format(snippet_path).expect("it to not fail").is_none());
    }

    #[test]
    fn zig_fmt() {
        let l = Zig {
            enabled: true,
            formatter: ZigFormatter::ZigFmt,
        };

        let snippet =
            setup_snippet(INPUT, Language::Zig.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(output, EXPECTED_OUTPUT);
    }
}

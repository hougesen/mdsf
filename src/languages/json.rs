use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{
        biome::format_using_biome, clang_format::format_using_clang_format,
        deno_format::format_using_deno_fmt, prettier::format_using_prettier,
    },
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum JsonFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "biome")]
    Biome,
    #[serde(rename = "deno_fmt")]
    DenoFmt,
    #[serde(rename = "clang-format")]
    ClangFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Json {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: JsonFormatter,
}

impl Default for Json {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: JsonFormatter::default(),
        }
    }
}

impl LanguageFormatter for Json {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            JsonFormatter::Biome => format_using_biome(snippet_path),
            JsonFormatter::Prettier => format_using_prettier(snippet_path, true),
            JsonFormatter::ClangFormat => format_using_clang_format(snippet_path),
            JsonFormatter::DenoFmt => format_using_deno_fmt(snippet_path),
        }
        .map(|res| res.1)
    }
}

#[cfg(test)]
mod test_json {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{Json, JsonFormatter};

    const INPUT: &str = "
              {
              \"key\": \"value\",
  \"key2\": [
      \"value2\",
      \"value3\",
      1
            , null]
 }
  ";

    const EXTENSION: &str = crate::languages::Language::Json.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Json::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Json {
            enabled: false,
            formatter: JsonFormatter::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Json {
            enabled: true,
            formatter: JsonFormatter::Prettier,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "{
  \"key\": \"value\",
  \"key2\": [\"value2\", \"value3\", 1, null],
}
";

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_biome() {
        let l = Json {
            enabled: true,
            formatter: JsonFormatter::Biome,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "{
\t\"key\": \"value\",
\t\"key2\": [\"value2\", \"value3\", 1, null]
}
";

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_clang_format() {
        let l = Json {
            enabled: true,
            formatter: JsonFormatter::ClangFormat,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output =
            "\n{ \"key\" : \"value\", \"key2\" : [ \"value2\", \"value3\", 1, null ] }\n";

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_deno_fmt() {
        let l = Json {
            enabled: true,
            formatter: JsonFormatter::DenoFmt,
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "{
  \"key\": \"value\",
  \"key2\": [
    \"value2\",
    \"value3\",
    1,
    null
  ]
}
";

        assert_eq!(output, expected_output);
    }
}

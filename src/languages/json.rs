use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{
        biome::format_using_biome, clang_format::format_using_clang_format,
        deno_format::format_using_deno_fmt, format_multiple, prettier::format_using_prettier,
        MdsfFormatter,
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
    pub formatter: MdsfFormatter<JsonFormatter>,
}

impl Default for Json {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<JsonFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<JsonFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![
            Self::Single(JsonFormatter::Biome),
            Self::Single(JsonFormatter::Prettier),
            Self::Single(JsonFormatter::DenoFmt),
            Self::Single(JsonFormatter::ClangFormat),
        ])
    }
}

impl LanguageFormatter<JsonFormatter> for Json {
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
        formatter: &JsonFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            JsonFormatter::Biome => format_using_biome(snippet_path),
            JsonFormatter::Prettier => format_using_prettier(snippet_path, true),
            JsonFormatter::ClangFormat => format_using_clang_format(snippet_path),
            JsonFormatter::DenoFmt => format_using_deno_fmt(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_json {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
    };

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
            formatter: MdsfFormatter::<JsonFormatter>::default(),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Json {
            enabled: true,
            formatter: MdsfFormatter::Single(JsonFormatter::Prettier),
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
            formatter: MdsfFormatter::Single(JsonFormatter::Biome),
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
            formatter: MdsfFormatter::Single(JsonFormatter::ClangFormat),
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
            formatter: MdsfFormatter::Single(JsonFormatter::DenoFmt),
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

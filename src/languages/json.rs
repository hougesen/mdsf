use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{
    biome::format_using_biome, clang_format::format_using_clang_format,
    deno_fmt::format_using_deno_fmt, prettier::format_using_prettier, MdsfFormatter,
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Json {
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

impl Default for Lang<Json> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Json>::default(),
        }
    }
}

impl Default for MdsfFormatter<Json> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Json::Prettier),
            Self::Single(Json::Biome),
            Self::Single(Json::DenoFmt),
            Self::Single(Json::ClangFormat),
        ])])
    }
}

impl LanguageFormatter for Json {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Biome => format_using_biome(snippet_path),
            Self::Prettier => format_using_prettier(snippet_path, true),
            Self::ClangFormat => format_using_clang_format(snippet_path),
            Self::DenoFmt => format_using_deno_fmt(snippet_path),
        }
    }
}

impl core::fmt::Display for Json {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Prettier => write!(f, "prettier"),
            Self::Biome => write!(f, "biome"),
            Self::DenoFmt => write!(f, "deno_fmt"),
            Self::ClangFormat => write!(f, "clang-format"),
        }
    }
}

#[cfg(test)]
mod test_json {
    use super::Json;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::{JsonFlavor, Lang},
        LineInfo,
    };

    const INPUT: &str = "{
              \"key\": \"value\",
  \"key2\": [
      \"value2\",
      \"value3\",
      1
            , null]
 }
  ";

    const EXTENSION: &str = crate::languages::Language::Json(JsonFlavor::Json).to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Json>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Json> {
            enabled: false,
            formatter: MdsfFormatter::<Json>::default(),
        }
        .format(snippet_path, &LineInfo::fake())
        .expect("it to not fail")
        .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Lang::<Json> {
            enabled: true,
            formatter: MdsfFormatter::Single(Json::Prettier),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "{
  \"key\": \"value\",
  \"key2\": [\"value2\", \"value3\", 1, null]
}
";

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_biome() {
        let l = Lang::<Json> {
            enabled: true,
            formatter: MdsfFormatter::Single(Json::Biome),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "{
\t\"key\": \"value\",
\t\"key2\": [\"value2\", \"value3\", 1, null]
}
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn test_clang_format() {
        let l = Lang::<Json> {
            enabled: true,
            formatter: MdsfFormatter::Single(Json::ClangFormat),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
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

    #[test_with::executable(deno)]
    #[test]
    fn test_deno_fmt() {
        let l = Lang::<Json> {
            enabled: true,
            formatter: MdsfFormatter::Single(Json::DenoFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
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

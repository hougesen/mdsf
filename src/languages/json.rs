use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{
        biome::format_using_biome, clang_format::format_using_clang_format,
        prettier::format_using_prettier,
    },
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
pub enum JsonFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "biome")]
    Biome,
    #[serde(rename = "clang-format")]
    ClangFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq))]
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
            JsonFormatter::Biome => format_using_biome(snippet_path).map(|res| res.1),
            JsonFormatter::Prettier => format_using_prettier(snippet_path, true).map(|res| res.1),
            JsonFormatter::ClangFormat => format_using_clang_format(snippet_path).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test {
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
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        Json::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
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
}

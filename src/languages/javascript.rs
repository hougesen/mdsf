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
pub enum JavaScriptFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "biome")]
    Biome,
    #[serde(rename = "clang-format")]
    ClangFormat,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct JavaScript {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: JavaScriptFormatter,
}

impl Default for JavaScript {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: JavaScriptFormatter::default(),
        }
    }
}

impl LanguageFormatter for JavaScript {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            JavaScriptFormatter::Biome => format_using_biome(snippet_path).map(|res| res.1),
            JavaScriptFormatter::Prettier => {
                format_using_prettier(snippet_path, true).map(|res| res.1)
            }
            JavaScriptFormatter::ClangFormat => {
                format_using_clang_format(snippet_path).map(|res| res.1)
            }
        }
    }
}

#[cfg(test)]
mod test_javascript {
    use crate::{
        formatters::setup_snippet,
        languages::{Language, LanguageFormatter},
    };

    use super::{JavaScript, JavaScriptFormatter};

    const INPUT: &str = "
    async function asyncAddition(
            a,b
        )
    {
        return a+b
    }

            ";

    #[test]
    fn it_should_be_enabled_by_default() {
        let snippet =
            setup_snippet(INPUT, Language::JavaScript.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        JavaScript::default()
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet =
            setup_snippet(INPUT, Language::JavaScript.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        let prettier = JavaScript {
            enabled: false,
            formatter: JavaScriptFormatter::Prettier,
        };

        assert!(prettier
            .format(snippet_path)
            .expect("it to not fail")
            .is_none());

        let biome = JavaScript {
            enabled: false,
            formatter: JavaScriptFormatter::Biome,
        };

        assert!(biome
            .format(snippet_path)
            .expect("it to not fail")
            .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = JavaScript {
            enabled: true,
            formatter: JavaScriptFormatter::Prettier,
        };

        let snippet =
            setup_snippet(INPUT, Language::JavaScript.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "async function asyncAddition(a, b) {
  return a + b;
}
";

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_biome() {
        let l = JavaScript {
            enabled: true,
            formatter: JavaScriptFormatter::Biome,
        };

        let snippet =
            setup_snippet(INPUT, Language::JavaScript.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "async function asyncAddition(a, b) {
\treturn a + b;
}
";

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_clang_format() {
        let input = "    async function asyncAddition(  a,b) {
            a * b;
        return a+b
    }            ";

        let expected_output = "async function asyncAddition(a, b) {\n  a * b;\n  return a + b\n}";

        let l = JavaScript {
            enabled: true,
            formatter: JavaScriptFormatter::ClangFormat,
        };

        let snippet =
            setup_snippet(input, Language::JavaScript.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(expected_output, output);
    }
}

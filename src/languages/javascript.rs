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
pub enum JavaScriptFormatter {
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
pub struct JavaScript {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<JavaScriptFormatter>,
}

impl Default for JavaScript {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<JavaScriptFormatter>::default(),
        }
    }
}

impl Default for MdsfFormatter<JavaScriptFormatter> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![
            Self::Single(JavaScriptFormatter::Biome),
            Self::Single(JavaScriptFormatter::Prettier),
            Self::Single(JavaScriptFormatter::DenoFmt),
            Self::Single(JavaScriptFormatter::ClangFormat),
        ])
    }
}

impl LanguageFormatter<JavaScriptFormatter> for JavaScript {
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
        formatter: &JavaScriptFormatter,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            JavaScriptFormatter::Biome => format_using_biome(snippet_path),
            JavaScriptFormatter::Prettier => format_using_prettier(snippet_path, true),
            JavaScriptFormatter::ClangFormat => format_using_clang_format(snippet_path),
            JavaScriptFormatter::DenoFmt => format_using_deno_fmt(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_javascript {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::LanguageFormatter,
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

    const EXTENSION: &str = crate::languages::Language::JavaScript.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(JavaScript::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let prettier = JavaScript {
            enabled: false,
            formatter: MdsfFormatter::Single(JavaScriptFormatter::Prettier),
        };

        assert!(prettier
            .format(snippet_path)
            .expect("it to not fail")
            .is_none());

        let biome = JavaScript {
            enabled: false,
            formatter: MdsfFormatter::Single(JavaScriptFormatter::Biome),
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
            formatter: MdsfFormatter::Single(JavaScriptFormatter::Prettier),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
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
            formatter: MdsfFormatter::Single(JavaScriptFormatter::Biome),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
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
            formatter: MdsfFormatter::Single(JavaScriptFormatter::ClangFormat),
        };

        let snippet = setup_snippet(input, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_deno_fmt() {
        let input = "    async function asyncAddition(  a,b) {
            a * b;
        return a+b
    }            ";

        let expected_output =
            "async function asyncAddition(a, b) {\n  a * b;\n  return a + b;\n}\n";

        let l = JavaScript {
            enabled: true,
            formatter: MdsfFormatter::Single(JavaScriptFormatter::DenoFmt),
        };

        let snippet = setup_snippet(input, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        assert_eq!(expected_output, output);
    }
}

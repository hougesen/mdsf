use schemars::JsonSchema;

use crate::formatters::{
    biome::format_using_biome, clang_format::format_using_clang_format,
    deno_format::format_using_deno_fmt, prettier::format_using_prettier, MdsfFormatter,
};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum JavaScript {
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

impl Default for Lang<JavaScript> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<JavaScript>::default(),
        }
    }
}

impl Default for MdsfFormatter<JavaScript> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(JavaScript::Prettier),
            Self::Single(JavaScript::Biome),
            Self::Single(JavaScript::DenoFmt),
            Self::Single(JavaScript::ClangFormat),
        ])])
    }
}

impl LanguageFormatter for JavaScript {
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

#[cfg(test)]
mod test_javascript {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::JavaScript;

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
        assert!(Lang::<JavaScript>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let prettier = Lang::<JavaScript> {
            enabled: false,
            formatter: MdsfFormatter::Single(JavaScript::Prettier),
        };

        assert!(prettier
            .format(snippet_path)
            .expect("it to not fail")
            .is_none());

        let biome = Lang::<JavaScript> {
            enabled: false,
            formatter: MdsfFormatter::Single(JavaScript::Biome),
        };

        assert!(biome
            .format(snippet_path)
            .expect("it to not fail")
            .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Lang::<JavaScript> {
            enabled: true,
            formatter: MdsfFormatter::Single(JavaScript::Prettier),
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
        let l = Lang::<JavaScript> {
            enabled: true,
            formatter: MdsfFormatter::Single(JavaScript::Biome),
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

        let l = Lang::<JavaScript> {
            enabled: true,
            formatter: MdsfFormatter::Single(JavaScript::ClangFormat),
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

        let l = Lang::<JavaScript> {
            enabled: true,
            formatter: MdsfFormatter::Single(JavaScript::DenoFmt),
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

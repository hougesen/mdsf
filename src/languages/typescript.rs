use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::formatters::{
    biome::format_using_biome, deno_fmt::format_using_deno_fmt, prettier::format_using_prettier,
    MdsfFormatter,
};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum TypeScript {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "biome")]
    Biome,
    #[serde(rename = "deno_fmt")]
    DenoFmt,
}

impl Default for Lang<TypeScript> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<TypeScript>::default(),
        }
    }
}

impl Default for MdsfFormatter<TypeScript> {
    #[inline]
    fn default() -> Self {
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(TypeScript::Prettier),
            Self::Single(TypeScript::Biome),
            Self::Single(TypeScript::DenoFmt),
        ])])
    }
}

impl LanguageFormatter for TypeScript {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Biome => format_using_biome(snippet_path),
            Self::Prettier => format_using_prettier(snippet_path, true),
            Self::DenoFmt => format_using_deno_fmt(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_typescript {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::{typescript::TypeScript, Lang, TypeScriptFlavor},
    };

    const INPUT: &str = "
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            ";

    const EXTENSION: &str =
        crate::languages::Language::TypeScript(TypeScriptFlavor::TypeScript).to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<TypeScript>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let prettier = Lang::<TypeScript> {
            enabled: false,
            formatter: MdsfFormatter::Single(TypeScript::Prettier),
        };

        assert!(prettier
            .format(snippet_path)
            .expect("it to not fail")
            .is_none());

        let biome = Lang::<TypeScript> {
            enabled: false,
            formatter: MdsfFormatter::Single(TypeScript::Biome),
        };

        assert!(biome
            .format(snippet_path)
            .expect("it to not fail")
            .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Lang::<TypeScript> {
            enabled: true,
            formatter: MdsfFormatter::Single(TypeScript::Prettier),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output =
            "async function asyncAddition(a: number, b: number): Promise<number> {
  return a + b;
}
";

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_biome() {
        let l = Lang::<TypeScript> {
            enabled: true,
            formatter: MdsfFormatter::Single(TypeScript::Biome),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output =
            "async function asyncAddition(a: number, b: number): Promise<number> {
\treturn a + b;
}
";

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(deno)]
    #[test]
    fn test_deno_fmt() {
        let l = Lang::<TypeScript> {
            enabled: true,
            formatter: MdsfFormatter::Single(TypeScript::DenoFmt),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "async function asyncAddition(
  a: number,
  b: number,
): Promise<
  number
> {
  return a + b;
}
";

        assert_eq!(output, expected_output);
    }
}

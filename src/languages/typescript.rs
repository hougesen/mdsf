use schemars::JsonSchema;

use crate::{
    config::default_enabled,
    formatters::{
        biome::format_using_biome, deno_format::format_using_deno_fmt,
        prettier::format_using_prettier,
    },
};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum TypeScriptFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
    #[serde(rename = "biome")]
    Biome,
    #[serde(rename = "deno_fmt")]
    DenoFmt,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct TypeScript {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: TypeScriptFormatter,
}

impl Default for TypeScript {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: TypeScriptFormatter::default(),
        }
    }
}

impl LanguageFormatter for TypeScript {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            TypeScriptFormatter::Biome => format_using_biome(snippet_path),
            TypeScriptFormatter::Prettier => format_using_prettier(snippet_path, true),
            TypeScriptFormatter::DenoFmt => format_using_deno_fmt(snippet_path),
        }
        .map(|res| res.1)
    }
}

#[cfg(test)]
mod test_typescript {
    use crate::{formatters::setup_snippet, languages::LanguageFormatter};

    use super::{TypeScript, TypeScriptFormatter};

    const INPUT: &str = "
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            ";

    const EXTENSION: &str = crate::languages::Language::TypeScript.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(TypeScript::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let prettier = TypeScript {
            enabled: false,
            formatter: TypeScriptFormatter::Prettier,
        };

        assert!(prettier
            .format(snippet_path)
            .expect("it to not fail")
            .is_none());

        let biome = TypeScript {
            enabled: false,
            formatter: TypeScriptFormatter::Biome,
        };

        assert!(biome
            .format(snippet_path)
            .expect("it to not fail")
            .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = TypeScript {
            enabled: true,
            formatter: TypeScriptFormatter::Prettier,
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
        let l = TypeScript {
            enabled: true,
            formatter: TypeScriptFormatter::Biome,
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

    #[test]
    fn test_deno_fmt() {
        let l = TypeScript {
            enabled: true,
            formatter: TypeScriptFormatter::DenoFmt,
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

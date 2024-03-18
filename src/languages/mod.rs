use schemars::JsonSchema;

use crate::formatters::MdsfFormatter;

pub enum Language {
    Blade,
    C,
    Crystal,
    Cpp,
    CSharp,
    Css,
    Dart,
    Elm,
    Elixir,
    Gleam,
    GraphQL,
    Go,
    Html,
    Java,
    JavaScript,
    Json,
    Just,
    Lua,
    Markdown,
    Nim,
    ObjectiveC,
    OCaml,
    Protobuf,
    Python,
    ReScript,
    Roc,
    Ruby,
    Rust,
    Shell,
    Sql,
    Toml,
    TypeScript,
    Vue,
    Xml,
    Yaml,
    Zig,
    // TODO: Haskell,
    // TODO: PHP,
    // TODO: Kotlin,
    // TODO: FSharp,
    // TODO: Swift,
    // TODO: Svelte,
    // TODO: Julia,
    // TODO: Dockerfile,
    // TODO: XML,
    // TODO: D,
    // TODO: Erlang,
    // TODO: Scala,
    // TODO: R,
}

pub mod blade;
pub mod c;
pub mod cpp;
pub mod crystal;
pub mod csharp;
pub mod css;
pub mod dart;
pub mod elixir;
pub mod elm;
pub mod gleam;
pub mod go;
pub mod graphql;
pub mod html;
pub mod java;
pub mod javascript;
pub mod json;
pub mod just;
pub mod lua;
pub mod markdown;
pub mod nim;
pub mod objective_c;
pub mod ocaml;
pub mod protobuf;
pub mod python;
pub mod rescript;
pub mod roc;
pub mod ruby;
pub mod rust;
pub mod shell;
pub mod sql;
pub mod toml;
pub mod typescript;
pub mod vue;
pub mod xml;
pub mod yaml;
pub mod zig;

pub trait LanguageFormatter {
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)>;
}

impl Language {
    #[inline]
    pub fn maybe_from_str(input: &str) -> Option<Self> {
        match input {
            "blade" => Some(Self::Blade),
            "c" | "clang" => Some(Self::C),
            "cpp" | "c++" => Some(Self::Cpp),
            "crystal" | "cr" => Some(Self::Crystal),
            "csharp" | "c#" => Some(Self::CSharp),
            "css" | "scss" => Some(Self::Css),
            "dart" => Some(Self::Dart),
            "elixir" => Some(Self::Elixir),
            "gleam" => Some(Self::Gleam),
            "go" | "golang" => Some(Self::Go),
            "html" => Some(Self::Html),
            "java" => Some(Self::Java),
            "javascript" | "js" | "jsx" => Some(Self::JavaScript),
            "json" | "jsonc" => Some(Self::Json),
            "just" | "justfile" => Some(Self::Just),
            "lua" => Some(Self::Lua),
            "markdown" | "md" => Some(Self::Markdown),
            "nim" => Some(Self::Nim),
            "objectivec" | "objective-c" | "objc" => Some(Self::ObjectiveC),
            "ocaml" => Some(Self::OCaml),
            "profobuf" | "profo" => Some(Self::Protobuf),
            "python" => Some(Self::Python),
            "roc" => Some(Self::Roc),
            "ruby" => Some(Self::Ruby),
            "rust" | "rb" => Some(Self::Rust),
            "shell" | "sh" | "bash" | "zsh" => Some(Self::Shell),
            "sql" | "bigquery" | "db2" | "db2i" | "hive" | "mariadb" | "mysql" | "n1ql"
            | "plsql" | "postgresql" | "redshift" | "singlestoredb" | "snowflake" | "spark"
            | "sqlite" | "transactsql" | "trino" | "tsql" => Some(Self::Sql),
            "toml" => Some(Self::Toml),
            "typescript" | "ts" | "tsx" => Some(Self::TypeScript),
            "vue" => Some(Self::Vue),
            "yml" | "yaml" => Some(Self::Yaml),
            "zig" => Some(Self::Zig),
            "graphql" | "gql" => Some(Self::GraphQL),
            "elm" => Some(Self::Elm),
            "rescript" => Some(Self::ReScript),
            "xml" => Some(Self::Xml),
            _ => None,
        }
    }

    #[inline]
    pub const fn to_file_ext(&self) -> &'static str {
        match self {
            // NOTE: since scss is a superset of css we might as well support both at the same time
            Self::C => ".c",
            Self::Cpp => ".cpp",
            Self::Crystal => ".cr",
            Self::CSharp => ".cs",
            Self::Css => ".scss",
            Self::Dart => ".dart",
            Self::Elixir => ".ex",
            Self::Gleam => ".gleam",
            Self::Go => ".go",
            Self::Html => ".html",
            Self::Java => ".java",
            Self::JavaScript => ".js",
            Self::Json => ".jsonc",
            // NOTE: just does not have any file ext
            Self::Just => ".justfile",
            Self::Lua => ".lua",
            Self::Markdown => ".md",
            Self::Nim => ".nim",
            Self::ObjectiveC => ".m",
            Self::Protobuf => ".proto",
            Self::Python => ".py",
            Self::Roc => ".roc",
            Self::Ruby => ".rb",
            Self::Rust => ".rs",
            Self::Shell => ".sh",
            Self::Sql => ".sql",
            Self::Toml => ".toml",
            Self::TypeScript => ".ts",
            Self::Vue => ".vue",
            Self::Yaml => ".yml",
            Self::Zig => ".zig",
            Self::GraphQL => ".gql",
            Self::Elm => ".elm",
            Self::Blade => ".blade.php",
            Self::OCaml => ".ml",
            Self::ReScript => ".res",
            Self::Xml => ".xml",
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct Lang<T>
where
    T: LanguageFormatter,
{
    pub enabled: bool,
    pub formatter: MdsfFormatter<T>,
}

impl<T: LanguageFormatter> Lang<T> {
    #[inline]
    pub fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        Self::format_multiple(&self.formatter, snippet_path, false)
            .map(|(_should_continue, output)| output)
    }

    #[inline]
    pub fn format_multiple(
        formatter: &MdsfFormatter<T>,
        snippet_path: &std::path::Path,
        nested: bool,
    ) -> std::io::Result<(bool, Option<String>)> {
        match formatter {
            MdsfFormatter::Single(f) => f.format_snippet(snippet_path),

            MdsfFormatter::Multiple(formatters) => {
                let mut r = Ok((true, None));

                for f in formatters {
                    r = Self::format_multiple(f, snippet_path, true);

                    if r.as_ref()
                        .is_ok_and(|(should_continue, _code)| !should_continue)
                        && nested
                    {
                        break;
                    }
                }

                r
            }
        }
    }
}

#[cfg(test)]
mod test_lang {
    use std::io::Write;

    use crate::formatters::{setup_snippet, MdsfFormatter};

    use super::{Lang, LanguageFormatter};

    enum TestLanguage {
        A,
        B,
        C,
        // Will fail
        D,
        E,
    }

    impl LanguageFormatter for TestLanguage {
        fn format_snippet(
            &self,
            snippet_path: &std::path::Path,
        ) -> std::io::Result<(bool, Option<String>)> {
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(snippet_path)?;

            let (should_fail, _) = match self {
                Self::A => (false, file.write(b"a")),
                Self::B => (false, file.write(b"b")),
                Self::C => (false, file.write(b"c")),
                Self::D | Self::E => (true, Ok(0)),
            };

            Ok((should_fail, std::fs::read_to_string(snippet_path).ok()))
        }
    }

    #[test]
    fn test_single() {
        let l = Lang::<TestLanguage> {
            enabled: true,
            formatter: MdsfFormatter::Single(TestLanguage::A),
        };

        let file = setup_snippet("", ".txt").expect("it to create a snippet");

        let snippet_path = file.path();

        let code = l
            .format(snippet_path)
            .expect("it to be ok")
            .expect("it to be some");

        assert_eq!("a", code);
    }

    #[test]
    fn test_multiple_sequentially() {
        let l = Lang::<TestLanguage> {
            enabled: true,
            formatter: MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(TestLanguage::A),
                MdsfFormatter::Single(TestLanguage::B),
                MdsfFormatter::Single(TestLanguage::C),
            ]),
        };

        let file = setup_snippet("", ".txt").expect("it to create a snippet");

        let snippet_path = file.path();

        let code = l
            .format(snippet_path)
            .expect("it to be ok")
            .expect("it to be some");

        assert_eq!("abc", code);
    }

    #[test]
    fn test_multiple_fallback() {
        let l = Lang::<TestLanguage> {
            enabled: true,
            formatter: MdsfFormatter::Multiple(vec![
                MdsfFormatter::Multiple(vec![
                    MdsfFormatter::Single(TestLanguage::D),
                    MdsfFormatter::Single(TestLanguage::A),
                ]),
                MdsfFormatter::Multiple(vec![
                    MdsfFormatter::Single(TestLanguage::E),
                    MdsfFormatter::Single(TestLanguage::B),
                ]),
                MdsfFormatter::Single(TestLanguage::C),
            ]),
        };

        let file = setup_snippet("", ".txt").expect("it to create a snippet");

        let snippet_path = file.path();

        let code = l
            .format(snippet_path)
            .expect("it to be ok")
            .expect("it to be some");

        assert_eq!("abc", code);
    }
}

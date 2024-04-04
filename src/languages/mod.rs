use schemars::JsonSchema;

use crate::formatters::MdsfFormatter;

pub mod blade;
pub mod c;
pub mod cabal;
pub mod clojure;
pub mod cpp;
pub mod crystal;
pub mod csharp;
pub mod css;
pub mod dart;
pub mod elixir;
pub mod elm;
pub mod erlang;
pub mod fsharp;
pub mod gleam;
pub mod go;
pub mod graphql;
pub mod groovy;
pub mod haskell;
pub mod hcl;
pub mod html;
pub mod java;
pub mod javascript;
pub mod json;
pub mod julia;
pub mod just;
pub mod kotlin;
pub mod lua;
pub mod markdown;
pub mod nim;
pub mod nix;
pub mod objective_c;
pub mod ocaml;
pub mod perl;
pub mod protobuf;
pub mod purescript;
pub mod python;
pub mod rescript;
pub mod roc;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod shell;
pub mod sql;
pub mod swift;
pub mod toml;
pub mod typescript;
pub mod vue;
pub mod xml;
pub mod yaml;
pub mod zig;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JsonFlavor {
    Json,
    JsonC,
    Json5,
}

impl core::fmt::Display for JsonFlavor {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Json => f.write_str("json"),
            Self::JsonC => f.write_str("jsonc"),
            Self::Json5 => f.write_str("json5"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CssFlavor {
    Css,
    Scss,
    Sass,
    Less,
}

impl core::fmt::Display for CssFlavor {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Css => f.write_str("css"),
            Self::Scss => f.write_str("scss"),
            Self::Sass => f.write_str("sass"),
            Self::Less => f.write_str("less"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShellFlavor {
    Shell,
    Bash,
    Zsh,
}

impl core::fmt::Display for ShellFlavor {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Shell => f.write_str("shell"),
            Self::Bash => f.write_str("bash"),
            Self::Zsh => f.write_str("zsh"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JavaScriptFlavor {
    JavaScript,
    JSX,
}

impl core::fmt::Display for JavaScriptFlavor {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::JavaScript => f.write_str("javascript"),
            Self::JSX => f.write_str("jsx"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TypeScriptFlavor {
    TypeScript,
    TSX,
}

impl core::fmt::Display for TypeScriptFlavor {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TypeScript => f.write_str("typescript"),
            Self::TSX => f.write_str("tsx"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Blade,
    C,
    Cabal,
    Clojure,
    CSharp,
    Cpp,
    Crystal,
    Css(CssFlavor),
    Dart,
    Elixir,
    Elm,
    Erlang,
    FSharp,
    Gleam,
    Go,
    GraphQL,
    Groovy,
    Haskell,
    Html,
    Java,
    JavaScript(JavaScriptFlavor),
    Json(JsonFlavor),
    Julia,
    Just,
    Kotlin,
    Lua,
    Markdown,
    Nim,
    Nix,
    OCaml,
    ObjectiveC,
    Perl,
    Protobuf,
    PureScript,
    Python,
    ReScript,
    Roc,
    Ruby,
    Rust,
    Scala,
    Shell(ShellFlavor),
    Sql,
    Swift,
    Hcl,
    Toml,
    TypeScript(TypeScriptFlavor),
    Vue,
    Xml,
    Yaml,
    Zig,
    // TODO: PHP,
    // TODO: Svelte,
    // TODO: Dockerfile,
    // TODO: XML,
    // TODO: D,
    // TODO: R,
}

impl core::fmt::Display for Language {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Blade => f.write_str("blade"),
            Self::C => f.write_str("c"),
            Self::Cabal => f.write_str("cabal"),
            Self::Clojure => f.write_str("clojure"),
            Self::CSharp => f.write_str("c#"),
            Self::Cpp => f.write_str("c++"),
            Self::Crystal => f.write_str("crystal"),
            Self::Css(flavor) => flavor.fmt(f),
            Self::Dart => f.write_str("dart"),
            Self::Elixir => f.write_str("elixir"),
            Self::Elm => f.write_str("elm"),
            Self::Erlang => f.write_str("erlang"),
            Self::FSharp => f.write_str("f#"),
            Self::Gleam => f.write_str("gleam"),
            Self::Go => f.write_str("go"),
            Self::GraphQL => f.write_str("graphql"),
            Self::Groovy => f.write_str("groovy"),
            Self::Haskell => f.write_str("haskell"),
            Self::Html => f.write_str("html"),
            Self::Java => f.write_str("java"),
            Self::JavaScript(flavor) => flavor.fmt(f),
            Self::Json(flavor) => flavor.fmt(f),
            Self::Julia => f.write_str("julia"),
            Self::Just => f.write_str("just"),
            Self::Kotlin => f.write_str("kotlin"),
            Self::Lua => f.write_str("lua"),
            Self::Markdown => f.write_str("markdown"),
            Self::Nim => f.write_str("nim"),
            Self::OCaml => f.write_str("ocaml"),
            Self::ObjectiveC => f.write_str("objective c"),
            Self::Perl => f.write_str("perl"),
            Self::Protobuf => f.write_str("protobuf"),
            Self::PureScript => f.write_str("purescript"),
            Self::Python => f.write_str("python"),
            Self::ReScript => f.write_str("rescript"),
            Self::Roc => f.write_str("roc"),
            Self::Ruby => f.write_str("ruby"),
            Self::Rust => f.write_str("rust"),
            Self::Scala => f.write_str("scala"),
            Self::Shell(flavor) => flavor.fmt(f),
            Self::Sql => f.write_str("sql"),
            Self::Swift => f.write_str("swift"),
            Self::Hcl => f.write_str("hcl"),
            Self::Toml => f.write_str("toml"),
            Self::TypeScript(flavor) => flavor.fmt(f),
            Self::Vue => f.write_str("vue"),
            Self::Xml => f.write_str("xml"),
            Self::Yaml => f.write_str("yaml"),
            Self::Zig => f.write_str("zig"),
            Self::Nix => f.write_str("nix"),
        }
    }
}

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
            "cabal" => Some(Self::Cabal),
            "cpp" | "c++" => Some(Self::Cpp),
            "crystal" | "cr" => Some(Self::Crystal),
            "csharp" | "c#" => Some(Self::CSharp),
            "css" => Some(Self::Css(CssFlavor::Css)),
            "scss" => Some(Self::Css(CssFlavor::Scss)),
            "sass" => Some(Self::Css(CssFlavor::Sass)),
            "less" => Some(Self::Css(CssFlavor::Less)),
            "dart" => Some(Self::Dart),
            "elixir" => Some(Self::Elixir),
            "elm" => Some(Self::Elm),
            "erlang" => Some(Self::Erlang),
            "fsharp" => Some(Self::FSharp),
            "gleam" => Some(Self::Gleam),
            "go" | "golang" => Some(Self::Go),
            "graphql" | "gql" => Some(Self::GraphQL),
            "groovy" => Some(Self::Groovy),
            "haskell" => Some(Self::Haskell),
            "html" | "html5" => Some(Self::Html),
            "java" => Some(Self::Java),
            "javascript" | "js" => Some(Self::JavaScript(JavaScriptFlavor::JavaScript)),
            "jsx" => Some(Self::JavaScript(JavaScriptFlavor::JSX)),
            "json" => Some(Self::Json(JsonFlavor::Json)),
            "jsonc" => Some(Self::Json(JsonFlavor::JsonC)),
            "json5" => Some(Self::Json(JsonFlavor::Json5)),
            "julia" | "jl" => Some(Self::Julia),
            "just" | "justfile" => Some(Self::Just),
            "kotlin" => Some(Self::Kotlin),
            "lua" => Some(Self::Lua),
            "markdown" | "md" => Some(Self::Markdown),
            "nim" => Some(Self::Nim),
            "objectivec" | "objective-c" | "objc" => Some(Self::ObjectiveC),
            "ocaml" => Some(Self::OCaml),
            "perl" => Some(Self::Perl),
            "profobuf" | "profo" => Some(Self::Protobuf),
            "python" => Some(Self::Python),
            "purescript" => Some(Self::PureScript),
            "rescript" => Some(Self::ReScript),
            "roc" => Some(Self::Roc),
            "ruby" | "rb" => Some(Self::Ruby),
            "rust" | "rs" => Some(Self::Rust),
            "scala" => Some(Self::Scala),
            "shell" | "sh" | "shell-script" => Some(Self::Shell(ShellFlavor::Shell)),
            "bash" => Some(Self::Shell(ShellFlavor::Bash)),
            "zsh" => Some(Self::Shell(ShellFlavor::Zsh)),
            "sql" | "bigquery" | "db2" | "db2i" | "hive" | "mariadb" | "mysql" | "n1ql"
            | "plsql" | "postgresql" | "redshift" | "singlestoredb" | "snowflake" | "spark"
            | "sqlite" | "transactsql" | "trino" | "tsql" => Some(Self::Sql),
            "swift" => Some(Self::Swift),
            "terraform" | "hcl" | "opentofu" => Some(Self::Hcl),
            "toml" => Some(Self::Toml),
            "typescript" | "ts" => Some(Self::TypeScript(TypeScriptFlavor::TypeScript)),
            "tsx" => Some(Self::TypeScript(TypeScriptFlavor::TSX)),
            "vue" => Some(Self::Vue),
            "xml" => Some(Self::Xml),
            "yml" | "yaml" => Some(Self::Yaml),
            "zig" => Some(Self::Zig),
            "clojure" => Some(Self::Clojure),
            "nix" | "nixos" => Some(Self::Nix),
            _ => None,
        }
    }

    #[inline]
    pub const fn to_file_ext(&self) -> &'static str {
        match self {
            Self::Blade => ".blade.php",
            Self::C => ".c",
            Self::Cabal => ".cabal",
            Self::CSharp => ".cs",
            Self::Cpp => ".cpp",
            Self::Crystal => ".cr",
            Self::Css(CssFlavor::Css) => ".css",
            Self::Css(CssFlavor::Scss) => ".scss",
            Self::Css(CssFlavor::Sass) => ".sass",
            Self::Css(CssFlavor::Less) => ".less",
            Self::Dart => ".dart",
            Self::Elixir => ".ex",
            Self::Elm => ".elm",
            Self::Erlang => ".erl",
            Self::FSharp => ".fs",
            Self::Gleam => ".gleam",
            Self::Go => ".go",
            Self::GraphQL => ".gql",
            Self::Groovy => ".groovy",
            Self::Haskell => ".hs",
            Self::Html => ".html",
            Self::Java => ".java",
            Self::JavaScript(JavaScriptFlavor::JavaScript) => ".js",
            Self::JavaScript(JavaScriptFlavor::JSX) => ".jsx",
            Self::Json(JsonFlavor::Json) => ".json",
            Self::Json(JsonFlavor::JsonC) => ".jsonc",
            Self::Json(JsonFlavor::Json5) => ".json5",
            Self::Julia => ".jl",
            Self::Just => ".justfile",
            Self::Kotlin => ".kt",
            Self::Lua => ".lua",
            Self::Markdown => ".md",
            Self::Nim => ".nim",
            Self::OCaml => ".ml",
            Self::ObjectiveC => ".m",
            Self::Perl => ".pl",
            Self::Nix => ".nix",
            Self::Protobuf => ".proto",
            Self::PureScript => ".purs",
            Self::Python => ".py",
            Self::ReScript => ".res",
            Self::Roc => ".roc",
            Self::Ruby => ".rb",
            Self::Rust => ".rs",
            Self::Shell(ShellFlavor::Shell) => ".sh",
            Self::Shell(ShellFlavor::Bash) => ".bash",
            Self::Shell(ShellFlavor::Zsh) => ".zsh",
            Self::Scala => ".scala",
            Self::Sql => ".sql",
            Self::Swift => ".swift",
            Self::Hcl => ".tf",
            Self::Toml => ".toml",
            Self::TypeScript(TypeScriptFlavor::TypeScript) => ".ts",
            Self::TypeScript(TypeScriptFlavor::TSX) => ".tsx",
            Self::Vue => ".vue",
            Self::Xml => ".xml",
            Self::Yaml => ".yml",
            Self::Zig => ".zig",
            Self::Clojure => ".clj",
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

    use super::{Lang, LanguageFormatter};
    use crate::formatters::{setup_snippet, MdsfFormatter};

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

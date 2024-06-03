use schemars::JsonSchema;

use crate::{
    error::MdsfError,
    formatters::MdsfFormatter,
    terminal::{
        print_binary_not_in_path, print_error_formatting, print_formatter_info,
        print_formatter_time,
    },
    LineInfo,
};

pub mod assembly;
pub mod bazel;
pub mod bicep;
pub mod blade;
pub mod c;
pub mod cabal;
pub mod clojure;
pub mod cpp;
pub mod crystal;
pub mod csharp;
pub mod css;
pub mod d;
pub mod dart;
pub mod elixir;
pub mod elm;
pub mod erb;
pub mod erlang;
pub mod fennel;
pub mod fortran;
pub mod fsharp;
pub mod gdscript;
pub mod gleam;
pub mod go;
pub mod graphql;
pub mod groovy;
pub mod handlebars;
pub mod haskell;
pub mod hcl;
pub mod html;
pub mod java;
pub mod javascript;
pub mod json;
pub mod julia;
pub mod just;
pub mod kcl;
pub mod kotlin;
pub mod lua;
pub mod markdown;
pub mod mustache;
pub mod nim;
pub mod nix;
pub mod nunjucks;
pub mod objective_c;
pub mod ocaml;
pub mod perl;
pub mod protobuf;
pub mod puppet;
pub mod purescript;
pub mod python;
pub mod rescript;
pub mod restructuredtext;
pub mod roc;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod shell;
pub mod solidity;
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
    Fish,
}

impl core::fmt::Display for ShellFlavor {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Shell => f.write_str("shell"),
            Self::Bash => f.write_str("bash"),
            Self::Zsh => f.write_str("zsh"),
            Self::Fish => f.write_str("fish"),
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
    Assembly,
    Bazel,
    Bicep,
    Blade,
    C,
    CSharp,
    Cabal,
    Clojure,
    Cpp,
    Crystal,
    Css(CssFlavor),
    D,
    Dart,
    Elixir,
    Elm,
    Erb,
    Erlang,
    Fennel,
    FSharp,
    Fortran,
    Gdscript,
    Gleam,
    Go,
    GraphQL,
    Groovy,
    Handlebars,
    Haskell,
    Hcl,
    Html,
    Java,
    JavaScript(JavaScriptFlavor),
    Json(JsonFlavor),
    Julia,
    Just,
    Kcl,
    Kotlin,
    Lua,
    Markdown,
    Mustache,
    Nim,
    Nix,
    Nunjucks,
    OCaml,
    ObjectiveC,
    Perl,
    Protobuf,
    Puppet,
    PureScript,
    Python,
    ReScript,
    ReStructuredText,
    Roc,
    Ruby,
    Rust,
    Scala,
    Shell(ShellFlavor),
    Solidity,
    Sql,
    Swift,
    Toml,
    TypeScript(TypeScriptFlavor),
    Vue,
    Xml,
    Yaml,
    Zig,
}

impl core::fmt::Display for Language {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Assembly => f.write_str("assembly"),
            Self::Bazel => f.write_str("bazel"),
            Self::Bicep => f.write_str("bicep"),
            Self::Blade => f.write_str("blade"),
            Self::C => f.write_str("c"),
            Self::CSharp => f.write_str("c#"),
            Self::Cabal => f.write_str("cabal"),
            Self::Clojure => f.write_str("clojure"),
            Self::Cpp => f.write_str("c++"),
            Self::Crystal => f.write_str("crystal"),
            Self::Css(flavor) => flavor.fmt(f),
            Self::D => f.write_str("d"),
            Self::Dart => f.write_str("dart"),
            Self::Elixir => f.write_str("elixir"),
            Self::Elm => f.write_str("elm"),
            Self::Erb => f.write_str("erb"),
            Self::Erlang => f.write_str("erlang"),
            Self::Fennel => f.write_str("fennel"),
            Self::FSharp => f.write_str("f#"),
            Self::Fortran => f.write_str("fortran"),
            Self::Gdscript => f.write_str("gdscript"),
            Self::Gleam => f.write_str("gleam"),
            Self::Go => f.write_str("go"),
            Self::GraphQL => f.write_str("graphql"),
            Self::Groovy => f.write_str("groovy"),
            Self::Handlebars => f.write_str("handlebars"),
            Self::Haskell => f.write_str("haskell"),
            Self::Hcl => f.write_str("hcl"),
            Self::Html => f.write_str("html"),
            Self::Java => f.write_str("java"),
            Self::JavaScript(flavor) => flavor.fmt(f),
            Self::Json(flavor) => flavor.fmt(f),
            Self::Julia => f.write_str("julia"),
            Self::Just => f.write_str("just"),
            Self::Kcl => f.write_str("kcl"),
            Self::Kotlin => f.write_str("kotlin"),
            Self::Lua => f.write_str("lua"),
            Self::Markdown => f.write_str("markdown"),
            Self::Mustache => f.write_str("mustache"),
            Self::Nim => f.write_str("nim"),
            Self::Nix => f.write_str("nix"),
            Self::Nunjucks => f.write_str("nunjucks"),
            Self::OCaml => f.write_str("ocaml"),
            Self::ObjectiveC => f.write_str("objective c"),
            Self::Perl => f.write_str("perl"),
            Self::Protobuf => f.write_str("protobuf"),
            Self::Puppet => f.write_str("puppet"),
            Self::PureScript => f.write_str("purescript"),
            Self::Python => f.write_str("python"),
            Self::ReScript => f.write_str("rescript"),
            Self::ReStructuredText => f.write_str("restructuredtext"),
            Self::Roc => f.write_str("roc"),
            Self::Ruby => f.write_str("ruby"),
            Self::Rust => f.write_str("rust"),
            Self::Scala => f.write_str("scala"),
            Self::Shell(flavor) => flavor.fmt(f),
            Self::Solidity => f.write_str("solidity"),
            Self::Sql => f.write_str("sql"),
            Self::Swift => f.write_str("swift"),
            Self::Toml => f.write_str("toml"),
            Self::TypeScript(flavor) => flavor.fmt(f),
            Self::Vue => f.write_str("vue"),
            Self::Xml => f.write_str("xml"),
            Self::Yaml => f.write_str("yaml"),
            Self::Zig => f.write_str("zig"),
        }
    }
}

pub trait LanguageFormatter {
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError>;
}

impl Language {
    #[inline]
    pub fn maybe_from_str(input: &str) -> Option<Self> {
        match input.to_ascii_lowercase().as_str() {
            "assembly" | "asm" => Some(Self::Assembly),
            "bazel" => Some(Self::Bazel),
            "bash" => Some(Self::Shell(ShellFlavor::Bash)),
            "blade" => Some(Self::Blade),
            "bicep" => Some(Self::Bicep),
            "c" | "clang" => Some(Self::C),
            "cabal" => Some(Self::Cabal),
            "clojure" => Some(Self::Clojure),
            "cpp" | "c++" => Some(Self::Cpp),
            "crystal" | "cr" => Some(Self::Crystal),
            "csharp" | "c#" => Some(Self::CSharp),
            "css" => Some(Self::Css(CssFlavor::Css)),
            "d" => Some(Self::D),
            "dart" => Some(Self::Dart),
            "elixir" => Some(Self::Elixir),
            "elm" => Some(Self::Elm),
            "erb" => Some(Self::Erb),
            "erlang" => Some(Self::Erlang),
            "fennel" | "fnl" => Some(Self::Fennel),
            "fish" => Some(Self::Shell(ShellFlavor::Fish)),
            "fortran" => Some(Self::Fortran),
            "fsharp" => Some(Self::FSharp),
            "gdscript" => Some(Self::Gdscript),
            "gleam" => Some(Self::Gleam),
            "go" | "golang" => Some(Self::Go),
            "graphql" | "gql" => Some(Self::GraphQL),
            "groovy" => Some(Self::Groovy),
            "handlebars" | ".hbs" | "hbs" => Some(Self::Handlebars),
            "haskell" => Some(Self::Haskell),
            "html" | "html5" => Some(Self::Html),
            "java" => Some(Self::Java),
            "javascript" | "js" => Some(Self::JavaScript(JavaScriptFlavor::JavaScript)),
            "json" => Some(Self::Json(JsonFlavor::Json)),
            "json5" => Some(Self::Json(JsonFlavor::Json5)),
            "jsonc" => Some(Self::Json(JsonFlavor::JsonC)),
            "jsx" => Some(Self::JavaScript(JavaScriptFlavor::JSX)),
            "julia" | "jl" => Some(Self::Julia),
            "just" | "justfile" => Some(Self::Just),
            "kcl" => Some(Self::Kcl),
            "kotlin" => Some(Self::Kotlin),
            "less" => Some(Self::Css(CssFlavor::Less)),
            "lua" => Some(Self::Lua),
            "markdown" | "md" => Some(Self::Markdown),
            "mustache" => Some(Self::Mustache),
            "nim" => Some(Self::Nim),
            "nix" | "nixos" => Some(Self::Nix),
            "nunjucks" => Some(Self::Nunjucks),
            "objectivec" | "objective-c" | "objc" => Some(Self::ObjectiveC),
            "ocaml" => Some(Self::OCaml),
            "perl" => Some(Self::Perl),
            "profobuf" | "profo" => Some(Self::Protobuf),
            "puppet" => Some(Self::Puppet),
            "purescript" => Some(Self::PureScript),
            "python" => Some(Self::Python),
            "rescript" => Some(Self::ReScript),
            "restructuredtext" | "rst" => Some(Self::ReStructuredText),
            "roc" => Some(Self::Roc),
            "ruby" | "rb" => Some(Self::Ruby),
            "rust" | "rs" => Some(Self::Rust),
            "sass" => Some(Self::Css(CssFlavor::Sass)),
            "scala" => Some(Self::Scala),
            "scss" => Some(Self::Css(CssFlavor::Scss)),
            "shell" | "sh" | "shell-script" => Some(Self::Shell(ShellFlavor::Shell)),
            "solidity" => Some(Self::Solidity),
            "sql" | "bigquery" | "db2" | "db2i" | "hive" | "mariadb" | "mysql" | "n1ql"
            | "plsql" | "postgresql" | "redshift" | "singlestoredb" | "snowflake" | "spark"
            | "sqlite" | "transactsql" | "trino" | "tsql" => Some(Self::Sql),
            "swift" => Some(Self::Swift),
            "terraform" | "hcl" | "opentofu" => Some(Self::Hcl),
            "toml" => Some(Self::Toml),
            "tsx" => Some(Self::TypeScript(TypeScriptFlavor::TSX)),
            "typescript" | "ts" => Some(Self::TypeScript(TypeScriptFlavor::TypeScript)),
            "vue" => Some(Self::Vue),
            "xml" => Some(Self::Xml),
            "yml" | "yaml" => Some(Self::Yaml),
            "zig" => Some(Self::Zig),
            "zsh" => Some(Self::Shell(ShellFlavor::Zsh)),
            _ => None,
        }
    }

    #[inline]
    pub const fn to_file_ext(&self) -> &'static str {
        match self {
            Self::Assembly => ".s",
            Self::Bazel => ".bzl",
            Self::Bicep => ".bicep",
            Self::Blade => ".blade.php",
            Self::C => ".c",
            Self::CSharp => ".cs",
            Self::Cabal => ".cabal",
            Self::Clojure => ".clj",
            Self::Cpp => ".cpp",
            Self::Crystal => ".cr",
            Self::Css(CssFlavor::Css) => ".css",
            Self::Css(CssFlavor::Less) => ".less",
            Self::Css(CssFlavor::Sass) => ".sass",
            Self::Css(CssFlavor::Scss) => ".scss",
            Self::D => ".d",
            Self::Dart => ".dart",
            Self::Elixir => ".ex",
            Self::Elm => ".elm",
            Self::Erb => ".erb",
            Self::Erlang => ".erl",
            Self::Fennel => ".fnl",
            Self::FSharp => ".fs",
            Self::Fortran => ".f",
            Self::Gdscript => ".gd",
            Self::Gleam => ".gleam",
            Self::Go => ".go",
            Self::GraphQL => ".gql",
            Self::Groovy => ".groovy",
            Self::Handlebars => ".hbs",
            Self::Haskell => ".hs",
            Self::Hcl => ".tf",
            Self::Html => ".html",
            Self::Java => ".java",
            Self::JavaScript(JavaScriptFlavor::JSX) => ".jsx",
            Self::JavaScript(JavaScriptFlavor::JavaScript) => ".js",
            Self::Json(JsonFlavor::Json) => ".json",
            Self::Json(JsonFlavor::Json5) => ".json5",
            Self::Json(JsonFlavor::JsonC) => ".jsonc",
            Self::Julia => ".jl",
            Self::Just => ".justfile",
            Self::Kcl => ".kcl",
            Self::Kotlin => ".kt",
            Self::Lua => ".lua",
            Self::Markdown => ".md",
            Self::Mustache => ".mustache",
            Self::Nim => ".nim",
            Self::Nix => ".nix",
            Self::Nunjucks => ".njk",
            Self::OCaml => ".ml",
            Self::ObjectiveC => ".m",
            Self::Perl => ".pl",
            Self::Protobuf => ".proto",
            Self::Puppet => ".pp",
            Self::PureScript => ".purs",
            Self::Python => ".py",
            Self::ReScript => ".res",
            Self::ReStructuredText => ".rst",
            Self::Roc => ".roc",
            Self::Ruby => ".rb",
            Self::Rust => ".rs",
            Self::Scala => ".scala",
            Self::Shell(ShellFlavor::Bash) => ".bash",
            Self::Shell(ShellFlavor::Shell) => ".sh",
            Self::Shell(ShellFlavor::Fish) => ".fish",
            Self::Shell(ShellFlavor::Zsh) => ".zsh",
            Self::Solidity => ".sol",
            Self::Sql => ".sql",
            Self::Swift => ".swift",
            Self::Toml => ".toml",
            Self::TypeScript(TypeScriptFlavor::TSX) => ".tsx",
            Self::TypeScript(TypeScriptFlavor::TypeScript) => ".ts",
            Self::Vue => ".vue",
            Self::Xml => ".xml",
            Self::Yaml => ".yml",
            Self::Zig => ".zig",
        }
    }
}

const fn default_enabled() -> bool {
    true
}

#[derive(serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[schemars(deny_unknown_fields)]
pub struct Lang<T>
where
    T: LanguageFormatter + core::fmt::Display,
    MdsfFormatter<T>: Default,
{
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: MdsfFormatter<T>,
}

impl<T> Lang<T>
where
    T: LanguageFormatter + core::fmt::Display,
    MdsfFormatter<T>: Default,
{
    #[inline]
    pub fn format(
        &self,
        snippet_path: &std::path::Path,
        info: &LineInfo,
    ) -> Result<Option<String>, MdsfError> {
        if !self.enabled {
            return Ok(None);
        }

        Self::format_multiple(&self.formatter, snippet_path, info, false)
            .map(|(_should_continue, output)| output)
    }

    #[inline]
    pub fn format_multiple(
        formatter: &MdsfFormatter<T>,
        snippet_path: &std::path::Path,
        info: &LineInfo,
        nested: bool,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match formatter {
            MdsfFormatter::Single(f) => {
                let formatter_name = f.to_string();

                print_formatter_info(&formatter_name, info);

                let time = std::time::Instant::now();

                let r = f.format_snippet(snippet_path);

                print_formatter_time(&formatter_name, info, time.elapsed());

                if let Err(e) = &r {
                    if let MdsfError::MissingBinary(binary) = e {
                        print_binary_not_in_path(
                            if &formatter_name == binary {
                                formatter_name
                            } else {
                                format!("{binary} ({formatter_name})")
                            }
                            .as_str(),
                        );

                        return Ok((false, None));
                    } else if matches!(e, MdsfError::FormatterError) {
                        print_error_formatting(&formatter_name, info);
                        return Ok((false, None));
                    }
                }

                r
            }

            MdsfFormatter::Multiple(formatters) => {
                let mut r = Ok((true, None));

                for f in formatters {
                    r = Self::format_multiple(f, snippet_path, info, true);

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
    use crate::{
        error::MdsfError,
        formatters::{setup_snippet, MdsfFormatter},
        LineInfo,
    };

    #[derive(serde::Serialize)]
    enum TestLanguage {
        A,
        B,
        C,
        // Will fail
        D,
        E,
    }

    impl Default for MdsfFormatter<TestLanguage> {
        fn default() -> Self {
            Self::Single(TestLanguage::A)
        }
    }

    impl core::fmt::Display for TestLanguage {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::A => write!(f, "A"),
                Self::B => write!(f, "B"),
                Self::C => write!(f, "C"),
                Self::D => write!(f, "D"),
                Self::E => write!(f, "E"),
            }
        }
    }

    impl LanguageFormatter for TestLanguage {
        fn format_snippet(
            &self,
            snippet_path: &std::path::Path,
        ) -> Result<(bool, Option<String>), MdsfError> {
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
            .format(snippet_path, &LineInfo::fake())
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
            .format(snippet_path, &LineInfo::fake())
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
            .format(snippet_path, &LineInfo::fake())
            .expect("it to be ok")
            .expect("it to be some");

        assert_eq!("abc", code);
    }
}

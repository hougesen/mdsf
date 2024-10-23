pub mod alejandra;
pub mod ameba;
pub mod asmfmt;
pub mod astyle;
pub mod auto_optional;
pub mod autocorrect;
pub mod autoflake;
pub mod autopep_8;
pub mod bean_black;
pub mod beautysh;
pub mod bicep_format;
pub mod biome_check;
pub mod biome_format;
pub mod biome_lint;
pub mod black;
pub mod blade_formatter;
pub mod blue;
pub mod bpfmt;
pub mod brittany;
pub mod brunette;
pub mod bsfmt;
pub mod buf;
pub mod buildifier;
pub mod cabal_format;
pub mod caramel_fmt;
pub mod clang_format;
pub mod clang_tidy;
pub mod cljfmt;
pub mod cljstyle;
pub mod codespell;
pub mod crlfmt;
pub mod crystal_format;
pub mod css_beautify;
pub mod csscomb;
pub mod d_2;
pub mod dart_fix;
pub mod dart_format;
pub mod dcm_fix;
pub mod dcm_format;
pub mod deno_fmt;
pub mod deno_lint;
pub mod dfmt;
pub mod dhall;
pub mod djlint;
pub mod docformatter;
pub mod docstrfmt;
pub mod dotenv_linter;
pub mod dotnet;
pub mod dprint;
pub mod easy_coding_standard;
pub mod efmt;
pub mod elm_format;
pub mod eslint;
pub mod gersemi;
pub mod haml_lint;
pub mod prettier;
pub mod yapf;
pub mod yew_fmt;
pub mod zig;
pub mod ziggy;
pub mod zprint;

#[derive(serde::Serialize, serde::Deserialize, Hash)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum Tooling {
    #[serde(rename = "alejandra")]
    #[doc = "The Uncompromising Nix Code Formatter - [https://github.com/kamadorueda/alejandra](https://github.com/kamadorueda/alejandra)"]
    Alejandra,
    #[serde(rename = "ameba")]
    #[doc = "A static code analysis tool for Crystal - [https://github.com/crystal-ameba/ameba](https://github.com/crystal-ameba/ameba)"]
    Ameba,
    #[serde(rename = "asmfmt")]
    #[doc = "Go Assembler Formatter - [https://github.com/klauspost/asmfmt](https://github.com/klauspost/asmfmt)"]
    Asmfmt,
    #[serde(rename = "astyle")]
    #[doc = "A Free, Fast, and Small Automatic Formatter for C, C++, C++/CLI, Objective-C, C#, and Java Source Code - [https://gitlab.com/saalen/astyle](https://gitlab.com/saalen/astyle)"]
    Astyle,
    #[serde(rename = "auto_optional")]
    #[doc = "Adds the Optional type-hint to arguments where the default value is None - [https://pypi.org/project/auto-optional/](https://pypi.org/project/auto-optional/)"]
    AutoOptional,
    #[serde(rename = "autocorrect")]
    #[doc = "A linter and formatter to help you to improve copywriting, correct spaces, words, and punctuations between CJK (Chinese, Japanese, Korean) - [https://github.com/huacnlee/autocorrect](https://github.com/huacnlee/autocorrect)"]
    Autocorrect,
    #[serde(rename = "autoflake")]
    #[doc = "Removes unused imports and unused variables as reported by pyflakes - [https://github.com/pycqa/autoflake](https://github.com/pycqa/autoflake)"]
    Autoflake,
    #[serde(rename = "autopep_8")]
    #[doc = "A tool that automatically formats Python code to conform to the PEP 8 style guid - [https://pypi.org/project/autopep8/](https://pypi.org/project/autopep8/)"]
    Autopep8,
    #[serde(rename = "bean_black")]
    #[doc = "Opinionated code formatter, just like Python's black code formatter but for Beancount - [https://github.com/LaunchPlatform/beancount-black](https://github.com/LaunchPlatform/beancount-black)"]
    BeanBlack,
    #[serde(rename = "beautysh")]
    #[doc = "A Bash beautifier for the masses - [https://pypi.org/project/beautysh/](https://pypi.org/project/beautysh/)"]
    Beautysh,
    #[serde(rename = "bicep_format")]
    #[doc = "Bicep is a declarative language for describing and deploying Azure resources - [https://github.com/Azure/bicep](https://github.com/Azure/bicep)"]
    BicepFormat,
    #[serde(rename = "biome_check")]
    #[doc = "One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)"]
    BiomeCheck,
    #[serde(rename = "biome_format")]
    #[doc = "One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)"]
    BiomeFormat,
    #[serde(rename = "biome_lint")]
    #[doc = "One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)"]
    BiomeLint,
    #[serde(rename = "black")]
    #[doc = "The uncompromising Python code formatter - [https://github.com/psf/black](https://github.com/psf/black)"]
    Black,
    #[serde(rename = "blade_formatter")]
    #[doc = "An opinionated blade template formatter for Laravel that respects readability - [https://github.com/shufo/blade-formatter](https://github.com/shufo/blade-formatter)"]
    BladeFormatter,
    #[serde(rename = "blue")]
    #[doc = "The slightly less uncompromising Python code formatter - [https://blue.readthedocs.io/en/latest/](https://blue.readthedocs.io/en/latest/)"]
    Blue,
    #[serde(rename = "bpfmt")]
    #[doc = "A formatter for Blueprint files - [https://source.android.com/docs/setup/reference/androidbp#formatter](https://source.android.com/docs/setup/reference/androidbp#formatter)"]
    Bpfmt,
    #[serde(rename = "brittany")]
    #[doc = "A Haskell source code formatter - [https://github.com/lspitzner/brittany](https://github.com/lspitzner/brittany)"]
    Brittany,
    #[serde(rename = "brunette")]
    #[doc = "A best practice Python code formatter - [https://github.com/odwyersoftware/brunette](https://github.com/odwyersoftware/brunette)"]
    Brunette,
    #[serde(rename = "bsfmt")]
    #[doc = "A code formatter for BrightScript and BrighterScript - [https://github.com/rokucommunity/brighterscript-formatter](https://github.com/rokucommunity/brighterscript-formatter)"]
    Bsfmt,
    #[serde(rename = "buf")]
    #[doc = "The best way of working with Protocol Buffers - [https://buf.build/docs/reference/cli/buf/format/](https://buf.build/docs/reference/cli/buf/format/)"]
    Buf,
    #[serde(rename = "buildifier")]
    #[doc = " - []()"]
    Buildifier,
    #[serde(rename = "cabal_format")]
    #[doc = "Cabal is a system for building and packaging Haskell libraries and programs - [https://www.haskell.org/cabal/](https://www.haskell.org/cabal/)"]
    CabalFormat,
    #[serde(rename = "caramel_fmt")]
    #[doc = " - []()"]
    CaramelFmt,
    #[serde(rename = "clang_format")]
    #[doc = "A tool to format C/C++/Java/JavaScript/JSON/Objective-C/Protobuf/C# code - [https://clang.llvm.org/docs/ClangFormat.html](https://clang.llvm.org/docs/ClangFormat.html)"]
    ClangFormat,
    #[serde(rename = "clang_tidy")]
    #[doc = " - []()"]
    ClangTidy,
    #[serde(rename = "cljfmt")]
    #[doc = "A tool for formatting Clojure code - [https://github.com/weavejester/cljfmt](https://github.com/weavejester/cljfmt)"]
    Cljfmt,
    #[serde(rename = "cljstyle")]
    #[doc = "A tool for formatting Clojure code - [https://github.com/greglook/cljstyle](https://github.com/greglook/cljstyle)"]
    Cljstyle,
    #[serde(rename = "codespell")]
    #[doc = " - []()"]
    Codespell,
    #[serde(rename = "crlfmt")]
    #[doc = " - []()"]
    Crlfmt,
    #[serde(rename = "crystal_format")]
    #[doc = "Tools for the Crystal programming language - [https://crystal-lang.org/](https://crystal-lang.org/)"]
    CrystalFormat,
    #[serde(rename = "css_beautify")]
    #[doc = " - []()"]
    CssBeautify,
    #[serde(rename = "csscomb")]
    #[doc = " - []()"]
    Csscomb,
    #[serde(rename = "d_2")]
    #[doc = " - []()"]
    D2,
    #[serde(rename = "dart_fix")]
    #[doc = " - []()"]
    DartFix,
    #[serde(rename = "dart_format")]
    #[doc = " - []()"]
    DartFormat,
    #[serde(rename = "dcm_fix")]
    #[doc = " - []()"]
    DcmFix,
    #[serde(rename = "dcm_format")]
    #[doc = " - []()"]
    DcmFormat,
    #[serde(rename = "deno_fmt")]
    #[doc = "Formatter and linter for JavaScript and TypeScript - [https://docs.deno.com/runtime/reference/cli/](https://docs.deno.com/runtime/reference/cli/)"]
    DenoFmt,
    #[serde(rename = "deno_lint")]
    #[doc = "Formatter and linter for JavaScript and TypeScript - [https://docs.deno.com/runtime/reference/cli/](https://docs.deno.com/runtime/reference/cli/)"]
    DenoLint,
    #[serde(rename = "dfmt")]
    #[doc = " - []()"]
    Dfmt,
    #[serde(rename = "dhall")]
    #[doc = "Format Dhall files - [https://dhall-lang.org/](https://dhall-lang.org/)"]
    Dhall,
    #[serde(rename = "djlint")]
    #[doc = "Lint & Format HTML Templates - [https://www.djlint.com/](https://www.djlint.com/)"]
    Djlint,
    #[serde(rename = "docformatter")]
    #[doc = "Formats docstrings to follow PEP 257 - [https://pypi.org/project/docformatter/](https://pypi.org/project/docformatter/)"]
    Docformatter,
    #[serde(rename = "docstrfmt")]
    #[doc = "A formatter for Sphinx flavored reStructuredText - [https://pypi.org/project/docstrfmt/](https://pypi.org/project/docstrfmt/)"]
    Docstrfmt,
    #[serde(rename = "dotenv_linter")]
    #[doc = "Lightning-fast linter for .env files - [https://github.com/dotenv-linter/dotenv-linter](https://github.com/dotenv-linter/dotenv-linter)"]
    DotenvLinter,
    #[serde(rename = "dotnet")]
    #[doc = "An Opinionated Code Formatter for C# - [https://csharpier.com/](https://csharpier.com/)"]
    Dotnet,
    #[serde(rename = "dprint")]
    #[doc = "A pluggable and configurable code formatting platform written in Rust - [https://dprint.dev/](https://dprint.dev/)"]
    Dprint,
    #[serde(rename = "easy_coding_standard")]
    #[doc = "The Easiest way to add coding standard to your PHP project - [https://github.com/easy-coding-standard/easy-coding-standard](https://github.com/easy-coding-standard/easy-coding-standard)"]
    EasyCodingStandard,
    #[serde(rename = "efmt")]
    #[doc = "Erlang code formatter - [https://github.com/sile/efmt](https://github.com/sile/efmt)"]
    Efmt,
    #[serde(rename = "elm_format")]
    #[doc = "elm-format formats Elm source code according to a standard set of rules based on the official Elm Style Guide - [https://github.com/avh4/elm-format](https://github.com/avh4/elm-format)"]
    ElmFormat,
    #[serde(rename = "eslint")]
    #[doc = "Find and fix problems in your JavaScript code - [https://github.com/eslint/eslint/](https://github.com/eslint/eslint/)"]
    Eslint,
    #[serde(rename = "gersemi")]
    #[doc = "A formatter to make your CMake code the real treasure - [https://github.com/blankspruce/gersemi](https://github.com/blankspruce/gersemi)"]
    Gersemi,
    #[serde(rename = "haml_lint")]
    #[doc = "Tool for writing clean and consistent HAML - [https://github.com/sds/haml-lint](https://github.com/sds/haml-lint)"]
    HamlLint,
    #[serde(rename = "prettier")]
    #[doc = "Prettier is an opinionated code formatter - [https://github.com/prettier/prettier](https://github.com/prettier/prettier)"]
    Prettier,
    #[serde(rename = "yapf")]
    #[doc = " - []()"]
    Yapf,
    #[serde(rename = "yew_fmt")]
    #[doc = " - []()"]
    YewFmt,
    #[serde(rename = "zig")]
    #[doc = "Reformat Zig source into canonical form - [https://ziglang.org/](https://ziglang.org/)"]
    Zig,
    #[serde(rename = "ziggy")]
    #[doc = "Formats Ziggy documents and Ziggy schemas - [https://ziggy-lang.io/documentation/ziggy-fmt/](https://ziggy-lang.io/documentation/ziggy-fmt/)"]
    Ziggy,
    #[serde(rename = "zprint")]
    #[doc = "Executables, uberjar, and library to beautifully format Clojure and Clojurescript source code and s-expressions - [https://github.com/kkinnear/zprint](https://github.com/kkinnear/zprint)"]
    Zprint,
}

impl Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    pub fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), crate::error::MdsfError> {
        match self {
            Self::Alejandra => alejandra::run(snippet_path),
            Self::Ameba => ameba::run(snippet_path),
            Self::Asmfmt => asmfmt::run(snippet_path),
            Self::Astyle => astyle::run(snippet_path),
            Self::AutoOptional => auto_optional::run(snippet_path),
            Self::Autocorrect => autocorrect::run(snippet_path),
            Self::Autoflake => autoflake::run(snippet_path),
            Self::Autopep8 => autopep_8::run(snippet_path),
            Self::BeanBlack => bean_black::run(snippet_path),
            Self::Beautysh => beautysh::run(snippet_path),
            Self::BicepFormat => bicep_format::run(snippet_path),
            Self::BiomeCheck => biome_check::run(snippet_path),
            Self::BiomeFormat => biome_format::run(snippet_path),
            Self::BiomeLint => biome_lint::run(snippet_path),
            Self::Black => black::run(snippet_path),
            Self::BladeFormatter => blade_formatter::run(snippet_path),
            Self::Blue => blue::run(snippet_path),
            Self::Bpfmt => bpfmt::run(snippet_path),
            Self::Brittany => brittany::run(snippet_path),
            Self::Brunette => brunette::run(snippet_path),
            Self::Bsfmt => bsfmt::run(snippet_path),
            Self::Buf => buf::run(snippet_path),
            Self::Buildifier => buildifier::run(snippet_path),
            Self::CabalFormat => cabal_format::run(snippet_path),
            Self::CaramelFmt => caramel_fmt::run(snippet_path),
            Self::ClangFormat => clang_format::run(snippet_path),
            Self::ClangTidy => clang_tidy::run(snippet_path),
            Self::Cljfmt => cljfmt::run(snippet_path),
            Self::Cljstyle => cljstyle::run(snippet_path),
            Self::Codespell => codespell::run(snippet_path),
            Self::Crlfmt => crlfmt::run(snippet_path),
            Self::CrystalFormat => crystal_format::run(snippet_path),
            Self::CssBeautify => css_beautify::run(snippet_path),
            Self::Csscomb => csscomb::run(snippet_path),
            Self::D2 => d_2::run(snippet_path),
            Self::DartFix => dart_fix::run(snippet_path),
            Self::DartFormat => dart_format::run(snippet_path),
            Self::DcmFix => dcm_fix::run(snippet_path),
            Self::DcmFormat => dcm_format::run(snippet_path),
            Self::DenoFmt => deno_fmt::run(snippet_path),
            Self::DenoLint => deno_lint::run(snippet_path),
            Self::Dfmt => dfmt::run(snippet_path),
            Self::Dhall => dhall::run(snippet_path),
            Self::Djlint => djlint::run(snippet_path),
            Self::Docformatter => docformatter::run(snippet_path),
            Self::Docstrfmt => docstrfmt::run(snippet_path),
            Self::DotenvLinter => dotenv_linter::run(snippet_path),
            Self::Dotnet => dotnet::run(snippet_path),
            Self::Dprint => dprint::run(snippet_path),
            Self::EasyCodingStandard => easy_coding_standard::run(snippet_path),
            Self::Efmt => efmt::run(snippet_path),
            Self::ElmFormat => elm_format::run(snippet_path),
            Self::Eslint => eslint::run(snippet_path),
            Self::Gersemi => gersemi::run(snippet_path),
            Self::HamlLint => haml_lint::run(snippet_path),
            Self::Prettier => prettier::run(snippet_path),
            Self::Yapf => yapf::run(snippet_path),
            Self::YewFmt => yew_fmt::run(snippet_path),
            Self::Zig => zig::run(snippet_path),
            Self::Ziggy => ziggy::run(snippet_path),
            Self::Zprint => zprint::run(snippet_path),
        }
    }
}

impl AsRef<str> for Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    fn as_ref(&self) -> &str {
        match self {
            Self::Alejandra => "alejandra",
            Self::Ameba => "ameba",
            Self::Asmfmt => "asmfmt",
            Self::Astyle => "astyle",
            Self::AutoOptional => "auto_optional",
            Self::Autocorrect => "autocorrect",
            Self::Autoflake => "autoflake",
            Self::Autopep8 => "autopep_8",
            Self::BeanBlack => "bean_black",
            Self::Beautysh => "beautysh",
            Self::BicepFormat => "bicep_format",
            Self::BiomeCheck => "biome_check",
            Self::BiomeFormat => "biome_format",
            Self::BiomeLint => "biome_lint",
            Self::Black => "black",
            Self::BladeFormatter => "blade_formatter",
            Self::Blue => "blue",
            Self::Bpfmt => "bpfmt",
            Self::Brittany => "brittany",
            Self::Brunette => "brunette",
            Self::Bsfmt => "bsfmt",
            Self::Buf => "buf",
            Self::Buildifier => "buildifier",
            Self::CabalFormat => "cabal_format",
            Self::CaramelFmt => "caramel_fmt",
            Self::ClangFormat => "clang_format",
            Self::ClangTidy => "clang_tidy",
            Self::Cljfmt => "cljfmt",
            Self::Cljstyle => "cljstyle",
            Self::Codespell => "codespell",
            Self::Crlfmt => "crlfmt",
            Self::CrystalFormat => "crystal_format",
            Self::CssBeautify => "css_beautify",
            Self::Csscomb => "csscomb",
            Self::D2 => "d_2",
            Self::DartFix => "dart_fix",
            Self::DartFormat => "dart_format",
            Self::DcmFix => "dcm_fix",
            Self::DcmFormat => "dcm_format",
            Self::DenoFmt => "deno_fmt",
            Self::DenoLint => "deno_lint",
            Self::Dfmt => "dfmt",
            Self::Dhall => "dhall",
            Self::Djlint => "djlint",
            Self::Docformatter => "docformatter",
            Self::Docstrfmt => "docstrfmt",
            Self::DotenvLinter => "dotenv_linter",
            Self::Dotnet => "dotnet",
            Self::Dprint => "dprint",
            Self::EasyCodingStandard => "easy_coding_standard",
            Self::Efmt => "efmt",
            Self::ElmFormat => "elm_format",
            Self::Eslint => "eslint",
            Self::Gersemi => "gersemi",
            Self::HamlLint => "haml_lint",
            Self::Prettier => "prettier",
            Self::Yapf => "yapf",
            Self::YewFmt => "yew_fmt",
            Self::Zig => "zig",
            Self::Ziggy => "ziggy",
            Self::Zprint => "zprint",
        }
    }
}

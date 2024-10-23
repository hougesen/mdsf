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
pub mod csharpier;
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
pub mod dprint;
pub mod easy_coding_standard;
pub mod efmt;
pub mod elm_format;
pub mod erb_formatter;
pub mod erlfmt;
pub mod eslint;
pub mod fantomas;
pub mod fish_indent;
pub mod fixjson;
pub mod floskell;
pub mod fnlfmt;
pub mod forge_fmt;
pub mod fourmolu;
pub mod fprettify;
pub mod gci;
pub mod gdformat;
pub mod gersemi;
pub mod gleam_format;
pub mod gluon;
pub mod gofmt;
pub mod gofumpt;
pub mod goimports;
pub mod goimports_reviser;
pub mod golines;
pub mod google_java_format;
pub mod grain_format;
pub mod haml_lint;
pub mod hfmt;
pub mod hindent;
pub mod html_beautify;
pub mod htmlbeautifier;
pub mod imba_fmt;
pub mod isort;
pub mod joker;
pub mod js_beautify;
pub mod jsona;
pub mod jsonnetfmt;
pub mod juliaformatter_jl;
pub mod luaformatter;
pub mod mix_format;
pub mod nickel_format;
pub mod prettier;
pub mod scalariform;
pub mod stylua;
pub mod superhtml;
pub mod typos;
pub mod vhdl_style_guide;
pub mod wfindent;
pub mod yapf;
pub mod yew_fmt;
pub mod zig_fmt;
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
    #[doc = "A bazel BUILD file formatter and - [https://github.com/bazelbuild/buildtools](https://github.com/bazelbuild/buildtools)"]
    Buildifier,
    #[serde(rename = "cabal_format")]
    #[doc = "Cabal is a system for building and packaging Haskell libraries and programs - [https://www.haskell.org/cabal/](https://www.haskell.org/cabal/)"]
    CabalFormat,
    #[serde(rename = "caramel_fmt")]
    #[doc = "Formatter for the Caramel programming language - [https://caramel.run/](https://caramel.run/)"]
    CaramelFmt,
    #[serde(rename = "clang_format")]
    #[doc = "A tool to format C/C++/Java/JavaScript/JSON/Objective-C/Protobuf/C# code - [https://clang.llvm.org/docs/ClangFormat.html](https://clang.llvm.org/docs/ClangFormat.html)"]
    ClangFormat,
    #[serde(rename = "clang_tidy")]
    #[doc = "clang-tidy is a clang-based C++ “linter” tool - [https://clang.llvm.org/extra/clang-tidy/](https://clang.llvm.org/extra/clang-tidy/)"]
    ClangTidy,
    #[serde(rename = "cljfmt")]
    #[doc = "A tool for formatting Clojure code - [https://github.com/weavejester/cljfmt](https://github.com/weavejester/cljfmt)"]
    Cljfmt,
    #[serde(rename = "cljstyle")]
    #[doc = "A tool for formatting Clojure code - [https://github.com/greglook/cljstyle](https://github.com/greglook/cljstyle)"]
    Cljstyle,
    #[serde(rename = "codespell")]
    #[doc = "Check code for common misspellings - [https://github.com/codespell-project/codespell](https://github.com/codespell-project/codespell)"]
    Codespell,
    #[serde(rename = "crlfmt")]
    #[doc = "Formatter for CockroachDB's additions to the Go style guide - [https://github.com/cockroachdb/crlfmt](https://github.com/cockroachdb/crlfmt)"]
    Crlfmt,
    #[serde(rename = "crystal_format")]
    #[doc = "Tools for the Crystal programming language - [https://crystal-lang.org/](https://crystal-lang.org/)"]
    CrystalFormat,
    #[serde(rename = "csharpier")]
    #[doc = "An Opinionated Code Formatter for C# - [https://csharpier.com/](https://csharpier.com/)"]
    Csharpier,
    #[serde(rename = "css_beautify")]
    #[doc = "A css formatter - [https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html](https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html)"]
    CssBeautify,
    #[serde(rename = "csscomb")]
    #[doc = "CSS coding style formatter - [https://github.com/csscomb/csscomb.js](https://github.com/csscomb/csscomb.js)"]
    Csscomb,
    #[serde(rename = "d_2")]
    #[doc = "Formatter for the d2 language - [https://d2lang.com/](https://d2lang.com/)"]
    D2,
    #[serde(rename = "dart_fix")]
    #[doc = "Formatter and linter for Dart - [https://dart.dev/tools](https://dart.dev/tools)"]
    DartFix,
    #[serde(rename = "dart_format")]
    #[doc = "Formatter and linter for Dart - [https://dart.dev/tools](https://dart.dev/tools)"]
    DartFormat,
    #[serde(rename = "dcm_fix")]
    #[doc = "Code Quality Tool for Flutter Developers - [https://dcm.dev/](https://dcm.dev/)"]
    DcmFix,
    #[serde(rename = "dcm_format")]
    #[doc = "Code Quality Tool for Flutter Developers - [https://dcm.dev/](https://dcm.dev/)"]
    DcmFormat,
    #[serde(rename = "deno_fmt")]
    #[doc = "Formatter and linter for JavaScript and TypeScript - [https://docs.deno.com/runtime/reference/cli/](https://docs.deno.com/runtime/reference/cli/)"]
    DenoFmt,
    #[serde(rename = "deno_lint")]
    #[doc = "Formatter and linter for JavaScript and TypeScript - [https://docs.deno.com/runtime/reference/cli/](https://docs.deno.com/runtime/reference/cli/)"]
    DenoLint,
    #[serde(rename = "dfmt")]
    #[doc = "Dfmt is a formatter for D source code - [https://github.com/dlang-community/dfmt](https://github.com/dlang-community/dfmt)"]
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
    #[serde(rename = "erb_formatter")]
    #[doc = "Format ERB files with speed and precision - [https://github.com/nebulab/erb-formatter](https://github.com/nebulab/erb-formatter)"]
    ErbFormatter,
    #[serde(rename = "erlfmt")]
    #[doc = "An automated code formatter for Erlang - [https://github.com/WhatsApp/erlfmt](https://github.com/WhatsApp/erlfmt)"]
    Erlfmt,
    #[serde(rename = "eslint")]
    #[doc = "Find and fix problems in your JavaScript code - [https://github.com/eslint/eslint/](https://github.com/eslint/eslint/)"]
    Eslint,
    #[serde(rename = "fantomas")]
    #[doc = "FSharp source code formatter - [https://github.com/fsprojects/fantomas](https://github.com/fsprojects/fantomas)"]
    Fantomas,
    #[serde(rename = "fish_indent")]
    #[doc = "Fish indenter and prettifier - [https://fishshell.com/docs/current/cmds/fish_indent.html](https://fishshell.com/docs/current/cmds/fish_indent.html)"]
    FishIndent,
    #[serde(rename = "fixjson")]
    #[doc = "JSON Fixer for Humans using (relaxed) JSON5 - [https://github.com/rhysd/fixjson](https://github.com/rhysd/fixjson)"]
    Fixjson,
    #[serde(rename = "floskell")]
    #[doc = "Floskell is a flexible Haskell source code pretty printer - [https://github.com/ennocramer/floskell](https://github.com/ennocramer/floskell)"]
    Floskell,
    #[serde(rename = "fnlfmt")]
    #[doc = "A formatter for Fennel code - [https://git.sr.ht/~technomancy/fnlfmt](https://git.sr.ht/~technomancy/fnlfmt)"]
    Fnlfmt,
    #[serde(rename = "forge_fmt")]
    #[doc = "A Solidity formatter - [https://github.com/foundry-rs/foundry](https://github.com/foundry-rs/foundry)"]
    ForgeFmt,
    #[serde(rename = "fourmolu")]
    #[doc = "A formatter for Haskell source code - [https://hackage.haskell.org/package/fourmolu](https://hackage.haskell.org/package/fourmolu)"]
    Fourmolu,
    #[serde(rename = "fprettify")]
    #[doc = "Auto-formatter for modern Fortran source code - [https://github.com/fortran-lang/fprettify](https://github.com/fortran-lang/fprettify)"]
    Fprettify,
    #[serde(rename = "gci")]
    #[doc = "GCI, a tool that control golang package import order and make it always deterministic - [https://github.com/daixiang0/gci](https://github.com/daixiang0/gci)"]
    Gci,
    #[serde(rename = "gdformat")]
    #[doc = " - []()"]
    Gdformat,
    #[serde(rename = "gersemi")]
    #[doc = "A formatter to make your CMake code the real treasure - [https://github.com/blankspruce/gersemi](https://github.com/blankspruce/gersemi)"]
    Gersemi,
    #[serde(rename = "gleam_format")]
    #[doc = " - [https://gleam.run](https://gleam.run)"]
    GleamFormat,
    #[serde(rename = "gluon")]
    #[doc = " - []()"]
    Gluon,
    #[serde(rename = "gofmt")]
    #[doc = "Gofmt formats Go programs - [https://pkg.go.dev/cmd/gofmt](https://pkg.go.dev/cmd/gofmt)"]
    Gofmt,
    #[serde(rename = "gofumpt")]
    #[doc = " - []()"]
    Gofumpt,
    #[serde(rename = "goimports")]
    #[doc = "goimports updates your Go import lines, adding missing ones and removing unreferenced ones - [https://pkg.go.dev/golang.org/x/tools/cmd/goimports](https://pkg.go.dev/golang.org/x/tools/cmd/goimports)"]
    Goimports,
    #[serde(rename = "goimports_reviser")]
    #[doc = "Right imports sorting & code formatting tool (goimports alternative) - [https://github.com/incu6us/goimports-reviser](https://github.com/incu6us/goimports-reviser)"]
    GoimportsReviser,
    #[serde(rename = "golines")]
    #[doc = " - []()"]
    Golines,
    #[serde(rename = "google_java_format")]
    #[doc = " - []()"]
    GoogleJavaFormat,
    #[serde(rename = "grain_format")]
    #[doc = " - []()"]
    GrainFormat,
    #[serde(rename = "haml_lint")]
    #[doc = "Tool for writing clean and consistent HAML - [https://github.com/sds/haml-lint](https://github.com/sds/haml-lint)"]
    HamlLint,
    #[serde(rename = "hfmt")]
    #[doc = "Format Haskell programs. Inspired by the gofmt utility - [https://github.com/danstiner/hfmt](https://github.com/danstiner/hfmt)"]
    Hfmt,
    #[serde(rename = "hindent")]
    #[doc = " - []()"]
    Hindent,
    #[serde(rename = "html_beautify")]
    #[doc = "A html formatter - [https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html](https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html)"]
    HtmlBeautify,
    #[serde(rename = "htmlbeautifier")]
    #[doc = "A normaliser/beautifier for HTML that also understands embedded Ruby. Ideal for tidying up Rails templates - [https://github.com/threedaymonk/htmlbeautifier](https://github.com/threedaymonk/htmlbeautifier)"]
    Htmlbeautifier,
    #[serde(rename = "imba_fmt")]
    #[doc = "A formatter for Imba - [https://imba.io/](https://imba.io/)"]
    ImbaFmt,
    #[serde(rename = "isort")]
    #[doc = " - []()"]
    Isort,
    #[serde(rename = "joker")]
    #[doc = "Small Clojure interpreter, linter and formatter - [https://github.com/candid82/joker](https://github.com/candid82/joker)"]
    Joker,
    #[serde(rename = "js_beautify")]
    #[doc = "A JavaScript formatter - [https://github.com/beautifier/js-beautify](https://github.com/beautifier/js-beautify)"]
    JsBeautify,
    #[serde(rename = "jsona")]
    #[doc = " - []()"]
    Jsona,
    #[serde(rename = "jsonnetfmt")]
    #[doc = " - []()"]
    Jsonnetfmt,
    #[serde(rename = "juliaformatter_jl")]
    #[doc = " - []()"]
    JuliaformatterJl,
    #[serde(rename = "luaformatter")]
    #[doc = " - []()"]
    Luaformatter,
    #[serde(rename = "mix_format")]
    #[doc = "Code formatter for Elixir - [https://hexdocs.pm/mix/main/Mix.Tasks.Format.html](https://hexdocs.pm/mix/main/Mix.Tasks.Format.html)"]
    MixFormat,
    #[serde(rename = "nickel_format")]
    #[doc = "Better configuration for less - [https://nickel-lang.org/](https://nickel-lang.org/)"]
    NickelFormat,
    #[serde(rename = "prettier")]
    #[doc = "Prettier is an opinionated code formatter - [https://github.com/prettier/prettier](https://github.com/prettier/prettier)"]
    Prettier,
    #[serde(rename = "scalariform")]
    #[doc = "Scala source code formatter - [https://github.com/scala-ide/scalariform](https://github.com/scala-ide/scalariform)"]
    Scalariform,
    #[serde(rename = "stylua")]
    #[doc = "An opinionated Lua code formatter - [https://github.com/JohnnyMorganz/StyLua](https://github.com/JohnnyMorganz/StyLua)"]
    Stylua,
    #[serde(rename = "superhtml")]
    #[doc = " - []()"]
    Superhtml,
    #[serde(rename = "typos")]
    #[doc = "Source code spell checker - [https://github.com/crate-ci/typos](https://github.com/crate-ci/typos)"]
    Typos,
    #[serde(rename = "vhdl_style_guide")]
    #[doc = "Style guide enforcement for VHDL - [https://github.com/jeremiah-c-leary/vhdl-style-guide](https://github.com/jeremiah-c-leary/vhdl-style-guide)"]
    VhdlStyleGuide,
    #[serde(rename = "wfindent")]
    #[doc = "Indents and optionally converts Fortran program sources - [https://github.com/wvermin/findent](https://github.com/wvermin/findent)"]
    Wfindent,
    #[serde(rename = "yapf")]
    #[doc = "A formatter for Python files - [https://github.com/google/yapf](https://github.com/google/yapf)"]
    Yapf,
    #[serde(rename = "yew_fmt")]
    #[doc = "Code formatter for the Yew framework - [https://github.com/its-the-shrimp/yew-fmt](https://github.com/its-the-shrimp/yew-fmt)"]
    YewFmt,
    #[serde(rename = "zig_fmt")]
    #[doc = "Reformat Zig source into canonical form - [https://ziglang.org/](https://ziglang.org/)"]
    ZigFmt,
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
            Self::Csharpier => csharpier::run(snippet_path),
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
            Self::Dprint => dprint::run(snippet_path),
            Self::EasyCodingStandard => easy_coding_standard::run(snippet_path),
            Self::Efmt => efmt::run(snippet_path),
            Self::ElmFormat => elm_format::run(snippet_path),
            Self::ErbFormatter => erb_formatter::run(snippet_path),
            Self::Erlfmt => erlfmt::run(snippet_path),
            Self::Eslint => eslint::run(snippet_path),
            Self::Fantomas => fantomas::run(snippet_path),
            Self::FishIndent => fish_indent::run(snippet_path),
            Self::Fixjson => fixjson::run(snippet_path),
            Self::Floskell => floskell::run(snippet_path),
            Self::Fnlfmt => fnlfmt::run(snippet_path),
            Self::ForgeFmt => forge_fmt::run(snippet_path),
            Self::Fourmolu => fourmolu::run(snippet_path),
            Self::Fprettify => fprettify::run(snippet_path),
            Self::Gci => gci::run(snippet_path),
            Self::Gdformat => gdformat::run(snippet_path),
            Self::Gersemi => gersemi::run(snippet_path),
            Self::GleamFormat => gleam_format::run(snippet_path),
            Self::Gluon => gluon::run(snippet_path),
            Self::Gofmt => gofmt::run(snippet_path),
            Self::Gofumpt => gofumpt::run(snippet_path),
            Self::Goimports => goimports::run(snippet_path),
            Self::GoimportsReviser => goimports_reviser::run(snippet_path),
            Self::Golines => golines::run(snippet_path),
            Self::GoogleJavaFormat => google_java_format::run(snippet_path),
            Self::GrainFormat => grain_format::run(snippet_path),
            Self::HamlLint => haml_lint::run(snippet_path),
            Self::Hfmt => hfmt::run(snippet_path),
            Self::Hindent => hindent::run(snippet_path),
            Self::HtmlBeautify => html_beautify::run(snippet_path),
            Self::Htmlbeautifier => htmlbeautifier::run(snippet_path),
            Self::ImbaFmt => imba_fmt::run(snippet_path),
            Self::Isort => isort::run(snippet_path),
            Self::Joker => joker::run(snippet_path),
            Self::JsBeautify => js_beautify::run(snippet_path),
            Self::Jsona => jsona::run(snippet_path),
            Self::Jsonnetfmt => jsonnetfmt::run(snippet_path),
            Self::JuliaformatterJl => juliaformatter_jl::run(snippet_path),
            Self::Luaformatter => luaformatter::run(snippet_path),
            Self::MixFormat => mix_format::run(snippet_path),
            Self::NickelFormat => nickel_format::run(snippet_path),
            Self::Prettier => prettier::run(snippet_path),
            Self::Scalariform => scalariform::run(snippet_path),
            Self::Stylua => stylua::run(snippet_path),
            Self::Superhtml => superhtml::run(snippet_path),
            Self::Typos => typos::run(snippet_path),
            Self::VhdlStyleGuide => vhdl_style_guide::run(snippet_path),
            Self::Wfindent => wfindent::run(snippet_path),
            Self::Yapf => yapf::run(snippet_path),
            Self::YewFmt => yew_fmt::run(snippet_path),
            Self::ZigFmt => zig_fmt::run(snippet_path),
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
            Self::Csharpier => "csharpier",
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
            Self::Dprint => "dprint",
            Self::EasyCodingStandard => "easy_coding_standard",
            Self::Efmt => "efmt",
            Self::ElmFormat => "elm_format",
            Self::ErbFormatter => "erb_formatter",
            Self::Erlfmt => "erlfmt",
            Self::Eslint => "eslint",
            Self::Fantomas => "fantomas",
            Self::FishIndent => "fish_indent",
            Self::Fixjson => "fixjson",
            Self::Floskell => "floskell",
            Self::Fnlfmt => "fnlfmt",
            Self::ForgeFmt => "forge_fmt",
            Self::Fourmolu => "fourmolu",
            Self::Fprettify => "fprettify",
            Self::Gci => "gci",
            Self::Gdformat => "gdformat",
            Self::Gersemi => "gersemi",
            Self::GleamFormat => "gleam_format",
            Self::Gluon => "gluon",
            Self::Gofmt => "gofmt",
            Self::Gofumpt => "gofumpt",
            Self::Goimports => "goimports",
            Self::GoimportsReviser => "goimports_reviser",
            Self::Golines => "golines",
            Self::GoogleJavaFormat => "google_java_format",
            Self::GrainFormat => "grain_format",
            Self::HamlLint => "haml_lint",
            Self::Hfmt => "hfmt",
            Self::Hindent => "hindent",
            Self::HtmlBeautify => "html_beautify",
            Self::Htmlbeautifier => "htmlbeautifier",
            Self::ImbaFmt => "imba_fmt",
            Self::Isort => "isort",
            Self::Joker => "joker",
            Self::JsBeautify => "js_beautify",
            Self::Jsona => "jsona",
            Self::Jsonnetfmt => "jsonnetfmt",
            Self::JuliaformatterJl => "juliaformatter_jl",
            Self::Luaformatter => "luaformatter",
            Self::MixFormat => "mix_format",
            Self::NickelFormat => "nickel_format",
            Self::Prettier => "prettier",
            Self::Scalariform => "scalariform",
            Self::Stylua => "stylua",
            Self::Superhtml => "superhtml",
            Self::Typos => "typos",
            Self::VhdlStyleGuide => "vhdl_style_guide",
            Self::Wfindent => "wfindent",
            Self::Yapf => "yapf",
            Self::YewFmt => "yew_fmt",
            Self::ZigFmt => "zig_fmt",
            Self::Ziggy => "ziggy",
            Self::Zprint => "zprint",
        }
    }
}

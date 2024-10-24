pub mod alejandra;
pub mod ameba;
pub mod asmfmt;
pub mod astyle;
pub mod auto_optional;
pub mod autocorrect;
pub mod autoflake;
pub mod autopep_8;
pub mod beancount_black;
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
pub mod buf_format;
pub mod buildifier;
pub mod cabal_format;
pub mod caramel_fmt;
pub mod clang_format;
pub mod clang_tidy;
pub mod cljfmt_fix;
pub mod cljstyle;
pub mod codespell;
pub mod crlfmt;
pub mod crystal_format;
pub mod csharpier;
pub mod css_beautify;
pub mod csscomb;
pub mod d_2_fmt;
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
pub mod dotenv_linter_fix;
pub mod dprint_fmt;
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
pub mod gluon_fmt;
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
pub mod jsona_format;
pub mod jsona_lint;
pub mod jsonnetfmt;
pub mod juliaformatter_jl;
pub mod just;
pub mod kcl_fmt;
pub mod kdlfmt;
pub mod ktfmt;
pub mod ktlint;
pub mod kulala_fmt;
pub mod leptosfmt;
pub mod liquidsoap_prettier;
pub mod luaformatter;
pub mod markdownfmt;
pub mod markdownlint;
pub mod markuplint;
pub mod mdformat;
pub mod misspell;
pub mod mix_format;
pub mod mojo_format;
pub mod nickel_format;
pub mod nimpretty;
pub mod nixfmt;
pub mod nixpkgs_fmt;
pub mod nph;
pub mod npm_groovy_lint;
pub mod ocamlformat;
pub mod ocp_indent;
pub mod ormolu;
pub mod oxlint;
pub mod packer_fmt;
pub mod perltidy;
pub mod pg_format;
pub mod php_cs_fixer_fix;
pub mod phpcbf;
pub mod phpinsights_fix;
pub mod pint;
pub mod prettier;
pub mod pretty_php;
pub mod prettypst;
pub mod puppet_lint;
pub mod purs_tidy;
pub mod pycln;
pub mod pyink;
pub mod qmlfmt;
pub mod raco_fmt;
pub mod refmt;
pub mod rescript_format;
pub mod roc_format;
pub mod rstfmt;
pub mod rubocop;
pub mod rubyfmt;
pub mod ruff_check;
pub mod ruff_format;
pub mod rufo;
pub mod rune_fmt;
pub mod rustfmt;
pub mod rustywind;
pub mod scalafmt;
pub mod scalariform;
pub mod shfmt;
pub mod sleek;
pub mod smlfmt;
pub mod snakefmt;
pub mod sql_formatter;
pub mod sqlfluff_fix;
pub mod sqlfluff_format;
pub mod sqlfmt;
pub mod standardjs;
pub mod standardrb;
pub mod stylefmt;
pub mod stylelint;
pub mod stylish_haskell;
pub mod stylua;
pub mod superhtml_fmt;
pub mod swift_format;
pub mod swiftformat;
pub mod taplo;
pub mod templ_fmt;
pub mod terraform_fmt;
pub mod tlint_format;
pub mod tofu_fmt;
pub mod topiary;
pub mod ts_standard;
pub mod twig_cs_fixer_lint;
pub mod typos;
pub mod typstfmt;
pub mod typstyle;
pub mod ufmt;
pub mod uiua_fmt;
pub mod usort;
pub mod v_fmt;
pub mod veryl_fmt;
pub mod vhdl_style_guide;
pub mod wfindent;
pub mod xmlformat;
pub mod xmllint;
pub mod xo;
pub mod yamlfix;
pub mod yamlfmt;
pub mod yapf;
pub mod yew_fmt;
pub mod zig_fmt;
pub mod ziggy_fmt;
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

    #[serde(rename = "auto-optional")]
    #[doc = "Adds the Optional type-hint to arguments where the default value is None - [https://pypi.org/project/auto-optional/](https://pypi.org/project/auto-optional/)"]
    AutoOptional,

    #[serde(rename = "autocorrect")]
    #[doc = "A linter and formatter to help you to improve copywriting, correct spaces, words, and punctuations between CJK (Chinese, Japanese, Korean) - [https://github.com/huacnlee/autocorrect](https://github.com/huacnlee/autocorrect)"]
    Autocorrect,

    #[serde(rename = "autoflake")]
    #[doc = "Removes unused imports and unused variables as reported by pyflakes - [https://github.com/pycqa/autoflake](https://github.com/pycqa/autoflake)"]
    Autoflake,

    #[serde(rename = "autopep8")]
    #[doc = "A tool that automatically formats Python code to conform to the PEP 8 style guid - [https://pypi.org/project/autopep8/](https://pypi.org/project/autopep8/)"]
    Autopep8,

    #[serde(rename = "beancount-black")]
    #[doc = "Opinionated code formatter, just like Python's black code formatter but for Beancount - [https://github.com/LaunchPlatform/beancount-black](https://github.com/LaunchPlatform/beancount-black)"]
    BeancountBlack,

    #[serde(rename = "beautysh")]
    #[doc = "A Bash beautifier for the masses - [https://pypi.org/project/beautysh/](https://pypi.org/project/beautysh/)"]
    Beautysh,

    #[serde(rename = "bicep:format")]
    #[doc = "Bicep is a declarative language for describing and deploying Azure resources - [https://github.com/Azure/bicep](https://github.com/Azure/bicep)"]
    BicepFormat,

    #[serde(rename = "biome:check")]
    #[doc = "One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)"]
    BiomeCheck,

    #[serde(rename = "biome:format")]
    #[doc = "One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)"]
    BiomeFormat,

    #[serde(rename = "biome:lint")]
    #[doc = "One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)"]
    BiomeLint,

    #[serde(rename = "black")]
    #[doc = "The uncompromising Python code formatter - [https://github.com/psf/black](https://github.com/psf/black)"]
    Black,

    #[serde(rename = "blade-formatter")]
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

    #[serde(rename = "buf:format")]
    #[doc = "The best way of working with Protocol Buffers - [https://buf.build/docs/reference/cli/buf/format/](https://buf.build/docs/reference/cli/buf/format/)"]
    BufFormat,

    #[serde(rename = "buildifier")]
    #[doc = "A bazel BUILD file formatter and - [https://github.com/bazelbuild/buildtools](https://github.com/bazelbuild/buildtools)"]
    Buildifier,

    #[serde(rename = "cabal:format")]
    #[doc = "Cabal is a system for building and packaging Haskell libraries and programs - [https://www.haskell.org/cabal/](https://www.haskell.org/cabal/)"]
    CabalFormat,

    #[serde(rename = "caramel:fmt")]
    #[doc = "Formatter for the Caramel programming language - [https://caramel.run/](https://caramel.run/)"]
    CaramelFmt,

    #[serde(rename = "clang-format")]
    #[doc = "A tool to format C/C++/Java/JavaScript/JSON/Objective-C/Protobuf/C# code - [https://clang.llvm.org/docs/ClangFormat.html](https://clang.llvm.org/docs/ClangFormat.html)"]
    ClangFormat,

    #[serde(rename = "clang-tidy")]
    #[doc = "clang-tidy is a clang-based C++ “linter” tool - [https://clang.llvm.org/extra/clang-tidy/](https://clang.llvm.org/extra/clang-tidy/)"]
    ClangTidy,

    #[serde(rename = "cljfmt:fix")]
    #[doc = "A tool for formatting Clojure code - [https://github.com/weavejester/cljfmt](https://github.com/weavejester/cljfmt)"]
    CljfmtFix,

    #[serde(rename = "cljstyle")]
    #[doc = "A tool for formatting Clojure code - [https://github.com/greglook/cljstyle](https://github.com/greglook/cljstyle)"]
    Cljstyle,

    #[serde(rename = "codespell")]
    #[doc = "Check code for common misspellings - [https://github.com/codespell-project/codespell](https://github.com/codespell-project/codespell)"]
    Codespell,

    #[serde(rename = "crlfmt")]
    #[doc = "Formatter for CockroachDB's additions to the Go style guide - [https://github.com/cockroachdb/crlfmt](https://github.com/cockroachdb/crlfmt)"]
    Crlfmt,

    #[serde(rename = "crystal:format")]
    #[doc = "Tools for the Crystal programming language - [https://crystal-lang.org/](https://crystal-lang.org/)"]
    CrystalFormat,

    #[serde(rename = "csharpier")]
    #[doc = "An Opinionated Code Formatter for C# - [https://csharpier.com/](https://csharpier.com/)"]
    Csharpier,

    #[serde(rename = "css-beautify")]
    #[doc = "A css formatter - [https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html](https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html)"]
    CssBeautify,

    #[serde(rename = "csscomb")]
    #[doc = "CSS coding style formatter - [https://github.com/csscomb/csscomb.js](https://github.com/csscomb/csscomb.js)"]
    Csscomb,

    #[serde(rename = "d2:fmt")]
    #[doc = "Formatter for the d2 language - [https://d2lang.com/](https://d2lang.com/)"]
    D2Fmt,

    #[serde(rename = "dart:fix")]
    #[doc = "Formatter and linter for Dart - [https://dart.dev/tools](https://dart.dev/tools)"]
    DartFix,

    #[serde(rename = "dart:format")]
    #[doc = "Formatter and linter for Dart - [https://dart.dev/tools](https://dart.dev/tools)"]
    DartFormat,

    #[serde(rename = "dcm:fix")]
    #[doc = "Code Quality Tool for Flutter Developers - [https://dcm.dev/](https://dcm.dev/)"]
    DcmFix,

    #[serde(rename = "dcm:format")]
    #[doc = "Code Quality Tool for Flutter Developers - [https://dcm.dev/](https://dcm.dev/)"]
    DcmFormat,

    #[serde(rename = "deno:fmt")]
    #[doc = "Formatter and linter for JavaScript and TypeScript - [https://docs.deno.com/runtime/reference/cli/](https://docs.deno.com/runtime/reference/cli/)"]
    DenoFmt,

    #[serde(rename = "deno:lint")]
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

    #[serde(rename = "dotenv-linter:fix")]
    #[doc = "Lightning-fast linter for .env files - [https://github.com/dotenv-linter/dotenv-linter](https://github.com/dotenv-linter/dotenv-linter)"]
    DotenvLinterFix,

    #[serde(rename = "dprint:fmt")]
    #[doc = "A pluggable and configurable code formatting platform written in Rust - [https://dprint.dev/](https://dprint.dev/)"]
    DprintFmt,

    #[serde(rename = "easy-coding-standard")]
    #[doc = "The Easiest way to add coding standard to your PHP project - [https://github.com/easy-coding-standard/easy-coding-standard](https://github.com/easy-coding-standard/easy-coding-standard)"]
    EasyCodingStandard,

    #[serde(rename = "efmt")]
    #[doc = "Erlang code formatter - [https://github.com/sile/efmt](https://github.com/sile/efmt)"]
    Efmt,

    #[serde(rename = "elm-format")]
    #[doc = "elm-format formats Elm source code according to a standard set of rules based on the official Elm Style Guide - [https://github.com/avh4/elm-format](https://github.com/avh4/elm-format)"]
    ElmFormat,

    #[serde(rename = "erb-formatter")]
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

    #[serde(rename = "forge:fmt")]
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
    #[doc = "https://github.com/scony/godot-gdscript-toolkit - [GDScript linter](GDScript linter)"]
    Gdformat,

    #[serde(rename = "gersemi")]
    #[doc = "A formatter to make your CMake code the real treasure - [https://github.com/blankspruce/gersemi](https://github.com/blankspruce/gersemi)"]
    Gersemi,

    #[serde(rename = "gleam:format")]
    #[doc = "Format Gleam source code - [https://gleam.run](https://gleam.run)"]
    GleamFormat,

    #[serde(rename = "gluon:fmt")]
    #[doc = "Code formatting for the gluon programming language - [https://github.com/gluon-lang/gluon](https://github.com/gluon-lang/gluon)"]
    GluonFmt,

    #[serde(rename = "gofmt")]
    #[doc = "Gofmt formats Go programs - [https://pkg.go.dev/cmd/gofmt](https://pkg.go.dev/cmd/gofmt)"]
    Gofmt,

    #[serde(rename = "gofumpt")]
    #[doc = "A stricter gofmt - [https://github.com/mvdan/gofumpt](https://github.com/mvdan/gofumpt)"]
    Gofumpt,

    #[serde(rename = "goimports")]
    #[doc = "goimports updates your Go import lines, adding missing ones and removing unreferenced ones - [https://pkg.go.dev/golang.org/x/tools/cmd/goimports](https://pkg.go.dev/golang.org/x/tools/cmd/goimports)"]
    Goimports,

    #[serde(rename = "goimports-reviser")]
    #[doc = "Right imports sorting & code formatting tool (goimports alternative) - [https://github.com/incu6us/goimports-reviser](https://github.com/incu6us/goimports-reviser)"]
    GoimportsReviser,

    #[serde(rename = "golines")]
    #[doc = "A golang formatter that fixes long lines - [https://github.com/segmentio/golines](https://github.com/segmentio/golines)"]
    Golines,

    #[serde(rename = "google-java-format")]
    #[doc = "Reformats Java source code to comply with Google Java Style - [https://github.com/google/google-java-format](https://github.com/google/google-java-format)"]
    GoogleJavaFormat,

    #[serde(rename = "grain:format")]
    #[doc = "Code formatter for the Grain programming language - [https://grain-lang.org/docs/tooling/grain_cli](https://grain-lang.org/docs/tooling/grain_cli)"]
    GrainFormat,

    #[serde(rename = "haml-lint")]
    #[doc = "Tool for writing clean and consistent HAML - [https://github.com/sds/haml-lint](https://github.com/sds/haml-lint)"]
    HamlLint,

    #[serde(rename = "hfmt")]
    #[doc = "Format Haskell programs. Inspired by the gofmt utility - [https://github.com/danstiner/hfmt](https://github.com/danstiner/hfmt)"]
    Hfmt,

    #[serde(rename = "hindent")]
    #[doc = "Extensible Haskell pretty printer - [https://github.com/mihaimaruseac/hindent](https://github.com/mihaimaruseac/hindent)"]
    Hindent,

    #[serde(rename = "html-beautify")]
    #[doc = "A html formatter - [https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html](https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html)"]
    HtmlBeautify,

    #[serde(rename = "htmlbeautifier")]
    #[doc = "A normaliser/beautifier for HTML that also understands embedded Ruby. Ideal for tidying up Rails templates - [https://github.com/threedaymonk/htmlbeautifier](https://github.com/threedaymonk/htmlbeautifier)"]
    Htmlbeautifier,

    #[serde(rename = "imba:fmt")]
    #[doc = "A formatter for Imba - [https://imba.io/](https://imba.io/)"]
    ImbaFmt,

    #[serde(rename = "isort")]
    #[doc = "A Python utility to sort imports - [https://github.com/timothycrosley/isort](https://github.com/timothycrosley/isort)"]
    Isort,

    #[serde(rename = "joker")]
    #[doc = "Small Clojure interpreter, linter and formatter - [https://github.com/candid82/joker](https://github.com/candid82/joker)"]
    Joker,

    #[serde(rename = "js-beautify")]
    #[doc = "A JavaScript formatter - [https://github.com/beautifier/js-beautify](https://github.com/beautifier/js-beautify)"]
    JsBeautify,

    #[serde(rename = "jsona:format")]
    #[doc = "JSONA linter and formatter - [https://github.com/jsona/jsona](https://github.com/jsona/jsona)"]
    JsonaFormat,

    #[serde(rename = "jsona:lint")]
    #[doc = "JSONA linter and formatter - [https://github.com/jsona/jsona](https://github.com/jsona/jsona)"]
    JsonaLint,

    #[serde(rename = "jsonnetfmt")]
    #[doc = "Formatter for automatically fixing jsonnet stylistic problems - [https://jsonnet.org/learning/tools.html](https://jsonnet.org/learning/tools.html)"]
    Jsonnetfmt,

    #[serde(rename = "juliaformatter.jl")]
    #[doc = "An opinionated code formatter for Julia. Plot twist - the opinion is your own - [https://github.com/domluna/JuliaFormatter.jl](https://github.com/domluna/JuliaFormatter.jl)"]
    JuliaformatterJl,

    #[serde(rename = "just")]
    #[doc = "A formatter for justfiles - [https://github.com/casey/just](https://github.com/casey/just)"]
    Just,

    #[serde(rename = "kcl:fmt")]
    #[doc = "KCL Format tool supports reformatting KCL files to the standard code style - [https://www.kcl-lang.io/docs/tools/cli/kcl/fmt](https://www.kcl-lang.io/docs/tools/cli/kcl/fmt)"]
    KclFmt,

    #[serde(rename = "kdlfmt")]
    #[doc = "A formatter for kdl documents - [https://github.com/hougesen/kdlfmt](https://github.com/hougesen/kdlfmt)"]
    Kdlfmt,

    #[serde(rename = "ktfmt")]
    #[doc = "program that reformats Kotlin source code to comply with the common community standard for Kotlin code conventions - [https://github.com/facebook/ktfmt](https://github.com/facebook/ktfmt)"]
    Ktfmt,

    #[serde(rename = "ktlint")]
    #[doc = "An anti-bikeshedding Kotlin linter with built-in formatter - [https://github.com/pinterest/ktlint](https://github.com/pinterest/ktlint)"]
    Ktlint,

    #[serde(rename = "kulala-fmt")]
    #[doc = "An opinionated 🦄 .http and .rest 🐼 files linter 💄 and formatter ⚡. - [https://github.com/mistweaverco/kulala-fmt](https://github.com/mistweaverco/kulala-fmt)"]
    KulalaFmt,

    #[serde(rename = "leptosfmt")]
    #[doc = "A formatter for the leptos view! macro - [https://github.com/bram209/leptosfmt](https://github.com/bram209/leptosfmt)"]
    Leptosfmt,

    #[serde(rename = "liquidsoap-prettier")]
    #[doc = "Prettier plugin for liquidsoap script - [https://github.com/savonet/liquidsoap-prettier](https://github.com/savonet/liquidsoap-prettier)"]
    LiquidsoapPrettier,

    #[serde(rename = "luaformatter")]
    #[doc = "Code formatter for Lua - [https://github.com/Koihik/LuaFormatter](https://github.com/Koihik/LuaFormatter)"]
    Luaformatter,

    #[serde(rename = "markdownfmt")]
    #[doc = "Like gofmt, but for Markdown - [https://github.com/shurcooL/markdownfmt](https://github.com/shurcooL/markdownfmt)"]
    Markdownfmt,

    #[serde(rename = "markdownlint")]
    #[doc = "A Node.js style checker and lint tool for Markdown/CommonMark files - [https://github.com/davidanson/markdownlint](https://github.com/davidanson/markdownlint)"]
    Markdownlint,

    #[serde(rename = "markuplint")]
    #[doc = "An HTML linter for all markup developers - [https://markuplint.dev/](https://markuplint.dev/)"]
    Markuplint,

    #[serde(rename = "mdformat")]
    #[doc = "CommonMark compliant Markdown formatter - [https://github.com/executablebooks/mdformat](https://github.com/executablebooks/mdformat)"]
    Mdformat,

    #[serde(rename = "misspell")]
    #[doc = "Correct commonly misspelled English words in source files - [https://github.com/client9/misspell/](https://github.com/client9/misspell/)"]
    Misspell,

    #[serde(rename = "mix:format")]
    #[doc = "Code formatter for Elixir - [https://hexdocs.pm/mix/main/Mix.Tasks.Format.html](https://hexdocs.pm/mix/main/Mix.Tasks.Format.html)"]
    MixFormat,

    #[serde(rename = "mojo:format")]
    #[doc = "Formats Mojo source files - [https://docs.modular.com/mojo/cli/format](https://docs.modular.com/mojo/cli/format)"]
    MojoFormat,

    #[serde(rename = "nickel:format")]
    #[doc = "Better configuration for less - [https://nickel-lang.org/](https://nickel-lang.org/)"]
    NickelFormat,

    #[serde(rename = "nimpretty")]
    #[doc = "Code formatter for the Nim programming language - [https://github.com/nim-lang/nim](https://github.com/nim-lang/nim)"]
    Nimpretty,

    #[serde(rename = "nixfmt")]
    #[doc = "The official (but not yet stable) formatter for Nix code - [https://github.com/serokell/nixfmt](https://github.com/serokell/nixfmt)"]
    Nixfmt,

    #[serde(rename = "nixpkgs-fmt")]
    #[doc = "Nix code formatter for nixpkgs - [https://github.com/nix-community/nixpkgs-fmt](https://github.com/nix-community/nixpkgs-fmt)"]
    NixpkgsFmt,

    #[serde(rename = "nph")]
    #[doc = "An opinionated code formatter for Nim - [https://github.com/arnetheduck/nph](https://github.com/arnetheduck/nph)"]
    Nph,

    #[serde(rename = "npm-groovy-lint")]
    #[doc = "Lint, format and auto-fix your Groovy / Jenkinsfile / Gradle files - [https://github.com/nvuillam/npm-groovy-lint](https://github.com/nvuillam/npm-groovy-lint)"]
    NpmGroovyLint,

    #[serde(rename = "ocamlformat")]
    #[doc = "Auto-formatter for OCaml code - [https://github.com/ocaml-ppx/ocamlformat](https://github.com/ocaml-ppx/ocamlformat)"]
    Ocamlformat,

    #[serde(rename = "ocp-indent")]
    #[doc = "Indentation tool for OCaml - [https://github.com/OCamlPro/ocp-indent](https://github.com/OCamlPro/ocp-indent)"]
    OcpIndent,

    #[serde(rename = "ormolu")]
    #[doc = "A formatter for Haskell source code - [https://github.com/tweag/ormolu](https://github.com/tweag/ormolu)"]
    Ormolu,

    #[serde(rename = "oxlint")]
    #[doc = "Oxlint is designed to catch erroneous or useless code without requiring any configurations by default - [https://oxc.rs/docs/guide/usage/linter.html](https://oxc.rs/docs/guide/usage/linter.html)"]
    Oxlint,

    #[serde(rename = "packer:fmt")]
    #[doc = "Packer is used to format HCL2 configuration files - [https://developer.hashicorp.com/packer/docs/commands/fmt](https://developer.hashicorp.com/packer/docs/commands/fmt)"]
    PackerFmt,

    #[serde(rename = "perltidy")]
    #[doc = "Perl::Tidy, a source code formatter for Perl - [https://github.com/perltidy/perltidy](https://github.com/perltidy/perltidy)"]
    Perltidy,

    #[serde(rename = "pg_format")]
    #[doc = "A PostgreSQL SQL syntax beautifier - [https://github.com/darold/pgFormatter](https://github.com/darold/pgFormatter)"]
    PgFormat,

    #[serde(rename = "php-cs-fixer:fix")]
    #[doc = "A tool to automatically fix PHP Coding Standards issues - [https://github.com/PHP-CS-Fixer/PHP-CS-Fixer](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer)"]
    PhpCsFixerFix,

    #[serde(rename = "phpcbf")]
    #[doc = "PHP Code Beautifier and Fixer fixes violations of a defined coding standard - [https://phpqa.io/projects/phpcbf.html](https://phpqa.io/projects/phpcbf.html)"]
    Phpcbf,

    #[serde(rename = "phpinsights:fix")]
    #[doc = "Instant PHP quality checks from your console - [https://github.com/nunomaduro/phpinsights](https://github.com/nunomaduro/phpinsights)"]
    PhpinsightsFix,

    #[serde(rename = "pint")]
    #[doc = "Laravel Pint is an opinionated PHP code style fixer for minimalists - [https://github.com/laravel/pint](https://github.com/laravel/pint)"]
    Pint,

    #[serde(rename = "prettier")]
    #[doc = "Prettier is an opinionated code formatter - [https://github.com/prettier/prettier](https://github.com/prettier/prettier)"]
    Prettier,

    #[serde(rename = "pretty-php")]
    #[doc = "The opinionated PHP code formatter - [https://github.com/lkrms/pretty-php](https://github.com/lkrms/pretty-php)"]
    PrettyPhp,

    #[serde(rename = "prettypst")]
    #[doc = "Formatter for Typst - [https://github.com/antonWetzel/prettypst](https://github.com/antonWetzel/prettypst)"]
    Prettypst,

    #[serde(rename = "puppet-lint")]
    #[doc = "Check that your Puppet manifests conform to the style guide - [https://github.com/puppetlabs/puppet-lint](https://github.com/puppetlabs/puppet-lint)"]
    PuppetLint,

    #[serde(rename = "purs-tidy")]
    #[doc = "PureScript code formatter - [https://github.com/natefaubion/purescript-tidy](https://github.com/natefaubion/purescript-tidy)"]
    PursTidy,

    #[serde(rename = "pycln")]
    #[doc = "A formatter for finding and removing unused import statements - [https://github.com/hadialqattan/pycln](https://github.com/hadialqattan/pycln)"]
    Pycln,

    #[serde(rename = "pyink")]
    #[doc = "Pyink is a Python formatter, forked from Black with a few different formatting behaviors - [https://github.com/google/pyink](https://github.com/google/pyink)"]
    Pyink,

    #[serde(rename = "qmlfmt")]
    #[doc = "qmlfmt - command line application that formats QML files - [https://github.com/jesperhh/qmlfmt](https://github.com/jesperhh/qmlfmt)"]
    Qmlfmt,

    #[serde(rename = "raco:fmt")]
    #[doc = "An extensible code formatter for Racket - [https://docs.racket-lang.org/fmt/](https://docs.racket-lang.org/fmt/)"]
    RacoFmt,

    #[serde(rename = "refmt")]
    #[doc = "refmt stands by Reason Formatter and it formats Reason programs, is a parser and pretty-printer for Reason - [https://reasonml.github.io/docs/en/refmt](https://reasonml.github.io/docs/en/refmt)"]
    Refmt,

    #[serde(rename = "rescript:format")]
    #[doc = "Formatter for ReScript - [https://rescript-lang.org/](https://rescript-lang.org/)"]
    RescriptFormat,

    #[serde(rename = "roc:format")]
    #[doc = "Tools for the roc programming language - [https://github.com/roc-lang/roc](https://github.com/roc-lang/roc)"]
    RocFormat,

    #[serde(rename = "rstfmt")]
    #[doc = "A formatter for reStructuredText - [https://github.com/dzhu/rstfmt](https://github.com/dzhu/rstfmt)"]
    Rstfmt,

    #[serde(rename = "rubocop")]
    #[doc = "A Ruby static code analyzer and formatter, based on the community Ruby style guide - [https://github.com/rubocop/rubocop](https://github.com/rubocop/rubocop)"]
    Rubocop,

    #[serde(rename = "rubyfmt")]
    #[doc = "Ruby Autoformatter - [https://github.com/fables-tales/rubyfmt](https://github.com/fables-tales/rubyfmt)"]
    Rubyfmt,

    #[serde(rename = "ruff:check")]
    #[doc = "An extremely fast Python linter and code formatter, written in Rust - [https://docs.astral.sh/ruff](https://docs.astral.sh/ruff)"]
    RuffCheck,

    #[serde(rename = "ruff:format")]
    #[doc = "An extremely fast Python linter and code formatter, written in Rust - [https://docs.astral.sh/ruff](https://docs.astral.sh/ruff)"]
    RuffFormat,

    #[serde(rename = "rufo")]
    #[doc = "The Ruby Formatter - [https://github.com/ruby-formatter/rufo](https://github.com/ruby-formatter/rufo)"]
    Rufo,

    #[serde(rename = "rune:fmt")]
    #[doc = "Tools for the Rune programming language - [https://github.com/rune-rs/rune](https://github.com/rune-rs/rune)"]
    RuneFmt,

    #[serde(rename = "rustfmt")]
    #[doc = "The official code formatter for Rust - [https://github.com/rust-lang/rustfmt](https://github.com/rust-lang/rustfmt)"]
    Rustfmt,

    #[serde(rename = "rustywind")]
    #[doc = "CLI for organizing Tailwind CSS classes - [https://github.com/avencera/rustywind](https://github.com/avencera/rustywind)"]
    Rustywind,

    #[serde(rename = "scalafmt")]
    #[doc = "Code formatter for Scala - [https://github.com/scalameta/scalafmt](https://github.com/scalameta/scalafmt)"]
    Scalafmt,

    #[serde(rename = "scalariform")]
    #[doc = "Scala source code formatter - [https://github.com/scala-ide/scalariform](https://github.com/scala-ide/scalariform)"]
    Scalariform,

    #[serde(rename = "shfmt")]
    #[doc = "Shell script formatter - [https://github.com/mvdan/sh](https://github.com/mvdan/sh)"]
    Shfmt,

    #[serde(rename = "sleek")]
    #[doc = "Sleek is a CLI tool for formatting SQL. It helps you maintain a consistent style across your SQL code, enhancing readability and productivity - [https://github.com/nrempel/sleek](https://github.com/nrempel/sleek)"]
    Sleek,

    #[serde(rename = "smlfmt")]
    #[doc = "A custom parser/auto-formatter for Standard ML - [https://github.com/shwestrick/smlfmt](https://github.com/shwestrick/smlfmt)"]
    Smlfmt,

    #[serde(rename = "snakefmt")]
    #[doc = "The uncompromising Snakemake code formatter - [https://github.com/snakemake/snakefmt](https://github.com/snakemake/snakefmt)"]
    Snakefmt,

    #[serde(rename = "sql-formatter")]
    #[doc = "A whitespace formatter for different query languages - [https://github.com/sql-formatter-org/sql-formatter](https://github.com/sql-formatter-org/sql-formatter)"]
    SqlFormatter,

    #[serde(rename = "sqlfluff:fix")]
    #[doc = "A modular SQL linter and auto-formatter with support for multiple dialects and templated code - [https://github.com/sqlfluff/sqlfluff](https://github.com/sqlfluff/sqlfluff)"]
    SqlfluffFix,

    #[serde(rename = "sqlfluff:format")]
    #[doc = "A modular SQL linter and auto-formatter with support for multiple dialects and templated code - [https://github.com/sqlfluff/sqlfluff](https://github.com/sqlfluff/sqlfluff)"]
    SqlfluffFormat,

    #[serde(rename = "sqlfmt")]
    #[doc = "sqlfmt formats your dbt SQL files so you don't have to - [https://github.com/tconbeer/sqlfmt](https://github.com/tconbeer/sqlfmt)"]
    Sqlfmt,

    #[serde(rename = "standardjs")]
    #[doc = "JavaScript style guide, linter, and formatter - [https://github.com/standard/standard](https://github.com/standard/standard)"]
    Standardjs,

    #[serde(rename = "standardrb")]
    #[doc = "Ruby's bikeshed-proof linter and formatter - [https://github.com/standardrb/standard](https://github.com/standardrb/standard)"]
    Standardrb,

    #[serde(rename = "stylefmt")]
    #[doc = "stylefmt is a tool that automatically formats stylesheets - [https://github.com/matype/stylefmt](https://github.com/matype/stylefmt)"]
    Stylefmt,

    #[serde(rename = "stylelint")]
    #[doc = "A mighty CSS linter that helps you avoid errors and enforce conventions - [https://github.com/stylelint/stylelint](https://github.com/stylelint/stylelint)"]
    Stylelint,

    #[serde(rename = "stylish-haskell")]
    #[doc = "Haskell code prettifier - [https://github.com/haskell/stylish-haskell](https://github.com/haskell/stylish-haskell)"]
    StylishHaskell,

    #[serde(rename = "stylua")]
    #[doc = "An opinionated Lua code formatter - [https://github.com/JohnnyMorganz/StyLua](https://github.com/JohnnyMorganz/StyLua)"]
    Stylua,

    #[serde(rename = "superhtml:fmt")]
    #[doc = "HTML Language Server & Templating Language Library - [https://github.com/kristoff-it/superhtml](https://github.com/kristoff-it/superhtml)"]
    SuperhtmlFmt,

    #[serde(rename = "swift-format")]
    #[doc = "Formatting technology for Swift source code - [https://github.com/apple/swift-format](https://github.com/apple/swift-format)"]
    SwiftFormat,

    #[serde(rename = "swiftformat")]
    #[doc = "A command-line tool and Xcode Extension for formatting Swift code - [https://github.com/nicklockwood/SwiftFormat](https://github.com/nicklockwood/SwiftFormat)"]
    Swiftformat,

    #[serde(rename = "taplo")]
    #[doc = "A TOML toolkit written in Rust - [https://github.com/tamasfe/taplo](https://github.com/tamasfe/taplo)"]
    Taplo,

    #[serde(rename = "templ:fmt")]
    #[doc = "Tooling for the Templ template language - [https://templ.guide/](https://templ.guide/)"]
    TemplFmt,

    #[serde(rename = "terraform:fmt")]
    #[doc = "The terraform fmt command is used to rewrite Terraform configuration files to a canonical format and style - [https://www.terraform.io/docs/cli/commands/fmt.html](https://www.terraform.io/docs/cli/commands/fmt.html)"]
    TerraformFmt,

    #[serde(rename = "tlint:format")]
    #[doc = "Tighten linter for Laravel conventions - [https://github.com/tighten/tlint](https://github.com/tighten/tlint)"]
    TlintFormat,

    #[serde(rename = "tofu:fmt")]
    #[doc = "The tofu fmt command is used to rewrite OpenTofu configuration files to a canonical format and style - [https://opentofu.org/docs/cli/commands/fmt/](https://opentofu.org/docs/cli/commands/fmt/)"]
    TofuFmt,

    #[serde(rename = "topiary")]
    #[doc = "Topiary aims to be a uniform formatter for simple languages, as part of the Tree-sitter ecosystem - [https://github.com/tweag/topiary](https://github.com/tweag/topiary)"]
    Topiary,

    #[serde(rename = "ts-standard")]
    #[doc = "Typescript style guide, linter, and formatter using StandardJS - [https://github.com/standard/ts-standard](https://github.com/standard/ts-standard)"]
    TsStandard,

    #[serde(rename = "twig-cs-fixer:lint")]
    #[doc = "A tool to automatically fix Twig Coding Standards issues - [https://github.com/VincentLanglet/Twig-CS-Fixer](https://github.com/VincentLanglet/Twig-CS-Fixer)"]
    TwigCsFixerLint,

    #[serde(rename = "typos")]
    #[doc = "Source code spell checker - [https://github.com/crate-ci/typos](https://github.com/crate-ci/typos)"]
    Typos,

    #[serde(rename = "typstfmt")]
    #[doc = "Basic formatter for the Typst language - [https://github.com/astrale-sharp/typstfmt](https://github.com/astrale-sharp/typstfmt)"]
    Typstfmt,

    #[serde(rename = "typstyle")]
    #[doc = "Beautiful and reliable typst code formatter - [https://github.com/Enter-tainer/typstyle](https://github.com/Enter-tainer/typstyle)"]
    Typstyle,

    #[serde(rename = "ufmt")]
    #[doc = "Safe, atomic formatting with black and usort - [https://github.com/omnilib/ufmt](https://github.com/omnilib/ufmt)"]
    Ufmt,

    #[serde(rename = "uiua:fmt")]
    #[doc = "A stack-based array programming language - [https://github.com/uiua-lang/uiua](https://github.com/uiua-lang/uiua)"]
    UiuaFmt,

    #[serde(rename = "usort")]
    #[doc = "Safe, minimal import sorting for Python projects - [https://github.com/facebook/usort](https://github.com/facebook/usort)"]
    Usort,

    #[serde(rename = "v:fmt")]
    #[doc = "Tooling for V lang - [https://vlang.io/](https://vlang.io/)"]
    VFmt,

    #[serde(rename = "veryl:fmt")]
    #[doc = "Veryl: A Modern Hardware Description Language - [https://github.com/veryl-lang/veryl](https://github.com/veryl-lang/veryl)"]
    VerylFmt,

    #[serde(rename = "vhdl-style-guide")]
    #[doc = "Style guide enforcement for VHDL - [https://github.com/jeremiah-c-leary/vhdl-style-guide](https://github.com/jeremiah-c-leary/vhdl-style-guide)"]
    VhdlStyleGuide,

    #[serde(rename = "wfindent")]
    #[doc = "Indents and optionally converts Fortran program sources - [https://github.com/wvermin/findent](https://github.com/wvermin/findent)"]
    Wfindent,

    #[serde(rename = "xmlformat")]
    #[doc = "Format and compress XML documents - [https://github.com/pamoller/xmlformatter](https://github.com/pamoller/xmlformatter)"]
    Xmlformat,

    #[serde(rename = "xmllint")]
    #[doc = "XML linter - [https://gnome.pages.gitlab.gnome.org/libxml2/xmllint.html](https://gnome.pages.gitlab.gnome.org/libxml2/xmllint.html)"]
    Xmllint,

    #[serde(rename = "xo")]
    #[doc = "JavaScript/TypeScript linter (ESLint wrapper) with great defaults - [http://github.com/xojs/xo](http://github.com/xojs/xo)"]
    Xo,

    #[serde(rename = "yamlfix")]
    #[doc = "A simple opinionated yaml formatter that keeps your comments - [https://github.com/lyz-code/yamlfix](https://github.com/lyz-code/yamlfix)"]
    Yamlfix,

    #[serde(rename = "yamlfmt")]
    #[doc = "An extensible command line tool or library to format yaml files - [https://github.com/google/yamlfmt](https://github.com/google/yamlfmt)"]
    Yamlfmt,

    #[serde(rename = "yapf")]
    #[doc = "A formatter for Python files - [https://github.com/google/yapf](https://github.com/google/yapf)"]
    Yapf,

    #[serde(rename = "yew-fmt")]
    #[doc = "Code formatter for the Yew framework - [https://github.com/its-the-shrimp/yew-fmt](https://github.com/its-the-shrimp/yew-fmt)"]
    YewFmt,

    #[serde(rename = "zig:fmt")]
    #[doc = "Reformat Zig source into canonical form - [https://ziglang.org/](https://ziglang.org/)"]
    ZigFmt,

    #[serde(rename = "ziggy:fmt")]
    #[doc = "Formats Ziggy documents and Ziggy schemas - [https://ziggy-lang.io/documentation/ziggy-fmt/](https://ziggy-lang.io/documentation/ziggy-fmt/)"]
    ZiggyFmt,

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
            Self::BeancountBlack => beancount_black::run(snippet_path),
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
            Self::BufFormat => buf_format::run(snippet_path),
            Self::Buildifier => buildifier::run(snippet_path),
            Self::CabalFormat => cabal_format::run(snippet_path),
            Self::CaramelFmt => caramel_fmt::run(snippet_path),
            Self::ClangFormat => clang_format::run(snippet_path),
            Self::ClangTidy => clang_tidy::run(snippet_path),
            Self::CljfmtFix => cljfmt_fix::run(snippet_path),
            Self::Cljstyle => cljstyle::run(snippet_path),
            Self::Codespell => codespell::run(snippet_path),
            Self::Crlfmt => crlfmt::run(snippet_path),
            Self::CrystalFormat => crystal_format::run(snippet_path),
            Self::Csharpier => csharpier::run(snippet_path),
            Self::CssBeautify => css_beautify::run(snippet_path),
            Self::Csscomb => csscomb::run(snippet_path),
            Self::D2Fmt => d_2_fmt::run(snippet_path),
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
            Self::DotenvLinterFix => dotenv_linter_fix::run(snippet_path),
            Self::DprintFmt => dprint_fmt::run(snippet_path),
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
            Self::GluonFmt => gluon_fmt::run(snippet_path),
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
            Self::JsonaFormat => jsona_format::run(snippet_path),
            Self::JsonaLint => jsona_lint::run(snippet_path),
            Self::Jsonnetfmt => jsonnetfmt::run(snippet_path),
            Self::JuliaformatterJl => juliaformatter_jl::run(snippet_path),
            Self::Just => just::run(snippet_path),
            Self::KclFmt => kcl_fmt::run(snippet_path),
            Self::Kdlfmt => kdlfmt::run(snippet_path),
            Self::Ktfmt => ktfmt::run(snippet_path),
            Self::Ktlint => ktlint::run(snippet_path),
            Self::KulalaFmt => kulala_fmt::run(snippet_path),
            Self::Leptosfmt => leptosfmt::run(snippet_path),
            Self::LiquidsoapPrettier => liquidsoap_prettier::run(snippet_path),
            Self::Luaformatter => luaformatter::run(snippet_path),
            Self::Markdownfmt => markdownfmt::run(snippet_path),
            Self::Markdownlint => markdownlint::run(snippet_path),
            Self::Markuplint => markuplint::run(snippet_path),
            Self::Mdformat => mdformat::run(snippet_path),
            Self::Misspell => misspell::run(snippet_path),
            Self::MixFormat => mix_format::run(snippet_path),
            Self::MojoFormat => mojo_format::run(snippet_path),
            Self::NickelFormat => nickel_format::run(snippet_path),
            Self::Nimpretty => nimpretty::run(snippet_path),
            Self::Nixfmt => nixfmt::run(snippet_path),
            Self::NixpkgsFmt => nixpkgs_fmt::run(snippet_path),
            Self::Nph => nph::run(snippet_path),
            Self::NpmGroovyLint => npm_groovy_lint::run(snippet_path),
            Self::Ocamlformat => ocamlformat::run(snippet_path),
            Self::OcpIndent => ocp_indent::run(snippet_path),
            Self::Ormolu => ormolu::run(snippet_path),
            Self::Oxlint => oxlint::run(snippet_path),
            Self::PackerFmt => packer_fmt::run(snippet_path),
            Self::Perltidy => perltidy::run(snippet_path),
            Self::PgFormat => pg_format::run(snippet_path),
            Self::PhpCsFixerFix => php_cs_fixer_fix::run(snippet_path),
            Self::Phpcbf => phpcbf::run(snippet_path),
            Self::PhpinsightsFix => phpinsights_fix::run(snippet_path),
            Self::Pint => pint::run(snippet_path),
            Self::Prettier => prettier::run(snippet_path),
            Self::PrettyPhp => pretty_php::run(snippet_path),
            Self::Prettypst => prettypst::run(snippet_path),
            Self::PuppetLint => puppet_lint::run(snippet_path),
            Self::PursTidy => purs_tidy::run(snippet_path),
            Self::Pycln => pycln::run(snippet_path),
            Self::Pyink => pyink::run(snippet_path),
            Self::Qmlfmt => qmlfmt::run(snippet_path),
            Self::RacoFmt => raco_fmt::run(snippet_path),
            Self::Refmt => refmt::run(snippet_path),
            Self::RescriptFormat => rescript_format::run(snippet_path),
            Self::RocFormat => roc_format::run(snippet_path),
            Self::Rstfmt => rstfmt::run(snippet_path),
            Self::Rubocop => rubocop::run(snippet_path),
            Self::Rubyfmt => rubyfmt::run(snippet_path),
            Self::RuffCheck => ruff_check::run(snippet_path),
            Self::RuffFormat => ruff_format::run(snippet_path),
            Self::Rufo => rufo::run(snippet_path),
            Self::RuneFmt => rune_fmt::run(snippet_path),
            Self::Rustfmt => rustfmt::run(snippet_path),
            Self::Rustywind => rustywind::run(snippet_path),
            Self::Scalafmt => scalafmt::run(snippet_path),
            Self::Scalariform => scalariform::run(snippet_path),
            Self::Shfmt => shfmt::run(snippet_path),
            Self::Sleek => sleek::run(snippet_path),
            Self::Smlfmt => smlfmt::run(snippet_path),
            Self::Snakefmt => snakefmt::run(snippet_path),
            Self::SqlFormatter => sql_formatter::run(snippet_path),
            Self::SqlfluffFix => sqlfluff_fix::run(snippet_path),
            Self::SqlfluffFormat => sqlfluff_format::run(snippet_path),
            Self::Sqlfmt => sqlfmt::run(snippet_path),
            Self::Standardjs => standardjs::run(snippet_path),
            Self::Standardrb => standardrb::run(snippet_path),
            Self::Stylefmt => stylefmt::run(snippet_path),
            Self::Stylelint => stylelint::run(snippet_path),
            Self::StylishHaskell => stylish_haskell::run(snippet_path),
            Self::Stylua => stylua::run(snippet_path),
            Self::SuperhtmlFmt => superhtml_fmt::run(snippet_path),
            Self::SwiftFormat => swift_format::run(snippet_path),
            Self::Swiftformat => swiftformat::run(snippet_path),
            Self::Taplo => taplo::run(snippet_path),
            Self::TemplFmt => templ_fmt::run(snippet_path),
            Self::TerraformFmt => terraform_fmt::run(snippet_path),
            Self::TlintFormat => tlint_format::run(snippet_path),
            Self::TofuFmt => tofu_fmt::run(snippet_path),
            Self::Topiary => topiary::run(snippet_path),
            Self::TsStandard => ts_standard::run(snippet_path),
            Self::TwigCsFixerLint => twig_cs_fixer_lint::run(snippet_path),
            Self::Typos => typos::run(snippet_path),
            Self::Typstfmt => typstfmt::run(snippet_path),
            Self::Typstyle => typstyle::run(snippet_path),
            Self::Ufmt => ufmt::run(snippet_path),
            Self::UiuaFmt => uiua_fmt::run(snippet_path),
            Self::Usort => usort::run(snippet_path),
            Self::VFmt => v_fmt::run(snippet_path),
            Self::VerylFmt => veryl_fmt::run(snippet_path),
            Self::VhdlStyleGuide => vhdl_style_guide::run(snippet_path),
            Self::Wfindent => wfindent::run(snippet_path),
            Self::Xmlformat => xmlformat::run(snippet_path),
            Self::Xmllint => xmllint::run(snippet_path),
            Self::Xo => xo::run(snippet_path),
            Self::Yamlfix => yamlfix::run(snippet_path),
            Self::Yamlfmt => yamlfmt::run(snippet_path),
            Self::Yapf => yapf::run(snippet_path),
            Self::YewFmt => yew_fmt::run(snippet_path),
            Self::ZigFmt => zig_fmt::run(snippet_path),
            Self::ZiggyFmt => ziggy_fmt::run(snippet_path),
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
            Self::BeancountBlack => "beancount_black",
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
            Self::BufFormat => "buf_format",
            Self::Buildifier => "buildifier",
            Self::CabalFormat => "cabal_format",
            Self::CaramelFmt => "caramel_fmt",
            Self::ClangFormat => "clang_format",
            Self::ClangTidy => "clang_tidy",
            Self::CljfmtFix => "cljfmt_fix",
            Self::Cljstyle => "cljstyle",
            Self::Codespell => "codespell",
            Self::Crlfmt => "crlfmt",
            Self::CrystalFormat => "crystal_format",
            Self::Csharpier => "csharpier",
            Self::CssBeautify => "css_beautify",
            Self::Csscomb => "csscomb",
            Self::D2Fmt => "d_2_fmt",
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
            Self::DotenvLinterFix => "dotenv_linter_fix",
            Self::DprintFmt => "dprint_fmt",
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
            Self::GluonFmt => "gluon_fmt",
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
            Self::JsonaFormat => "jsona_format",
            Self::JsonaLint => "jsona_lint",
            Self::Jsonnetfmt => "jsonnetfmt",
            Self::JuliaformatterJl => "juliaformatter_jl",
            Self::Just => "just",
            Self::KclFmt => "kcl_fmt",
            Self::Kdlfmt => "kdlfmt",
            Self::Ktfmt => "ktfmt",
            Self::Ktlint => "ktlint",
            Self::KulalaFmt => "kulala_fmt",
            Self::Leptosfmt => "leptosfmt",
            Self::LiquidsoapPrettier => "liquidsoap_prettier",
            Self::Luaformatter => "luaformatter",
            Self::Markdownfmt => "markdownfmt",
            Self::Markdownlint => "markdownlint",
            Self::Markuplint => "markuplint",
            Self::Mdformat => "mdformat",
            Self::Misspell => "misspell",
            Self::MixFormat => "mix_format",
            Self::MojoFormat => "mojo_format",
            Self::NickelFormat => "nickel_format",
            Self::Nimpretty => "nimpretty",
            Self::Nixfmt => "nixfmt",
            Self::NixpkgsFmt => "nixpkgs_fmt",
            Self::Nph => "nph",
            Self::NpmGroovyLint => "npm_groovy_lint",
            Self::Ocamlformat => "ocamlformat",
            Self::OcpIndent => "ocp_indent",
            Self::Ormolu => "ormolu",
            Self::Oxlint => "oxlint",
            Self::PackerFmt => "packer_fmt",
            Self::Perltidy => "perltidy",
            Self::PgFormat => "pg_format",
            Self::PhpCsFixerFix => "php_cs_fixer_fix",
            Self::Phpcbf => "phpcbf",
            Self::PhpinsightsFix => "phpinsights_fix",
            Self::Pint => "pint",
            Self::Prettier => "prettier",
            Self::PrettyPhp => "pretty_php",
            Self::Prettypst => "prettypst",
            Self::PuppetLint => "puppet_lint",
            Self::PursTidy => "purs_tidy",
            Self::Pycln => "pycln",
            Self::Pyink => "pyink",
            Self::Qmlfmt => "qmlfmt",
            Self::RacoFmt => "raco_fmt",
            Self::Refmt => "refmt",
            Self::RescriptFormat => "rescript_format",
            Self::RocFormat => "roc_format",
            Self::Rstfmt => "rstfmt",
            Self::Rubocop => "rubocop",
            Self::Rubyfmt => "rubyfmt",
            Self::RuffCheck => "ruff_check",
            Self::RuffFormat => "ruff_format",
            Self::Rufo => "rufo",
            Self::RuneFmt => "rune_fmt",
            Self::Rustfmt => "rustfmt",
            Self::Rustywind => "rustywind",
            Self::Scalafmt => "scalafmt",
            Self::Scalariform => "scalariform",
            Self::Shfmt => "shfmt",
            Self::Sleek => "sleek",
            Self::Smlfmt => "smlfmt",
            Self::Snakefmt => "snakefmt",
            Self::SqlFormatter => "sql_formatter",
            Self::SqlfluffFix => "sqlfluff_fix",
            Self::SqlfluffFormat => "sqlfluff_format",
            Self::Sqlfmt => "sqlfmt",
            Self::Standardjs => "standardjs",
            Self::Standardrb => "standardrb",
            Self::Stylefmt => "stylefmt",
            Self::Stylelint => "stylelint",
            Self::StylishHaskell => "stylish_haskell",
            Self::Stylua => "stylua",
            Self::SuperhtmlFmt => "superhtml_fmt",
            Self::SwiftFormat => "swift_format",
            Self::Swiftformat => "swiftformat",
            Self::Taplo => "taplo",
            Self::TemplFmt => "templ_fmt",
            Self::TerraformFmt => "terraform_fmt",
            Self::TlintFormat => "tlint_format",
            Self::TofuFmt => "tofu_fmt",
            Self::Topiary => "topiary",
            Self::TsStandard => "ts_standard",
            Self::TwigCsFixerLint => "twig_cs_fixer_lint",
            Self::Typos => "typos",
            Self::Typstfmt => "typstfmt",
            Self::Typstyle => "typstyle",
            Self::Ufmt => "ufmt",
            Self::UiuaFmt => "uiua_fmt",
            Self::Usort => "usort",
            Self::VFmt => "v_fmt",
            Self::VerylFmt => "veryl_fmt",
            Self::VhdlStyleGuide => "vhdl_style_guide",
            Self::Wfindent => "wfindent",
            Self::Xmlformat => "xmlformat",
            Self::Xmllint => "xmllint",
            Self::Xo => "xo",
            Self::Yamlfix => "yamlfix",
            Self::Yamlfmt => "yamlfmt",
            Self::Yapf => "yapf",
            Self::YewFmt => "yew_fmt",
            Self::ZigFmt => "zig_fmt",
            Self::ZiggyFmt => "ziggy_fmt",
            Self::Zprint => "zprint",
        }
    }
}

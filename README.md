# mdsf

Format markdown code snippets using your favorite code formatters.

<a href="https://crates.io/crates/mdsf"><img src="https://img.shields.io/crates/v/mdsf.svg"></a>
<a href="https://github.com/hougesen/mdsf/actions/workflows/validate.yml"><img src="https://github.com/hougesen/mdsf/actions/workflows/validate.yml/badge.svg"></a>

<!-- <a href="https://codecov.io/gh/hougesen/mdsf"><img src="https://codecov.io/gh/hougesen/mdsf/branch/main/graph/badge.svg"/></a> -->

<!-- START_SECTION:base-command-help -->

```
mdsf 0.2.7
Format markdown code snippets using your favorite code formatters
Mads Hougesen <mads@mhouge.dk>

Usage: mdsf <COMMAND>

Commands:
  format       Run formatters on input files
  verify       Verify files are formatted
  init         Create a new mdsf config
  completions  Generate shell completion
  cache-prune  Remove old caches
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

<!-- END_SECTION:base-command-help -->

## Installation

The latest version of `mdsf` can be downloaded directly from [github.com/hougesen/mdsf/releases](https://github.com/hougesen/mdsf/releases).

## Install

### Linux & MacOS

```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/hougesen/mdsf/releases/latest/download/mdsf-installer.sh | sh
```

### Windows

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/hougesen/mdsf/releases/latest/download/mdsf-installer.ps1 | iex"
```

### Cargo

Install using the [published crate](https://crates.io/crates/mdsf):

```shell
cargo install mdsf --locked
```

or directly from source:

```shell
git clone git@github.com:hougesen/mdsf.git

cargo install --path ./mdsf --bin mdsf
```

If you do not have Cargo installed, you need to [install it first](https://www.rust-lang.org/learn/get-started).

### npm/npx

You can install mdsf using [npm](https://www.npmjs.com/package/mdsf-cli):

```shell
npm install -g mdsf-cli

mdsf format .
```

or run it directly using npx:

```shell
npx mdsf-cli format .
```

### Homebrew

```shell
brew install hougesen/tap/mdsf
```

## Usage

```shell
mdsf format file.md
```

<!-- START_SECTION:format-command-help -->

```
Run formatters on input files

Usage: mdsf format [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...  Path to files and/or directories

Options:
      --config <CONFIG>        Path to config
      --debug                  Log stdout and stderr of formatters
      --log-level <LOG_LEVEL>  [possible values: trace, debug, info, warn, error, off]
      --threads <THREADS>      Amount of threads to use. Defaults to 0 (auto)
      --cache                  Only format changed codeblocks
  -h, --help                   Print help
  -V, --version                Print version
```

<!-- END_SECTION:format-command-help -->

### Verify code is formatted

```shell
mdsf verify docs/
```

<!-- START_SECTION:verify-command-help -->

```
Verify files are formatted

Usage: mdsf verify [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...  Path to files and/or directories

Options:
      --config <CONFIG>        Path to config
      --debug                  Log stdout and stderr of formatters
      --log-level <LOG_LEVEL>  [possible values: trace, debug, info, warn, error, off]
      --threads <THREADS>      Amount of threads to use. Defaults to 0 (auto)
  -h, --help                   Print help
  -V, --version                Print version
```

<!-- END_SECTION:verify-command-help -->

## Configuration

The default configuration of `mdsf` aims to as sane as possible. For that reason the default formatter for each language is the one most people have installed.

If you are interested in customizing which formatter is run, you can create a new `mdsf` configuration file by running

```shell
mdsf init
```

`mdsf` supports running multiple formatters on the save code snippet.

```json
{
  "languages": {
    // Only run `ruff` on Python snippets,
    "python": "ruff",
    // Run `usort` on file and then `black`
    "python": ["usort", "black"],
    // Run `usort`, if that fails run `isort`, finally run `black`
    "python": [["usort", "isort"], "black"],

    // Formatters listed under "*" will be run on any snippet.
    "*": ["typos"],

    // Formatters listed under "_" will only be run when there is not formatter configured for the file type OR globally ("*").
    "_": "prettier"
  }
}
```

### Supported tools

> \[!NOTE\]
> mdsf is not a package manager.
>
> Only tools that are already installed will be used.

<!-- START_SECTION:supported-tools -->

`mdsf` currently supports 189 tools. Feel free to open an issue/pull-request if your favorite tool is missing! 😃

| Formatter            | Description                                                                                                                                                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| alejandra            | The Uncompromising Nix Code Formatter - [https://github.com/kamadorueda/alejandra](https://github.com/kamadorueda/alejandra)                                                                                                    |
| ameba                | A static code analysis tool for Crystal - [https://github.com/crystal-ameba/ameba](https://github.com/crystal-ameba/ameba)                                                                                                      |
| asmfmt               | Go Assembler Formatter - [https://github.com/klauspost/asmfmt](https://github.com/klauspost/asmfmt)                                                                                                                             |
| astyle               | A Free, Fast, and Small Automatic Formatter for C, C++, C++/CLI, Objective-C, C#, and Java Source Code - [https://gitlab.com/saalen/astyle](https://gitlab.com/saalen/astyle)                                                   |
| auto-optional        | Adds the Optional type-hint to arguments where the default value is None - [https://pypi.org/project/auto-optional/](https://pypi.org/project/auto-optional/)                                                                   |
| autocorrect          | A linter and formatter to help you to improve copywriting, correct spaces, words, and punctuations between CJK (Chinese, Japanese, Korean) - [https://github.com/huacnlee/autocorrect](https://github.com/huacnlee/autocorrect) |
| autoflake            | Removes unused imports and unused variables as reported by pyflakes - [https://github.com/pycqa/autoflake](https://github.com/pycqa/autoflake)                                                                                  |
| autopep8             | A tool that automatically formats Python code to conform to the PEP 8 style guid - [https://pypi.org/project/autopep8/](https://pypi.org/project/autopep8/)                                                                     |
| beancount-black      | Opinionated code formatter, just like Python's black code formatter but for Beancount - [https://github.com/LaunchPlatform/beancount-black](https://github.com/LaunchPlatform/beancount-black)                                  |
| beautysh             | A Bash beautifier for the masses - [https://pypi.org/project/beautysh/](https://pypi.org/project/beautysh/)                                                                                                                     |
| bicep:format         | Bicep is a declarative language for describing and deploying Azure resources - [https://github.com/Azure/bicep](https://github.com/Azure/bicep)                                                                                 |
| biome:check          | One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)                                                                                                                                               |
| biome:format         | One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)                                                                                                                                               |
| biome:lint           | One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)                                                                                                                                               |
| black                | The uncompromising Python code formatter - [https://github.com/psf/black](https://github.com/psf/black)                                                                                                                         |
| blade-formatter      | An opinionated blade template formatter for Laravel that respects readability - [https://github.com/shufo/blade-formatter](https://github.com/shufo/blade-formatter)                                                            |
| blue                 | The slightly less uncompromising Python code formatter - [https://blue.readthedocs.io/en/latest/](https://blue.readthedocs.io/en/latest/)                                                                                       |
| bpfmt                | A formatter for Blueprint files - [https://source.android.com/docs/setup/reference/androidbp#formatter](https://source.android.com/docs/setup/reference/androidbp#formatter)                                                    |
| brittany             | A Haskell source code formatter - [https://github.com/lspitzner/brittany](https://github.com/lspitzner/brittany)                                                                                                                |
| brunette             | A best practice Python code formatter - [https://github.com/odwyersoftware/brunette](https://github.com/odwyersoftware/brunette)                                                                                                |
| bsfmt                | A code formatter for BrightScript and BrighterScript - [https://github.com/rokucommunity/brighterscript-formatter](https://github.com/rokucommunity/brighterscript-formatter)                                                   |
| buf:format           | The best way of working with Protocol Buffers - [https://buf.build/docs/reference/cli/buf/format/](https://buf.build/docs/reference/cli/buf/format/)                                                                            |
| buildifier           | A bazel BUILD file formatter and - [https://github.com/bazelbuild/buildtools](https://github.com/bazelbuild/buildtools)                                                                                                         |
| cabal:format         | Cabal is a system for building and packaging Haskell libraries and programs - [https://www.haskell.org/cabal/](https://www.haskell.org/cabal/)                                                                                  |
| caramel:fmt          | Formatter for the Caramel programming language - [https://caramel.run/](https://caramel.run/)                                                                                                                                   |
| clang-format         | A tool to format C/C++/Java/JavaScript/JSON/Objective-C/Protobuf/C# code - [https://clang.llvm.org/docs/ClangFormat.html](https://clang.llvm.org/docs/ClangFormat.html)                                                         |
| clang-tidy           | clang-tidy is a clang-based C++ “linter” tool - [https://clang.llvm.org/extra/clang-tidy/](https://clang.llvm.org/extra/clang-tidy/)                                                                                            |
| cljfmt:fix           | A tool for formatting Clojure code - [https://github.com/weavejester/cljfmt](https://github.com/weavejester/cljfmt)                                                                                                             |
| cljstyle             | A tool for formatting Clojure code - [https://github.com/greglook/cljstyle](https://github.com/greglook/cljstyle)                                                                                                               |
| codespell            | Check code for common misspellings - [https://github.com/codespell-project/codespell](https://github.com/codespell-project/codespell)                                                                                           |
| crlfmt               | Formatter for CockroachDB's additions to the Go style guide - [https://github.com/cockroachdb/crlfmt](https://github.com/cockroachdb/crlfmt)                                                                                    |
| crystal:format       | Tools for the Crystal programming language - [https://crystal-lang.org/](https://crystal-lang.org/)                                                                                                                             |
| csharpier            | An Opinionated Code Formatter for C# - [https://csharpier.com/](https://csharpier.com/)                                                                                                                                         |
| css-beautify         | A css formatter - [https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html](https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html)                                                              |
| csscomb              | CSS coding style formatter - [https://github.com/csscomb/csscomb.js](https://github.com/csscomb/csscomb.js)                                                                                                                     |
| d2:fmt               | Formatter for the d2 language - [https://d2lang.com/](https://d2lang.com/)                                                                                                                                                      |
| dart:fix             | Formatter and linter for Dart - [https://dart.dev/tools](https://dart.dev/tools)                                                                                                                                                |
| dart:format          | Formatter and linter for Dart - [https://dart.dev/tools](https://dart.dev/tools)                                                                                                                                                |
| dcm:fix              | Code Quality Tool for Flutter Developers - [https://dcm.dev/](https://dcm.dev/)                                                                                                                                                 |
| dcm:format           | Code Quality Tool for Flutter Developers - [https://dcm.dev/](https://dcm.dev/)                                                                                                                                                 |
| deno:fmt             | Formatter and linter for JavaScript and TypeScript - [https://docs.deno.com/runtime/reference/cli/](https://docs.deno.com/runtime/reference/cli/)                                                                               |
| deno:lint            | Formatter and linter for JavaScript and TypeScript - [https://docs.deno.com/runtime/reference/cli/](https://docs.deno.com/runtime/reference/cli/)                                                                               |
| dfmt                 | Dfmt is a formatter for D source code - [https://github.com/dlang-community/dfmt](https://github.com/dlang-community/dfmt)                                                                                                      |
| dhall                | Format Dhall files - [https://dhall-lang.org/](https://dhall-lang.org/)                                                                                                                                                         |
| djlint               | Lint & Format HTML Templates - [https://www.djlint.com/](https://www.djlint.com/)                                                                                                                                               |
| docformatter         | Formats docstrings to follow PEP 257 - [https://pypi.org/project/docformatter/](https://pypi.org/project/docformatter/)                                                                                                         |
| docstrfmt            | A formatter for Sphinx flavored reStructuredText - [https://pypi.org/project/docstrfmt/](https://pypi.org/project/docstrfmt/)                                                                                                   |
| dotenv-linter:fix    | Lightning-fast linter for .env files - [https://github.com/dotenv-linter/dotenv-linter](https://github.com/dotenv-linter/dotenv-linter)                                                                                         |
| dprint:fmt           | A pluggable and configurable code formatting platform written in Rust - [https://dprint.dev/](https://dprint.dev/)                                                                                                              |
| easy-coding-standard | The Easiest way to add coding standard to your PHP project - [https://github.com/easy-coding-standard/easy-coding-standard](https://github.com/easy-coding-standard/easy-coding-standard)                                       |
| efmt                 | Erlang code formatter - [https://github.com/sile/efmt](https://github.com/sile/efmt)                                                                                                                                            |
| elm-format           | elm-format formats Elm source code according to a standard set of rules based on the official Elm Style Guide - [https://github.com/avh4/elm-format](https://github.com/avh4/elm-format)                                        |
| erb-formatter        | Format ERB files with speed and precision - [https://github.com/nebulab/erb-formatter](https://github.com/nebulab/erb-formatter)                                                                                                |
| erlfmt               | An automated code formatter for Erlang - [https://github.com/WhatsApp/erlfmt](https://github.com/WhatsApp/erlfmt)                                                                                                               |
| eslint               | Find and fix problems in your JavaScript code - [https://github.com/eslint/eslint/](https://github.com/eslint/eslint/)                                                                                                          |
| fantomas             | FSharp source code formatter - [https://github.com/fsprojects/fantomas](https://github.com/fsprojects/fantomas)                                                                                                                 |
| fish_indent          | Fish indenter and prettifier - [https://fishshell.com/docs/current/cmds/fish_indent.html](https://fishshell.com/docs/current/cmds/fish_indent.html)                                                                             |
| fixjson              | JSON Fixer for Humans using (relaxed) JSON5 - [https://github.com/rhysd/fixjson](https://github.com/rhysd/fixjson)                                                                                                              |
| floskell             | Floskell is a flexible Haskell source code pretty printer - [https://github.com/ennocramer/floskell](https://github.com/ennocramer/floskell)                                                                                    |
| fnlfmt               | A formatter for Fennel code - [https://git.sr.ht/~technomancy/fnlfmt](https://git.sr.ht/~technomancy/fnlfmt)                                                                                                                    |
| forge:fmt            | A Solidity formatter - [https://github.com/foundry-rs/foundry](https://github.com/foundry-rs/foundry)                                                                                                                           |
| fourmolu             | A formatter for Haskell source code - [https://hackage.haskell.org/package/fourmolu](https://hackage.haskell.org/package/fourmolu)                                                                                              |
| fprettify            | Auto-formatter for modern Fortran source code - [https://github.com/fortran-lang/fprettify](https://github.com/fortran-lang/fprettify)                                                                                          |
| gci                  | GCI, a tool that control golang package import order and make it always deterministic - [https://github.com/daixiang0/gci](https://github.com/daixiang0/gci)                                                                    |
| gdformat             | https://github.com/scony/godot-gdscript-toolkit - [GDScript linter](GDScript linter)                                                                                                                                            |
| gersemi              | A formatter to make your CMake code the real treasure - [https://github.com/blankspruce/gersemi](https://github.com/blankspruce/gersemi)                                                                                        |
| gleam:format         | Format Gleam source code - [https://gleam.run](https://gleam.run)                                                                                                                                                               |
| gluon:fmt            | Code formatting for the gluon programming language - [https://github.com/gluon-lang/gluon](https://github.com/gluon-lang/gluon)                                                                                                 |
| gofmt                | Gofmt formats Go programs - [https://pkg.go.dev/cmd/gofmt](https://pkg.go.dev/cmd/gofmt)                                                                                                                                        |
| gofumpt              | A stricter gofmt - [https://github.com/mvdan/gofumpt](https://github.com/mvdan/gofumpt)                                                                                                                                         |
| goimports            | goimports updates your Go import lines, adding missing ones and removing unreferenced ones - [https://pkg.go.dev/golang.org/x/tools/cmd/goimports](https://pkg.go.dev/golang.org/x/tools/cmd/goimports)                         |
| goimports-reviser    | Right imports sorting & code formatting tool (goimports alternative) - [https://github.com/incu6us/goimports-reviser](https://github.com/incu6us/goimports-reviser)                                                             |
| golines              | A golang formatter that fixes long lines - [https://github.com/segmentio/golines](https://github.com/segmentio/golines)                                                                                                         |
| google-java-format   | Reformats Java source code to comply with Google Java Style - [https://github.com/google/google-java-format](https://github.com/google/google-java-format)                                                                      |
| grain:format         | Code formatter for the Grain programming language - [https://grain-lang.org/docs/tooling/grain_cli](https://grain-lang.org/docs/tooling/grain_cli)                                                                              |
| haml-lint            | Tool for writing clean and consistent HAML - [https://github.com/sds/haml-lint](https://github.com/sds/haml-lint)                                                                                                               |
| hfmt                 | Format Haskell programs. Inspired by the gofmt utility - [https://github.com/danstiner/hfmt](https://github.com/danstiner/hfmt)                                                                                                 |
| hindent              | Extensible Haskell pretty printer - [https://github.com/mihaimaruseac/hindent](https://github.com/mihaimaruseac/hindent)                                                                                                        |
| html-beautify        | A html formatter - [https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html](https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html)                                                             |
| htmlbeautifier       | A normaliser/beautifier for HTML that also understands embedded Ruby. Ideal for tidying up Rails templates - [https://github.com/threedaymonk/htmlbeautifier](https://github.com/threedaymonk/htmlbeautifier)                   |
| imba:fmt             | A formatter for Imba - [https://imba.io/](https://imba.io/)                                                                                                                                                                     |
| isort                | A Python utility to sort imports - [https://github.com/timothycrosley/isort](https://github.com/timothycrosley/isort)                                                                                                           |
| joker                | Small Clojure interpreter, linter and formatter - [https://github.com/candid82/joker](https://github.com/candid82/joker)                                                                                                        |
| js-beautify          | A JavaScript formatter - [https://github.com/beautifier/js-beautify](https://github.com/beautifier/js-beautify)                                                                                                                 |
| jsona:format         | JSONA linter and formatter - [https://github.com/jsona/jsona](https://github.com/jsona/jsona)                                                                                                                                   |
| jsona:lint           | JSONA linter and formatter - [https://github.com/jsona/jsona](https://github.com/jsona/jsona)                                                                                                                                   |
| jsonnetfmt           | Formatter for automatically fixing jsonnet stylistic problems - [https://jsonnet.org/learning/tools.html](https://jsonnet.org/learning/tools.html)                                                                              |
| juliaformatter.jl    | An opinionated code formatter for Julia. Plot twist - the opinion is your own - [https://github.com/domluna/JuliaFormatter.jl](https://github.com/domluna/JuliaFormatter.jl)                                                    |
| just                 | A formatter for justfiles - [https://github.com/casey/just](https://github.com/casey/just)                                                                                                                                      |
| kcl:fmt              | KCL Format tool supports reformatting KCL files to the standard code style - [https://www.kcl-lang.io/docs/tools/cli/kcl/fmt](https://www.kcl-lang.io/docs/tools/cli/kcl/fmt)                                                   |
| kdlfmt               | A formatter for kdl documents - [https://github.com/hougesen/kdlfmt](https://github.com/hougesen/kdlfmt)                                                                                                                        |
| ktfmt                | program that reformats Kotlin source code to comply with the common community standard for Kotlin code conventions - [https://github.com/facebook/ktfmt](https://github.com/facebook/ktfmt)                                     |
| ktlint               | An anti-bikeshedding Kotlin linter with built-in formatter - [https://github.com/pinterest/ktlint](https://github.com/pinterest/ktlint)                                                                                         |
| kulala-fmt           | An opinionated 🦄 .http and .rest 🐼 files linter 💄 and formatter ⚡. - [https://github.com/mistweaverco/kulala-fmt](https://github.com/mistweaverco/kulala-fmt)                                                               |
| leptosfmt            | A formatter for the leptos view! macro - [https://github.com/bram209/leptosfmt](https://github.com/bram209/leptosfmt)                                                                                                           |
| liquidsoap-prettier  | Prettier plugin for liquidsoap script - [https://github.com/savonet/liquidsoap-prettier](https://github.com/savonet/liquidsoap-prettier)                                                                                        |
| luaformatter         | Code formatter for Lua - [https://github.com/Koihik/LuaFormatter](https://github.com/Koihik/LuaFormatter)                                                                                                                       |
| markdownfmt          | Like gofmt, but for Markdown - [https://github.com/shurcooL/markdownfmt](https://github.com/shurcooL/markdownfmt)                                                                                                               |
| markdownlint         | A Node.js style checker and lint tool for Markdown/CommonMark files - [https://github.com/davidanson/markdownlint](https://github.com/davidanson/markdownlint)                                                                  |
| markuplint           | An HTML linter for all markup developers - [https://markuplint.dev/](https://markuplint.dev/)                                                                                                                                   |
| mdformat             | CommonMark compliant Markdown formatter - [https://github.com/executablebooks/mdformat](https://github.com/executablebooks/mdformat)                                                                                            |
| misspell             | Correct commonly misspelled English words in source files - [https://github.com/client9/misspell/](https://github.com/client9/misspell/)                                                                                        |
| mix:format           | Code formatter for Elixir - [https://hexdocs.pm/mix/main/Mix.Tasks.Format.html](https://hexdocs.pm/mix/main/Mix.Tasks.Format.html)                                                                                              |
| mojo:format          | Formats Mojo source files - [https://docs.modular.com/mojo/cli/format](https://docs.modular.com/mojo/cli/format)                                                                                                                |
| nickel:format        | Better configuration for less - [https://nickel-lang.org/](https://nickel-lang.org/)                                                                                                                                            |
| nimpretty            | Code formatter for the Nim programming language - [https://github.com/nim-lang/nim](https://github.com/nim-lang/nim)                                                                                                            |
| nixfmt               | The official (but not yet stable) formatter for Nix code - [https://github.com/serokell/nixfmt](https://github.com/serokell/nixfmt)                                                                                             |
| nixpkgs-fmt          | Nix code formatter for nixpkgs - [https://github.com/nix-community/nixpkgs-fmt](https://github.com/nix-community/nixpkgs-fmt)                                                                                                   |
| nph                  | An opinionated code formatter for Nim - [https://github.com/arnetheduck/nph](https://github.com/arnetheduck/nph)                                                                                                                |
| npm-groovy-lint      | Lint, format and auto-fix your Groovy / Jenkinsfile / Gradle files - [https://github.com/nvuillam/npm-groovy-lint](https://github.com/nvuillam/npm-groovy-lint)                                                                 |
| ocamlformat          | Auto-formatter for OCaml code - [https://github.com/ocaml-ppx/ocamlformat](https://github.com/ocaml-ppx/ocamlformat)                                                                                                            |
| ocp-indent           | Indentation tool for OCaml - [https://github.com/OCamlPro/ocp-indent](https://github.com/OCamlPro/ocp-indent)                                                                                                                   |
| ormolu               | A formatter for Haskell source code - [https://github.com/tweag/ormolu](https://github.com/tweag/ormolu)                                                                                                                        |
| oxlint               | Oxlint is designed to catch erroneous or useless code without requiring any configurations by default - [https://oxc.rs/docs/guide/usage/linter.html](https://oxc.rs/docs/guide/usage/linter.html)                              |
| packer:fmt           | Packer is used to format HCL2 configuration files - [https://developer.hashicorp.com/packer/docs/commands/fmt](https://developer.hashicorp.com/packer/docs/commands/fmt)                                                        |
| perltidy             | Perl::Tidy, a source code formatter for Perl - [https://github.com/perltidy/perltidy](https://github.com/perltidy/perltidy)                                                                                                     |
| pg_format            | A PostgreSQL SQL syntax beautifier - [https://github.com/darold/pgFormatter](https://github.com/darold/pgFormatter)                                                                                                             |
| php-cs-fixer:fix     | A tool to automatically fix PHP Coding Standards issues - [https://github.com/PHP-CS-Fixer/PHP-CS-Fixer](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer)                                                                          |
| phpcbf               | PHP Code Beautifier and Fixer fixes violations of a defined coding standard - [https://phpqa.io/projects/phpcbf.html](https://phpqa.io/projects/phpcbf.html)                                                                    |
| phpinsights:fix      | Instant PHP quality checks from your console - [https://github.com/nunomaduro/phpinsights](https://github.com/nunomaduro/phpinsights)                                                                                           |
| pint                 | Laravel Pint is an opinionated PHP code style fixer for minimalists - [https://github.com/laravel/pint](https://github.com/laravel/pint)                                                                                        |
| prettier             | Prettier is an opinionated code formatter - [https://github.com/prettier/prettier](https://github.com/prettier/prettier)                                                                                                        |
| pretty-php           | The opinionated PHP code formatter - [https://github.com/lkrms/pretty-php](https://github.com/lkrms/pretty-php)                                                                                                                 |
| prettypst            | Formatter for Typst - [https://github.com/antonWetzel/prettypst](https://github.com/antonWetzel/prettypst)                                                                                                                      |
| puppet-lint          | Check that your Puppet manifests conform to the style guide - [https://github.com/puppetlabs/puppet-lint](https://github.com/puppetlabs/puppet-lint)                                                                            |
| purs-tidy            | PureScript code formatter - [https://github.com/natefaubion/purescript-tidy](https://github.com/natefaubion/purescript-tidy)                                                                                                    |
| pycln                | A formatter for finding and removing unused import statements - [https://github.com/hadialqattan/pycln](https://github.com/hadialqattan/pycln)                                                                                  |
| pyink                | Pyink is a Python formatter, forked from Black with a few different formatting behaviors - [https://github.com/google/pyink](https://github.com/google/pyink)                                                                   |
| qmlfmt               | qmlfmt - command line application that formats QML files - [https://github.com/jesperhh/qmlfmt](https://github.com/jesperhh/qmlfmt)                                                                                             |
| raco:fmt             | An extensible code formatter for Racket - [https://docs.racket-lang.org/fmt/](https://docs.racket-lang.org/fmt/)                                                                                                                |
| refmt                | refmt stands by Reason Formatter and it formats Reason programs, is a parser and pretty-printer for Reason - [https://reasonml.github.io/docs/en/refmt](https://reasonml.github.io/docs/en/refmt)                               |
| rescript:format      | Formatter for ReScript - [https://rescript-lang.org/](https://rescript-lang.org/)                                                                                                                                               |
| roc:format           | Tools for the roc programming language - [https://github.com/roc-lang/roc](https://github.com/roc-lang/roc)                                                                                                                     |
| rstfmt               | A formatter for reStructuredText - [https://github.com/dzhu/rstfmt](https://github.com/dzhu/rstfmt)                                                                                                                             |
| rubocop              | A Ruby static code analyzer and formatter, based on the community Ruby style guide - [https://github.com/rubocop/rubocop](https://github.com/rubocop/rubocop)                                                                   |
| rubyfmt              | Ruby Autoformatter - [https://github.com/fables-tales/rubyfmt](https://github.com/fables-tales/rubyfmt)                                                                                                                         |
| ruff:check           | An extremely fast Python linter and code formatter, written in Rust - [https://docs.astral.sh/ruff](https://docs.astral.sh/ruff)                                                                                                |
| ruff:format          | An extremely fast Python linter and code formatter, written in Rust - [https://docs.astral.sh/ruff](https://docs.astral.sh/ruff)                                                                                                |
| rufo                 | The Ruby Formatter - [https://github.com/ruby-formatter/rufo](https://github.com/ruby-formatter/rufo)                                                                                                                           |
| rune:fmt             | Tools for the Rune programming language - [https://github.com/rune-rs/rune](https://github.com/rune-rs/rune)                                                                                                                    |
| rustfmt              | The official code formatter for Rust - [https://github.com/rust-lang/rustfmt](https://github.com/rust-lang/rustfmt)                                                                                                             |
| rustywind            | CLI for organizing Tailwind CSS classes - [https://github.com/avencera/rustywind](https://github.com/avencera/rustywind)                                                                                                        |
| scalafmt             | Code formatter for Scala - [https://github.com/scalameta/scalafmt](https://github.com/scalameta/scalafmt)                                                                                                                       |
| scalariform          | Scala source code formatter - [https://github.com/scala-ide/scalariform](https://github.com/scala-ide/scalariform)                                                                                                              |
| shfmt                | Shell script formatter - [https://github.com/mvdan/sh](https://github.com/mvdan/sh)                                                                                                                                             |
| sleek                | Sleek is a CLI tool for formatting SQL. It helps you maintain a consistent style across your SQL code, enhancing readability and productivity - [https://github.com/nrempel/sleek](https://github.com/nrempel/sleek)            |
| smlfmt               | A custom parser/auto-formatter for Standard ML - [https://github.com/shwestrick/smlfmt](https://github.com/shwestrick/smlfmt)                                                                                                   |
| snakefmt             | The uncompromising Snakemake code formatter - [https://github.com/snakemake/snakefmt](https://github.com/snakemake/snakefmt)                                                                                                    |
| sql-formatter        | A whitespace formatter for different query languages - [https://github.com/sql-formatter-org/sql-formatter](https://github.com/sql-formatter-org/sql-formatter)                                                                 |
| sqlfluff:fix         | A modular SQL linter and auto-formatter with support for multiple dialects and templated code - [https://github.com/sqlfluff/sqlfluff](https://github.com/sqlfluff/sqlfluff)                                                    |
| sqlfluff:format      | A modular SQL linter and auto-formatter with support for multiple dialects and templated code - [https://github.com/sqlfluff/sqlfluff](https://github.com/sqlfluff/sqlfluff)                                                    |
| sqlfmt               | sqlfmt formats your dbt SQL files so you don't have to - [https://github.com/tconbeer/sqlfmt](https://github.com/tconbeer/sqlfmt)                                                                                               |
| standardjs           | JavaScript style guide, linter, and formatter - [https://github.com/standard/standard](https://github.com/standard/standard)                                                                                                    |
| standardrb           | Ruby's bikeshed-proof linter and formatter - [https://github.com/standardrb/standard](https://github.com/standardrb/standard)                                                                                                   |
| stylefmt             | stylefmt is a tool that automatically formats stylesheets - [https://github.com/matype/stylefmt](https://github.com/matype/stylefmt)                                                                                            |
| stylelint            | A mighty CSS linter that helps you avoid errors and enforce conventions - [https://github.com/stylelint/stylelint](https://github.com/stylelint/stylelint)                                                                      |
| stylish-haskell      | Haskell code prettifier - [https://github.com/haskell/stylish-haskell](https://github.com/haskell/stylish-haskell)                                                                                                              |
| stylua               | An opinionated Lua code formatter - [https://github.com/JohnnyMorganz/StyLua](https://github.com/JohnnyMorganz/StyLua)                                                                                                          |
| superhtml:fmt        | HTML Language Server & Templating Language Library - [https://github.com/kristoff-it/superhtml](https://github.com/kristoff-it/superhtml)                                                                                       |
| swift-format         | Formatting technology for Swift source code - [https://github.com/apple/swift-format](https://github.com/apple/swift-format)                                                                                                    |
| swiftformat          | A command-line tool and Xcode Extension for formatting Swift code - [https://github.com/nicklockwood/SwiftFormat](https://github.com/nicklockwood/SwiftFormat)                                                                  |
| taplo                | A TOML toolkit written in Rust - [https://github.com/tamasfe/taplo](https://github.com/tamasfe/taplo)                                                                                                                           |
| templ:fmt            | Tooling for the Templ template language - [https://templ.guide/](https://templ.guide/)                                                                                                                                          |
| terraform:fmt        | The terraform fmt command is used to rewrite Terraform configuration files to a canonical format and style - [https://www.terraform.io/docs/cli/commands/fmt.html](https://www.terraform.io/docs/cli/commands/fmt.html)         |
| tlint:format         | Tighten linter for Laravel conventions - [https://github.com/tighten/tlint](https://github.com/tighten/tlint)                                                                                                                   |
| tofu:fmt             | The tofu fmt command is used to rewrite OpenTofu configuration files to a canonical format and style - [https://opentofu.org/docs/cli/commands/fmt/](https://opentofu.org/docs/cli/commands/fmt/)                               |
| topiary              | Topiary aims to be a uniform formatter for simple languages, as part of the Tree-sitter ecosystem - [https://github.com/tweag/topiary](https://github.com/tweag/topiary)                                                        |
| ts-standard          | Typescript style guide, linter, and formatter using StandardJS - [https://github.com/standard/ts-standard](https://github.com/standard/ts-standard)                                                                             |
| twig-cs-fixer:lint   | A tool to automatically fix Twig Coding Standards issues - [https://github.com/VincentLanglet/Twig-CS-Fixer](https://github.com/VincentLanglet/Twig-CS-Fixer)                                                                   |
| typos                | Source code spell checker - [https://github.com/crate-ci/typos](https://github.com/crate-ci/typos)                                                                                                                              |
| typstfmt             | Basic formatter for the Typst language - [https://github.com/astrale-sharp/typstfmt](https://github.com/astrale-sharp/typstfmt)                                                                                                 |
| typstyle             | Beautiful and reliable typst code formatter - [https://github.com/Enter-tainer/typstyle](https://github.com/Enter-tainer/typstyle)                                                                                              |
| ufmt                 | Safe, atomic formatting with black and usort - [https://github.com/omnilib/ufmt](https://github.com/omnilib/ufmt)                                                                                                               |
| uiua:fmt             | A stack-based array programming language - [https://github.com/uiua-lang/uiua](https://github.com/uiua-lang/uiua)                                                                                                               |
| usort                | Safe, minimal import sorting for Python projects - [https://github.com/facebook/usort](https://github.com/facebook/usort)                                                                                                       |
| v:fmt                | Tooling for V lang - [https://vlang.io/](https://vlang.io/)                                                                                                                                                                     |
| veryl:fmt            | Veryl: A Modern Hardware Description Language - [https://github.com/veryl-lang/veryl](https://github.com/veryl-lang/veryl)                                                                                                      |
| vhdl-style-guide     | Style guide enforcement for VHDL - [https://github.com/jeremiah-c-leary/vhdl-style-guide](https://github.com/jeremiah-c-leary/vhdl-style-guide)                                                                                 |
| wfindent             | Indents and optionally converts Fortran program sources - [https://github.com/wvermin/findent](https://github.com/wvermin/findent)                                                                                              |
| xmlformat            | Format and compress XML documents - [https://github.com/pamoller/xmlformatter](https://github.com/pamoller/xmlformatter)                                                                                                        |
| xmllint              | XML linter - [https://gnome.pages.gitlab.gnome.org/libxml2/xmllint.html](https://gnome.pages.gitlab.gnome.org/libxml2/xmllint.html)                                                                                             |
| xo                   | JavaScript/TypeScript linter (ESLint wrapper) with great defaults - [http://github.com/xojs/xo](http://github.com/xojs/xo)                                                                                                      |
| yamlfix              | A simple opinionated yaml formatter that keeps your comments - [https://github.com/lyz-code/yamlfix](https://github.com/lyz-code/yamlfix)                                                                                       |
| yamlfmt              | An extensible command line tool or library to format yaml files - [https://github.com/google/yamlfmt](https://github.com/google/yamlfmt)                                                                                        |
| yapf                 | A formatter for Python files - [https://github.com/google/yapf](https://github.com/google/yapf)                                                                                                                                 |
| yew-fmt              | Code formatter for the Yew framework - [https://github.com/its-the-shrimp/yew-fmt](https://github.com/its-the-shrimp/yew-fmt)                                                                                                   |
| zig:fmt              | Reformat Zig source into canonical form - [https://ziglang.org/](https://ziglang.org/)                                                                                                                                          |
| ziggy:fmt            | Formats Ziggy documents and Ziggy schemas - [https://ziggy-lang.io/documentation/ziggy-fmt/](https://ziggy-lang.io/documentation/ziggy-fmt/)                                                                                    |
| zprint               | Executables, uberjar, and library to beautifully format Clojure and Clojurescript source code and s-expressions - [https://github.com/kkinnear/zprint](https://github.com/kkinnear/zprint)                                      |

<!-- END_SECTION:supported-tools -->

## Shell completions

Shell completions can be generated using `mdsf completions <SHELL>`.

<!-- START_SECTION:completions-command-help -->

```
Generate shell completion

Usage: mdsf completions <SHELL>

Arguments:
  <SHELL>  [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

<!-- END_SECTION:completions-command-help -->

#### Bash

Add the following to your `.bashrc`.

```bash
eval "$(mdsf completions bash)"
```

#### Bash

Add the following to your `.zshrc`.

```bash
eval "$(mdsf completions zsh)"
```

#### Fish

Add the following to `~/.config/fish/config.fish`.

```fish
mdsf completions fish | source
```

#### PowerShell

Add the following to your PowerShell configuration (Can be found by running `$PROFILE`).

```powershell
Invoke-Expression (&mdsf completions powershell)
```

#### Elvish

Add the following to `~/.elvish/rc.elv`.

```elvish
eval (mdsf completions elvish)
```

## Acknowledgement

mdsf was inspired by the amazing neovim formatting plugin [conform.nvim](https://github.com/stevearc/conform.nvim).

## Alternatives to mdsf

- [conform.nvim](https://github.com/stevearc/conform.nvim) using `injected` mode.
- [mdformat](https://github.com/executablebooks/mdformat).

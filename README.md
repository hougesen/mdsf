# mdsf

Format, and lint, markdown code snippets using your favorite code tools.

<a href="https://crates.io/crates/mdsf"><img src="https://img.shields.io/crates/v/mdsf.svg"></a>
<a href="https://github.com/hougesen/mdsf/actions/workflows/validate.yml"><img src="https://github.com/hougesen/mdsf/actions/workflows/validate.yml/badge.svg"></a>

<!-- <a href="https://codecov.io/gh/hougesen/mdsf"><img src="https://codecov.io/gh/hougesen/mdsf/branch/main/graph/badge.svg"/></a> -->

## Table of contents

<!-- START_SECTION:toc -->

- [mdsf](#mdsf)
  - [Table of contents](#table-of-contents)
  - [Installation](#installation)
    - [Linux & MacOS](#linux--macos)
    - [Windows](#windows)
    - [Cargo](#cargo)
    - [npm/npx](#npmnpx)
    - [Homebrew](#homebrew)
  - [Usage](#usage)
    - [Formatting code](#formatting-code)
    - [Verifying code](#verifying-code)
  - [Configuration](#configuration)
    - [Tools](#tools)
    - [Commands](#commands)
  - [Shell completions](#shell-completions)
    - [Bash](#bash)
    - [Bash](#bash-1)
    - [Fish](#fish)
    - [PowerShell](#powershell)
    - [Elvish](#elvish)
    - [Nushell](#nushell)
  - [Acknowledgement](#acknowledgement)
  - [Alternatives to mdsf](#alternatives-to-mdsf)

<!-- END_SECTION:toc -->

## Installation

The latest version of `mdsf` can be downloaded directly from [github.com/hougesen/mdsf/releases](https://github.com/hougesen/mdsf/releases).

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

<!-- START_SECTION:base-command-help -->

```
mdsf 0.3.3-dev
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

### Formatting code

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

### Verifying code

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
    "python": "ruff:format",
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

### Tools

> \[!NOTE\]
> mdsf is not a package manager.
>
> Only tools that are already installed will be used.

<!-- START_SECTION:supported-tools -->

`mdsf` currently supports 231 tools. Feel free to open an issue/pull-request if your favorite tool/command is missing! 😃

| Name                                                                                    | Description                                                                                                                                   | Categories            | Languages                                                                 |
| --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- | ------------------------------------------------------------------------- |
| [actionlint](https://github.com/rhysd/actionlint)                                       | Static checker for GitHub Actions workflow files                                                                                              | `linter`              | `yaml`                                                                    |
| [alejandra](https://github.com/kamadorueda/alejandra)                                   | The Uncompromising Nix Code Formatter                                                                                                         | `formatter`           | `nix`                                                                     |
| [ameba](https://github.com/crystal-ameba/ameba)                                         | A static code analysis tool for Crystal                                                                                                       | `linter`              | `crystal`                                                                 |
| [ansible-lint](https://github.com/ansible/ansible-lint)                                 | ansible-lint checks playbooks for practices and behavior that could potentially be improved and can fix some of the most common ones for you  | `linter`              | `ansible`                                                                 |
| [asmfmt](https://github.com/klauspost/asmfmt)                                           | Go Assembler Formatter                                                                                                                        | `formatter`           | `go`                                                                      |
| [astyle](https://gitlab.com/saalen/astyle)                                              | A Free, Fast, and Small Automatic Formatter for C, C++, C++/CLI, Objective-C, C#, and Java Source Code                                        | `formatter`           | `c#`, `c++`, `c`, `java`, `objective-c`                                   |
| [auto-optional](https://github.com/Luttik/auto-optional)                                | Adds the Optional type-hint to arguments where the default value is None                                                                      | `formatter`           | `python`                                                                  |
| [autocorrect](https://github.com/huacnlee/autocorrect)                                  | A linter and formatter to help you to improve copywriting, correct spaces, words, and punctuations between CJK (Chinese, Japanese, Korean)    | `autocorrection`      |                                                                           |
| [autoflake](https://github.com/pycqa/autoflake)                                         | Removes unused imports and unused variables as reported by pyflakes                                                                           | `linter`              | `python`                                                                  |
| [autopep8](https://github.com/hhatto/autopep8)                                          | A tool that automatically formats Python code to conform to the PEP 8 style guid                                                              | `formatter`           | `python`                                                                  |
| [beancount-black](https://github.com/LaunchPlatform/beancount-black)                    | Opinionated code formatter, just like Python's black code formatter but for Beancount                                                         | `formatter`           | `beancount`                                                               |
| [beautysh](https://pypi.org/project/beautysh/)                                          | A Bash beautifier for the masses                                                                                                              | `formatter`           | `bash`, `shell`                                                           |
| [bibtex-tidy](https://github.com/FlamingTempura/bibtex-tidy)                            | Cleaner and Formatter for BibTeX files                                                                                                        | `formatter`           | `bibtex`                                                                  |
| [bicep](https://github.com/Azure/bicep)                                                 | Bicep is a declarative language for describing and deploying Azure resources                                                                  | `formatter`           | `bicep`                                                                   |
| [biome](https://github.com/biomejs/biome)                                               | One toolchain for your web project                                                                                                            | `formatter`, `linter` | `javascript`, `json`, `typescript`, `vue`                                 |
| [black](https://github.com/psf/black)                                                   | The uncompromising Python code formatter                                                                                                      | `formatter`           | `python`                                                                  |
| [blade-formatter](https://github.com/shufo/blade-formatter)                             | An opinionated blade template formatter for Laravel that respects readability                                                                 | `formatter`           | `blade`, `laravel`, `php`                                                 |
| [blue](https://github.com/grantjenks/blue)                                              | The slightly less uncompromising Python code formatter                                                                                        | `formatter`           | `python`                                                                  |
| [bpfmt](https://source.android.com/docs/setup/reference/androidbp#formatter)            | A formatter for Blueprint files                                                                                                               | `formatter`           | `blueprint`                                                               |
| [brittany](https://github.com/lspitzner/brittany)                                       | A Haskell source code formatter                                                                                                               | `formatter`           | `haskell`                                                                 |
| [brunette](https://github.com/odwyersoftware/brunette)                                  | A best practice Python code formatter                                                                                                         | `formatter`           | `python`                                                                  |
| [bsfmt](https://github.com/rokucommunity/brighterscript-formatter)                      | A code formatter for BrightScript and BrighterScript                                                                                          | `formatter`           | `brighterscript`, `brightscript`                                          |
| [bslint](https://github.com/rokucommunity/bslint)                                       | A linter for BrightScript and BrighterScript                                                                                                  | `linter`              | `brightscript`, `brightscripter`                                          |
| [buf](https://buf.build/docs/reference/cli/buf/format/)                                 | The best way of working with Protocol Buffers                                                                                                 | `formatter`           | `protobuf`                                                                |
| [buildifier](https://github.com/bazelbuild/buildtools)                                  | A bazel BUILD file formatter and                                                                                                              | `formatter`           | `bazel`                                                                   |
| [cabal-fmt](https://github.com/phadej/cabal-fmt)                                        | An experiment of formatting .cabal files                                                                                                      | `formatter`           | `cabal`                                                                   |
| [cabal-prettify](https://github.com/kindaro/cabal-prettify)                             | Prettify your Cabal package configuration files                                                                                               | `formatter`           | `cabal`                                                                   |
| [cabal](https://www.haskell.org/cabal/)                                                 | Cabal is a system for building and packaging Haskell libraries and programs                                                                   | `formatter`           | `cabal`                                                                   |
| [caddy](https://caddyserver.com/docs/command-line#caddy-fmt)                            | Formats or prettifies a Caddyfile                                                                                                             | `formatter`           | `caddy`                                                                   |
| [caramel](https://caramel.run/)                                                         | Formatter for the Caramel programming language                                                                                                | `formatter`           | `caramel`                                                                 |
| [clang-format](https://clang.llvm.org/docs/ClangFormat.html)                            | A tool to format C/C++/Java/JavaScript/JSON/Objective-C/Protobuf/C# code                                                                      | `formatter`           | `c#`, `c++`, `c`, `java`, `javascript`, `json`, `objective-c`, `protobuf` |
| [clang-tidy](https://clang.llvm.org/extra/clang-tidy/)                                  | clang-tidy is a clang-based C++ “linter” tool                                                                                                 | `linter`              | `c++`                                                                     |
| [cljfmt](https://github.com/weavejester/cljfmt)                                         | A tool for formatting Clojure code                                                                                                            | `formatter`           | `clojure`                                                                 |
| [cljstyle](https://github.com/greglook/cljstyle)                                        | A tool for formatting Clojure code                                                                                                            | `formatter`           | `clojure`                                                                 |
| [cmake-format](https://cmake-format.readthedocs.io/en/latest/cmake-format.html)         | cmake-format can format your listfiles nicely so that they don’t look like crap                                                               | `formatter`           | `cmake`                                                                   |
| [codespell](https://github.com/codespell-project/codespell)                             | Check code for common misspellings                                                                                                            | `autocorrection`      |                                                                           |
| [crlfmt](https://github.com/cockroachdb/crlfmt)                                         | Formatter for CockroachDB's additions to the Go style guide                                                                                   | `formatter`           | `go`                                                                      |
| [crystal](https://crystal-lang.org/)                                                    | Tools for the Crystal programming language                                                                                                    | `formatter`           | `crystal`                                                                 |
| [csharpier](https://csharpier.com/)                                                     | An Opinionated Code Formatter for C#                                                                                                          | `formatter`           | `c#`                                                                      |
| [css-beautify](https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html)  | A css formatter                                                                                                                               | `formatter`           | `css`                                                                     |
| [csscomb](https://github.com/csscomb/csscomb.js)                                        | CSS coding style formatter                                                                                                                    | `formatter`           | `css`                                                                     |
| [curlylint](https://github.com/thibaudcolas/curlylint)                                  | Experimental HTML templates linting for Jinja, Nunjucks, Django templates, Twig, Liquid                                                       | `linter`              | `django`, `jinja`, `liquid`, `nunjucks`, `twig`                           |
| [d2](https://d2lang.com/)                                                               | Formatter for the d2 language                                                                                                                 | `formatter`           | `d2`                                                                      |
| [dart](https://dart.dev/tools)                                                          | Formatter and linter for Dart                                                                                                                 | `formatter`, `linter` | `dart`, `flutter`                                                         |
| [dcm](https://dcm.dev/)                                                                 | Code Quality Tool for Flutter Developers                                                                                                      | `formatter`, `linter` | `dart`, `flutter`                                                         |
| [deadnix](https://github.com/astro/deadnix)                                             | Scan Nix files for dead code                                                                                                                  | `linter`              | `nix`                                                                     |
| [deno](https://docs.deno.com/runtime/reference/cli/)                                    | Formatter and linter for JavaScript and TypeScript                                                                                            | `formatter`, `linter` | `javascript`, `json`, `typescript`                                        |
| [dfmt](https://github.com/dlang-community/dfmt)                                         | Dfmt is a formatter for D source code                                                                                                         | `formatter`           | `d`                                                                       |
| [dhall](https://dhall-lang.org/)                                                        | Format Dhall files                                                                                                                            | `formatter`           | `dhall`                                                                   |
| [djade](https://github.com/adamchainz/djade)                                            | A Django template formatter                                                                                                                   | `formatter`           | `django`, `python`                                                        |
| [djlint](https://www.djlint.com/)                                                       | Lint & Format HTML Templates                                                                                                                  | `formatter`, `linter` | `handlebars`, `html`, `jinja`, `mustache`, `nunjucks`, `twig`             |
| [docformatter](https://pypi.org/project/docformatter/)                                  | Formats docstrings to follow PEP 257                                                                                                          | `formatter`           | `python`                                                                  |
| [dockfmt](https://github.com/jessfraz/dockfmt)                                          | Dockerfile format and parser. Like `gofmt` but for Dockerfiles                                                                                | `formatter`           | `docker`                                                                  |
| [docstrfmt](https://pypi.org/project/docstrfmt/)                                        | A formatter for Sphinx flavored reStructuredText                                                                                              | `formatter`           | `Sphinx`, `python`, `reStructuredText`                                    |
| [doctoc](https://github.com/thlorenz/doctoc)                                            | Generates table of contents for markdown files                                                                                                | `formatter`           | `markdown`                                                                |
| [dotenv-linter](https://github.com/dotenv-linter/dotenv-linter)                         | Lightning-fast linter for .env files                                                                                                          | `linter`              | `env`                                                                     |
| [dprint](https://dprint.dev/)                                                           | A pluggable and configurable code formatting platform written in Rust                                                                         | `formatter`           |                                                                           |
| [easy-coding-standard](https://github.com/easy-coding-standard/easy-coding-standard)    | The Easiest way to add coding standard to your PHP project                                                                                    | `formatter`, `linter` | `php`                                                                     |
| [efmt](https://github.com/sile/efmt)                                                    | Erlang code formatter                                                                                                                         | `formatter`           | `erlang`                                                                  |
| [elm-format](https://github.com/avh4/elm-format)                                        | elm-format formats Elm source code according to a standard set of rules based on the official Elm Style Guide                                 | `formatter`           | `elm`                                                                     |
| [erb-formatter](https://github.com/nebulab/erb-formatter)                               | Format ERB files with speed and precision                                                                                                     | `formatter`           | `erb`, `ruby`                                                             |
| [erlfmt](https://github.com/WhatsApp/erlfmt)                                            | An automated code formatter for Erlang                                                                                                        | `formatter`           | `erlang`                                                                  |
| [eslint](https://github.com/eslint/eslint/)                                             | Find and fix problems in your JavaScript code                                                                                                 | `linter`              | `javascript`, `typescript`                                                |
| [fantomas](https://github.com/fsprojects/fantomas)                                      | FSharp source code formatter                                                                                                                  | `formatter`           | `fsharp`                                                                  |
| [fish_indent](https://fishshell.com/docs/current/cmds/fish_indent.html)                 | Fish indenter and prettifier                                                                                                                  | `formatter`           | `fish`                                                                    |
| [fixjson](https://github.com/rhysd/fixjson)                                             | JSON Fixer for Humans using (relaxed) JSON5                                                                                                   | `formatter`, `linter` | `json5`, `json`                                                           |
| [floskell](https://github.com/ennocramer/floskell)                                      | Floskell is a flexible Haskell source code pretty printer                                                                                     | `formatter`           | `haskell`                                                                 |
| [fnlfmt](https://git.sr.ht/~technomancy/fnlfmt)                                         | A formatter for Fennel code                                                                                                                   | `formatter`           | `fennel`                                                                  |
| [forge](https://github.com/foundry-rs/foundry)                                          | A Solidity formatter                                                                                                                          | `formatter`           | `solidity`                                                                |
| [fourmolu](https://hackage.haskell.org/package/fourmolu)                                | A formatter for Haskell source code                                                                                                           | `formatter`           | `haskell`                                                                 |
| [fprettify](https://github.com/fortran-lang/fprettify)                                  | Auto-formatter for modern Fortran source code                                                                                                 | `formatter`           | `fortran`                                                                 |
| [gci](https://github.com/daixiang0/gci)                                                 | GCI, a tool that control golang package import order and make it always deterministic                                                         | `formatter`           | `go`                                                                      |
| [gdformat](https://github.com/scony/godot-gdscript-toolkit)                             | GDScript linter                                                                                                                               | `formatter`           | `gdscript`                                                                |
| [gersemi](https://github.com/blankspruce/gersemi)                                       | A formatter to make your CMake code the real treasure                                                                                         | `formatter`           | `cmake`                                                                   |
| [gleam](https://gleam.run)                                                              | Format Gleam source code                                                                                                                      | `formatter`           | `gleam`                                                                   |
| [gluon](https://github.com/gluon-lang/gluon)                                            | Code formatting for the gluon programming language                                                                                            | `formatter`           | `gluon`                                                                   |
| [gofmt](https://pkg.go.dev/cmd/gofmt)                                                   | Gofmt formats Go programs                                                                                                                     | `formatter`           | `go`                                                                      |
| [gofumpt](https://github.com/mvdan/gofumpt)                                             | A stricter gofmt                                                                                                                              | `formatter`           | `go`                                                                      |
| [goimports-reviser](https://github.com/incu6us/goimports-reviser)                       | Right imports sorting & code formatting tool (goimports alternative)                                                                          | `formatter`           | `go`                                                                      |
| [goimports](https://pkg.go.dev/golang.org/x/tools/cmd/goimports)                        | goimports updates your Go import lines, adding missing ones and removing unreferenced ones                                                    | `formatter`           | `go`                                                                      |
| [golines](https://github.com/segmentio/golines)                                         | A golang formatter that fixes long lines                                                                                                      | `formatter`           | `go`                                                                      |
| [google-java-format](https://github.com/google/google-java-format)                      | Reformats Java source code to comply with Google Java Style                                                                                   | `formatter`           | `java`                                                                    |
| [grain](https://grain-lang.org/docs/tooling/grain_cli)                                  | Code formatter for the Grain programming language                                                                                             | `formatter`           | `grain`                                                                   |
| [hadolint](https://github.com/hadolint/hadolint)                                        | Dockerfile linter, validate inline bash, written in Haskell                                                                                   | `linter`              | `dockerfile`                                                              |
| [haml-lint](https://github.com/sds/haml-lint)                                           | Tool for writing clean and consistent HAML                                                                                                    | `linter`              | `haml`                                                                    |
| [hclfmt](https://github.com/hashicorp/hcl)                                              | Formatter for hcl files                                                                                                                       | `formatter`           | `hcl`                                                                     |
| [hfmt](https://github.com/danstiner/hfmt)                                               | Format Haskell programs. Inspired by the gofmt utility                                                                                        | `formatter`           | `haskell`                                                                 |
| [hindent](https://github.com/mihaimaruseac/hindent)                                     | Extensible Haskell pretty printer                                                                                                             | `formatter`           | `haskell`                                                                 |
| [hlint](https://github.com/ndmitchell/hlint)                                            | Haskell source code suggestions                                                                                                               | `linter`              | `haskell`                                                                 |
| [html-beautify](https://github.com/beautifier/js-beautify?tab=readme-ov-file#css--html) | A html formatter                                                                                                                              | `formatter`           | `html`                                                                    |
| [htmlbeautifier](https://github.com/threedaymonk/htmlbeautifier)                        | A normaliser/beautifier for HTML that also understands embedded Ruby. Ideal for tidying up Rails templates                                    | `formatter`           | `erb`, `html`, `ruby`                                                     |
| [htmlhint](https://github.com/HTMLHint/HTMLHint)                                        | The static code analysis tool you need for your HTML                                                                                          | `linter`              | `html`                                                                    |
| [imba](https://imba.io/)                                                                | A formatter for Imba                                                                                                                          | `formatter`           | `imba`                                                                    |
| [isort](https://github.com/timothycrosley/isort)                                        | A Python utility to sort imports                                                                                                              | `formatter`           | `python`                                                                  |
| [joker](https://github.com/candid82/joker)                                              | Small Clojure interpreter, linter and formatter                                                                                               | `formatter`, `linter` | `clojure`                                                                 |
| [js-beautify](https://github.com/beautifier/js-beautify)                                | A JavaScript formatter                                                                                                                        | `formatter`           | `javascript`                                                              |
| [json5format](https://github.com/google/json5format)                                    | JSON5 (a.k.a., "JSON for Humans") formatter that preserves contextual comments                                                                | `formatter`           | `json5`, `json`                                                           |
| [jsona](https://github.com/jsona/jsona)                                                 | JSONA linter and formatter                                                                                                                    | `formatter`, `linter` | `jsona`                                                                   |
| [jsonlint](https://github.com/zaach/jsonlint)                                           | A JSON parser and validator with a CLI                                                                                                        | `formatter`, `linter` | `json`                                                                    |
| [jsonnet-lint](https://jsonnet.org/learning/tools.html)                                 | Linter for jsonnet files                                                                                                                      | `linter`              | `jsonnet`                                                                 |
| [jsonnetfmt](https://jsonnet.org/learning/tools.html)                                   | Formatter for automatically fixing jsonnet stylistic problems                                                                                 | `formatter`           | `jsonnet`                                                                 |
| [juliaformatter.jl](https://github.com/domluna/JuliaFormatter.jl)                       | An opinionated code formatter for Julia. Plot twist - the opinion is your own                                                                 | `formatter`           | `julia`                                                                   |
| [just](https://github.com/casey/just)                                                   | A formatter for justfiles                                                                                                                     | `formatter`           | `just`                                                                    |
| [kcl](https://www.kcl-lang.io/docs/tools/cli/kcl/fmt)                                   | KCL Format tool supports reformatting KCL files to the standard code style                                                                    | `formatter`           | `kcl`                                                                     |
| [kdlfmt](https://github.com/hougesen/kdlfmt)                                            | A formatter for kdl documents                                                                                                                 | `formatter`           | `kdl`                                                                     |
| [kdoc-formatter](https://github.com/tnorbye/kdoc-formatter)                             | Reformats Kotlin KDoc comments, reflowing text and other cleanup                                                                              | `formatter`           | `kotlin`                                                                  |
| [ktfmt](https://github.com/facebook/ktfmt)                                              | program that reformats Kotlin source code to comply with the common community standard for Kotlin code conventions                            | `formatter`           | `kotlin`                                                                  |
| [ktlint](https://github.com/pinterest/ktlint)                                           | An anti-bikeshedding Kotlin linter with built-in formatter                                                                                    | `linter`              | `kotlin`                                                                  |
| [kulala-fmt](https://github.com/mistweaverco/kulala-fmt)                                | An opinionated 🦄 .http and .rest 🐼 files linter 💄 and formatter ⚡                                                                         | `formatter`           | `http`                                                                    |
| [leptosfmt](https://github.com/bram209/leptosfmt)                                       | A formatter for the leptos view! macro                                                                                                        | `formatter`           | `rust`                                                                    |
| [liquidsoap-prettier](https://github.com/savonet/liquidsoap-prettier)                   | Prettier plugin for liquidsoap script                                                                                                         | `formatter`           | `liquidsoap`                                                              |
| [luacheck](https://github.com/lunarmodules/luacheck)                                    | A tool for linting and static analysis of Lua code                                                                                            | `formatter`           | `lua`                                                                     |
| [luaformatter](https://github.com/Koihik/LuaFormatter)                                  | Code formatter for Lua                                                                                                                        | `formatter`           | `lua`                                                                     |
| [markdownfmt](https://github.com/shurcooL/markdownfmt)                                  | Like gofmt, but for Markdown                                                                                                                  | `formatter`           | `markdown`                                                                |
| [markdownlint-cli2](https://github.com/davidanson/markdownlint-cli2)                    | A fast, flexible, configuration-based command-line interface for linting Markdown/CommonMark files with the markdownlint library              | `linter`              | `markdown`                                                                |
| [markdownlint](https://github.com/davidanson/markdownlint)                              | A Node.js style checker and lint tool for Markdown/CommonMark files                                                                           | `linter`              | `markdown`                                                                |
| [markuplint](https://markuplint.dev/)                                                   | An HTML linter for all markup developers                                                                                                      | `linter`              | `html`                                                                    |
| [mdformat](https://github.com/executablebooks/mdformat)                                 | CommonMark compliant Markdown formatter                                                                                                       | `formatter`           | `markdwon`                                                                |
| [mdslw](https://github.com/razziel89/mdslw)                                             | Prepare your markdown for easy diff'ing!                                                                                                      | `formatter`           | `markdown`                                                                |
| [meson](https://mesonbuild.com/)                                                        | Meson is an open source build system meant to be both extremely fast, and, even more importantly, as user friendly as possible                | `formatter`           | `meson`                                                                   |
| [misspell](https://github.com/client9/misspell/)                                        | Correct commonly misspelled English words in source files                                                                                     | `autocorrection`      |                                                                           |
| [mix](https://hexdocs.pm/mix/main/Mix.Tasks.Format.html)                                | Code formatter for Elixir                                                                                                                     | `formatter`           | `elixir`                                                                  |
| [mojo](https://docs.modular.com/mojo/cli/format)                                        | Formats Mojo source files                                                                                                                     | `formatter`           | `mojo`                                                                    |
| [mypy](https://github.com/python/mypy)                                                  | Optional static typing for Python                                                                                                             | `linter`              | `python`                                                                  |
| [nginxbeautifier](https://github.com/vasilevich/nginxbeautifier)                        | Format and beautify nginx config files                                                                                                        | `formatter`           | `nginx`                                                                   |
| [nginxfmt](https://github.com/slomkowski/nginx-config-formatter)                        | nginx config file formatter/beautifier written in Python with no additional dependencies                                                      | `formatter`           | `nginx`                                                                   |
| [nickel](https://nickel-lang.org/)                                                      | Better configuration for less                                                                                                                 | `formatter`           | `nickel`                                                                  |
| [nimpretty](https://github.com/nim-lang/nim)                                            | Code formatter for the Nim programming language                                                                                               | `formatter`           | `nim`                                                                     |
| [nixfmt](https://github.com/serokell/nixfmt)                                            | The official (but not yet stable) formatter for Nix code                                                                                      | `formatter`           | `nix`                                                                     |
| [nixpkgs-fmt](https://github.com/nix-community/nixpkgs-fmt)                             | Nix code formatter for nixpkgs                                                                                                                | `formatter`           | `nix`                                                                     |
| [nomad](https://developer.hashicorp.com/nomad/docs/commands)                            | CLI for HashiCorp Nomad                                                                                                                       | `formatter`           | `hcl`                                                                     |
| [nph](https://github.com/arnetheduck/nph)                                               | An opinionated code formatter for Nim                                                                                                         | `formatter`           | `nim`                                                                     |
| [npm-groovy-lint](https://github.com/nvuillam/npm-groovy-lint)                          | Lint, format and auto-fix your Groovy / Jenkinsfile / Gradle files                                                                            | `formatter`, `linter` | `groovy`                                                                  |
| [nufmt](https://github.com/nushell/nufmt)                                               | the nushell formatter                                                                                                                         | `formatter`           | `nushell`                                                                 |
| [ocamlformat](https://github.com/ocaml-ppx/ocamlformat)                                 | Auto-formatter for OCaml code                                                                                                                 | `formatter`           | `ocaml`                                                                   |
| [ocp-indent](https://github.com/OCamlPro/ocp-indent)                                    | Indentation tool for OCaml                                                                                                                    | `formatter`           | `ocaml`                                                                   |
| [odinfmt](https://github.com/DanielGavin/ols)                                           | Formatter for the Odin programming language                                                                                                   | `formatter`           | `odin`                                                                    |
| [oelint-adv](https://github.com/priv-kweihmann/oelint-adv)                              | Advanced oelint                                                                                                                               | `linter`              | `bitbake`                                                                 |
| [opa](https://www.openpolicyagent.org/docs/latest/cli/)                                 | Format Rego source files                                                                                                                      | `formatter`           | `rego`                                                                    |
| [ormolu](https://github.com/tweag/ormolu)                                               | A formatter for Haskell source code                                                                                                           | `formatter`           | `haskell`                                                                 |
| [oxlint](https://oxc.rs/docs/guide/usage/linter.html)                                   | Oxlint is designed to catch erroneous or useless code without requiring any configurations by default                                         | `linter`              | `javascript`, `typescript`                                                |
| [packer](https://developer.hashicorp.com/packer/docs/commands)                          | Packer is used to format HCL2 configuration files                                                                                             | `formatter`           | `hcl`                                                                     |
| [perltidy](https://github.com/perltidy/perltidy)                                        | Perl::Tidy, a source code formatter for Perl                                                                                                  | `formatter`           | `perl`                                                                    |
| [pg_format](https://github.com/darold/pgFormatter)                                      | A PostgreSQL SQL syntax beautifier                                                                                                            | `formatter`           | `sql`                                                                     |
| [php-cs-fixer](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer)                            | A tool to automatically fix PHP Coding Standards issues                                                                                       | `formatter`, `linter` | `php`                                                                     |
| [phpcbf](https://phpqa.io/projects/phpcbf.html)                                         | PHP Code Beautifier and Fixer fixes violations of a defined coding standard                                                                   | `formatter`           | `php`                                                                     |
| [phpinsights](https://github.com/nunomaduro/phpinsights)                                | Instant PHP quality checks from your console                                                                                                  | `linter`              | `php`                                                                     |
| [pint](https://github.com/laravel/pint)                                                 | Laravel Pint is an opinionated PHP code style fixer for minimalists                                                                           | `formatter`, `linter` | `php`                                                                     |
| [prettier](https://github.com/prettier/prettier)                                        | Prettier is an opinionated code formatter                                                                                                     | `formatter`           | `css`, `html`, `javascript`, `json`, `typescript`                         |
| [pretty-php](https://github.com/lkrms/pretty-php)                                       | The opinionated PHP code formatter                                                                                                            | `formatter`           | `php`                                                                     |
| [prettypst](https://github.com/antonWetzel/prettypst)                                   | Formatter for Typst                                                                                                                           | `formatter`           | `typst`                                                                   |
| [prisma](https://www.prisma.io)                                                         | Commands for interacting with the prisma ORM                                                                                                  | `formatter`           | `prisma`                                                                  |
| [protolint](https://github.com/yoheimuta/protolint)                                     | A pluggable linter and fixer to enforce Protocol Buffer style and conventions                                                                 | `linter`              | `protobuf`                                                                |
| [ptop](https://www.freepascal.org/tools/ptop.html)                                      | Free Pascal source formatter                                                                                                                  | `formatter`           | `pascal`                                                                  |
| [puppet-lint](https://github.com/puppetlabs/puppet-lint)                                | Check that your Puppet manifests conform to the style guide                                                                                   | `linter`              | `puppet`                                                                  |
| [purs-tidy](https://github.com/natefaubion/purescript-tidy)                             | PureScript code formatter                                                                                                                     | `formatter`           | `purescript`                                                              |
| [purty](https://gitlab.com/joneshf/purty)                                               | PureScript pretty-printer                                                                                                                     | `formatter`           | `purescript`                                                              |
| [pycln](https://github.com/hadialqattan/pycln)                                          | A formatter for finding and removing unused import statements                                                                                 | `formatter`           | `python`                                                                  |
| [pyink](https://github.com/google/pyink)                                                | Pyink is a Python formatter, forked from Black with a few different formatting behaviors                                                      | `formatter`           | `python`                                                                  |
| [pyment](https://github.com/dadadel/pyment)                                             | Format and convert Python docstrings and generates patches                                                                                    | `formatter`           | `python`                                                                  |
| [qmlfmt](https://github.com/jesperhh/qmlfmt)                                            | qmlfmt - command line application that formats QML files                                                                                      | `formatter`           | `qml`                                                                     |
| [quick-lint-js](https://github.com/quick-lint/quick-lint-js)                            | quick-lint-js finds bugs in JavaScript programs                                                                                               | `linter`              | `javascript`                                                              |
| [raco](https://docs.racket-lang.org/fmt/)                                               | An extensible code formatter for Racket                                                                                                       | `formatter`           | `racket`                                                                  |
| [refmt](https://reasonml.github.io/docs/en/refmt)                                       | refmt stands by Reason Formatter and it formats Reason programs, is a parser and pretty-printer for Reason                                    | `formatter`           | `reason`                                                                  |
| [reformat-gherkin](https://github.com/ducminh-phan/reformat-gherkin)                    | Reformat-gherkin automatically formats Gherkin files                                                                                          | `formatter`           | `gherkin`                                                                 |
| [reorder-python-imports](https://github.com/asottile/reorder-python-imports)            | Rewrites source to reorder python imports                                                                                                     | `formatter`           | `python`                                                                  |
| [rescript](https://rescript-lang.org/)                                                  | Formatter for ReScript                                                                                                                        | `formatter`           | `rescript`                                                                |
| [roc](https://github.com/roc-lang/roc)                                                  | Tools for the roc programming language                                                                                                        | `formatter`           | `roc`                                                                     |
| [rstfmt](https://github.com/dzhu/rstfmt)                                                | A formatter for reStructuredText                                                                                                              | `formatter`           | `restructuredtext`                                                        |
| [rubocop](https://github.com/rubocop/rubocop)                                           | A Ruby static code analyzer and formatter, based on the community Ruby style guide                                                            | `formatter`, `linter` | `ruby`                                                                    |
| [rubyfmt](https://github.com/fables-tales/rubyfmt)                                      | Ruby Autoformatter                                                                                                                            | `formatter`           | `ruby`                                                                    |
| [ruff](https://docs.astral.sh/ruff)                                                     | An extremely fast Python linter and code formatter, written in Rust                                                                           | `formatter`, `linter` | `python`                                                                  |
| [rufo](https://github.com/ruby-formatter/rufo)                                          | The Ruby Formatter                                                                                                                            | `formatter`           | `ruby`                                                                    |
| [rune](https://github.com/rune-rs/rune)                                                 | Tools for the Rune programming language                                                                                                       | `formatter`           | `rune`                                                                    |
| [rustfmt](https://github.com/rust-lang/rustfmt)                                         | The official code formatter for Rust                                                                                                          | `formatter`           | `rust`                                                                    |
| [rustywind](https://github.com/avencera/rustywind)                                      | CLI for organizing Tailwind CSS classes                                                                                                       | `formatter`           | `html`                                                                    |
| [scalafmt](https://github.com/scalameta/scalafmt)                                       | Code formatter for Scala                                                                                                                      | `formatter`           | `scala`                                                                   |
| [scalariform](https://github.com/scala-ide/scalariform)                                 | Scala source code formatter                                                                                                                   | `formatter`           | `scala`                                                                   |
| [shellharden](https://github.com/anordal/shellharden)                                   | The corrective bash syntax highlighter                                                                                                        | `linter`              | `bash`, `shell`                                                           |
| [shfmt](https://github.com/mvdan/sh)                                                    | Shell script formatter                                                                                                                        | `formatter`           | `shell`                                                                   |
| [sleek](https://github.com/nrempel/sleek)                                               | Sleek is a CLI tool for formatting SQL. It helps you maintain a consistent style across your SQL code, enhancing readability and productivity | `formatter`           | `sql`                                                                     |
| [smlfmt](https://github.com/shwestrick/smlfmt)                                          | A custom parser/auto-formatter for Standard ML                                                                                                | `formatter`           | `standard-ml`                                                             |
| [snakefmt](https://github.com/snakemake/snakefmt)                                       | The uncompromising Snakemake code formatter                                                                                                   | `formatter`           | `snakemake`                                                               |
| [sql-formatter](https://github.com/sql-formatter-org/sql-formatter)                     | A whitespace formatter for different query languages                                                                                          | `formatter`           | `sql`                                                                     |
| [sqlfluff](https://github.com/sqlfluff/sqlfluff)                                        | A modular SQL linter and auto-formatter with support for multiple dialects and templated code                                                 | `formatter`, `linter` | `sql`                                                                     |
| [sqlfmt](https://github.com/tconbeer/sqlfmt)                                            | sqlfmt formats your dbt SQL files so you don't have to                                                                                        | `formatter`           | `sql`                                                                     |
| [sqruff](https://github.com/quarylabs/sqruff)                                           | Fast SQL formatter/linter                                                                                                                     | `formatter`, `linter` | `sql`                                                                     |
| [standardjs](https://github.com/standard/standard)                                      | JavaScript style guide, linter, and formatter                                                                                                 | `formatter`, `linter` | `javascript`                                                              |
| [standardrb](https://github.com/standardrb/standard)                                    | Ruby's bikeshed-proof linter and formatter                                                                                                    | `formatter`, `linter` | `ruby`                                                                    |
| [statix](https://github.com/oppiliappan/statix)                                         | lints and suggestions for the nix programming language                                                                                        | `linter`              | `nix`                                                                     |
| [stylefmt](https://github.com/matype/stylefmt)                                          | stylefmt is a tool that automatically formats stylesheets                                                                                     | `formatter`           | `css`, `scss`                                                             |
| [stylelint](https://github.com/stylelint/stylelint)                                     | A mighty CSS linter that helps you avoid errors and enforce conventions                                                                       | `linter`              | `css`, `scss`                                                             |
| [stylish-haskell](https://github.com/haskell/stylish-haskell)                           | Haskell code prettifier                                                                                                                       | `formatter`           | `haskell`                                                                 |
| [stylua](https://github.com/JohnnyMorganz/StyLua)                                       | An opinionated Lua code formatter                                                                                                             | `formatter`           | `lua`                                                                     |
| [superhtml](https://github.com/kristoff-it/superhtml)                                   | HTML Language Server & Templating Language Library                                                                                            | `formatter`           | `html`                                                                    |
| [swift-format](https://github.com/apple/swift-format)                                   | Formatting technology for Swift source code                                                                                                   | `formatter`           | `swift`                                                                   |
| [swiftformat](https://github.com/nicklockwood/SwiftFormat)                              | A command-line tool and Xcode Extension for formatting Swift code                                                                             | `formatter`           | `swift`                                                                   |
| [taplo](https://github.com/tamasfe/taplo)                                               | A TOML toolkit written in Rust                                                                                                                | `formatter`           | `toml`                                                                    |
| [templ](https://templ.guide/)                                                           | Tooling for the Templ template language                                                                                                       | `formatter`           | `go`, `templ`                                                             |
| [terraform](https://www.terraform.io/docs/cli/commands/fmt.html)                        | The terraform fmt command is used to rewrite Terraform configuration files to a canonical format and style                                    | `formatter`           | `terraform`                                                               |
| [terragrunt](https://terragrunt.gruntwork.io/docs/reference/cli-options/#hclfmt)        | Recursively find hcl files and rewrite them into a canonical format                                                                           | `formatter`           | `hcl`                                                                     |
| [tex-fmt](https://github.com/WGUNDERWOOD/tex-fmt)                                       | An extremely fast LaTeX formatter written in Rust                                                                                             | `formatter`           | `latex`                                                                   |
| [tlint](https://github.com/tighten/tlint)                                               | Tighten linter for Laravel conventions                                                                                                        | `linter`              | `php`                                                                     |
| [tofu](https://opentofu.org/docs/cli/commands/fmt/)                                     | The tofu fmt command is used to rewrite OpenTofu configuration files to a canonical format and style                                          | `formatter`           | `terraform`, `tofu`                                                       |
| [toml-sort](https://github.com/pappasam/toml-sort)                                      | A command line utility to sort and format toml files                                                                                          | `formatter`           | `toml`                                                                    |
| [topiary](https://github.com/tweag/topiary)                                             | Topiary aims to be a uniform formatter for simple languages, as part of the Tree-sitter ecosystem                                             | `formatter`           |                                                                           |
| [ts-standard](https://github.com/standard/ts-standard)                                  | Typescript style guide, linter, and formatter using StandardJS                                                                                | `formatter`, `linter` | `typescript`                                                              |
| [tsqllint](https://github.com/tsqllint/tsqllint)                                        | Configurable linting for TSQL                                                                                                                 | `linter`              | `sql`                                                                     |
| [twig-cs-fixer](https://github.com/VincentLanglet/Twig-CS-Fixer)                        | A tool to automatically fix Twig Coding Standards issues                                                                                      | `formatter`, `linter` | `twig`                                                                    |
| [typos](https://github.com/crate-ci/typos)                                              | Source code spell checker                                                                                                                     | `autocorrection`      |                                                                           |
| [typstfmt](https://github.com/astrale-sharp/typstfmt)                                   | Basic formatter for the Typst language                                                                                                        | `formatter`           | `typst`                                                                   |
| [typstyle](https://github.com/Enter-tainer/typstyle)                                    | Beautiful and reliable typst code formatter                                                                                                   | `formatter`           | `typst`                                                                   |
| [ufmt](https://github.com/omnilib/ufmt)                                                 | Safe, atomic formatting with black and usort                                                                                                  | `formatter`           | `python`                                                                  |
| [uiua](https://github.com/uiua-lang/uiua)                                               | A stack-based array programming language                                                                                                      | `formatter`           | `uiua`                                                                    |
| [unimport](https://github.com/hakancelikdev/unimport)                                   | The ultimate linter and formatter for removing unused import statements in your code                                                          | `formatter`           | `python`                                                                  |
| [usort](https://github.com/facebook/usort)                                              | Safe, minimal import sorting for Python projects                                                                                              | `formatter`           | `python`                                                                  |
| [v](https://vlang.io/)                                                                  | Tooling for V lang                                                                                                                            | `formatter`           | `v`                                                                       |
| [vacuum](https://github.com/daveshanley/vacuum)                                         | vacuum is the worlds fastest OpenAPI 3, OpenAPI 2 / Swagger linter and quality analysis tool                                                  | `linter`              | `openapi`                                                                 |
| [veryl](https://github.com/veryl-lang/veryl)                                            | Veryl: A Modern Hardware Description Language                                                                                                 | `formatter`           | `veryl`                                                                   |
| [vhdl-style-guide](https://github.com/jeremiah-c-leary/vhdl-style-guide)                | Style guide enforcement for VHDL                                                                                                              | `formatter`           | `vhdl`                                                                    |
| [wfindent](https://github.com/wvermin/findent)                                          | Indents and optionally converts Fortran program sources                                                                                       | `formatter`           | `fortran`                                                                 |
| [xmlformat](https://github.com/pamoller/xmlformatter)                                   | Format and compress XML documents                                                                                                             | `formatter`           | `xml`                                                                     |
| [xmllint](https://gnome.pages.gitlab.gnome.org/libxml2/xmllint.html)                    | XML linter                                                                                                                                    | `linter`              | `xml`                                                                     |
| [xo](https://github.com/xojs/xo)                                                        | JavaScript/TypeScript linter (ESLint wrapper) with great defaults                                                                             | `linter`              | `javascript`, `typescript`                                                |
| [yamlfix](https://github.com/lyz-code/yamlfix)                                          | A simple opinionated yaml formatter that keeps your comments                                                                                  | `formatter`           | `yaml`                                                                    |
| [yamlfmt](https://github.com/google/yamlfmt)                                            | An extensible command line tool or library to format yaml files                                                                               | `formatter`           | `yaml`                                                                    |
| [yapf](https://github.com/google/yapf)                                                  | A formatter for Python files                                                                                                                  | `formatter`           | `python`                                                                  |
| [yew-fmt](https://github.com/its-the-shrimp/yew-fmt)                                    | Code formatter for the Yew framework                                                                                                          | `formatter`           | `rust`                                                                    |
| [zig](https://ziglang.org/)                                                             | Reformat Zig source into canonical form                                                                                                       | `formatter`           | `zig`                                                                     |
| [ziggy](https://ziggy-lang.io/documentation/ziggy-fmt/)                                 | Formats Ziggy documents and Ziggy schemas                                                                                                     | `formatter`           | `ziggy`                                                                   |
| [zprint](https://github.com/kkinnear/zprint)                                            | Executables, uberjar, and library to beautifully format Clojure and Clojurescript source code and s-expressions                               | `formatter`           | `clojure`, `clojurescript`                                                |

<!-- END_SECTION:supported-tools -->

### Commands

<!-- START_SECTION:supported-commands -->

`mdsf` currently supports 216 commands. Feel free to open an issue/pull-request if your favorite tool is missing! 😃

| Name                     | Command                                                                                 |
| ------------------------ | --------------------------------------------------------------------------------------- |
| `alejandra`              | `alejandra --quiet $PATH`                                                               |
| `ameba`                  | `ameba --fix $PATH`                                                                     |
| `asmfmt`                 | `asmfmt -w $PATH`                                                                       |
| `astyle`                 | `astyle --quiet $PATH`                                                                  |
| `auto-optional`          | `auto-optional $PATH`                                                                   |
| `autocorrect`            | `autocorrect --fix $PATH`                                                               |
| `autoflake`              | `autoflake --quiet --in-place $PATH`                                                    |
| `autopep8`               | `autopep8 --in-place $PATH`                                                             |
| `beancount-black`        | `bean-black $PATH`                                                                      |
| `beautysh`               | `beautysh $PATH`                                                                        |
| `bibtex-tidy`            | `bibtex-tidy -m $PATH`                                                                  |
| `bicep:format`           | `bicep format $PATH`                                                                    |
| `biome:check`            | `biome check --write $PATH`                                                             |
| `biome:format`           | `biome format --write $PATH`                                                            |
| `biome:lint`             | `biome lint --write $PATH`                                                              |
| `black`                  | `black --quiet $PATH`                                                                   |
| `blade-formatter`        | `blade-formatter --write $PATH`                                                         |
| `blue`                   | `blue --quiet $PATH`                                                                    |
| `bpfmt`                  | `bpfmt -w $PATH`                                                                        |
| `brittany`               | `brittany --write-mode=inplace $PATH`                                                   |
| `brunette`               | `brunette --quiet $PATH`                                                                |
| `bsfmt`                  | `bsfmt $PATH --write`                                                                   |
| `bslint`                 | `bslint --fix $PATH`                                                                    |
| `buf:format`             | `buf format --write $PATH`                                                              |
| `buildifier`             | `buildifier $PATH`                                                                      |
| `cabal-fmt`              | `cabal-fmt --inplace $PATH`                                                             |
| `cabal-prettify`         | `cabal-prettify $PATH`                                                                  |
| `cabal:format`           | `cabal format $PATH`                                                                    |
| `caddy:fmt`              | `caddy fmt $PATH -w`                                                                    |
| `caramel:fmt`            | `caramel fmt $PATH`                                                                     |
| `clang-format`           | `clang-format -i $PATH`                                                                 |
| `clang-tidy`             | `clang-tidy --fix $PATH`                                                                |
| `cljfmt:fix`             | `cljfmt fix $PATH`                                                                      |
| `cljstyle`               | `cljstyle fix $PATH`                                                                    |
| `codespell`              | `codespell $PATH --check-hidden --write-changes`                                        |
| `crlfmt`                 | `crlfmt -w $PATH`                                                                       |
| `crystal:format`         | `crystal tool format $PATH`                                                             |
| `csharpier`              | `dotnet csharpier $PATH`                                                                |
| `css-beautify`           | `css-beautify -r --type css -f $PATH`                                                   |
| `csscomb`                | `csscomb -t $PATH`                                                                      |
| `d2:fmt`                 | `d2 fmt $PATH`                                                                          |
| `dart:fix`               | `dart fix --apply $PATH`                                                                |
| `dart:format`            | `dart format $PATH`                                                                     |
| `dcm:fix`                | `dcm fix $PATH`                                                                         |
| `dcm:format`             | `dcm format $PATH`                                                                      |
| `deno:fmt`               | `deno fmt --quiet $PATH`                                                                |
| `deno:lint`              | `deno lint --fix $PATH`                                                                 |
| `dfmt`                   | `dfmt -i $PATH`                                                                         |
| `dhall`                  | `dhall format $PATH`                                                                    |
| `djade`                  | `djade $PATH`                                                                           |
| `djlint`                 | `djlint $PATH --reformat`                                                               |
| `docformatter`           | `docformatter --in-place $PATH`                                                         |
| `dockfmt`                | `dockfmt fmt -w $PATH`                                                                  |
| `docstrfmt`              | `docstrfmt $PATH`                                                                       |
| `doctoc`                 | `doctoc $PATH`                                                                          |
| `dotenv-linter:fix`      | `dotenv-linter fix $PATH`                                                               |
| `dprint:fmt`             | `dprint fmt $PATH`                                                                      |
| `easy-coding-standard`   | `ecs check $PATH --fix --no-interaction`                                                |
| `efmt`                   | `efmt -w $PATH`                                                                         |
| `elm-format`             | `elm-format --elm-version=0.19 --yes $PATH`                                             |
| `erb-formatter`          | `erb-format $PATH --write`                                                              |
| `erlfmt`                 | `erlfmt -w $PATH_STRING`                                                                |
| `eslint`                 | `eslint --fix $PATH`                                                                    |
| `fantomas`               | `fantomas $PATH`                                                                        |
| `fish_indent`            | `fish_indent -w $PATH`                                                                  |
| `fixjson`                | `fixjson -w $PATH`                                                                      |
| `floskell`               | `floskell $PATH`                                                                        |
| `fnlfmt`                 | `fnlfmt $PATH`                                                                          |
| `forge:fmt`              | `forge fmt $PATH`                                                                       |
| `fourmolu`               | `fourmolu -i $PATH`                                                                     |
| `fprettify`              | `fprettify $PATH`                                                                       |
| `gci`                    | `gci write --skip-generated --skip-vender $PATH`                                        |
| `gdformat`               | `gdformat $PATH`                                                                        |
| `gersemi`                | `gersemi -i -q $PATH`                                                                   |
| `gleam:format`           | `gleam format $PATH`                                                                    |
| `gluon:fmt`              | `gluon fmt $PATH`                                                                       |
| `gofmt`                  | `gofmt -w $PATH`                                                                        |
| `gofumpt`                | `gofumpt -w $PATH`                                                                      |
| `goimports-reviser`      | `goimports-reviser -format $PATH`                                                       |
| `goimports`              | `goimports -w $PATH`                                                                    |
| `golines`                | `golines -w $PATH`                                                                      |
| `google-java-format`     | `google-java-format -i $PATH`                                                           |
| `grain:format`           | `grain format $PATH -o $PATH`                                                           |
| `haml-lint`              | `haml-lint --auto-correct $PATH`                                                        |
| `hfmt`                   | `hfmt -w $PATH`                                                                         |
| `hindent`                | `hindent $PATH`                                                                         |
| `hlint`                  | `hlint --refactor -i $PATH`                                                             |
| `html-beautify`          | `html-beautify -r --type html -f $PATH`                                                 |
| `htmlbeautifier`         | `htmlbeautifier $PATH`                                                                  |
| `imba:fmt`               | `imba fmt -f $PATH`                                                                     |
| `isort`                  | `isort --quiet $PATH`                                                                   |
| `joker`                  | `joker --format --write $PATH`                                                          |
| `js-beautify`            | `js-beautify -r --type js -f $PATH`                                                     |
| `json5format`            | `json5format -r $PATH`                                                                  |
| `jsona:format`           | `jsona format $PATH`                                                                    |
| `jsona:lint`             | `jsona lint $PATH`                                                                      |
| `jsonlint`               | `jsonlint -i $PATH`                                                                     |
| `jsonnetfmt`             | `jsonnetfmt -i $PATH`                                                                   |
| `juliaformatter.jl`      | `julia -E using JuliaFormatter;format_file(\"{$PATH_STRING}\")`                         |
| `just`                   | `just --fmt --unstable --justfile $PATH`                                                |
| `kcl:fmt`                | `kcl fmt $PATH`                                                                         |
| `kdlfmt`                 | `kdlfmt format $PATH`                                                                   |
| `kdoc-formatter`         | `kdoc-formatter --quiet $PATH`                                                          |
| `ktfmt`                  | `ktfmt --format --log-level=error $PATH`                                                |
| `ktlint`                 | `ktlint --format --log-level=error $PATH`                                               |
| `kulala-fmt`             | `kulala-fmt $PATH`                                                                      |
| `leptosfmt`              | `leptosfmt --quiet $PATH`                                                               |
| `liquidsoap-prettier`    | `liquidsoap-prettier --write $PATH`                                                     |
| `luaformatter`           | `lua-format -i $PATH`                                                                   |
| `markdownfmt`            | `markdownfmt -w $PATH`                                                                  |
| `markdownlint-cli2`      | `markdownlint-cli2 --fix $PATH`                                                         |
| `markdownlint`           | `markdownlint --fix $PATH`                                                              |
| `markuplint`             | `markuplint --fix $PATH`                                                                |
| `mdformat`               | `mdformat $PATH`                                                                        |
| `mdslw`                  | `mdslw $PATH`                                                                           |
| `misspell`               | `misspell -w $PATH`                                                                     |
| `mix:format`             | `mix format $PATH`                                                                      |
| `mojo:format`            | `mojo format -q $PATH`                                                                  |
| `nginxbeautifier`        | `nginxbeautifier $PATH`                                                                 |
| `nickel:format`          | `nickel format $PATH`                                                                   |
| `nimpretty`              | `nimpretty $PATH`                                                                       |
| `nixfmt`                 | `nixfmt $PATH`                                                                          |
| `nixpkgs-fmt`            | `nixpkgs-fmt $PATH`                                                                     |
| `nph`                    | `nph $PATH`                                                                             |
| `npm-groovy-lint`        | `npm-groovy-lint --format $PATH`                                                        |
| `ocamlformat`            | `ocamlformat --ignore-invalid-option --inplace --enable-outside-detected-project $PATH` |
| `ocp-indent`             | `ocp-indent --inplace $PATH`                                                            |
| `opa:fmt`                | `opa fmt $PATH -w`                                                                      |
| `ormolu`                 | `ormolu --mode inplace $PATH`                                                           |
| `oxlint`                 | `oxlint --fix $PATH`                                                                    |
| `packer:fmt`             | `packer fmt $PATH`                                                                      |
| `perltidy`               | `perltidy -b $PATH`                                                                     |
| `pg_format`              | `pg_format --inplace $PATH`                                                             |
| `php-cs-fixer:fix`       | `php-cs-fixer fix $PATH`                                                                |
| `phpcbf`                 | `phpcbf $PATH`                                                                          |
| `phpinsights:fix`        | `phpinsights fix $PATH --no-interaction --quiet`                                        |
| `pint`                   | `pint $PATH`                                                                            |
| `prettier`               | `prettier --embedded-language-formatting off --log-level error --write $PATH`           |
| `pretty-php`             | `pretty-php $PATH`                                                                      |
| `prettypst`              | `prettypst $PATH`                                                                       |
| `protolint`              | `protolint lint -fix $PATH`                                                             |
| `ptop`                   | `ptop $PATH $PATH`                                                                      |
| `puppet-lint`            | `puppet-lint --fix $PATH`                                                               |
| `purs-tidy`              | `purs-tidy format-in-place $PATH`                                                       |
| `purty`                  | `purty --write $PATH`                                                                   |
| `pycln`                  | `pycln --no-gitignore --quiet $PATH`                                                    |
| `pyink`                  | `pyink --quiet $PATH`                                                                   |
| `pyment`                 | `pyment -w $PATH`                                                                       |
| `qmlfmt`                 | `qmlfmt -w $PATH`                                                                       |
| `raco:fmt`               | `raco fmt -i $PATH`                                                                     |
| `refmt`                  | `refmt --in-place $PATH`                                                                |
| `reformat-gherkin`       | `reformat-gherkin $PATH`                                                                |
| `reorder-python-imports` | `reorder-python-imports $PATH`                                                          |
| `rescript:format`        | `rescript format $PATH`                                                                 |
| `roc:format`             | `roc format $PATH`                                                                      |
| `rstfmt`                 | `rstfmt $PATH`                                                                          |
| `rubocop`                | `rubocop --fix-layout --autocorrect --format quiet $PATH`                               |
| `rubyfmt`                | `rubyfmt -i $PATH`                                                                      |
| `ruff:check`             | `ruff check --fix --quiet $PATH`                                                        |
| `ruff:format`            | `ruff format --quiet $PATH`                                                             |
| `rufo`                   | `rufo --simple-exit $PATH`                                                              |
| `rune:fmt`               | `rune fmt $PATH`                                                                        |
| `rustfmt`                | `rustfmt --edition 2021 --quiet $PATH`                                                  |
| `rustywind`              | `rustywind --write $PATH`                                                               |
| `scalafmt`               | `scalafmt --quiet --mode any $PATH`                                                     |
| `scalariform`            | `scalariform $PATH`                                                                     |
| `shellharden`            | `shellharden --transform --replace $PATH`                                               |
| `shfmt`                  | `shfmt --write $PATH`                                                                   |
| `sleek`                  | `sleek $PATH`                                                                           |
| `smlfmt`                 | `smlfmt --force $PATH`                                                                  |
| `snakefmt`               | `snakefmt $PATH`                                                                        |
| `sql-formatter`          | `sql-formatter --fix $PATH`                                                             |
| `sqlfluff:fix`           | `sqlfluff fix --dialect ansi $PATH`                                                     |
| `sqlfluff:format`        | `sqlfluff format --dialect ansi $PATH`                                                  |
| `sqlfmt`                 | `sqlfmt $PATH`                                                                          |
| `sqruff`                 | `sqruff fix $PATH`                                                                      |
| `standardjs`             | `standard --fix $PATH`                                                                  |
| `standardrb`             | `standardrb --fix $PATH`                                                                |
| `stylefmt`               | `stylefmt $PATH`                                                                        |
| `stylelint`              | `stylelint --fix $PATH`                                                                 |
| `stylish-haskell`        | `stylish-haskell --inplace $PATH`                                                       |
| `stylua`                 | `stylua --verify $PATH`                                                                 |
| `superhtml:fmt`          | `superhtml fmt $PATH`                                                                   |
| `swift-format`           | `swift-format --in-place $PATH`                                                         |
| `swiftformat`            | `swiftformat --quiet $PATH`                                                             |
| `taplo`                  | `taplo format $PATH`                                                                    |
| `templ:fmt`              | `templ fmt $PATH`                                                                       |
| `terraform:fmt`          | `terraform fmt -write=true $PATH`                                                       |
| `terragrunt:hclfmt`      | `terragrunt hclfmt --terragrunt-hclfmt-file $PATH`                                      |
| `tlint:format`           | `tlint format $PATH`                                                                    |
| `tofu:fmt`               | `tofu fmt -write=true $PATH`                                                            |
| `topiary`                | `topiary format $PATH`                                                                  |
| `ts-standard`            | `ts-standard --fix $PATH`                                                               |
| `tsqllint`               | `tsqllint --fix $PATH`                                                                  |
| `twig-cs-fixer:lint`     | `twig-cs-fixer lint $PATH --fix --no-interaction --quiet`                               |
| `typos`                  | `typos -w --no-ignore --hidden $PATH`                                                   |
| `typstfmt`               | `typstfmt $PATH`                                                                        |
| `typstyle`               | `typstyle -i $PATH`                                                                     |
| `ufmt`                   | `ufmt format $PATH`                                                                     |
| `uiua:fmt`               | `uiua fmt $PATH`                                                                        |
| `unimport`               | `unimport -r $PATH`                                                                     |
| `usort`                  | `usort format $PATH`                                                                    |
| `v:fmt`                  | `v fmt -w $PATH`                                                                        |
| `veryl:fmt`              | `veryl fmt $PATH`                                                                       |
| `vhdl-style-guide`       | `vsg -f $PATH --fix`                                                                    |
| `wfindent`               | `wfindent $PATH`                                                                        |
| `xmlformat`              | `xmlformat --overwrite $PATH`                                                           |
| `xmllint`                | `xmllint --format $PATH --output $PATH`                                                 |
| `xo`                     | `xo --fix $PATH`                                                                        |
| `yamlfix`                | `yamlfix $PATH`                                                                         |
| `yamlfmt`                | `yamlfmt -quiet $PATH`                                                                  |
| `yapf`                   | `yapf --in-place $PATH`                                                                 |
| `yew-fmt`                | `yew-fmt --edition 2021 --quiet $PATH`                                                  |
| `zig:fmt`                | `zig fmt $PATH`                                                                         |
| `ziggy:fmt`              | `ziggy fmt $PATH`                                                                       |
| `zprint`                 | `zprint -w $PATH`                                                                       |

<!-- END_SECTION:supported-commands -->

## Shell completions

Shell completions can be generated using `mdsf completions <SHELL>`.

<!-- START_SECTION:completions-command-help -->

```
Generate shell completion

Usage: mdsf completions <SHELL>

Arguments:
  <SHELL>  [possible values: bash, elvish, fish, nushell, powershell, zsh]

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

#### Nushell

Generate completions for [nushell](https://github.com/nushell/nushell).

```nushell
mdsf completions nushell
```

## Acknowledgement

mdsf was inspired by the amazing neovim formatting plugin [conform.nvim](https://github.com/stevearc/conform.nvim).

## Alternatives to mdsf

- [conform.nvim](https://github.com/stevearc/conform.nvim) using `injected` mode.
- [mdformat](https://github.com/executablebooks/mdformat).

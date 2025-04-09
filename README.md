# mdsf

Format, and lint, markdown code snippets using your favorite tools.

<a href="https://crates.io/crates/mdsf"><img src="https://img.shields.io/crates/v/mdsf.svg"></a>
<a href="https://github.com/hougesen/mdsf/actions/workflows/validate.yml"><img src="https://github.com/hougesen/mdsf/actions/workflows/validate.yml/badge.svg"></a>
<a href="https://codecov.io/gh/hougesen/mdsf"><img src="https://codecov.io/gh/hougesen/mdsf/branch/main/graph/badge.svg"/></a>

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
    - [Conda](#conda)
  - [Usage](#usage)
    - [Formatting code](#formatting-code)
      - [Caching formatting results](#caching-formatting-results)
        - [Removing old caches](#removing-old-caches)
    - [Verifying code](#verifying-code)
    - [GitHub Action](#github-action)
    - [Visual Studio Code](#visual-studio-code)
    - [Vim / NeoVim](#vim--neovim)
      - [conform.nvim](#conformnvim)
    - [treefmt](#treefmt)
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

You can install `mdsf` using [npm](https://www.npmjs.com/package/mdsf-cli):

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

### Conda

An _unofficial_ (and unsupported) Conda package can be found at [https://anaconda.org/conda-forge/mdsf](https://anaconda.org/conda-forge/mdsf).

```shell
conda install conda-forge::mdsf
```

## Usage

<!-- START_SECTION:base-command-help -->

```
mdsf 0.8.5-next
Format, and lint, markdown code snippets using your favorite tools
Mads Hougesen <mads@mhouge.dk>

Usage: mdsf [OPTIONS] <COMMAND>

Commands:
  format       Run formatters on input files
  verify       Verify files are formatted
  init         Create a new mdsf config
  completions  Generate shell completion
  cache-prune  Remove caches
  help         Print this message or the help of the given subcommand(s)

Options:
      --log-level <LOG_LEVEL>  [possible values: trace, debug, info, warn, error, off]
  -h, --help                   Print help
  -V, --version                Print version
```

<!-- END_SECTION:base-command-help -->

### Formatting code

The `format` command, as the name implies, is used to format documents.

```shell
mdsf format file.md
```

<!-- START_SECTION:format-command-help -->

```
Run formatters on input files

Usage: mdsf format [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...
          Path to files and/or directories

Options:
      --stdin
          Read input from stdin and write output to stdout

      --config <CONFIG>
          Path to config

      --debug
          Log stdout and stderr of formatters

      --threads <THREADS>
          Amount of threads to use.

          Defaults to 0 (auto).

      --cache
          Cache results

      --log-level <LOG_LEVEL>
          [possible values: trace, debug, info, warn, error, off]

      --timeout <TIMEOUT>
          Tool timeout in seconds

          Defaults to no timeout

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

<!-- END_SECTION:format-command-help -->

#### Caching formatting results

To speed formatting caching can be enabled by supplying the `format` command with the `--cache` argument.

```shell
mdsf format --cache docs/
```

##### Removing old caches

Old caches can be removed by running the `mdsf cache-prune` command.

<!-- START_SECTION:cache-prune-command-help -->

```
Remove caches

Usage: mdsf cache-prune [OPTIONS]

Options:
      --log-level <LOG_LEVEL>  [possible values: trace, debug, info, warn, error, off]
  -h, --help                   Print help
  -V, --version                Print version
```

<!-- END_SECTION:cache-prune-command-help -->

### Verifying code

You can verify that the document is formatted using the `mdsf verify` command.

```shell
mdsf verify docs/
```

<!-- START_SECTION:verify-command-help -->

```
Verify files are formatted

Usage: mdsf verify [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...
          Path to files and/or directories

Options:
      --stdin
          Read input from stdin and write output to stdout

      --config <CONFIG>
          Path to config

      --debug
          Log stdout and stderr of formatters

      --threads <THREADS>
          Amount of threads to use.

          Defaults to 0 (auto).

      --timeout <TIMEOUT>
          Tool timeout in seconds

          Defaults to no timeout

      --log-level <LOG_LEVEL>
          [possible values: trace, debug, info, warn, error, off]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

<!-- END_SECTION:verify-command-help -->

### GitHub Action

There are a lot of different ways to run `mdsf` using GitHub actions.

The easiest way, in my opinion, is to use the official GitHub action to install mdsf.

After that you can run the binary like you would in your terminal.

> \[!NOTE\]
> mdsf is not a package manager.
>
> You must also install the tools you wish to use in your GitHub action.

```yaml
name: mdsf

on: push

jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install mdsf
        uses: hougesen/mdsf@v0.8.5

      - name: Run mdsf
        run: mdsf format --log-level warn .

      - name: Commit changes
        uses: EndBug/add-and-commit@v9
        with:
          message: "style: formatted markdown code blocks"
```

### Visual Studio Code

[![](https://img.shields.io/visual-studio-marketplace/v/hougesen.mdsf?color=374151&label=Visual%20Studio%20Marketplace&labelColor=000&logo=visual-studio-code&logoColor=0098FF)](https://marketplace.visualstudio.com/items?itemName=hougesen.mdsf)
[![](https://img.shields.io/visual-studio-marketplace/v/hougesen.mdsf?color=374151&label=Open%20VSX%20Registry&labelColor=000&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2aWV3Qm94PSI0LjYgNSA5Ni4yIDEyMi43IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgogIDxwYXRoIGQ9Ik0zMCA0NC4yTDUyLjYgNUg3LjN6TTQuNiA4OC41aDQ1LjNMMjcuMiA0OS40em01MSAwbDIyLjYgMzkuMiAyMi42LTM5LjJ6IiBmaWxsPSIjYzE2MGVmIi8+CiAgPHBhdGggZD0iTTUyLjYgNUwzMCA0NC4yaDQ1LjJ6TTI3LjIgNDkuNGwyMi43IDM5LjEgMjIuNi0zOS4xem01MSAwTDU1LjYgODguNWg0NS4yeiIgZmlsbD0iI2E2MGVlNSIvPgo8L3N2Zz4=&logoColor=0098FF)](https://open-vsx.org/extension/hougesen/mdsf)

mdsf can be run using the VSCode extension.

> \[!NOTE\]
> The mdsf VS Code extension does currently not support installing mdsf.
> Which means mdsf must be installed using other means.

### Vim / NeoVim

#### conform.nvim

[conform.nvim](https://github.com/stevearc/conform.nvim) has native support for running mdsf.

```lua
local conform = require("conform")

conform.setup({
	formatters_by_ft = {
		markdown = { "mdsf" },
		-- ...
	},
	-- ...
})
```

### treefmt

Add the following to your `treefmt.toml` to run mdsf using [treefmt](https://github.com/numtide/treefmt).

```toml
# treefmt.toml

[formatter.mdsf]
command = "mdsf"
options = ["format"]
includes = ["*.md"]
```

## Configuration

The default configuration of `mdsf` aims to as simple as possible. For that reason the default formatter for each language is the one most people have installed.

If you are interested in customizing which formatter is run, you can create a new `mdsf` configuration file by running `mdsf init`.

<!-- START_SECTION:init-command-help -->

```
Create a new mdsf config

Usage: mdsf init [OPTIONS]

Options:
      --force                  Create config even if one already exists in current directory
      --log-level <LOG_LEVEL>  [possible values: trace, debug, info, warn, error, off]
  -h, --help                   Print help
  -V, --version                Print version
```

<!-- END_SECTION:init-command-help -->

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

`mdsf` currently supports 297 tools. Feel free to open an issue/pull-request if your favorite tool/command is missing! 😃

| Name                                                                                 | Description                                                                                                                                   | Categories            | Languages                                                                                                                         |
| ------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| [actionlint](https://github.com/rhysd/actionlint)                                    | Static checker for GitHub Actions workflow files                                                                                              | `linter`              | `yaml`                                                                                                                            |
| [air](https://github.com/posit-dev/air)                                              | R formatter and language server                                                                                                               | `formatter`           | `r`                                                                                                                               |
| [alejandra](https://github.com/kamadorueda/alejandra)                                | The Uncompromising Nix Code Formatter                                                                                                         | `formatter`           | `nix`                                                                                                                             |
| [alex](https://github.com/get-alex/alex)                                             | Catch insensitive, inconsiderate writing                                                                                                      | `spell-check`         | `markdown`                                                                                                                        |
| [ameba](https://github.com/crystal-ameba/ameba)                                      | A static code analysis tool for Crystal                                                                                                       | `linter`              | `crystal`                                                                                                                         |
| [ansible-lint](https://github.com/ansible/ansible-lint)                              | ansible-lint checks playbooks for practices and behavior that could potentially be improved and can fix some of the most common ones for you  | `linter`              | `ansible`                                                                                                                         |
| [asmfmt](https://github.com/klauspost/asmfmt)                                        | Go Assembler Formatter                                                                                                                        | `formatter`           | `go`                                                                                                                              |
| [astyle](https://gitlab.com/saalen/astyle)                                           | A Free, Fast, and Small Automatic Formatter for C, C++, C++/CLI, Objective-C, C#, and Java Source Code                                        | `formatter`           | `c#`, `c++`, `c`, `java`, `objective-c`                                                                                           |
| [auto-optional](https://github.com/luttik/auto-optional)                             | Adds the Optional type-hint to arguments where the default value is None                                                                      | `formatter`           | `python`                                                                                                                          |
| [autocorrect](https://github.com/huacnlee/autocorrect)                               | A linter and formatter to help you to improve copywriting, correct spaces, words, and punctuations between CJK (Chinese, Japanese, Korean)    | `spell-check`         |                                                                                                                                   |
| [autoflake](https://github.com/pycqa/autoflake)                                      | Removes unused imports and unused variables as reported by pyflakes                                                                           | `linter`              | `python`                                                                                                                          |
| [autopep8](https://github.com/hhatto/autopep8)                                       | A tool that automatically formats Python code to conform to the PEP 8 style guid                                                              | `formatter`           | `python`                                                                                                                          |
| [bashate](https://github.com/openstack/bashate)                                      | Code style enforcement for bash programs                                                                                                      | `formatter`           | `bash`                                                                                                                            |
| [beancount-black](https://github.com/launchplatform/beancount-black)                 | Opinionated code formatter, just like Python's black code formatter but for Beancount                                                         | `formatter`           | `beancount`                                                                                                                       |
| [beautysh](https://github.com/lovesegfault/beautysh)                                 | A Bash beautifier for the masses                                                                                                              | `formatter`           | `bash`, `shell`                                                                                                                   |
| [bibtex-tidy](https://github.com/flamingtempura/bibtex-tidy)                         | Cleaner and Formatter for BibTeX files                                                                                                        | `formatter`           | `bibtex`                                                                                                                          |
| [bicep](https://github.com/azure/bicep)                                              | Bicep is a declarative language for describing and deploying Azure resources                                                                  | `formatter`           | `bicep`                                                                                                                           |
| [biome](https://github.com/biomejs/biome)                                            | One toolchain for your web project                                                                                                            | `formatter`, `linter` | `javascript`, `json`, `typescript`, `vue`                                                                                         |
| [black](https://github.com/psf/black)                                                | The uncompromising Python code formatter                                                                                                      | `formatter`           | `python`                                                                                                                          |
| [blade-formatter](https://github.com/shufo/blade-formatter)                          | An opinionated blade template formatter for Laravel that respects readability                                                                 | `formatter`           | `blade`, `laravel`, `php`                                                                                                         |
| [blue](https://github.com/grantjenks/blue)                                           | The slightly less uncompromising Python code formatter                                                                                        | `formatter`           | `python`                                                                                                                          |
| [bpfmt](https://source.android.com/docs/setup/reference/androidbp#formatter)         | A formatter for Blueprint files                                                                                                               | `formatter`           | `blueprint`                                                                                                                       |
| [brittany](https://github.com/lspitzner/brittany)                                    | A Haskell source code formatter                                                                                                               | `formatter`           | `haskell`                                                                                                                         |
| [brunette](https://github.com/odwyersoftware/brunette)                               | A best practice Python code formatter                                                                                                         | `formatter`           | `python`                                                                                                                          |
| [bsfmt](https://github.com/rokucommunity/brighterscript-formatter)                   | A code formatter for BrightScript and BrighterScript                                                                                          | `formatter`           | `brighterscript`, `brightscript`                                                                                                  |
| [bslint](https://github.com/rokucommunity/bslint)                                    | A linter for BrightScript and BrighterScript                                                                                                  | `linter`              | `brightscript`, `brightscripter`                                                                                                  |
| [buf](https://buf.build/docs/reference/cli/buf/)                                     | The best way of working with Protocol Buffers                                                                                                 | `formatter`           | `protobuf`                                                                                                                        |
| [buildifier](https://github.com/bazelbuild/buildtools)                               | A bazel BUILD file formatter and                                                                                                              | `formatter`           | `bazel`                                                                                                                           |
| [cabal-fmt](https://github.com/phadej/cabal-fmt)                                     | An experiment of formatting .cabal files                                                                                                      | `formatter`           | `cabal`                                                                                                                           |
| [cabal-prettify](https://github.com/kindaro/cabal-prettify)                          | Prettify your Cabal package configuration files                                                                                               | `formatter`           | `cabal`                                                                                                                           |
| [cabal](https://www.haskell.org/cabal/)                                              | Cabal is a system for building and packaging Haskell libraries and programs                                                                   | `formatter`           | `cabal`                                                                                                                           |
| [caddy](https://caddyserver.com/docs/command-line#caddy-fmt)                         | Formats or prettifies a Caddyfile                                                                                                             | `formatter`           | `caddy`                                                                                                                           |
| [caramel](https://caramel.run/)                                                      | Formatter for the Caramel programming language                                                                                                | `formatter`           | `caramel`                                                                                                                         |
| [cfn-lint](https://github.com/aws-cloudformation/cfn-lint)                           | CloudFormation Linter                                                                                                                         | `linter`              | `cloudformation`, `json`, `yaml`                                                                                                  |
| [checkmake](https://github.com/mrtazz/checkmake)                                     | Experimental linter/analyzer for Makefiles                                                                                                    | `linter`              | `makefile`                                                                                                                        |
| [clang-format](https://clang.llvm.org/docs/ClangFormat.html)                         | A tool to format C/C++/Java/JavaScript/JSON/Objective-C/Protobuf/C# code                                                                      | `formatter`           | `c#`, `c++`, `c`, `java`, `javascript`, `json`, `objective-c`, `protobuf`                                                         |
| [clang-tidy](https://clang.llvm.org/extra/clang-tidy/)                               | clang-tidy is a clang-based C++ “linter” tool                                                                                                 | `linter`              | `c++`                                                                                                                             |
| [clj-kondo](https://github.com/clj-kondo/clj-kondo)                                  | Static analyzer and linter for Clojure code that sparks joy                                                                                   | `linter`              | `clojure`, `clojurescript`                                                                                                        |
| [cljfmt](https://github.com/weavejester/cljfmt)                                      | A tool for formatting Clojure code                                                                                                            | `formatter`           | `clojure`                                                                                                                         |
| [cljstyle](https://github.com/greglook/cljstyle)                                     | A tool for formatting Clojure code                                                                                                            | `formatter`           | `clojure`                                                                                                                         |
| [cmake-format](https://cmake-format.readthedocs.io/en/latest/cmake-format.html)      | cmake-format can format your listfiles nicely so that they don't look like crap                                                               | `formatter`           | `cmake`                                                                                                                           |
| [cmake-lint](https://cmake-format.readthedocs.io/en/latest/lint-usage.html)          | Lint CMake files                                                                                                                              | `linter`              | `cmake`                                                                                                                           |
| [codeql](https://docs.github.com/en/code-security/codeql-cli/codeql-cli-manual)      | Format queries and libraries with CodeQL                                                                                                      | `formatter`           | `codeql`                                                                                                                          |
| [codespell](https://github.com/codespell-project/codespell)                          | Check code for common misspellings                                                                                                            | `spell-check`         |                                                                                                                                   |
| [coffeelint](https://github.com/coffeelint/coffeelint)                               | Lint your CoffeeScript                                                                                                                        | `linter`              | `coffeescript`                                                                                                                    |
| [cppcheck](https://cppcheck.sourceforge.io/)                                         | Cppcheck is a static analysis tool for C/C++ code                                                                                             | `linter`              | `c++`, `c`                                                                                                                        |
| [cpplint](https://github.com/cpplint/cpplint)                                        | Static code checker for C++                                                                                                                   | `linter`              | `c++`                                                                                                                             |
| [crlfmt](https://github.com/cockroachdb/crlfmt)                                      | Formatter for CockroachDB's additions to the Go style guide                                                                                   | `formatter`           | `go`                                                                                                                              |
| [crystal](https://crystal-lang.org/)                                                 | Tools for the Crystal programming language                                                                                                    | `formatter`           | `crystal`                                                                                                                         |
| [csharpier](https://csharpier.com/)                                                  | An Opinionated Code Formatter for C#                                                                                                          | `formatter`           | `c#`                                                                                                                              |
| [css-beautify](https://github.com/beautifier/js-beautify)                            | A css formatter                                                                                                                               | `formatter`           | `css`                                                                                                                             |
| [csscomb](https://github.com/csscomb/csscomb.js)                                     | CSS coding style formatter                                                                                                                    | `formatter`           | `css`                                                                                                                             |
| [csslint](https://github.com/csslint/csslint)                                        | Automated linting of Cascading Stylesheets                                                                                                    | `linter`              | `css`                                                                                                                             |
| [curlylint](https://github.com/thibaudcolas/curlylint)                               | Experimental HTML templates linting for Jinja, Nunjucks, Django templates, Twig, Liquid                                                       | `linter`              | `django`, `html`, `jinja`, `liquid`, `nunjucks`, `twig`                                                                           |
| [d2](https://d2lang.com/)                                                            | A modern language that turns text to diagrams                                                                                                 | `formatter`           | `d2`                                                                                                                              |
| [dart](https://dart.dev/tools)                                                       | Formatter and linter for Dart                                                                                                                 | `formatter`, `linter` | `dart`, `flutter`                                                                                                                 |
| [dcm](https://dcm.dev/)                                                              | Code Quality Tool for Flutter Developers                                                                                                      | `formatter`, `linter` | `dart`, `flutter`                                                                                                                 |
| [deadnix](https://github.com/astro/deadnix)                                          | Scan Nix files for dead code                                                                                                                  | `linter`              | `nix`                                                                                                                             |
| [deno](https://docs.deno.com/runtime/reference/cli/)                                 | Formatter and linter for JavaScript and TypeScript                                                                                            | `formatter`, `linter` | `javascript`, `json`, `typescript`                                                                                                |
| [dfmt](https://github.com/dlang-community/dfmt)                                      | Dfmt is a formatter for D source code                                                                                                         | `formatter`           | `d`                                                                                                                               |
| [dhall](https://dhall-lang.org/)                                                     | Format Dhall files                                                                                                                            | `formatter`           | `dhall`                                                                                                                           |
| [djade](https://github.com/adamchainz/djade)                                         | A Django template formatter                                                                                                                   | `formatter`           | `django`, `python`                                                                                                                |
| [djlint](https://www.djlint.com/)                                                    | Lint & Format HTML Templates                                                                                                                  | `formatter`, `linter` | `handlebars`, `html`, `jinja`, `mustache`, `nunjucks`, `twig`                                                                     |
| [docformatter](https://github.com/PyCQA/docformatter)                                | Formats docstrings to follow PEP 257                                                                                                          | `formatter`           | `python`                                                                                                                          |
| [dockerfmt](https://github.com/reteps/dockerfmt)                                     | Dockerfile formatter. a modern dockfmt                                                                                                        | `formatter`           | `docker`                                                                                                                          |
| [dockfmt](https://github.com/jessfraz/dockfmt)                                       | Dockerfile format and parser. Like `gofmt` but for Dockerfiles                                                                                | `formatter`           | `docker`                                                                                                                          |
| [docstrfmt](https://github.com/LilSpazJoekp/docstrfmt)                               | A formatter for Sphinx flavored reStructuredText                                                                                              | `formatter`           | `python`, `restructuredtext`, `sphinx`                                                                                            |
| [doctoc](https://github.com/thlorenz/doctoc)                                         | Generates table of contents for markdown files                                                                                                | `formatter`           | `markdown`                                                                                                                        |
| [dotenv-linter](https://github.com/dotenv-linter/dotenv-linter)                      | Lightning-fast linter for .env files                                                                                                          | `linter`              | `env`                                                                                                                             |
| [dprint](https://dprint.dev/)                                                        | A pluggable and configurable code formatting platform written in Rust                                                                         | `formatter`           |                                                                                                                                   |
| [dscanner](https://github.com/dlang-community/d-scanner)                             | Swiss-army knife for D source code                                                                                                            | `linter`              | `d`                                                                                                                               |
| [easy-coding-standard](https://github.com/easy-coding-standard/easy-coding-standard) | The Easiest way to add coding standard to your PHP project                                                                                    | `formatter`, `linter` | `php`                                                                                                                             |
| [efmt](https://github.com/sile/efmt)                                                 | Erlang code formatter                                                                                                                         | `formatter`           | `erlang`                                                                                                                          |
| [elm-format](https://github.com/avh4/elm-format)                                     | elm-format formats Elm source code according to a standard set of rules based on the official Elm Style Guide                                 | `formatter`           | `elm`                                                                                                                             |
| [eradicate](https://github.com/PyCQA/eradicate)                                      | Removes commented-out code from Python files                                                                                                  | `linter`              | `python`                                                                                                                          |
| [erb-formatter](https://github.com/nebulab/erb-formatter)                            | Format ERB files with speed and precision                                                                                                     | `formatter`           | `erb`, `ruby`                                                                                                                     |
| [erlfmt](https://github.com/whatsapp/erlfmt)                                         | An automated code formatter for Erlang                                                                                                        | `formatter`           | `erlang`                                                                                                                          |
| [eslint](https://github.com/eslint/eslint/)                                          | Find and fix problems in your JavaScript code                                                                                                 | `linter`              | `javascript`, `typescript`                                                                                                        |
| [fantomas](https://github.com/fsprojects/fantomas)                                   | FSharp source code formatter                                                                                                                  | `formatter`           | `fsharp`                                                                                                                          |
| [fish_indent](https://fishshell.com/docs/current/cmds/fish_indent.html)              | Fish indenter and prettifier                                                                                                                  | `formatter`           | `fish`                                                                                                                            |
| [fixjson](https://github.com/rhysd/fixjson)                                          | JSON Fixer for Humans using (relaxed) JSON5                                                                                                   | `formatter`, `linter` | `json5`, `json`                                                                                                                   |
| [floskell](https://github.com/ennocramer/floskell)                                   | Floskell is a flexible Haskell source code pretty printer                                                                                     | `formatter`           | `haskell`                                                                                                                         |
| [flynt](https://github.com/ikamensh/flynt)                                           | A tool to automatically convert old string literal formatting to f-strings                                                                    | `formatter`           | `python`                                                                                                                          |
| [fnlfmt](https://git.sr.ht/~technomancy/fnlfmt)                                      | A formatter for Fennel code                                                                                                                   | `formatter`           | `fennel`                                                                                                                          |
| [forge](https://github.com/foundry-rs/foundry)                                       | A Solidity formatter                                                                                                                          | `formatter`           | `solidity`                                                                                                                        |
| [fortitude](https://github.com/plasmafair/fortitude)                                 | A Fortran linter, written in Rust                                                                                                             | `linter`              |                                                                                                                                   |
| [fortran-linter](https://github.com/cphyc/fortran-linter)                            | A simple fortran syntax checker, including automatic fixing of the code                                                                       | `formatter`, `linter` | `fortran`                                                                                                                         |
| [fourmolu](https://github.com/fourmolu/fourmolu)                                     | A formatter for Haskell source code                                                                                                           | `formatter`           | `haskell`                                                                                                                         |
| [fprettify](https://github.com/fortran-lang/fprettify)                               | Auto-formatter for modern Fortran source code                                                                                                 | `formatter`           | `fortran`                                                                                                                         |
| [futhark](https://futhark.readthedocs.io/en/latest/man/futhark-fmt.html)             | Code formatter for the furhark programming language                                                                                           | `formatter`           | `futhark`                                                                                                                         |
| [gci](https://github.com/daixiang0/gci)                                              | GCI, a tool that control golang package import order and make it always deterministic                                                         | `formatter`           | `go`                                                                                                                              |
| [gdformat](https://github.com/scony/godot-gdscript-toolkit)                          | GDScript formatter                                                                                                                            | `formatter`           | `gdscript`                                                                                                                        |
| [gdlint](https://github.com/scony/godot-gdscript-toolkit)                            | GDScript linter                                                                                                                               | `linter`              | `gdscript`                                                                                                                        |
| [gersemi](https://github.com/blankspruce/gersemi)                                    | A formatter to make your CMake code the real treasure                                                                                         | `formatter`           | `cmake`                                                                                                                           |
| [gleam](https://gleam.run/)                                                          | Format Gleam source code                                                                                                                      | `formatter`           | `gleam`                                                                                                                           |
| [gluon](https://github.com/gluon-lang/gluon)                                         | Code formatting for the gluon programming language                                                                                            | `formatter`           | `gluon`                                                                                                                           |
| [gofmt](https://pkg.go.dev/cmd/gofmt)                                                | Gofmt formats Go programs                                                                                                                     | `formatter`           | `go`                                                                                                                              |
| [gofumpt](https://github.com/mvdan/gofumpt)                                          | A stricter gofmt                                                                                                                              | `formatter`           | `go`                                                                                                                              |
| [goimports-reviser](https://github.com/incu6us/goimports-reviser)                    | Right imports sorting & code formatting tool (goimports alternative)                                                                          | `formatter`           | `go`                                                                                                                              |
| [goimports](https://pkg.go.dev/golang.org/x/tools/cmd/goimports)                     | goimports updates your Go import lines, adding missing ones and removing unreferenced ones                                                    | `formatter`           | `go`                                                                                                                              |
| [golangci-lint](https://github.com/golangci/golangci-lint)                           | Fast linters runner for Go                                                                                                                    | `formatter`, `linter` | `go`                                                                                                                              |
| [golines](https://github.com/segmentio/golines)                                      | A golang formatter that fixes long lines                                                                                                      | `formatter`           | `go`                                                                                                                              |
| [google-java-format](https://github.com/google/google-java-format)                   | Reformats Java source code to comply with Google Java Style                                                                                   | `formatter`           | `java`                                                                                                                            |
| [gospel](https://github.com/kortschak/gospel)                                        | Misspelled word linter for Go comments, string literals and embedded files                                                                    | `spell-check`         | `go`                                                                                                                              |
| [grain](https://grain-lang.org/docs/tooling/grain_cli)                               | Code formatter for the Grain programming language                                                                                             | `formatter`           | `grain`                                                                                                                           |
| [hadolint](https://github.com/hadolint/hadolint)                                     | Dockerfile linter, validate inline bash, written in Haskell                                                                                   | `linter`              | `dockerfile`                                                                                                                      |
| [haml-lint](https://github.com/sds/haml-lint)                                        | Tool for writing clean and consistent HAML                                                                                                    | `linter`              | `haml`                                                                                                                            |
| [hclfmt](https://github.com/hashicorp/hcl)                                           | Formatter for hcl files                                                                                                                       | `formatter`           | `hcl`                                                                                                                             |
| [hfmt](https://github.com/danstiner/hfmt)                                            | Format Haskell programs. Inspired by the gofmt utility                                                                                        | `formatter`           | `haskell`                                                                                                                         |
| [hindent](https://github.com/mihaimaruseac/hindent)                                  | Extensible Haskell pretty printer                                                                                                             | `formatter`           | `haskell`                                                                                                                         |
| [hlint](https://github.com/ndmitchell/hlint)                                         | Haskell source code suggestions                                                                                                               | `linter`              | `haskell`                                                                                                                         |
| [html-beautify](https://github.com/beautifier/js-beautify)                           | A html formatter                                                                                                                              | `formatter`           | `html`                                                                                                                            |
| [htmlbeautifier](https://github.com/threedaymonk/htmlbeautifier)                     | A normaliser/beautifier for HTML that also understands embedded Ruby. Ideal for tidying up Rails templates                                    | `formatter`           | `erb`, `html`, `ruby`                                                                                                             |
| [htmlhint](https://github.com/htmlhint/htmlhint)                                     | The static code analysis tool you need for your HTML                                                                                          | `linter`              | `html`                                                                                                                            |
| [hurlfmt](https://hurl.dev/)                                                         | Formatter for hurl files                                                                                                                      | `formatter`           | `hurl`                                                                                                                            |
| [imba](https://imba.io/)                                                             | A formatter for Imba                                                                                                                          | `formatter`           | `imba`                                                                                                                            |
| [inko](https://github.com/inko-lang/inko)                                            | Code formatter for the inko programming language                                                                                              | `formatter`           | `inko`                                                                                                                            |
| [isort](https://github.com/timothycrosley/isort)                                     | A Python utility to sort imports                                                                                                              | `formatter`           | `python`                                                                                                                          |
| [joker](https://github.com/candid82/joker)                                           | Small Clojure interpreter, linter and formatter                                                                                               | `formatter`, `linter` | `clojure`                                                                                                                         |
| [jq](https://github.com/jqlang/jq)                                                   | JSON processor                                                                                                                                | `formatter`           | `json`                                                                                                                            |
| [jqfmt](https://github.com/noperator/jqfmt)                                          | like gofmt, but for jq                                                                                                                        | `formatter`           | `jq`                                                                                                                              |
| [js-beautify](https://github.com/beautifier/js-beautify)                             | A JavaScript formatter                                                                                                                        | `formatter`           | `javascript`                                                                                                                      |
| [json5format](https://github.com/google/json5format)                                 | JSON5 (a.k.a., JSON for Humans) formatter that preserves contextual comments                                                                  | `formatter`           | `json5`, `json`                                                                                                                   |
| [jsona](https://github.com/jsona/jsona)                                              | JSONA linter and formatter                                                                                                                    | `formatter`, `linter` | `jsona`                                                                                                                           |
| [jsonlint](https://github.com/zaach/jsonlint)                                        | A JSON parser and validator with a CLI                                                                                                        | `formatter`, `linter` | `json`                                                                                                                            |
| [jsonnet-lint](https://jsonnet.org/learning/tools.html)                              | Linter for jsonnet files                                                                                                                      | `linter`              | `jsonnet`                                                                                                                         |
| [jsonnetfmt](https://jsonnet.org/learning/tools.html)                                | Formatter for automatically fixing jsonnet stylistic problems                                                                                 | `formatter`           | `jsonnet`                                                                                                                         |
| [jsonpp](https://github.com/jmhodges/jsonpp)                                         | A fast command line JSON pretty printer                                                                                                       | `formatter`           | `json`                                                                                                                            |
| [juliaformatter.jl](https://github.com/domluna/juliaformatter.jl)                    | An opinionated code formatter for Julia. Plot twist - the opinion is your own                                                                 | `formatter`           | `julia`                                                                                                                           |
| [just](https://github.com/casey/just)                                                | A formatter for justfiles                                                                                                                     | `formatter`           | `just`                                                                                                                            |
| [kcl](https://www.kcl-lang.io/docs/tools/cli/kcl/fmt)                                | KCL Format tool supports reformatting KCL files to the standard code style                                                                    | `formatter`           | `kcl`                                                                                                                             |
| [kdlfmt](https://github.com/hougesen/kdlfmt)                                         | A formatter for kdl documents                                                                                                                 | `formatter`           | `kdl`                                                                                                                             |
| [kdoc-formatter](https://github.com/tnorbye/kdoc-formatter)                          | Reformats Kotlin KDoc comments, reflowing text and other cleanup                                                                              | `formatter`           | `kotlin`                                                                                                                          |
| [ktfmt](https://github.com/facebook/ktfmt)                                           | program that reformats Kotlin source code to comply with the common community standard for Kotlin code conventions                            | `formatter`           | `kotlin`                                                                                                                          |
| [ktlint](https://github.com/pinterest/ktlint)                                        | An anti-bikeshedding Kotlin linter with built-in formatter                                                                                    | `linter`              | `kotlin`                                                                                                                          |
| [kulala-fmt](https://github.com/mistweaverco/kulala-fmt)                             | An opinionated .http and .rest file linter and formatter                                                                                      | `formatter`           | `http`                                                                                                                            |
| [leptosfmt](https://github.com/bram209/leptosfmt)                                    | A formatter for the leptos view! macro                                                                                                        | `formatter`           | `rust`                                                                                                                            |
| [liquidsoap-prettier](https://github.com/savonet/liquidsoap-prettier)                | Prettier plugin for liquidsoap script                                                                                                         | `formatter`           | `liquidsoap`                                                                                                                      |
| [luacheck](https://github.com/lunarmodules/luacheck)                                 | A tool for linting and static analysis of Lua code                                                                                            | `formatter`           | `lua`                                                                                                                             |
| [luaformatter](https://github.com/koihik/luaformatter)                               | Code formatter for Lua                                                                                                                        | `formatter`           | `lua`                                                                                                                             |
| [mado](https://github.com/akiomik/mado)                                              | A fast Markdown linter written in Rust                                                                                                        | `linter`              | `markdown`                                                                                                                        |
| [mago](https://github.com/carthage-software/mago)                                    | A fast linter and formatter for PHP                                                                                                           | `formatter`, `linter` | `php`                                                                                                                             |
| [markdownfmt](https://github.com/shurcool/markdownfmt)                               | Like gofmt, but for Markdown                                                                                                                  | `formatter`           | `markdown`                                                                                                                        |
| [markdownlint-cli2](https://github.com/davidanson/markdownlint-cli2)                 | A fast, flexible, configuration-based command-line interface for linting Markdown/CommonMark files with the markdownlint library              | `linter`              | `markdown`                                                                                                                        |
| [markdownlint](https://github.com/davidanson/markdownlint)                           | A Node.js style checker and lint tool for Markdown/CommonMark files                                                                           | `linter`              | `markdown`                                                                                                                        |
| [markuplint](https://markuplint.dev/)                                                | An HTML linter for all markup developers                                                                                                      | `linter`              | `html`                                                                                                                            |
| [md-padding](https://github.com/harttle/md-padding)                                  | Fix mixed spaces in Markdown: Chinese and English, numbers, links                                                                             | `formatter`           | `markdown`                                                                                                                        |
| [mdformat](https://github.com/executablebooks/mdformat)                              | CommonMark compliant Markdown formatter                                                                                                       | `formatter`           | `markdwon`                                                                                                                        |
| [mdsf](https://github.com/hougesen/mdsf)                                             | Run mdsf inside mdsf                                                                                                                          | `formatter`           | `markdown`                                                                                                                        |
| [mdslw](https://github.com/razziel89/mdslw)                                          | Prepare your markdown for easy diff'ing!                                                                                                      | `formatter`           | `markdown`                                                                                                                        |
| [meson](https://mesonbuild.com/)                                                     | Meson is an open source build system meant to be both extremely fast, and, even more importantly, as user friendly as possible                | `formatter`           | `meson`                                                                                                                           |
| [mise](https://github.com/jdx/mise)                                                  | The front-end to your dev env                                                                                                                 |                       |                                                                                                                                   |
| [misspell](https://github.com/client9/misspell/)                                     | Correct commonly misspelled English words in source files                                                                                     | `spell-check`         |                                                                                                                                   |
| [mix](https://hexdocs.pm/mix/main/Mix.Tasks.Format.html)                             | Code formatter for Elixir                                                                                                                     | `formatter`           | `elixir`                                                                                                                          |
| [mojo](https://docs.modular.com/mojo/cli/format)                                     | Formats Mojo source files                                                                                                                     | `formatter`           | `mojo`                                                                                                                            |
| [muon](https://github.com/muon-build/muon)                                           | An implementation of the meson build system                                                                                                   | `formatter`, `linter` | `meson`                                                                                                                           |
| [mypy](https://github.com/python/mypy)                                               | Optional static typing for Python                                                                                                             | `linter`              | `python`                                                                                                                          |
| [nasmfmt](https://github.com/yamnikov-oleg/nasmfmt)                                  | Formatter for NASM source files                                                                                                               | `formatter`           | `assembly`                                                                                                                        |
| [nginxbeautifier](https://github.com/vasilevich/nginxbeautifier)                     | Format and beautify nginx config files                                                                                                        | `formatter`           | `nginx`                                                                                                                           |
| [nginxfmt](https://github.com/slomkowski/nginx-config-formatter)                     | nginx config file formatter/beautifier written in Python with no additional dependencies                                                      | `formatter`           | `nginx`                                                                                                                           |
| [nickel](https://nickel-lang.org/)                                                   | Better configuration for less                                                                                                                 | `formatter`           | `nickel`                                                                                                                          |
| [nimpretty](https://github.com/nim-lang/nim)                                         | Code formatter for the Nim programming language                                                                                               | `formatter`           | `nim`                                                                                                                             |
| [nixfmt](https://github.com/nixos/nixfmt)                                            | The official (but not yet stable) formatter for Nix code                                                                                      | `formatter`           | `nix`                                                                                                                             |
| [nixpkgs-fmt](https://github.com/nix-community/nixpkgs-fmt)                          | Nix code formatter for nixpkgs                                                                                                                | `formatter`           | `nix`                                                                                                                             |
| [nomad](https://developer.hashicorp.com/nomad/docs/commands)                         | CLI for HashiCorp Nomad                                                                                                                       | `formatter`           | `hcl`                                                                                                                             |
| [nph](https://github.com/arnetheduck/nph)                                            | An opinionated code formatter for Nim                                                                                                         | `formatter`           | `nim`                                                                                                                             |
| [npm-groovy-lint](https://github.com/nvuillam/npm-groovy-lint)                       | Lint, format and auto-fix your Groovy / Jenkinsfile / Gradle files                                                                            | `formatter`, `linter` | `groovy`                                                                                                                          |
| [nufmt](https://github.com/nushell/nufmt)                                            | the nushell formatter                                                                                                                         | `formatter`           | `nushell`                                                                                                                         |
| [ocamlformat](https://github.com/ocaml-ppx/ocamlformat)                              | Auto-formatter for OCaml code                                                                                                                 | `formatter`           | `ocaml`                                                                                                                           |
| [ocp-indent](https://github.com/ocamlpro/ocp-indent)                                 | Indentation tool for OCaml                                                                                                                    | `formatter`           | `ocaml`                                                                                                                           |
| [odinfmt](https://github.com/danielgavin/ols)                                        | Formatter for the Odin programming language                                                                                                   | `formatter`           | `odin`                                                                                                                            |
| [oelint-adv](https://github.com/priv-kweihmann/oelint-adv)                           | Advanced oelint                                                                                                                               | `linter`              | `bitbake`                                                                                                                         |
| [opa](https://www.openpolicyagent.org/docs/latest/cli/)                              | Format Rego source files                                                                                                                      | `formatter`           | `rego`                                                                                                                            |
| [ormolu](https://github.com/tweag/ormolu)                                            | A formatter for Haskell source code                                                                                                           | `formatter`           | `haskell`                                                                                                                         |
| [oxlint](https://oxc.rs/docs/guide/usage/linter.html)                                | Oxlint is designed to catch erroneous or useless code without requiring any configurations by default                                         | `linter`              | `javascript`, `typescript`                                                                                                        |
| [packer](https://developer.hashicorp.com/packer/docs/commands)                       | Packer is used to format HCL2 configuration files                                                                                             | `formatter`           | `hcl`                                                                                                                             |
| [pasfmt](https://github.com/integrated-application-development/pasfmt)               | Delphi code formatter                                                                                                                         | `formatter`           | `delphi`, `pascal`                                                                                                                |
| [perflint](https://github.com/tonybaloney/perflint)                                  | Python Linter for performance anti patterns                                                                                                   | `linter`              | `python`                                                                                                                          |
| [perltidy](https://github.com/perltidy/perltidy)                                     | Perl::Tidy, a source code formatter for Perl                                                                                                  | `formatter`           | `perl`                                                                                                                            |
| [pg_format](https://github.com/darold/pgformatter)                                   | A PostgreSQL SQL syntax beautifier                                                                                                            | `formatter`           | `sql`                                                                                                                             |
| [php-cs-fixer](https://github.com/php-cs-fixer/php-cs-fixer)                         | A tool to automatically fix PHP Coding Standards issues                                                                                       | `formatter`, `linter` | `php`                                                                                                                             |
| [phpcbf](https://phpqa.io/projects/phpcbf.html)                                      | PHP Code Beautifier and Fixer fixes violations of a defined coding standard                                                                   | `formatter`           | `php`                                                                                                                             |
| [phpinsights](https://github.com/nunomaduro/phpinsights)                             | Instant PHP quality checks from your console                                                                                                  | `linter`              | `php`                                                                                                                             |
| [pint](https://github.com/laravel/pint)                                              | Laravel Pint is an opinionated PHP code style fixer for minimalists                                                                           | `formatter`, `linter` | `php`                                                                                                                             |
| [prettier](https://github.com/prettier/prettier)                                     | Prettier is an opinionated code formatter                                                                                                     | `formatter`           | `angular`, `css`, `ember`, `graphql`, `handlebars`, `html`, `javascript`, `json`, `less`, `markdown`, `scss`, `typescript`, `vue` |
| [pretty-php](https://github.com/lkrms/pretty-php)                                    | The opinionated PHP code formatter                                                                                                            | `formatter`           | `php`                                                                                                                             |
| [prettypst](https://github.com/antonwetzel/prettypst)                                | Formatter for Typst                                                                                                                           | `formatter`           | `typst`                                                                                                                           |
| [prisma](https://www.prisma.io/docs/orm/tools/prisma-cli)                            | Commands for interacting with the prisma ORM                                                                                                  | `formatter`           | `prisma`                                                                                                                          |
| [proselint](https://github.com/amperser/proselint)                                   | A linter for prose                                                                                                                            | `spell-check`         |                                                                                                                                   |
| [protolint](https://github.com/yoheimuta/protolint)                                  | A pluggable linter and fixer to enforce Protocol Buffer style and conventions                                                                 | `linter`              | `protobuf`                                                                                                                        |
| [ptop](https://www.freepascal.org/tools/ptop.html)                                   | Free Pascal source formatter                                                                                                                  | `formatter`           | `pascal`                                                                                                                          |
| [pug-lint](https://github.com/pugjs/pug-lint)                                        | An unopinionated and configurable linter and style checker for Pug                                                                            | `linter`              | `pug`                                                                                                                             |
| [puppet-lint](https://github.com/puppetlabs/puppet-lint)                             | Check that your Puppet manifests conform to the style guide                                                                                   | `linter`              | `puppet`                                                                                                                          |
| [purs-tidy](https://github.com/natefaubion/purescript-tidy)                          | PureScript code formatter                                                                                                                     | `formatter`           | `purescript`                                                                                                                      |
| [purty](https://gitlab.com/joneshf/purty)                                            | PureScript pretty-printer                                                                                                                     | `formatter`           | `purescript`                                                                                                                      |
| [pycln](https://github.com/hadialqattan/pycln)                                       | A formatter for finding and removing unused import statements                                                                                 | `formatter`           | `python`                                                                                                                          |
| [pycodestyle](https://github.com/pycqa/pycodestyle)                                  | Simple Python style checker in one Python file                                                                                                | `linter`              | `python`                                                                                                                          |
| [pydoclint](https://github.com/jsh9/pydoclint)                                       | A Python docstring linter that checks arguments, returns, yields, and raises sections                                                         | `linter`              | `python`                                                                                                                          |
| [pydocstringformatter](https://github.com/danielnoord/pydocstringformatter)          | Automatically format your Python docstrings to conform with PEP 8 and PEP 257                                                                 | `formatter`           | `python`                                                                                                                          |
| [pydocstyle](https://github.com/PyCQA/pydocstyle)                                    | docstring style checker                                                                                                                       | `formatter`           | `python`                                                                                                                          |
| [pyflakes](https://github.com/pycqa/pyflakes)                                        | A simple program which checks Python source files for errors                                                                                  | `linter`              | `python`                                                                                                                          |
| [pyink](https://github.com/google/pyink)                                             | Pyink is a Python formatter, forked from Black with a few different formatting behaviors                                                      | `formatter`           | `python`                                                                                                                          |
| [pylint](https://github.com/pylint-dev/pylint)                                       | Pylint is a static code analyser for Python 2 or 3                                                                                            | `linter`              | `python`                                                                                                                          |
| [pyment](https://github.com/dadadel/pyment)                                          | Format and convert Python docstrings and generates patches                                                                                    | `formatter`           | `python`                                                                                                                          |
| [pyupgrade](https://github.com/asottile/pyupgrade)                                   | A tool to automatically upgrade Python syntax to newer versions                                                                               | `linter`              | `python`                                                                                                                          |
| [qmlfmt](https://github.com/jesperhh/qmlfmt)                                         | Command line application that formats QML files                                                                                               | `formatter`           | `qml`                                                                                                                             |
| [quick-lint-js](https://github.com/quick-lint/quick-lint-js)                         | quick-lint-js finds bugs in JavaScript programs                                                                                               | `linter`              | `javascript`                                                                                                                      |
| [raco](https://docs.racket-lang.org/fmt/)                                            | An extensible code formatter for Racket                                                                                                       | `formatter`           | `racket`                                                                                                                          |
| [reek](https://github.com/troessner/reek)                                            | Code smell detector for Ruby                                                                                                                  | `linter`              | `ruby`                                                                                                                            |
| [refmt](https://reasonml.github.io/docs/en/refmt)                                    | refmt stands by Reason Formatter and it formats Reason programs, is a parser and pretty-printer for Reason                                    | `formatter`           | `reason`                                                                                                                          |
| [reformat-gherkin](https://github.com/ducminh-phan/reformat-gherkin)                 | Reformat-gherkin automatically formats Gherkin files                                                                                          | `formatter`           | `gherkin`                                                                                                                         |
| [refurb](https://github.com/dosisod/refurb)                                          | A tool for refurbishing and modernizing Python codebases                                                                                      | `linter`              | `python`                                                                                                                          |
| [regal](https://github.com/styrainc/regal)                                           | Regal is a linter and language server for Rego, bringing your policy development experience to the next level                                 | `linter`              | `rego`                                                                                                                            |
| [reorder-python-imports](https://github.com/asottile/reorder-python-imports)         | Rewrites source to reorder python imports                                                                                                     | `formatter`           | `python`                                                                                                                          |
| [rescript](https://github.com/rescript-lang/rescript)                                | Formatter for ReScript                                                                                                                        | `formatter`           | `rescript`                                                                                                                        |
| [revive](https://github.com/mgechev/revive)                                          | ~6x faster, stricter, configurable, extensible, and beautiful drop-in replacement for golint                                                  | `linter`              | `go`                                                                                                                              |
| [roc](https://github.com/roc-lang/roc)                                               | Tools for the roc programming language                                                                                                        | `formatter`           | `roc`                                                                                                                             |
| [rstfmt](https://github.com/dzhu/rstfmt)                                             | A formatter for reStructuredText                                                                                                              | `formatter`           | `restructuredtext`                                                                                                                |
| [rubocop](https://github.com/rubocop/rubocop)                                        | A Ruby static code analyzer and formatter, based on the community Ruby style guide                                                            | `formatter`, `linter` | `ruby`                                                                                                                            |
| [rubyfmt](https://github.com/fables-tales/rubyfmt)                                   | Ruby Autoformatter                                                                                                                            | `formatter`           | `ruby`                                                                                                                            |
| [ruff](https://github.com/astral-sh/ruff)                                            | An extremely fast Python linter and code formatter, written in Rust                                                                           | `formatter`, `linter` | `python`                                                                                                                          |
| [rufo](https://github.com/ruby-formatter/rufo)                                       | The Ruby Formatter                                                                                                                            | `formatter`           | `ruby`                                                                                                                            |
| [rune](https://github.com/rune-rs/rune)                                              | Tools for the Rune programming language                                                                                                       | `formatter`           | `rune`                                                                                                                            |
| [runic](https://github.com/fredrikekre/runic.jl)                                     | Julia code formatter                                                                                                                          | `formatter`           | `julia`                                                                                                                           |
| [rustfmt](https://github.com/rust-lang/rustfmt)                                      | The official code formatter for Rust                                                                                                          | `formatter`           | `rust`                                                                                                                            |
| [rustywind](https://github.com/avencera/rustywind)                                   | CLI for organizing Tailwind CSS classes                                                                                                       | `formatter`           | `html`                                                                                                                            |
| [salt-lint](https://github.com/warpnet/salt-lint)                                    | A command-line utility that checks for best practices in SaltStack                                                                            | `linter`              | `salt`                                                                                                                            |
| [scalafmt](https://github.com/scalameta/scalafmt)                                    | Code formatter for Scala                                                                                                                      | `formatter`           | `scala`                                                                                                                           |
| [scalariform](https://github.com/scala-ide/scalariform)                              | Scala source code formatter                                                                                                                   | `formatter`           | `scala`                                                                                                                           |
| [selene](https://github.com/kampfkarren/selene)                                      | A blazing-fast modern Lua linter written in Rust                                                                                              | `linter`              | `lua`                                                                                                                             |
| [semistandard](https://github.com/standard/semistandard)                             | All the goodness of standardjs with semicolons sprinkled on top                                                                               | `formatter`, `linter` | `javascript`                                                                                                                      |
| [shellcheck](https://github.com/koalaman/shellcheck)                                 | ShellCheck, a static analysis tool for shell scripts                                                                                          | `linter`              | `bash`, `shell`                                                                                                                   |
| [shellharden](https://github.com/anordal/shellharden)                                | The corrective bash syntax highlighter                                                                                                        | `linter`              | `bash`, `shell`                                                                                                                   |
| [shfmt](https://github.com/mvdan/sh)                                                 | Shell script formatter                                                                                                                        | `formatter`           | `shell`                                                                                                                           |
| [sleek](https://github.com/nrempel/sleek)                                            | Sleek is a CLI tool for formatting SQL. It helps you maintain a consistent style across your SQL code, enhancing readability and productivity | `formatter`           | `sql`                                                                                                                             |
| [slim-lint](https://github.com/sds/slim-lint)                                        | Tool for analyzing Slim templates                                                                                                             | `linter`              | `slim`                                                                                                                            |
| [smlfmt](https://github.com/shwestrick/smlfmt)                                       | A custom parser/auto-formatter for Standard ML                                                                                                | `formatter`           | `standard-ml`                                                                                                                     |
| [snakefmt](https://github.com/snakemake/snakefmt)                                    | The uncompromising Snakemake code formatter                                                                                                   | `formatter`           | `snakemake`                                                                                                                       |
| [solhint](https://github.com/protofire/solhint)                                      | Solhint is an open-source project to provide a linting utility for Solidity code                                                              | `linter`              | `solidity`                                                                                                                        |
| [sphinx-lint](https://github.com/sphinx-contrib/sphinx-lint)                         | Check for stylistic and formal issues in .rst and .py files included in the documentation                                                     | `linter`              | `python`, `restructredtext`                                                                                                       |
| [sql-formatter](https://github.com/sql-formatter-org/sql-formatter)                  | A whitespace formatter for different query languages                                                                                          | `formatter`           | `sql`                                                                                                                             |
| [sqlfluff](https://github.com/sqlfluff/sqlfluff)                                     | A modular SQL linter and auto-formatter with support for multiple dialects and templated code                                                 | `formatter`, `linter` | `sql`                                                                                                                             |
| [sqlfmt](https://github.com/tconbeer/sqlfmt)                                         | sqlfmt formats your dbt SQL files so you don't have to                                                                                        | `formatter`           | `sql`                                                                                                                             |
| [sqruff](https://github.com/quarylabs/sqruff)                                        | Fast SQL formatter/linter                                                                                                                     | `formatter`, `linter` | `sql`                                                                                                                             |
| [standardjs](https://github.com/standard/standard)                                   | JavaScript style guide, linter, and formatter                                                                                                 | `formatter`, `linter` | `javascript`                                                                                                                      |
| [standardrb](https://github.com/standardrb/standard)                                 | Ruby's bikeshed-proof linter and formatter                                                                                                    | `formatter`, `linter` | `ruby`                                                                                                                            |
| [statix](https://github.com/oppiliappan/statix)                                      | lints and suggestions for the nix programming language                                                                                        | `linter`              | `nix`                                                                                                                             |
| [stylefmt](https://github.com/matype/stylefmt)                                       | stylefmt is a tool that automatically formats stylesheets                                                                                     | `formatter`           | `css`, `scss`                                                                                                                     |
| [stylelint](https://github.com/stylelint/stylelint)                                  | A mighty CSS linter that helps you avoid errors and enforce conventions                                                                       | `linter`              | `css`, `scss`                                                                                                                     |
| [stylish-haskell](https://github.com/haskell/stylish-haskell)                        | Haskell code prettifier                                                                                                                       | `formatter`           | `haskell`                                                                                                                         |
| [stylua](https://github.com/johnnymorganz/stylua)                                    | An opinionated Lua code formatter                                                                                                             | `formatter`           | `lua`                                                                                                                             |
| [superhtml](https://github.com/kristoff-it/superhtml)                                | HTML Language Server & Templating Language Library                                                                                            | `formatter`           | `html`                                                                                                                            |
| [swift-format](https://github.com/apple/swift-format)                                | Formatting technology for Swift source code                                                                                                   | `formatter`           | `swift`                                                                                                                           |
| [swiftformat](https://github.com/nicklockwood/swiftformat)                           | A command-line tool and Xcode Extension for formatting Swift code                                                                             | `formatter`           | `swift`                                                                                                                           |
| [taplo](https://github.com/tamasfe/taplo)                                            | A TOML toolkit written in Rust                                                                                                                | `formatter`           | `toml`                                                                                                                            |
| [templ](https://github.com/a-h/templ)                                                | Tooling for the Templ template language                                                                                                       | `formatter`           | `go`, `templ`                                                                                                                     |
| [terraform](https://www.terraform.io/docs/cli/commands/fmt.html)                     | The terraform fmt command is used to rewrite Terraform configuration files to a canonical format and style                                    | `formatter`           | `terraform`                                                                                                                       |
| [terragrunt](https://terragrunt.gruntwork.io/docs/reference/cli-options/#hclfmt)     | Recursively find hcl files and rewrite them into a canonical format                                                                           | `formatter`           | `hcl`                                                                                                                             |
| [tex-fmt](https://github.com/wgunderwood/tex-fmt)                                    | An extremely fast LaTeX formatter written in Rust                                                                                             | `formatter`           | `latex`                                                                                                                           |
| [textlint](https://github.com/textlint/textlint)                                     | The pluggable natural language linter for text and markdown                                                                                   | `spell-check`         |                                                                                                                                   |
| [tlint](https://github.com/tighten/tlint)                                            | Tighten linter for Laravel conventions                                                                                                        | `linter`              | `php`                                                                                                                             |
| [tofu](https://opentofu.org/docs/cli/commands/fmt/)                                  | The tofu fmt command is used to rewrite OpenTofu configuration files to a canonical format and style                                          | `formatter`           | `terraform`, `tofu`                                                                                                               |
| [toml-sort](https://github.com/pappasam/toml-sort)                                   | A command line utility to sort and format toml files                                                                                          | `formatter`           | `toml`                                                                                                                            |
| [topiary](https://github.com/tweag/topiary)                                          | Topiary aims to be a uniform formatter for simple languages, as part of the Tree-sitter ecosystem                                             | `formatter`           |                                                                                                                                   |
| [tryceratops](https://github.com/guilatrova/tryceratops)                             | A linter to prevent exception handling antipatterns in Python                                                                                 | `linter`              | `python`                                                                                                                          |
| [ts-standard](https://github.com/standard/ts-standard)                               | Typescript style guide, linter, and formatter using StandardJS                                                                                | `formatter`, `linter` | `typescript`                                                                                                                      |
| [tsqllint](https://github.com/tsqllint/tsqllint)                                     | Configurable linting for TSQL                                                                                                                 | `linter`              | `sql`                                                                                                                             |
| [twig-cs-fixer](https://github.com/vincentlanglet/twig-cs-fixer)                     | A tool to automatically fix Twig Coding Standards issues                                                                                      | `formatter`, `linter` | `twig`                                                                                                                            |
| [twigcs](https://github.com/friendsoftwig/twigcs)                                    | The missing checkstyle for twig                                                                                                               | `linter`              | `php`, `twig`                                                                                                                     |
| [typos](https://github.com/crate-ci/typos)                                           | Source code spell checker                                                                                                                     | `spell-check`         |                                                                                                                                   |
| [typstfmt](https://github.com/astrale-sharp/typstfmt)                                | Basic formatter for the Typst language                                                                                                        | `formatter`           | `typst`                                                                                                                           |
| [typstyle](https://github.com/enter-tainer/typstyle)                                 | Beautiful and reliable typst code formatter                                                                                                   | `formatter`           | `typst`                                                                                                                           |
| [ufmt](https://github.com/omnilib/ufmt)                                              | Safe, atomic formatting with black and usort                                                                                                  | `formatter`           | `python`                                                                                                                          |
| [uiua](https://github.com/uiua-lang/uiua)                                            | A stack-based array programming language                                                                                                      | `formatter`           | `uiua`                                                                                                                            |
| [unimport](https://github.com/hakancelikdev/unimport)                                | The ultimate linter and formatter for removing unused import statements in your code                                                          | `formatter`           | `python`                                                                                                                          |
| [usort](https://github.com/facebook/usort)                                           | Safe, minimal import sorting for Python projects                                                                                              | `formatter`           | `python`                                                                                                                          |
| [v](https://vlang.io/)                                                               | Tooling for V lang                                                                                                                            | `formatter`           | `v`                                                                                                                               |
| [vacuum](https://github.com/daveshanley/vacuum)                                      | vacuum is the worlds fastest OpenAPI 3, OpenAPI 2 / Swagger linter and quality analysis tool                                                  | `linter`              | `json`, `openapi`, `yaml`                                                                                                         |
| [veryl](https://github.com/veryl-lang/veryl)                                         | Veryl: A Modern Hardware Description Language                                                                                                 | `formatter`           | `veryl`                                                                                                                           |
| [vhdl-style-guide](https://github.com/jeremiah-c-leary/vhdl-style-guide)             | Style guide enforcement for VHDL                                                                                                              | `formatter`           | `vhdl`                                                                                                                            |
| [vint](https://github.com/vimjas/vint)                                               | Lint Vim script                                                                                                                               | `linter`              | `vimscript`                                                                                                                       |
| [wa](https://github.com/wa-lang/wa/)                                                 | Formatter for the wa programming language                                                                                                     | `formatter`           | `wa`                                                                                                                              |
| [wfindent](https://github.com/wvermin/findent)                                       | Indents and optionally converts Fortran program sources                                                                                       | `formatter`           | `fortran`                                                                                                                         |
| [write-good](https://github.com/btford/write-good)                                   | Naive linter for English prose                                                                                                                | `linter`              |                                                                                                                                   |
| [xmlformat](https://github.com/pamoller/xmlformatter)                                | Format and compress XML documents                                                                                                             | `formatter`           | `xml`                                                                                                                             |
| [xmllint](https://gnome.pages.gitlab.gnome.org/libxml2/xmllint.html)                 | XML linter                                                                                                                                    | `linter`              | `xml`                                                                                                                             |
| [xo](https://github.com/xojs/xo)                                                     | JavaScript/TypeScript linter (ESLint wrapper) with great defaults                                                                             | `linter`              | `javascript`, `typescript`                                                                                                        |
| [xq](https://github.com/sibprogrammer/xq)                                            | Command-line XML and HTML beautifier and content extractor                                                                                    | `formatter`           | `html`, `xml`                                                                                                                     |
| [yamlfix](https://github.com/lyz-code/yamlfix)                                       | A simple opinionated yaml formatter that keeps your comments                                                                                  | `formatter`           | `yaml`                                                                                                                            |
| [yamlfmt](https://github.com/google/yamlfmt)                                         | An extensible command line tool or library to format yaml files                                                                               | `formatter`           | `yaml`                                                                                                                            |
| [yamllint](https://github.com/adrienverge/yamllint)                                  | A linter for YAML files                                                                                                                       | `linter`              | `yaml`                                                                                                                            |
| [yapf](https://github.com/google/yapf)                                               | A formatter for Python files                                                                                                                  | `formatter`           | `python`                                                                                                                          |
| [yew-fmt](https://github.com/its-the-shrimp/yew-fmt)                                 | Code formatter for the Yew framework                                                                                                          | `formatter`           | `rust`                                                                                                                            |
| [yq](https://github.com/mikefarah/yq)                                                | yq is a portable command-line YAML, JSON, XML, CSV, TOML and properties processor                                                             | `formatter`           | `yaml`                                                                                                                            |
| [zig](https://ziglang.org/)                                                          | Reformat Zig source into canonical form                                                                                                       | `formatter`           | `zig`                                                                                                                             |
| [ziggy](https://ziggy-lang.io/)                                                      | Formats Ziggy documents and Ziggy schemas                                                                                                     | `formatter`           | `ziggy`                                                                                                                           |
| [zprint](https://github.com/kkinnear/zprint)                                         | Executables beautifully format Clojure and Clojurescript source code and s-expressions                                                        | `formatter`           | `clojure`, `clojurescript`                                                                                                        |

<!-- END_SECTION:supported-tools -->

### Commands

<!-- START_SECTION:supported-commands -->

`mdsf` currently supports 328 commands. Feel free to open an issue/pull-request if your favorite tool/command is missing! 😃

| Name                         | Command                                                                                 |
| ---------------------------- | --------------------------------------------------------------------------------------- |
| `actionlint`                 | `actionlint $PATH`                                                                      |
| `air:format`                 | `air format $PATH`                                                                      |
| `alejandra`                  | `alejandra --quiet $PATH`                                                               |
| `alex`                       | `alex --quiet $PATH`                                                                    |
| `ameba`                      | `ameba --fix $PATH`                                                                     |
| `ansible-lint`               | `ansible-lint $PATH`                                                                    |
| `asmfmt`                     | `asmfmt -w $PATH`                                                                       |
| `astyle`                     | `astyle --quiet $PATH`                                                                  |
| `auto-optional`              | `auto-optional $PATH`                                                                   |
| `autocorrect`                | `autocorrect --fix $PATH`                                                               |
| `autoflake`                  | `autoflake --quiet --in-place $PATH`                                                    |
| `autopep8`                   | `autopep8 --in-place $PATH`                                                             |
| `bashate`                    | `bashate $PATH`                                                                         |
| `beancount-black`            | `bean-black $PATH`                                                                      |
| `beautysh`                   | `beautysh $PATH`                                                                        |
| `bibtex-tidy`                | `bibtex-tidy -m $PATH`                                                                  |
| `bicep:format`               | `bicep format $PATH`                                                                    |
| `biome:check:unsafe`         | `biome check --write --unsafe $PATH`                                                    |
| `biome:check`                | `biome check --write $PATH`                                                             |
| `biome:format`               | `biome format --write $PATH`                                                            |
| `biome:lint:unsafe`          | `biome lint --write --unsafe $PATH`                                                     |
| `biome:lint`                 | `biome lint --write $PATH`                                                              |
| `black`                      | `black --quiet $PATH`                                                                   |
| `blade-formatter`            | `blade-formatter --write $PATH`                                                         |
| `blue`                       | `blue --quiet $PATH`                                                                    |
| `bpfmt`                      | `bpfmt -w $PATH`                                                                        |
| `brittany`                   | `brittany --write-mode=inplace $PATH`                                                   |
| `brunette`                   | `brunette --quiet $PATH`                                                                |
| `bsfmt`                      | `bsfmt $PATH --write`                                                                   |
| `bslint`                     | `bslint --fix $PATH`                                                                    |
| `buf:format`                 | `buf format --write $PATH`                                                              |
| `buf:lint`                   | `buf lint $PATH`                                                                        |
| `buildifier`                 | `buildifier $PATH`                                                                      |
| `cabal-fmt`                  | `cabal-fmt --inplace $PATH`                                                             |
| `cabal-prettify`             | `cabal-prettify $PATH`                                                                  |
| `cabal:format`               | `cabal format $PATH`                                                                    |
| `caddy:fmt`                  | `caddy fmt $PATH -w`                                                                    |
| `caramel:fmt`                | `caramel fmt $PATH`                                                                     |
| `cfn-lint`                   | `cfn-lint $PATH`                                                                        |
| `checkmake`                  | `checkmake $PATH`                                                                       |
| `clang-format`               | `clang-format -i $PATH`                                                                 |
| `clang-tidy`                 | `clang-tidy --fix $PATH`                                                                |
| `clj-kondo`                  | `clj-kondo --lint $PATH`                                                                |
| `cljfmt:fix`                 | `cljfmt fix $PATH`                                                                      |
| `cljstyle`                   | `cljstyle fix $PATH`                                                                    |
| `cmake-format`               | `cmake-format -i $PATH`                                                                 |
| `cmake-lint`                 | `cmake-lint $PATH`                                                                      |
| `codeql:query:format`        | `codeql query format -i $PATH`                                                          |
| `codespell`                  | `codespell $PATH --check-hidden --write-changes`                                        |
| `coffeelint`                 | `coffeelint -q $PATH`                                                                   |
| `cppcheck`                   | `cppcheck $PATH`                                                                        |
| `cpplint`                    | `cpplint --quiet $PATH`                                                                 |
| `crlfmt`                     | `crlfmt -w $PATH`                                                                       |
| `crystal:format`             | `crystal tool format $PATH`                                                             |
| `csharpier`                  | `dotnet csharpier $PATH`                                                                |
| `css-beautify`               | `css-beautify -r --type css -f $PATH`                                                   |
| `csscomb`                    | `csscomb -t $PATH`                                                                      |
| `csslint`                    | `csslint --quiet $PATH`                                                                 |
| `curlylint`                  | `curlylint -q $PATH`                                                                    |
| `d2:fmt`                     | `d2 fmt $PATH`                                                                          |
| `dart:fix`                   | `dart fix --apply $PATH`                                                                |
| `dart:format`                | `dart format $PATH`                                                                     |
| `dcm:fix`                    | `dcm fix $PATH`                                                                         |
| `dcm:format`                 | `dcm format $PATH`                                                                      |
| `deadnix`                    | `deadnix -q --edit $PATH`                                                               |
| `deno:fmt`                   | `deno fmt --quiet $PATH`                                                                |
| `deno:lint`                  | `deno lint --fix $PATH`                                                                 |
| `dfmt`                       | `dfmt -i $PATH`                                                                         |
| `dhall`                      | `dhall format $PATH`                                                                    |
| `djade`                      | `djade $PATH`                                                                           |
| `djlint`                     | `djlint $PATH --reformat`                                                               |
| `docformatter`               | `docformatter --in-place $PATH`                                                         |
| `dockerfmt`                  | `dockerfmt -w -n $PATH`                                                                 |
| `dockfmt`                    | `dockfmt fmt -w $PATH`                                                                  |
| `docstrfmt`                  | `docstrfmt $PATH`                                                                       |
| `doctoc`                     | `doctoc $PATH`                                                                          |
| `dotenv-linter:fix`          | `dotenv-linter fix $PATH`                                                               |
| `dprint:fmt`                 | `dprint fmt $PATH`                                                                      |
| `dscanner:fix`               | `dscanner fix $PATH`                                                                    |
| `dscanner:lint`              | `dscanner lint $PATH`                                                                   |
| `easy-coding-standard`       | `ecs check $PATH --fix --no-interaction`                                                |
| `efmt`                       | `efmt -w $PATH`                                                                         |
| `elm-format`                 | `elm-format --elm-version=0.19 --yes $PATH`                                             |
| `eradicate`                  | `eradicate --in-place $PATH`                                                            |
| `erb-formatter`              | `erb-format $PATH --write`                                                              |
| `erlfmt`                     | `erlfmt -w $PATH_STRING`                                                                |
| `eslint`                     | `eslint --fix $PATH`                                                                    |
| `fantomas`                   | `fantomas $PATH`                                                                        |
| `fish_indent`                | `fish_indent -w $PATH`                                                                  |
| `fixjson`                    | `fixjson -w $PATH`                                                                      |
| `floskell`                   | `floskell $PATH`                                                                        |
| `flynt`                      | `flynt $PATH`                                                                           |
| `fnlfmt`                     | `fnlfmt $PATH`                                                                          |
| `forge:fmt`                  | `forge fmt $PATH`                                                                       |
| `fortitude:check:fix:unsafe` | `fortitude check --quiet --no-respect-gitignore --fix --unsafe-fixes $PATH`             |
| `fortitude:check:fix`        | `fortitude check --quiet --no-respect-gitignore --fix $PATH`                            |
| `fortitude:check`            | `fortitude check --quiet --no-respect-gitignore $PATH`                                  |
| `fortran-linter`             | `fortran-linter -i $PATH`                                                               |
| `fourmolu`                   | `fourmolu -i $PATH`                                                                     |
| `fprettify`                  | `fprettify $PATH`                                                                       |
| `futhark:fmt`                | `futhark fmt $PATH`                                                                     |
| `gci`                        | `gci write --skip-generated --skip-vendor $PATH`                                        |
| `gdformat`                   | `gdformat $PATH`                                                                        |
| `gdlint`                     | `gdlint $PATH`                                                                          |
| `gersemi`                    | `gersemi -i -q $PATH`                                                                   |
| `gleam:format`               | `gleam format $PATH`                                                                    |
| `gluon:fmt`                  | `gluon fmt $PATH`                                                                       |
| `gofmt`                      | `gofmt -w $PATH`                                                                        |
| `gofumpt`                    | `gofumpt -w $PATH`                                                                      |
| `goimports-reviser`          | `goimports-reviser -format $PATH`                                                       |
| `goimports`                  | `goimports -w $PATH`                                                                    |
| `golangci-lint:fmt`          | `golangci-lint fmt $PATH`                                                               |
| `golangci-lint:run:fix`      | `golangci-lint run --fix $PATH`                                                         |
| `golangci-lint:run`          | `golangci-lint run $PATH`                                                               |
| `golines`                    | `golines -w $PATH`                                                                      |
| `google-java-format`         | `google-java-format -i $PATH`                                                           |
| `gospel`                     | `gospel $PATH`                                                                          |
| `grain:format`               | `grain format $PATH -o $PATH`                                                           |
| `hadolint`                   | `hadolint $PATH`                                                                        |
| `haml-lint`                  | `haml-lint --auto-correct $PATH`                                                        |
| `hclfmt`                     | `hclfmt -w $PATH`                                                                       |
| `hfmt`                       | `hfmt -w $PATH`                                                                         |
| `hindent`                    | `hindent $PATH`                                                                         |
| `hlint`                      | `hlint $PATH`                                                                           |
| `html-beautify`              | `html-beautify -r --type html -f $PATH`                                                 |
| `htmlbeautifier`             | `htmlbeautifier $PATH`                                                                  |
| `htmlhint`                   | `htmlhint $PATH`                                                                        |
| `hurlfmt`                    | `hurlfmt --in-place $PATH`                                                              |
| `imba:fmt`                   | `imba fmt -f $PATH`                                                                     |
| `inko:fmt`                   | `inko fmt $PATH`                                                                        |
| `isort`                      | `isort --quiet $PATH`                                                                   |
| `joker`                      | `joker --format --write $PATH`                                                          |
| `jq`                         | `jq `                                                                                   |
| `jqfmt`                      | `jqfmt `                                                                                |
| `js-beautify`                | `js-beautify -r --type js -f $PATH`                                                     |
| `json5format`                | `json5format -r $PATH`                                                                  |
| `jsona:format`               | `jsona format $PATH`                                                                    |
| `jsona:lint`                 | `jsona lint $PATH`                                                                      |
| `jsonlint`                   | `jsonlint -i $PATH`                                                                     |
| `jsonnet-lint`               | `jsonnet-lint $PATH`                                                                    |
| `jsonnetfmt`                 | `jsonnetfmt -i $PATH`                                                                   |
| `jsonpp`                     | `jsonpp -s`                                                                             |
| `juliaformatter.jl`          | `julia -E using JuliaFormatter;format_file(\"{$PATH_STRING}\")`                         |
| `just`                       | `just --fmt --unstable --justfile $PATH`                                                |
| `kcl:fmt`                    | `kcl fmt $PATH`                                                                         |
| `kcl:lint`                   | `kcl lint $PATH`                                                                        |
| `kdlfmt`                     | `kdlfmt format $PATH`                                                                   |
| `kdoc-formatter`             | `kdoc-formatter --quiet $PATH`                                                          |
| `ktfmt`                      | `ktfmt $PATH`                                                                           |
| `ktlint`                     | `ktlint --format --log-level=error $PATH`                                               |
| `kulala-fmt:check`           | `kulala-fmt check $PATH`                                                                |
| `kulala-fmt:format`          | `kulala-fmt format $PATH`                                                               |
| `leptosfmt`                  | `leptosfmt $PATH`                                                                       |
| `liquidsoap-prettier`        | `liquidsoap-prettier --write $PATH`                                                     |
| `luacheck`                   | `luacheck $PATH`                                                                        |
| `luaformatter`               | `lua-format -i $PATH`                                                                   |
| `mado:check`                 | `mado check $PATH`                                                                      |
| `mago:format`                | `mago format $PATH`                                                                     |
| `mago:lint:fix:unsafe`       | `mago lint --fix --potentially-unsafe --unsafe $PATH`                                   |
| `mago:lint:fix`              | `mago lint --fix $PATH`                                                                 |
| `mago:lint`                  | `mago lint $PATH`                                                                       |
| `markdownfmt`                | `markdownfmt -w $PATH`                                                                  |
| `markdownlint-cli2`          | `markdownlint-cli2 --fix $PATH`                                                         |
| `markdownlint`               | `markdownlint --fix $PATH`                                                              |
| `markuplint`                 | `markuplint --fix $PATH`                                                                |
| `md-padding`                 | `md-padding -i $PATH`                                                                   |
| `mdformat`                   | `mdformat $PATH`                                                                        |
| `mdsf:format`                | `mdsf format $PATH`                                                                     |
| `mdsf:verify`                | `mdsf verify $PATH`                                                                     |
| `mdslw`                      | `mdslw $PATH`                                                                           |
| `meson:fmt`                  | `meson fmt -i $PATH`                                                                    |
| `mise:fmt`                   | `mise fmt --stdin`                                                                      |
| `misspell`                   | `misspell -w $PATH`                                                                     |
| `mix:format`                 | `mix format $PATH`                                                                      |
| `mojo:format`                | `mojo format -q $PATH`                                                                  |
| `muon:fmt`                   | `muon fmt -i $PATH`                                                                     |
| `muon:lint`                  | `muon lint -i $PATH`                                                                    |
| `mypy`                       | `mypy $PATH`                                                                            |
| `nasmfmt`                    | `nasmfmt $PATH`                                                                         |
| `nginxbeautifier`            | `nginxbeautifier $PATH`                                                                 |
| `nginxfmt`                   | `nginxfmt $PATH`                                                                        |
| `nickel:format`              | `nickel format $PATH`                                                                   |
| `nimpretty`                  | `nimpretty $PATH`                                                                       |
| `nixfmt`                     | `nixfmt $PATH`                                                                          |
| `nixpkgs-fmt`                | `nixpkgs-fmt $PATH`                                                                     |
| `nomad:fmt`                  | `nomad fmt $PATH`                                                                       |
| `nph`                        | `nph $PATH`                                                                             |
| `npm-groovy-lint`            | `npm-groovy-lint --format $PATH`                                                        |
| `nufmt`                      | `nufmt $PATH`                                                                           |
| `ocamlformat`                | `ocamlformat --ignore-invalid-option --inplace --enable-outside-detected-project $PATH` |
| `ocp-indent`                 | `ocp-indent --inplace $PATH`                                                            |
| `odinfmt`                    | `odinfmt -w $PATH`                                                                      |
| `oelint-adv`                 | `oelint-adv --fix --nobackup --quiet $PATH`                                             |
| `opa:fmt`                    | `opa fmt $PATH -w`                                                                      |
| `ormolu`                     | `ormolu --mode inplace $PATH`                                                           |
| `oxlint`                     | `oxlint --fix $PATH`                                                                    |
| `packer:fix`                 | `packer fix $PATH`                                                                      |
| `packer:fmt`                 | `packer fmt $PATH`                                                                      |
| `packer:validate`            | `packer validate $PATH`                                                                 |
| `pasfmt`                     | `pasfmt $PATH`                                                                          |
| `perflint`                   | `perflint $PATH`                                                                        |
| `perltidy`                   | `perltidy -b $PATH`                                                                     |
| `pg_format`                  | `pg_format --inplace $PATH`                                                             |
| `php-cs-fixer:fix`           | `php-cs-fixer fix $PATH`                                                                |
| `phpcbf`                     | `phpcbf $PATH`                                                                          |
| `phpinsights:fix`            | `phpinsights fix $PATH --no-interaction --quiet`                                        |
| `pint`                       | `pint $PATH`                                                                            |
| `prettier`                   | `prettier --embedded-language-formatting off --log-level error --write $PATH`           |
| `pretty-php`                 | `pretty-php $PATH`                                                                      |
| `prettypst`                  | `prettypst $PATH`                                                                       |
| `prisma:format`              | `prisma format --schema={$PATH_STRING}`                                                 |
| `proselint`                  | `proselint $PATH`                                                                       |
| `protolint`                  | `protolint lint -fix $PATH`                                                             |
| `ptop`                       | `ptop $PATH $PATH`                                                                      |
| `pug-lint`                   | `pug-lint $PATH`                                                                        |
| `puppet-lint`                | `puppet-lint --fix $PATH`                                                               |
| `purs-tidy`                  | `purs-tidy format-in-place $PATH`                                                       |
| `purty`                      | `purty --write $PATH`                                                                   |
| `pycln`                      | `pycln --no-gitignore --quiet $PATH`                                                    |
| `pycodestyle`                | `pycodestyle $PATH`                                                                     |
| `pydoclint`                  | `pydoclint $PATH`                                                                       |
| `pydocstringformatter`       | `pydocstringformatter -w $PATH`                                                         |
| `pydocstyle`                 | `pydocstyle $PATH`                                                                      |
| `pyflakes`                   | `pyflakes $PATH`                                                                        |
| `pyink`                      | `pyink --quiet $PATH`                                                                   |
| `pylint`                     | `pylint --module-naming-style=any $PATH`                                                |
| `pyment`                     | `pyment -w $PATH`                                                                       |
| `pyupgrade`                  | `pyupgrade $PATH`                                                                       |
| `qmlfmt`                     | `qmlfmt -w $PATH`                                                                       |
| `quick-lint-js`              | `quick-lint-js $PATH`                                                                   |
| `raco:fmt`                   | `raco fmt -i $PATH`                                                                     |
| `reek`                       | `reek $PATH`                                                                            |
| `refmt`                      | `refmt --in-place $PATH`                                                                |
| `reformat-gherkin`           | `reformat-gherkin $PATH`                                                                |
| `refurb`                     | `refurb $PATH`                                                                          |
| `regal:fix`                  | `regal fix $PATH`                                                                       |
| `regal:lint`                 | `regal lint $PATH`                                                                      |
| `reorder-python-imports`     | `reorder-python-imports $PATH`                                                          |
| `rescript:format`            | `rescript format $PATH`                                                                 |
| `revive`                     | `revive $PATH`                                                                          |
| `roc:format`                 | `roc format $PATH`                                                                      |
| `rstfmt`                     | `rstfmt $PATH`                                                                          |
| `rubocop`                    | `rubocop --fix-layout --autocorrect --format quiet $PATH`                               |
| `rubyfmt`                    | `rubyfmt -i $PATH`                                                                      |
| `ruff:check`                 | `ruff check --fix --quiet $PATH`                                                        |
| `ruff:format`                | `ruff format --quiet $PATH`                                                             |
| `rufo`                       | `rufo --simple-exit $PATH`                                                              |
| `rune:fmt`                   | `rune fmt $PATH`                                                                        |
| `runic`                      | `runic --inplace $PATH`                                                                 |
| `rustfmt`                    | `rustfmt --edition 2021 --quiet $PATH`                                                  |
| `rustywind`                  | `rustywind --write $PATH`                                                               |
| `salt-lint`                  | `salt-lint $PATH`                                                                       |
| `scalafmt`                   | `scalafmt --quiet --mode any $PATH`                                                     |
| `scalariform`                | `scalariform $PATH`                                                                     |
| `selene`                     | `selene --no-summary --quiet $PATH`                                                     |
| `semistandard`               | `semistandard --fix --stdin`                                                            |
| `shellcheck`                 | `shellcheck $PATH`                                                                      |
| `shellharden`                | `shellharden --transform --replace $PATH`                                               |
| `shfmt`                      | `shfmt --write $PATH`                                                                   |
| `sleek`                      | `sleek $PATH`                                                                           |
| `slim-lint`                  | `slim-lint $PATH`                                                                       |
| `smlfmt`                     | `smlfmt --force $PATH`                                                                  |
| `snakefmt`                   | `snakefmt $PATH`                                                                        |
| `solhint`                    | `solhint --quiet --fix --noPrompt $PATH`                                                |
| `sphinx-lint`                | `sphinx-lint $PATH`                                                                     |
| `sql-formatter`              | `sql-formatter --fix $PATH`                                                             |
| `sqlfluff:fix`               | `sqlfluff fix --disable-progress-bar --nocolor --dialect ansi $PATH`                    |
| `sqlfluff:format`            | `sqlfluff format --disable-progress-bar --nocolor --dialect ansi $PATH`                 |
| `sqlfluff:lint`              | `sqlfluff lint --disable-progress-bar --nocolor --dialect ansi $PATH`                   |
| `sqlfmt`                     | `sqlfmt $PATH`                                                                          |
| `sqruff`                     | `sqruff fix --force $PATH`                                                              |
| `standardjs`                 | `standard --fix --stdin`                                                                |
| `standardrb`                 | `standardrb --fix $PATH`                                                                |
| `statix:check`               | `statix check $PATH`                                                                    |
| `statix:fix`                 | `statix fix $PATH`                                                                      |
| `stylefmt`                   | `stylefmt $PATH`                                                                        |
| `stylelint`                  | `stylelint --fix $PATH`                                                                 |
| `stylish-haskell`            | `stylish-haskell --inplace $PATH`                                                       |
| `stylua`                     | `stylua --verify $PATH`                                                                 |
| `superhtml:fmt`              | `superhtml fmt $PATH`                                                                   |
| `swift-format`               | `swift-format --in-place $PATH`                                                         |
| `swiftformat`                | `swiftformat --quiet $PATH`                                                             |
| `taplo`                      | `taplo format $PATH`                                                                    |
| `templ:fmt`                  | `templ fmt $PATH`                                                                       |
| `terraform:fmt`              | `terraform fmt -write=true $PATH`                                                       |
| `terragrunt:hclfmt`          | `terragrunt hclfmt --terragrunt-hclfmt-file $PATH`                                      |
| `tex-fmt`                    | `tex-fmt $PATH`                                                                         |
| `textlint:fix`               | `textlint --fix $PATH`                                                                  |
| `textlint`                   | `textlint $PATH`                                                                        |
| `tlint:format`               | `tlint format $PATH`                                                                    |
| `tofu:fmt`                   | `tofu fmt -write=true $PATH`                                                            |
| `toml-sort`                  | `toml-sort -i $PATH`                                                                    |
| `topiary`                    | `topiary format $PATH`                                                                  |
| `tryceratops`                | `tryceratops --autofix $PATH`                                                           |
| `ts-standard`                | `ts-standard --fix $PATH`                                                               |
| `tsqllint`                   | `tsqllint --fix $PATH`                                                                  |
| `twig-cs-fixer:lint`         | `twig-cs-fixer lint $PATH --fix --no-interaction --quiet`                               |
| `twigcs`                     | `twigcs $PATH`                                                                          |
| `typos`                      | `typos -w --no-ignore --hidden $PATH`                                                   |
| `typstfmt`                   | `typstfmt $PATH`                                                                        |
| `typstyle`                   | `typstyle -i $PATH`                                                                     |
| `ufmt`                       | `ufmt format $PATH`                                                                     |
| `uiua:fmt`                   | `uiua fmt $PATH`                                                                        |
| `unimport`                   | `unimport -r $PATH`                                                                     |
| `usort`                      | `usort format $PATH`                                                                    |
| `v:fmt`                      | `v fmt -w $PATH`                                                                        |
| `vacuum:lint`                | `vacuum lint $PATH`                                                                     |
| `veryl:fmt`                  | `veryl fmt $PATH`                                                                       |
| `vhdl-style-guide`           | `vsg -f $PATH --fix`                                                                    |
| `vint:neovim`                | `vint --enable-neovim $PATH`                                                            |
| `vint`                       | `vint $PATH`                                                                            |
| `wa:fmt`                     | `wa fmt $PATH`                                                                          |
| `wfindent`                   | `wfindent $PATH`                                                                        |
| `write-good`                 | `write-good $PATH`                                                                      |
| `xmlformat`                  | `xmlformat --overwrite $PATH`                                                           |
| `xmllint`                    | `xmllint --format $PATH --output $PATH`                                                 |
| `xo`                         | `xo --fix --stdin`                                                                      |
| `xq:html`                    | `xq --html`                                                                             |
| `xq`                         | `xq `                                                                                   |
| `yamlfix`                    | `yamlfix $PATH`                                                                         |
| `yamlfmt`                    | `yamlfmt -quiet $PATH`                                                                  |
| `yamllint`                   | `yamllint $PATH`                                                                        |
| `yapf`                       | `yapf --in-place $PATH`                                                                 |
| `yew-fmt`                    | `yew-fmt --edition 2021 $PATH`                                                          |
| `yq`                         | `yq --inplace $PATH`                                                                    |
| `zig:fmt`                    | `zig fmt $PATH`                                                                         |
| `ziggy:fmt`                  | `ziggy fmt $PATH`                                                                       |
| `zprint`                     | `zprint -w $PATH`                                                                       |

<!-- END_SECTION:supported-commands -->

## Shell completions

Shell completions can be generated using `mdsf completions <SHELL>`.

<!-- START_SECTION:completions-command-help -->

```
Generate shell completion

Usage: mdsf completions [OPTIONS] <SHELL>

Arguments:
  <SHELL>  [possible values: bash, elvish, fish, nushell, powershell, zsh]

Options:
      --log-level <LOG_LEVEL>  [possible values: trace, debug, info, warn, error, off]
  -h, --help                   Print help
  -V, --version                Print version
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

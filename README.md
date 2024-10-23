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

`mdsf` currently supports 61 tools. Feel free to open an issue/pull-request if your favorite tool is missing! 😃

| Formatter            | Description                                                                                                                                                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| alejandra            | The Uncompromising Nix Code Formatter - [https://github.com/kamadorueda/alejandra](https://github.com/kamadorueda/alejandra)                                                                                                    |
| ameba                | A static code analysis tool for Crystal - [https://github.com/crystal-ameba/ameba](https://github.com/crystal-ameba/ameba)                                                                                                      |
| asmfmt               | Go Assembler Formatter - [https://github.com/klauspost/asmfmt](https://github.com/klauspost/asmfmt)                                                                                                                             |
| astyle               | A Free, Fast, and Small Automatic Formatter for C, C++, C++/CLI, Objective-C, C#, and Java Source Code - [https://gitlab.com/saalen/astyle](https://gitlab.com/saalen/astyle)                                                   |
| auto_optional        | Adds the Optional type-hint to arguments where the default value is None - [https://pypi.org/project/auto-optional/](https://pypi.org/project/auto-optional/)                                                                   |
| autocorrect          | A linter and formatter to help you to improve copywriting, correct spaces, words, and punctuations between CJK (Chinese, Japanese, Korean) - [https://github.com/huacnlee/autocorrect](https://github.com/huacnlee/autocorrect) |
| autoflake            | Removes unused imports and unused variables as reported by pyflakes - [https://github.com/pycqa/autoflake](https://github.com/pycqa/autoflake)                                                                                  |
| autopep_8            | A tool that automatically formats Python code to conform to the PEP 8 style guid - [https://pypi.org/project/autopep8/](https://pypi.org/project/autopep8/)                                                                     |
| bean_black           | Opinionated code formatter, just like Python's black code formatter but for Beancount - [https://github.com/LaunchPlatform/beancount-black](https://github.com/LaunchPlatform/beancount-black)                                  |
| beautysh             | A Bash beautifier for the masses - [https://pypi.org/project/beautysh/](https://pypi.org/project/beautysh/)                                                                                                                     |
| bicep_format         | Bicep is a declarative language for describing and deploying Azure resources - [https://github.com/Azure/bicep](https://github.com/Azure/bicep)                                                                                 |
| biome_check          | One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)                                                                                                                                               |
| biome_format         | One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)                                                                                                                                               |
| biome_lint           | One toolchain for your web project - [https://biomejs.dev/](https://biomejs.dev/)                                                                                                                                               |
| black                | The uncompromising Python code formatter - [https://github.com/psf/black](https://github.com/psf/black)                                                                                                                         |
| blade_formatter      | An opinionated blade template formatter for Laravel that respects readability - [https://github.com/shufo/blade-formatter](https://github.com/shufo/blade-formatter)                                                            |
| blue                 | The slightly less uncompromising Python code formatter - [https://blue.readthedocs.io/en/latest/](https://blue.readthedocs.io/en/latest/)                                                                                       |
| bpfmt                | A formatter for Blueprint files - [https://source.android.com/docs/setup/reference/androidbp#formatter](https://source.android.com/docs/setup/reference/androidbp#formatter)                                                    |
| brittany             | A Haskell source code formatter - [https://github.com/lspitzner/brittany](https://github.com/lspitzner/brittany)                                                                                                                |
| brunette             | A best practice Python code formatter - [https://github.com/odwyersoftware/brunette](https://github.com/odwyersoftware/brunette)                                                                                                |
| bsfmt                | A code formatter for BrightScript and BrighterScript - [https://github.com/rokucommunity/brighterscript-formatter](https://github.com/rokucommunity/brighterscript-formatter)                                                   |
| buf                  | The best way of working with Protocol Buffers - [https://buf.build/docs/reference/cli/buf/format/](https://buf.build/docs/reference/cli/buf/format/)                                                                            |
| buildifier           | - []()                                                                                                                                                                                                                          |
| cabal_format         | Cabal is a system for building and packaging Haskell libraries and programs - [https://www.haskell.org/cabal/](https://www.haskell.org/cabal/)                                                                                  |
| caramel_fmt          | - []()                                                                                                                                                                                                                          |
| clang_format         | A tool to format C/C++/Java/JavaScript/JSON/Objective-C/Protobuf/C# code - [https://clang.llvm.org/docs/ClangFormat.html](https://clang.llvm.org/docs/ClangFormat.html)                                                         |
| clang_tidy           | - []()                                                                                                                                                                                                                          |
| cljfmt               | A tool for formatting Clojure code - [https://github.com/weavejester/cljfmt](https://github.com/weavejester/cljfmt)                                                                                                             |
| cljstyle             | A tool for formatting Clojure code - [https://github.com/greglook/cljstyle](https://github.com/greglook/cljstyle)                                                                                                               |
| codespell            | - []()                                                                                                                                                                                                                          |
| crlfmt               | - []()                                                                                                                                                                                                                          |
| crystal_format       | Tools for the Crystal programming language - [https://crystal-lang.org/](https://crystal-lang.org/)                                                                                                                             |
| css_beautify         | - []()                                                                                                                                                                                                                          |
| csscomb              | - []()                                                                                                                                                                                                                          |
| d_2                  | - []()                                                                                                                                                                                                                          |
| dart_fix             | - []()                                                                                                                                                                                                                          |
| dart_format          | - []()                                                                                                                                                                                                                          |
| dcm_fix              | - []()                                                                                                                                                                                                                          |
| dcm_format           | - []()                                                                                                                                                                                                                          |
| deno_fmt             | Formatter and linter for JavaScript and TypeScript - [https://docs.deno.com/runtime/reference/cli/](https://docs.deno.com/runtime/reference/cli/)                                                                               |
| deno_lint            | Formatter and linter for JavaScript and TypeScript - [https://docs.deno.com/runtime/reference/cli/](https://docs.deno.com/runtime/reference/cli/)                                                                               |
| dfmt                 | - []()                                                                                                                                                                                                                          |
| dhall                | Format Dhall files - [https://dhall-lang.org/](https://dhall-lang.org/)                                                                                                                                                         |
| djlint               | Lint & Format HTML Templates - [https://www.djlint.com/](https://www.djlint.com/)                                                                                                                                               |
| docformatter         | Formats docstrings to follow PEP 257 - [https://pypi.org/project/docformatter/](https://pypi.org/project/docformatter/)                                                                                                         |
| docstrfmt            | A formatter for Sphinx flavored reStructuredText - [https://pypi.org/project/docstrfmt/](https://pypi.org/project/docstrfmt/)                                                                                                   |
| dotenv_linter        | Lightning-fast linter for .env files - [https://github.com/dotenv-linter/dotenv-linter](https://github.com/dotenv-linter/dotenv-linter)                                                                                         |
| dotnet               | An Opinionated Code Formatter for C# - [https://csharpier.com/](https://csharpier.com/)                                                                                                                                         |
| dprint               | A pluggable and configurable code formatting platform written in Rust - [https://dprint.dev/](https://dprint.dev/)                                                                                                              |
| easy_coding_standard | The Easiest way to add coding standard to your PHP project - [https://github.com/easy-coding-standard/easy-coding-standard](https://github.com/easy-coding-standard/easy-coding-standard)                                       |
| efmt                 | Erlang code formatter - [https://github.com/sile/efmt](https://github.com/sile/efmt)                                                                                                                                            |
| elm_format           | elm-format formats Elm source code according to a standard set of rules based on the official Elm Style Guide - [https://github.com/avh4/elm-format](https://github.com/avh4/elm-format)                                        |
| eslint               | Find and fix problems in your JavaScript code - [https://github.com/eslint/eslint/](https://github.com/eslint/eslint/)                                                                                                          |
| gersemi              | A formatter to make your CMake code the real treasure - [https://github.com/blankspruce/gersemi](https://github.com/blankspruce/gersemi)                                                                                        |
| haml_lint            | Tool for writing clean and consistent HAML - [https://github.com/sds/haml-lint](https://github.com/sds/haml-lint)                                                                                                               |
| prettier             | Prettier is an opinionated code formatter - [https://github.com/prettier/prettier](https://github.com/prettier/prettier)                                                                                                        |
| yapf                 | - []()                                                                                                                                                                                                                          |
| yew_fmt              | - []()                                                                                                                                                                                                                          |
| zig                  | Reformat Zig source into canonical form - [https://ziglang.org/](https://ziglang.org/)                                                                                                                                          |
| ziggy                | Formats Ziggy documents and Ziggy schemas - [https://ziggy-lang.io/documentation/ziggy-fmt/](https://ziggy-lang.io/documentation/ziggy-fmt/)                                                                                    |
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

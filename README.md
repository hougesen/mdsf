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

### Commands

> \[!NOTE\]
> mdsf is not a package manager.
>
> Only tools that are already installed will be used.

<!-- START_SECTION:supported-tools -->

`mdsf` currently supports 205 commands. Feel free to open an issue/pull-request if your favorite tool is missing! 😃

| Name                     | Command                                                                                |
| ------------------------ | -------------------------------------------------------------------------------------- |
| `alejandra`              | `alejandra --quiet PATH`                                                               |
| `ameba`                  | `ameba --fix PATH`                                                                     |
| `asmfmt`                 | `asmfmt -w PATH`                                                                       |
| `astyle`                 | `astyle --quiet PATH`                                                                  |
| `auto-optional`          | `auto-optional PATH`                                                                   |
| `autocorrect`            | `autocorrect --fix PATH`                                                               |
| `autoflake`              | `autoflake --quiet --in-place PATH`                                                    |
| `autopep8`               | `autopep8 --in-place PATH`                                                             |
| `beancount-black`        | `bean-black PATH`                                                                      |
| `beautysh`               | `beautysh PATH`                                                                        |
| `bibtex-tidy`            | `bibtex-tidy -m PATH`                                                                  |
| `bicep:format`           | `bicep format PATH`                                                                    |
| `biome:check`            | `biome check --write PATH`                                                             |
| `biome:format`           | `biome format --write PATH`                                                            |
| `biome:lint`             | `biome lint --write PATH`                                                              |
| `black`                  | `black --quiet PATH`                                                                   |
| `blade-formatter`        | `blade-formatter --write PATH`                                                         |
| `blue`                   | `blue --quiet PATH`                                                                    |
| `bpfmt`                  | `bpfmt -w PATH`                                                                        |
| `brittany`               | `brittany --write-mode=inplace PATH`                                                   |
| `brunette`               | `brunette --quiet PATH`                                                                |
| `bsfmt`                  | `bsfmt PATH --write`                                                                   |
| `bslint`                 | `bslint --fix PATH`                                                                    |
| `buf:format`             | `buf format --write PATH`                                                              |
| `buildifier`             | `buildifier PATH`                                                                      |
| `cabal-fmt`              | `cabal-fmt --inplace PATH`                                                             |
| `cabal-prettify`         | `cabal-prettify PATH`                                                                  |
| `cabal:format`           | `cabal format PATH`                                                                    |
| `caddy:fmt`              | `caddy fmt PATH -w`                                                                    |
| `caramel:fmt`            | `caramel fmt PATH`                                                                     |
| `clang-format`           | `clang-format -i PATH`                                                                 |
| `clang-tidy`             | `clang-tidy --fix PATH`                                                                |
| `cljfmt:fix`             | `cljfmt fix PATH`                                                                      |
| `cljstyle`               | `cljstyle fix PATH`                                                                    |
| `codespell`              | `codespell PATH --check-hidden --write-changes`                                        |
| `crlfmt`                 | `crlfmt -w PATH`                                                                       |
| `crystal:format`         | `crystal tool format PATH`                                                             |
| `csharpier`              | `dotnet csharpier PATH`                                                                |
| `css-beautify`           | `css-beautify -r --type css -f PATH`                                                   |
| `csscomb`                | `csscomb -t PATH`                                                                      |
| `d2:fmt`                 | `d2 fmt PATH`                                                                          |
| `dart:fix`               | `dart fix --apply PATH`                                                                |
| `dart:format`            | `dart format PATH`                                                                     |
| `dcm:fix`                | `dcm fix PATH`                                                                         |
| `dcm:format`             | `dcm format PATH`                                                                      |
| `deno:fmt`               | `deno fmt --quiet PATH`                                                                |
| `deno:lint`              | `deno lint --fix PATH`                                                                 |
| `dfmt`                   | `dfmt -i PATH`                                                                         |
| `dhall`                  | `dhall format PATH`                                                                    |
| `djlint`                 | `djlint PATH --reformat`                                                               |
| `docformatter`           | `docformatter --in-place PATH`                                                         |
| `docstrfmt`              | `docstrfmt PATH`                                                                       |
| `doctoc`                 | `doctoc PATH`                                                                          |
| `dotenv-linter:fix`      | `dotenv-linter fix PATH`                                                               |
| `dprint:fmt`             | `dprint fmt PATH`                                                                      |
| `easy-coding-standard`   | `ecs check PATH --fix --no-interaction`                                                |
| `efmt`                   | `efmt -w PATH`                                                                         |
| `elm-format`             | `elm-format --elm-version=0.19 --yes PATH`                                             |
| `erb-formatter`          | `erb-format PATH --write`                                                              |
| `erlfmt`                 | `erlfmt -w PATH_STRING`                                                                |
| `eslint`                 | `eslint --fix PATH`                                                                    |
| `fantomas`               | `fantomas PATH`                                                                        |
| `fish_indent`            | `fish_indent -w PATH`                                                                  |
| `fixjson`                | `fixjson -w PATH`                                                                      |
| `floskell`               | `floskell PATH`                                                                        |
| `fnlfmt`                 | `fnlfmt PATH`                                                                          |
| `forge:fmt`              | `forge fmt PATH`                                                                       |
| `fourmolu`               | `fourmolu -i PATH`                                                                     |
| `fprettify`              | `fprettify PATH`                                                                       |
| `gci`                    | `gci write --skip-generated --skip-vender PATH`                                        |
| `gdformat`               | `gdformat PATH`                                                                        |
| `gersemi`                | `gersemi -i -q PATH`                                                                   |
| `gleam:format`           | `gleam format PATH`                                                                    |
| `gluon:fmt`              | `gluon fmt PATH`                                                                       |
| `gofmt`                  | `gofmt -w PATH`                                                                        |
| `gofumpt`                | `gofumpt -w PATH`                                                                      |
| `goimports-reviser`      | `goimports-reviser -format PATH`                                                       |
| `goimports`              | `goimports -w PATH`                                                                    |
| `golines`                | `golines -w PATH`                                                                      |
| `google-java-format`     | `google-java-format -i PATH`                                                           |
| `grain:format`           | `grain format PATH -o PATH`                                                            |
| `haml-lint`              | `haml-lint --auto-correct PATH`                                                        |
| `hfmt`                   | `hfmt -w PATH`                                                                         |
| `hindent`                | `hindent PATH`                                                                         |
| `hlint`                  | `hlint --refactor -i PATH`                                                             |
| `html-beautify`          | `html-beautify -r --type html -f PATH`                                                 |
| `htmlbeautifier`         | `htmlbeautifier PATH`                                                                  |
| `imba:fmt`               | `imba fmt -f PATH`                                                                     |
| `isort`                  | `isort --quiet PATH`                                                                   |
| `joker`                  | `joker --format --write PATH`                                                          |
| `js-beautify`            | `js-beautify -r --type js -f PATH`                                                     |
| `jsona:format`           | `jsona format PATH`                                                                    |
| `jsona:lint`             | `jsona lint PATH`                                                                      |
| `jsonlint`               | `jsonlint -i PATH`                                                                     |
| `jsonnetfmt`             | `jsonnetfmt -i PATH`                                                                   |
| `juliaformatter.jl`      | `julia -E using JuliaFormatter;format_file(\"\")`                                      |
| `just`                   | `just --fmt --unstable --justfile PATH`                                                |
| `kcl:fmt`                | `kcl fmt PATH`                                                                         |
| `kdlfmt`                 | `kdlfmt format PATH`                                                                   |
| `ktfmt`                  | `ktfmt --format --log-level=error PATH`                                                |
| `ktlint`                 | `ktlint --format --log-level=error PATH`                                               |
| `kulala-fmt`             | `kulala-fmt PATH`                                                                      |
| `leptosfmt`              | `leptosfmt --quiet PATH`                                                               |
| `liquidsoap-prettier`    | `liquidsoap-prettier --write PATH`                                                     |
| `luaformatter`           | `lua-format -i PATH`                                                                   |
| `markdownfmt`            | `markdownfmt -w PATH`                                                                  |
| `markdownlint-cli2`      | `markdownlint-cli2 --fix PATH`                                                         |
| `markdownlint`           | `markdownlint --fix PATH`                                                              |
| `markuplint`             | `markuplint --fix PATH`                                                                |
| `mdformat`               | `mdformat PATH`                                                                        |
| `mdslw`                  | `mdslw PATH`                                                                           |
| `misspell`               | `misspell -w PATH`                                                                     |
| `mix:format`             | `mix format PATH`                                                                      |
| `mojo:format`            | `mojo format -q PATH`                                                                  |
| `nginxbeautifier`        | `nginxbeautifier PATH`                                                                 |
| `nickel:format`          | `nickel format PATH`                                                                   |
| `nimpretty`              | `nimpretty PATH`                                                                       |
| `nixfmt`                 | `nixfmt PATH`                                                                          |
| `nixpkgs-fmt`            | `nixpkgs-fmt PATH`                                                                     |
| `nph`                    | `nph PATH`                                                                             |
| `npm-groovy-lint`        | `npm-groovy-lint --format PATH`                                                        |
| `ocamlformat`            | `ocamlformat --ignore-invalid-option --inplace --enable-outside-detected-project PATH` |
| `ocp-indent`             | `ocp-indent --inplace PATH`                                                            |
| `ormolu`                 | `ormolu --mode inplace PATH`                                                           |
| `oxlint`                 | `oxlint --fix PATH`                                                                    |
| `packer:fmt`             | `packer fmt PATH`                                                                      |
| `perltidy`               | `perltidy -b PATH`                                                                     |
| `pg_format`              | `pg_format --inplace PATH`                                                             |
| `php-cs-fixer:fix`       | `php-cs-fixer fix PATH`                                                                |
| `phpcbf`                 | `phpcbf PATH`                                                                          |
| `phpinsights:fix`        | `phpinsights fix PATH --no-interaction --quiet`                                        |
| `pint`                   | `pint PATH`                                                                            |
| `prettier`               | `prettier --embedded-language-formatting off --log-level error --write PATH`           |
| `pretty-php`             | `pretty-php PATH`                                                                      |
| `prettypst`              | `prettypst PATH`                                                                       |
| `protolint`              | `protolint lint -fix PATH`                                                             |
| `puppet-lint`            | `puppet-lint --fix PATH`                                                               |
| `purs-tidy`              | `purs-tidy format-in-place PATH`                                                       |
| `pycln`                  | `pycln --no-gitignore --quiet PATH`                                                    |
| `pyink`                  | `pyink --quiet PATH`                                                                   |
| `qmlfmt`                 | `qmlfmt -w PATH`                                                                       |
| `raco:fmt`               | `raco fmt -i PATH`                                                                     |
| `refmt`                  | `refmt --in-place PATH`                                                                |
| `reformat-gherkin`       | `reformat-gherkin PATH`                                                                |
| `reorder-python-imports` | `reorder-python-imports PATH`                                                          |
| `rescript:format`        | `rescript format PATH`                                                                 |
| `roc:format`             | `roc format PATH`                                                                      |
| `rstfmt`                 | `rstfmt PATH`                                                                          |
| `rubocop`                | `rubocop --fix-layout --autocorrect --format quiet PATH`                               |
| `rubyfmt`                | `rubyfmt -i PATH`                                                                      |
| `ruff:check`             | `ruff check --fix --quiet PATH`                                                        |
| `ruff:format`            | `ruff format --quiet PATH`                                                             |
| `rufo`                   | `rufo --simple-exit PATH`                                                              |
| `rune:fmt`               | `rune fmt PATH`                                                                        |
| `rustfmt`                | `rustfmt --edition 2021 --quiet PATH`                                                  |
| `rustywind`              | `rustywind --write PATH`                                                               |
| `scalafmt`               | `scalafmt --quiet --mode any PATH`                                                     |
| `scalariform`            | `scalariform PATH`                                                                     |
| `shellharden`            | `shellharden --transform --replace PATH`                                               |
| `shfmt`                  | `shfmt --write PATH`                                                                   |
| `sleek`                  | `sleek PATH`                                                                           |
| `smlfmt`                 | `smlfmt --force PATH`                                                                  |
| `snakefmt`               | `snakefmt PATH`                                                                        |
| `sql-formatter`          | `sql-formatter --fix PATH`                                                             |
| `sqlfluff:fix`           | `sqlfluff fix --dialect ansi PATH`                                                     |
| `sqlfluff:format`        | `sqlfluff format --dialect ansi PATH`                                                  |
| `sqlfmt`                 | `sqlfmt PATH`                                                                          |
| `standardjs`             | `standard --fix PATH`                                                                  |
| `standardrb`             | `standardrb --fix PATH`                                                                |
| `stylefmt`               | `stylefmt PATH`                                                                        |
| `stylelint`              | `stylelint --fix PATH`                                                                 |
| `stylish-haskell`        | `stylish-haskell --inplace PATH`                                                       |
| `stylua`                 | `stylua --verify PATH`                                                                 |
| `superhtml:fmt`          | `superhtml fmt PATH`                                                                   |
| `swift-format`           | `swift-format --in-place PATH`                                                         |
| `swiftformat`            | `swiftformat --quiet PATH`                                                             |
| `taplo`                  | `taplo format PATH`                                                                    |
| `templ:fmt`              | `templ fmt PATH`                                                                       |
| `terraform:fmt`          | `terraform fmt -write=true PATH`                                                       |
| `terragrunt:hclfmt`      | `terragrunt hclfmt --terragrunt-hclfmt-file PATH`                                      |
| `tlint:format`           | `tlint format PATH`                                                                    |
| `tofu:fmt`               | `tofu fmt -write=true PATH`                                                            |
| `topiary`                | `topiary format PATH`                                                                  |
| `ts-standard`            | `ts-standard --fix PATH`                                                               |
| `twig-cs-fixer:lint`     | `twig-cs-fixer lint PATH --fix --no-interaction --quiet`                               |
| `typos`                  | `typos -w --no-ignore --hidden PATH`                                                   |
| `typstfmt`               | `typstfmt PATH`                                                                        |
| `typstyle`               | `typstyle -i PATH`                                                                     |
| `ufmt`                   | `ufmt format PATH`                                                                     |
| `uiua:fmt`               | `uiua fmt PATH`                                                                        |
| `usort`                  | `usort format PATH`                                                                    |
| `v:fmt`                  | `v fmt -w PATH`                                                                        |
| `veryl:fmt`              | `veryl fmt PATH`                                                                       |
| `vhdl-style-guide`       | `vsg -f PATH --fix`                                                                    |
| `wfindent`               | `wfindent PATH`                                                                        |
| `xmlformat`              | `xmlformat --overwrite PATH`                                                           |
| `xmllint`                | `xmllint --format PATH --output PATH`                                                  |
| `xo`                     | `xo --fix PATH`                                                                        |
| `yamlfix`                | `yamlfix PATH`                                                                         |
| `yamlfmt`                | `yamlfmt -quiet PATH`                                                                  |
| `yapf`                   | `yapf --in-place PATH`                                                                 |
| `yew-fmt`                | `yew-fmt --edition 2021 --quiet PATH`                                                  |
| `zig:fmt`                | `zig fmt PATH`                                                                         |
| `ziggy:fmt`              | `ziggy fmt PATH`                                                                       |
| `zprint`                 | `zprint -w PATH`                                                                       |

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

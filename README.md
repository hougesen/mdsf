# mdsf

Format markdown code snippets using your favorite code formatters.

## Installation

The latest version of `mdsf` can be downloaded directly from [github.com/hougesen/mdsf/releases](https://github.com/hougesen/mdsf/releases).

### Cargo

Install using the [published crate](https://crates.io/crates/mdsf):

```sh
cargo install mdsf
```

or directly from source:

```sh
git clone git@github.com:hougesen/mdsf.git

cargo install --path ./mdsf --bin mdsf
```

## Usage

```sh
mdsf format <NAME_OF_FOLDER_OR_FOLDER>
```

## Configuration

The default configuration of `mdsf` aims to as sane as possible. For that reason the default formatter for each language is the one most people have installed.

If you are interested in customizing which formatter is run, you can create a new `mdsf` configuration file by running

```sh
mdsf init
```

`mdsf` supports running multiple formatters on the save code snippet.

```json
{
  // Only run `ruff` on Python snippets,
  "python": "ruff",
  // Run `usort` on file and then `black`
  "python": ["usort", "black"],
  // Run `usort`, if that fails run `isort`, finally run `black`
  "python": [["usort", "isort"], "black"]
}
```

### Supported languages & formatters

> \[!NOTE\]
> mdsf is not a tool for installing formatters.
>
> Only formatters that are already installed will be used.

<!-- START_SECTION:supported-languages -->

| Language   | Formatters                                                    |
| ---------- | ------------------------------------------------------------- |
| Blade      | `blade-formatter`                                             |
| C          | `clang-format`                                                |
| CSharp     | `clang-format`, `csharpier`                                   |
| Cabal      | `cabal_format`                                                |
| Clojure    | `cljstyle`                                                    |
| Cpp        | `clang-format`                                                |
| Crystal    | `crystal_format`                                              |
| Css        | `prettier`, `stylelint`                                       |
| Dart       | `dart_format`                                                 |
| Elixir     | `mix_format`                                                  |
| Elm        | `elm-format`                                                  |
| Erlang     | `efmt`, `erlfmt`                                              |
| FSharp     | `fantomas`                                                    |
| Fortran    | `fprettify`                                                   |
| Gleam      | `gleam_format`                                                |
| Go         | `gofmt`, `gofumpt`, `goimports`                               |
| GraphQL    | `prettier`                                                    |
| Groovy     | `npm-groovy-lint`                                             |
| Haskell    | `fourmolu`, `hindent`, `ormolu`, `stylish-haskell`            |
| Hcl        | `terraform_fmt`, `tofu_fmt`                                   |
| Html       | `prettier`                                                    |
| Java       | `clang-format`, `google-java-format`                          |
| JavaScript | `biome`, `clang-format`, `deno_fmt`, `prettier`, `standardjs` |
| Json       | `biome`, `clang-format`, `deno_fmt`, `prettier`               |
| Julia      | `juliaformatter.jl`                                           |
| Just       | `just_fmt`                                                    |
| Kcl        | `kcl_fmt`                                                     |
| Kotlin     | `ktfmt`, `ktlint`                                             |
| Lua        | `luaformatter`, `stylua`                                      |
| Markdown   | `prettier`                                                    |
| Nim        | `nimpretty`                                                   |
| Nix        | `alejandra`, `nixfmt`, `nixpkgs-fmt`                          |
| OCaml      | `ocamlformat`, `ocp-indent`                                   |
| ObjectiveC | `clang-format`                                                |
| Perl       | `perltidy`                                                    |
| Protobuf   | `buf`, `clang-format`                                         |
| PureScript | `purs-tidy`                                                   |
| Python     | `autopep8`, `black`, `blue`, `isort`, `ruff`, `usort`, `yapf` |
| ReScript   | `rescript_format`                                             |
| Roc        | `roc_format`                                                  |
| Ruby       | `rubocop`, `rubyfmt`, `rufo`, `standardrb`                    |
| Rust       | `rustfmt`                                                     |
| Scala      | `scalafmt`                                                    |
| Shell      | `beautysh`, `shfmt`                                           |
| Sql        | `sql-formatter`, `sqlfluff`                                   |
| Swift      | `swift-format`, `swiftformat`                                 |
| Toml       | `taplo`                                                       |
| TypeScript | `biome`, `deno_fmt`, `prettier`                               |
| Vue        | `prettier`                                                    |
| Xml        | `xmlformat`, `xmllint`                                        |
| Yaml       | `prettier`, `yamlfix`, `yamlfmt`                              |
| Zig        | `zigfmt`                                                      |

<!-- END_SECTION:supported-languages -->

## Acknowledgement

mdsf was inspired by the amazing neovim formatting plugin [conform.nvim](https://github.com/stevearc/conform.nvim).

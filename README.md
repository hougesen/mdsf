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

| Language    | Formatters                                                    |
| ----------- | ------------------------------------------------------------- |
| C           | `clang-format`                                                |
| Cpp         | `clang-format`                                                |
| Crystal     | `crystal_format`                                              |
| CSharp      | `clang-format`                                                |
| CSS         | `prettier`                                                    |
| Dart        | `dart_format`                                                 |
| Elixir      | `mix_format`                                                  |
| Gleam       | `gleam_format`                                                |
| Go          | `gofmt`, `gofumpt`                                            |
| HTML        | `prettier`                                                    |
| Java        | `clang-format`                                                |
| JavaScript  | `prettier`, `biome`, `clang-format`, `deno_fmt`               |
| JSON        | `prettier`, `biome`, `clang-format`, `deno_fmt`               |
| Lua         | `stylua`                                                      |
| Nim         | `nimpretty`                                                   |
| Objective C | `clang-format`                                                |
| Protobuf    | `clang-format`                                                |
| Python      | `ruff`, `black`, `blue`, `yapf`, `autopep8`, `isort`, `usort` |
| Roc         | `roc_format`                                                  |
| Ruby        | `rubocop`                                                     |
| Rust        | `rustfmt`                                                     |
| Shell       | `shfmt`                                                       |
| SQL         | `sqlfluff`, `sql-formatter`                                   |
| TOML        | `taplo`                                                       |
| TypeScript  | `prettier`, `biome`, `deno_fmt`                               |
| Vue         | `prettier`                                                    |
| YAML        | `prettier`                                                    |
| Zig         | `zigfmt`                                                      |

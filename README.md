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

### Supported languages & formatters

> \[!NOTE\]
> mdsf is not a tool for installing formatters.
>
> Only formatters that are already installed will be used.

| Language    | Formatters                                  |
| ----------- | ------------------------------------------- |
| C           | `clang-format`                              |
| CSS         | `prettier`                                  |
| Cpp         | `clang-format`                              |
| Crystal     | `crystal_format`                            |
| Dart        | `dart_format`                               |
| Elixir      | `mix_format`                                |
| Gleam       | `gleam_format`                              |
| Go          | `gofmt`, `gofumpt`                          |
| HTML        | `prettier`                                  |
| JSON        | `prettier`, `biome`, `clang-format`         |
| Java        | `clang-format`                              |
| JavaScript  | `prettier`, `biome`, `clang-format`         |
| Lua         | `stylua`                                    |
| Nim         | `nimpretty`                                 |
| Objective C | `clang-format`                              |
| Protobuf    | `clang-format`                              |
| Python      | `ruff`, `black`, `blue`, `yapf`, `autopep8` |
| Ruby        | `rubocop`                                   |
| Rust        | `rustfmt`                                   |
| SQL         | `sqlfluff`, `sql-formatter`                 |
| Shell       | `shfmt`                                     |
| TOML        | `taplo`                                     |
| TypeScript  | `prettier`, `biome`                         |
| Vue         | `prettier`                                  |
| YAML        | `prettier`                                  |
| Zig         | `zigfmt`                                    |

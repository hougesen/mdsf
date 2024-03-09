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

| Language   | Formatters          |
| ---------- | ------------------- |
| CSS        | `prettier`          |
| Elixir     | `mix_format`        |
| Go         | `gofmt`, `gofumpt`  |
| Gleam      | `gleam_format`      |
| HTML       | `prettier`          |
| JSON       | `prettier`, `biome` |
| JavaScript | `prettier`, `biome` |
| Lua        | `stylua`            |
| Nim        | `nimpretty`         |
| Python     | `ruff`              |
| Ruby       | `rubocop`           |
| Rust       | `rustfmt`           |
| Shell      | `shfmt`             |
| TOML       | `taplo`             |
| TypeScript | `prettier`, `biome` |
| Vue        | `prettier`          |
| YAML       | `prettier`          |
| Zig        | `zigfmt`            |

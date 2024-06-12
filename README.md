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

### Shell completions

Shell completions can be generated using `mdsf completions <SHELL>`.

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

| Language | Formatters                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tooling  | `alejandra`, `asmfmt`, `auto-optional`, `autocorrect`, `autopep8`, `beautysh`, `bicep_format`, `biome`, `black`, `blade-formatter`, `blue`, `bpfmt`, `buf`, `buildifier`, `cabal_format`, `clang-format`, `cljstyle`, `codespell`, `crlfmt`, `crystal_format`, `csharpier`, `dart_format`, `deno_fmt`, `dfmt`, `djlint`, `docstrfmt`, `efmt`, `elm-format`, `erb-formatter`, `erlfmt`, `fantomas`, `findent`, `fish_indent`, `fnlfmt`, `forge_fmt`, `fourmolu`, `fprettify`, `gci`, `gdformat`, `gleam_format`, `gofmt`, `gofumpt`, `goimports`, `goimports-reviser`, `golines`, `google-java-format`, `hindent`, `htmlbeautifier`, `isort`, `joker`, `juliaformatter.jl`, `just_fmt`, `kcl_fmt`, `ktfmt`, `ktlint`, `leptosfmt`, `luaformatter`, `mdformat`, `misspell`, `mix_format`, `nimpretty`, `nixfmt`, `nixpkgs-fmt`, `npm-groovy-lint`, `ocamlformat`, `ocp-indent`, `ormolu`, `perltidy`, `prettier`, `puppet-lint`, `purs-tidy`, `pyink`, `rescript_format`, `roc_format`, `rstfmt`, `rubocop`, `rubyfmt`, `ruff`, `rufo`, `rustfmt`, `scalafmt`, `shfmt`, `sql-formatter`, `sqlfluff`, `standardjs`, `standardrb`, `stylelint`, `stylish-haskell`, `stylua`, `swift-format`, `swiftformat`, `taplo`, `terraform_fmt`, `tofu_fmt`, `typos`, `usort`, `xmlformat`, `xmllint`, `yamlfix`, `yamlfmt`, `yapf`, `yew-fmt`, `zigfmt`, `zprint` |

<!-- END_SECTION:supported-languages -->

## Acknowledgement

mdsf was inspired by the amazing neovim formatting plugin [conform.nvim](https://github.com/stevearc/conform.nvim).

## Alternatives to mdsf

- [conform.nvim](https://github.com/stevearc/conform.nvim) using `injected` mode.
- [mdformat](https://github.com/executablebooks/mdformat).

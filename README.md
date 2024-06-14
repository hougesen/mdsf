# mdsf

Format markdown code snippets using your favorite code formatters.

## Installation

The latest version of `mdsf` can be downloaded directly from [github.com/hougesen/mdsf/releases](https://github.com/hougesen/mdsf/releases).

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

## Usage

```shell
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

```shell
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

### Supported tools

> \[!NOTE\]
> mdsf is not a package manager.
>
> Only tools that are already installed will be used.

<!-- START_SECTION:supported-languages -->

`mdsf` currently supports 105 tools.

| Formatter          | Description                                                                                                            |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------- |
| alejandra          | [https://github.com/kamadorueda/alejandra](https://github.com/kamadorueda/alejandra)                                   |
| asmfmt             | [https://github.com/klauspost/asmfmt](https://github.com/klauspost/asmfmt)                                             |
| auto-optional      | [https://pypi.org/project/auto-optional/](https://pypi.org/project/auto-optional/)                                     |
| autocorrect        | [https://github.com/huacnlee/autocorrect](https://github.com/huacnlee/autocorrect)                                     |
| autopep8           | [https://pypi.org/project/autopep8/](https://pypi.org/project/autopep8/)                                               |
| beautysh           | [https://pypi.org/project/beautysh/](https://pypi.org/project/beautysh/)                                               |
| bicep_format       | [https://github.com/Azure/bicep](https://github.com/Azure/bicep)                                                       |
| biome              | [https://biomejs.dev](https://biomejs.dev)                                                                             |
| black              | [https://github.com/psf/black](https://github.com/psf/black)                                                           |
| blade-formatter    | [https://github.com/shufo/blade-formatter](https://github.com/shufo/blade-formatter)                                   |
| blue               | [https://blue.readthedocs.io/en/latest/](https://blue.readthedocs.io/en/latest/)                                       |
| bpfmt              | [https://source.android.com/docs/setup/reference/androidbp](https://source.android.com/docs/setup/reference/androidbp) |
| buf                | [https://buf.build/docs/reference/cli/buf/format](https://buf.build/docs/reference/cli/buf/format)                     |
| buildifier         | [https://github.com/bazelbuild/buildtools](https://github.com/bazelbuild/buildtools)                                   |
| cabal_format       | [https://www.haskell.org/cabal/](https://www.haskell.org/cabal/)                                                       |
| clang-format       | [https://docs.kernel.org/process/clang-format.html](https://docs.kernel.org/process/clang-format.html)                 |
| cljstyle           | [https://github.com/greglook/cljstyle](https://github.com/greglook/cljstyle)                                           |
| codespell          | [https://github.com/codespell-project/codespell](https://github.com/codespell-project/codespell)                       |
| crlfmt             | [https://github.com/cockroachdb/crlfmt](https://github.com/cockroachdb/crlfmt)                                         |
| crystal_format     | [https://crystal-lang.org/](https://crystal-lang.org/)                                                                 |
| csharpier          | [https://csharpier.com/](https://csharpier.com/)                                                                       |
| dart_format        | [https://dart.dev/tools/dart-format](https://dart.dev/tools/dart-format)                                               |
| deno_fmt           | [https://dart.dev/tools/dart-format](https://dart.dev/tools/dart-format)                                               |
| dfmt               | [https://github.com/dlang-community/dfmt](https://github.com/dlang-community/dfmt)                                     |
| djlint             | [https://www.djlint.com/](https://www.djlint.com/)                                                                     |
| docstrfmt          | [https://pypi.org/project/docstrfmt/](https://pypi.org/project/docstrfmt/)                                             |
| efmt               | [https://github.com/sile/efmt](https://github.com/sile/efmt)                                                           |
| elm-format         | [https://github.com/avh4/elm-format](https://github.com/avh4/elm-format)                                               |
| erb-formatter      | [https://github.com/nebulab/erb-formatter](https://github.com/nebulab/erb-formatter)                                   |
| erlfmt             | [https://github.com/WhatsApp/erlfmt](https://github.com/WhatsApp/erlfmt)                                               |
| fantomas           | [https://github.com/fsprojects/fantomas](https://github.com/fsprojects/fantomas)                                       |
| findent            | [https://pypi.org/project/findent/](https://pypi.org/project/findent/)                                                 |
| fish_indent        | [https://fishshell.com/docs/current/cmds/fish_indent.html](https://fishshell.com/docs/current/cmds/fish_indent.html)   |
| fnlfmt             | [https://git.sr.ht/~technomancy/fnlfmt](https://git.sr.ht/~technomancy/fnlfmt)                                         |
| forge_fmt          | [https://docs.rs/forge-fmt/latest/forge_fmt/](https://docs.rs/forge-fmt/latest/forge_fmt/)                             |
| fourmolu           | [https://hackage.haskell.org/package/fourmolu](https://hackage.haskell.org/package/fourmolu)                           |
| fprettify          | [https://github.com/fortran-lang/fprettify](https://github.com/fortran-lang/fprettify)                                 |
| gci                | [https://github.com/daixiang0/gci](https://github.com/daixiang0/gci)                                                   |
| gdformat           | [https://godotengine.org/asset-library/asset/1057](https://godotengine.org/asset-library/asset/1057)                   |
| gleam_format       | [https://gleam.run/](https://gleam.run/)                                                                               |
| gofmt              | [https://pkg.go.dev/cmd/gofmt](https://pkg.go.dev/cmd/gofmt)                                                           |
| gofumpt            | [https://github.com/mvdan/gofumpt](https://github.com/mvdan/gofumpt)                                                   |
| goimports          | [https://pkg.go.dev/golang.org/x/tools/cmd/goimports](https://pkg.go.dev/golang.org/x/tools/cmd/goimports)             |
| goimports-reviser  | [https://github.com/incu6us/goimports-reviser](https://github.com/incu6us/goimports-reviser)                           |
| golines            | [https://github.com/segmentio/golines](https://github.com/segmentio/golines)                                           |
| google-java-format | [https://github.com/google/google-java-format](https://github.com/google/google-java-format)                           |
| hindent            | [https://hackage.haskell.org/package/hindent](https://hackage.haskell.org/package/hindent)                             |
| htmlbeautifier     | [https://github.com/threedaymonk/htmlbeautifier](https://github.com/threedaymonk/htmlbeautifier)                       |
| isort              | [https://pycqa.github.io/isort/](https://pycqa.github.io/isort/)                                                       |
| joker              | [https://github.com/candid82/joker](https://github.com/candid82/joker)                                                 |
| juliaformatter.jl  | [https://github.com/domluna/JuliaFormatter.jl](https://github.com/domluna/JuliaFormatter.jl)                           |
| just_fmt           | [https://github.com/casey/just](https://github.com/casey/just)                                                         |
| kcl_fmt            | [https://www.kcl-lang.io/docs/tools/cli/kcl/fmt](https://www.kcl-lang.io/docs/tools/cli/kcl/fmt)                       |
| kdlfmt             | [https://github.com/hougesen/kdlfmt](https://github.com/hougesen/kdlfmt)                                               |
| ktfmt              | [https://github.com/facebook/ktfmt](https://github.com/facebook/ktfmt)                                                 |
| ktlint             | [https://github.com/pinterest/ktlint](https://github.com/pinterest/ktlint)                                             |
| leptosfmt          | [https://github.com/bram209/leptosfmt](https://github.com/bram209/leptosfmt)                                           |
| luaformatter       | [https://github.com/Koihik/LuaFormatter](https://github.com/Koihik/LuaFormatter)                                       |
| mdformat           | [https://github.com/executablebooks/mdformat](https://github.com/executablebooks/mdformat)                             |
| misspell           | [https://github.com/client9/misspell/](https://github.com/client9/misspell/)                                           |
| mix_format         | [https://hexdocs.pm/mix/main/Mix.Tasks.Format.html](https://hexdocs.pm/mix/main/Mix.Tasks.Format.html)                 |
| nimpretty          | [https://github.com/nim-lang/nim](https://github.com/nim-lang/nim)                                                     |
| nixfmt             | [https://github.com/serokell/nixfmt](https://github.com/serokell/nixfmt)                                               |
| nixpkgs-fmt        | [https://github.com/nix-community/nixpkgs-fmt](https://github.com/nix-community/nixpkgs-fmt)                           |
| npm-groovy-lint    | [https://github.com/nvuillam/npm-groovy-lint](https://github.com/nvuillam/npm-groovy-lint)                             |
| ocamlformat        | [https://github.com/ocaml-ppx/ocamlformat](https://github.com/ocaml-ppx/ocamlformat)                                   |
| ocp-indent         | [https://github.com/OCamlPro/ocp-indent](https://github.com/OCamlPro/ocp-indent)                                       |
| ormolu             | [https://hackage.haskell.org/package/ormolu](https://hackage.haskell.org/package/ormolu)                               |
| perltidy           | [https://github.com/perltidy/perltidy](https://github.com/perltidy/perltidy)                                           |
| prettier           | [https://github.com/prettier/prettier](https://github.com/prettier/prettier)                                           |
| puppet-lint        | [https://github.com/puppetlabs/puppet-lint](https://github.com/puppetlabs/puppet-lint)                                 |
| purs-tidy          | [https://github.com/natefaubion/purescript-tidy](https://github.com/natefaubion/purescript-tidy)                       |
| pyink              | [https://github.com/google/pyink](https://github.com/google/pyink)                                                     |
| rescript_format    | [https://rescript-lang.org/](https://rescript-lang.org/)                                                               |
| roc_format         | [https://github.com/roc-lang/roc](https://github.com/roc-lang/roc)                                                     |
| rstfmt             | [https://github.com/dzhu/rstfmt](https://github.com/dzhu/rstfmt)                                                       |
| rubocop            | [https://github.com/rubocop/rubocop](https://github.com/rubocop/rubocop)                                               |
| rubyfmt            | [https://github.com/fables-tales/rubyfmt](https://github.com/fables-tales/rubyfmt)                                     |
| ruff               | [https://docs.astral.sh/ruff/](https://docs.astral.sh/ruff/)                                                           |
| rufo               | [https://github.com/ruby-formatter/rufo](https://github.com/ruby-formatter/rufo)                                       |
| rustfmt            | [https://github.com/rust-lang/rustfmt](https://github.com/rust-lang/rustfmt)                                           |
| scalafmt           | [https://github.com/scalameta/scalafmt](https://github.com/scalameta/scalafmt)                                         |
| shfmt              | [https://github.com/mvdan/sh](https://github.com/mvdan/sh)                                                             |
| sql-formatter      | [https://github.com/sql-formatter-org/sql-formatter](https://github.com/sql-formatter-org/sql-formatter)               |
| sqlfluff           | [https://github.com/sqlfluff/sqlfluff](https://github.com/sqlfluff/sqlfluff)                                           |
| standardjs         | [https://standardjs.com/](https://standardjs.com/)                                                                     |
| standardrb         | [https://github.com/standardrb/standard](https://github.com/standardrb/standard)                                       |
| stylelint          | [https://github.com/stylelint/stylelint](https://github.com/stylelint/stylelint)                                       |
| stylish-haskell    | [https://github.com/haskell/stylish-haskell](https://github.com/haskell/stylish-haskell)                               |
| stylua             | [https://github.com/JohnnyMorganz/StyLua](https://github.com/JohnnyMorganz/StyLua)                                     |
| swift-format       | [https://github.com/apple/swift-format](https://github.com/apple/swift-format)                                         |
| swiftformat        | [https://github.com/nicklockwood/SwiftFormat](https://github.com/nicklockwood/SwiftFormat)                             |
| taplo              | [https://github.com/tamasfe/taplo](https://github.com/tamasfe/taplo)                                                   |
| terraform_fmt      | [https://www.terraform.io/docs/cli/commands/fmt.html](https://www.terraform.io/docs/cli/commands/fmt.html)             |
| tofu_fmt           | [https://opentofu.org/docs/cli/commands/fmt/](https://opentofu.org/docs/cli/commands/fmt/)                             |
| typos              | [https://github.com/crate-ci/typos](https://github.com/crate-ci/typos)                                                 |
| usort              | [https://github.com/facebook/usort](https://github.com/facebook/usort)                                                 |
| xmlformat          | [https://github.com/pamoller/xmlformatter](https://github.com/pamoller/xmlformatter)                                   |
| xmllint            | [http://xmlsoft.org/xmllint.html](http://xmlsoft.org/xmllint.html)                                                     |
| yamlfix            | [https://github.com/lyz-code/yamlfix](https://github.com/lyz-code/yamlfix)                                             |
| yamlfmt            | [https://github.com/google/yamlfmt](https://github.com/google/yamlfmt)                                                 |
| yapf               | [https://github.com/google/yapf](https://github.com/google/yapf)                                                       |
| yew-fmt            | [https://github.com/its-the-shrimp/yew-fmt](https://github.com/its-the-shrimp/yew-fmt)                                 |
| zigfmt             | [https://ziglang.org/](https://ziglang.org/)                                                                           |
| zprint             | [https://github.com/kkinnear/zprint](https://github.com/kkinnear/zprint)                                               |

<!-- END_SECTION:supported-languages -->

## Acknowledgement

mdsf was inspired by the amazing neovim formatting plugin [conform.nvim](https://github.com/stevearc/conform.nvim).

## Alternatives to mdsf

- [conform.nvim](https://github.com/stevearc/conform.nvim) using `injected` mode.
- [mdformat](https://github.com/executablebooks/mdformat).

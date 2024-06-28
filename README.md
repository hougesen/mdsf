# mdsf

Format markdown code snippets using your favorite code formatters.

<!-- START_SECTION:base-command-help -->

```
mdsf 0.2.1
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

<!-- START_SECTION:format-command-help -->

```
Run formatters on input files

Usage: mdsf format [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to file or directory

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
mdsf verify <NAME_OF_FOLDER_OR_FOLDER>
```

<!-- START_SECTION:verify-command-help -->

```
Verify files are formatted

Usage: mdsf verify [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to file or directory

Options:
      --config <CONFIG>        Path to config
      --debug                  Log stdout and stderr of formatters
      --log-level <LOG_LEVEL>  [possible values: trace, debug, info, warn, error, off]
      --threads <THREADS>      Amount of threads to use. Defaults to 0 (auto)
  -h, --help                   Print help
  -V, --version                Print version
```

<!-- END_SECTION:verify-command-help -->

### Shell completions

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

<!-- START_SECTION:supported-tools -->

`mdsf` currently supports 166 tools.

| Formatter            | Description                                                                                                                  |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| alejandra            | [https://github.com/kamadorueda/alejandra](https://github.com/kamadorueda/alejandra)                                         |
| ameba                | [https://github.com/crystal-ameba/ameba](https://github.com/crystal-ameba/ameba)                                             |
| asmfmt               | [https://github.com/klauspost/asmfmt](https://github.com/klauspost/asmfmt)                                                   |
| astyle               | [https://astyle.sourceforge.net](https://astyle.sourceforge.net)                                                             |
| auto-optional        | [https://pypi.org/project/auto-optional/](https://pypi.org/project/auto-optional/)                                           |
| autocorrect          | [https://github.com/huacnlee/autocorrect](https://github.com/huacnlee/autocorrect)                                           |
| autoflake            | [https://github.com/pycqa/autoflake](https://github.com/pycqa/autoflake)                                                     |
| autopep8             | [https://pypi.org/project/autopep8/](https://pypi.org/project/autopep8/)                                                     |
| beautysh             | [https://pypi.org/project/beautysh/](https://pypi.org/project/beautysh/)                                                     |
| bicep_format         | [https://github.com/Azure/bicep](https://github.com/Azure/bicep)                                                             |
| biome                | [https://biomejs.dev](https://biomejs.dev)                                                                                   |
| biome_check          | [https://biomejs.dev](https://biomejs.dev)                                                                                   |
| biome_lint           | [https://biomejs.dev](https://biomejs.dev)                                                                                   |
| black                | [https://github.com/psf/black](https://github.com/psf/black)                                                                 |
| blade-formatter      | [https://github.com/shufo/blade-formatter](https://github.com/shufo/blade-formatter)                                         |
| blue                 | [https://blue.readthedocs.io/en/latest/](https://blue.readthedocs.io/en/latest/)                                             |
| bpfmt                | [https://source.android.com/docs/setup/reference/androidbp](https://source.android.com/docs/setup/reference/androidbp)       |
| brittany             | [https://github.com/lspitzner/brittany](https://github.com/lspitzner/brittany)                                               |
| bsfmt                | [https://github.com/rokucommunity/brighterscript-formatter](https://github.com/rokucommunity/brighterscript-formatter)       |
| buf                  | [https://buf.build/docs/reference/cli/buf/format](https://buf.build/docs/reference/cli/buf/format)                           |
| buildifier           | [https://github.com/bazelbuild/buildtools](https://github.com/bazelbuild/buildtools)                                         |
| cabal_format         | [https://www.haskell.org/cabal/](https://www.haskell.org/cabal/)                                                             |
| caramel_fmt          | [https://caramel.run](https://caramel.run)                                                                                   |
| clang-format         | [https://docs.kernel.org/process/clang-format.html](https://docs.kernel.org/process/clang-format.html)                       |
| clang-tidy           | [https://clang.llvm.org/extra/clang-tidy](https://clang.llvm.org/extra/clang-tidy)                                           |
| cljstyle             | [https://github.com/greglook/cljstyle](https://github.com/greglook/cljstyle)                                                 |
| codespell            | [https://github.com/codespell-project/codespell](https://github.com/codespell-project/codespell)                             |
| crlfmt               | [https://github.com/cockroachdb/crlfmt](https://github.com/cockroachdb/crlfmt)                                               |
| crystal_format       | [https://crystal-lang.org/](https://crystal-lang.org/)                                                                       |
| csharpier            | [https://csharpier.com/](https://csharpier.com/)                                                                             |
| css-beautify         | [https://github.com/beautifier/js-beautify](https://github.com/beautifier/js-beautify)                                       |
| d2                   | [https://d2lang.com/](https://d2lang.com/)                                                                                   |
| dart_fix             | [https://dart.dev/tools/dart-fix](https://dart.dev/tools/dart-fix)                                                           |
| dart_format          | [https://dart.dev/tools/dart-format](https://dart.dev/tools/dart-format)                                                     |
| dcm_fix              | [https://dcm.dev/docs/cli/fix/](https://dcm.dev/docs/cli/fix/)                                                               |
| dcm_format           | [https://dcm.dev/docs/cli/format/](https://dcm.dev/docs/cli/format/)                                                         |
| deno_fmt             | [https://docs.deno.com/runtime/manual/tools/formatter](https://docs.deno.com/runtime/manual/tools/formatter)                 |
| deno_lint            | [https://docs.deno.com/runtime/manual/tools/linter](https://docs.deno.com/runtime/manual/tools/linter)                       |
| dfmt                 | [https://github.com/dlang-community/dfmt](https://github.com/dlang-community/dfmt)                                           |
| dhall                | [https://dhall-lang.org/](https://dhall-lang.org/)                                                                           |
| djlint               | [https://www.djlint.com/](https://www.djlint.com/)                                                                           |
| docformatter         | [https://pypi.org/project/docformatter/](https://pypi.org/project/docformatter/)                                             |
| docstrfmt            | [https://pypi.org/project/docstrfmt/](https://pypi.org/project/docstrfmt/)                                                   |
| dotenv-linter        | [https://github.com/dotenv-linter/dotenv-linter](https://github.com/dotenv-linter/dotenv-linter)                             |
| dprint               | [https://dprint.dev](https://dprint.dev)                                                                                     |
| easy-cofing-standard | [https://github.com/easy-coding-standard/easy-coding-standard](https://github.com/easy-coding-standard/easy-coding-standard) |
| efmt                 | [https://github.com/sile/efmt](https://github.com/sile/efmt)                                                                 |
| elm-format           | [https://github.com/avh4/elm-format](https://github.com/avh4/elm-format)                                                     |
| erb-formatter        | [https://github.com/nebulab/erb-formatter](https://github.com/nebulab/erb-formatter)                                         |
| erlfmt               | [https://github.com/WhatsApp/erlfmt](https://github.com/WhatsApp/erlfmt)                                                     |
| eslint               | [https://eslint.org](https://eslint.org)                                                                                     |
| fantomas             | [https://github.com/fsprojects/fantomas](https://github.com/fsprojects/fantomas)                                             |
| findent              | [https://pypi.org/project/findent/](https://pypi.org/project/findent/)                                                       |
| fish_indent          | [https://fishshell.com/docs/current/cmds/fish_indent.html](https://fishshell.com/docs/current/cmds/fish_indent.html)         |
| fixjson              | [https://github.com/rhysd/fixjson](https://github.com/rhysd/fixjson)                                                         |
| floskell             | [https://github.com/ennocramer/floskell](https://github.com/ennocramer/floskell)                                             |
| fnlfmt               | [https://git.sr.ht/~technomancy/fnlfmt](https://git.sr.ht/~technomancy/fnlfmt)                                               |
| forge_fmt            | [https://docs.rs/forge-fmt/latest/forge_fmt/](https://docs.rs/forge-fmt/latest/forge_fmt/)                                   |
| fourmolu             | [https://hackage.haskell.org/package/fourmolu](https://hackage.haskell.org/package/fourmolu)                                 |
| fprettify            | [https://github.com/fortran-lang/fprettify](https://github.com/fortran-lang/fprettify)                                       |
| gci                  | [https://github.com/daixiang0/gci](https://github.com/daixiang0/gci)                                                         |
| gdformat             | [https://github.com/scony/godot-gdscript-toolkit](https://github.com/scony/godot-gdscript-toolkit)                           |
| gersemi              | [https://github.com/blankspruce/gersemi](https://github.com/blankspruce/gersemi)                                             |
| gleam_format         | [https://gleam.run/](https://gleam.run/)                                                                                     |
| gluon_fmt            | [https://github.com/gluon-lang/gluon](https://github.com/gluon-lang/gluon)                                                   |
| gofmt                | [https://pkg.go.dev/cmd/gofmt](https://pkg.go.dev/cmd/gofmt)                                                                 |
| gofumpt              | [https://github.com/mvdan/gofumpt](https://github.com/mvdan/gofumpt)                                                         |
| goimports            | [https://pkg.go.dev/golang.org/x/tools/cmd/goimports](https://pkg.go.dev/golang.org/x/tools/cmd/goimports)                   |
| goimports-reviser    | [https://github.com/incu6us/goimports-reviser](https://github.com/incu6us/goimports-reviser)                                 |
| golines              | [https://github.com/segmentio/golines](https://github.com/segmentio/golines)                                                 |
| google-java-format   | [https://github.com/google/google-java-format](https://github.com/google/google-java-format)                                 |
| grain_format         | [https://grain-lang.org](https://grain-lang.org)                                                                             |
| haml-lint            | [https://github.com/sds/haml-lint](https://github.com/sds/haml-lint)                                                         |
| hfmt                 | [https://github.com/danstiner/hfmt](https://github.com/danstiner/hfmt)                                                       |
| hindent              | [https://hackage.haskell.org/package/hindent](https://hackage.haskell.org/package/hindent)                                   |
| html-beautify        | [https://github.com/beautifier/js-beautify](https://github.com/beautifier/js-beautify)                                       |
| htmlbeautifier       | [https://github.com/threedaymonk/htmlbeautifier](https://github.com/threedaymonk/htmlbeautifier)                             |
| imba_fmt             | [https://github.com/imba/imba](https://github.com/imba/imba)                                                                 |
| isort                | [https://pycqa.github.io/isort/](https://pycqa.github.io/isort/)                                                             |
| joker                | [https://github.com/candid82/joker](https://github.com/candid82/joker)                                                       |
| js-beautify          | [https://github.com/beautifier/js-beautify](https://github.com/beautifier/js-beautify)                                       |
| jsona_format         | [https://github.com/jsona/jsona](https://github.com/jsona/jsona)                                                             |
| jsonnetfmt           | [https://jsonnet.org/learning/tools.html](https://jsonnet.org/learning/tools.html)                                           |
| juliaformatter.jl    | [https://github.com/domluna/JuliaFormatter.jl](https://github.com/domluna/JuliaFormatter.jl)                                 |
| just_fmt             | [https://github.com/casey/just](https://github.com/casey/just)                                                               |
| kcl_fmt              | [https://www.kcl-lang.io/docs/tools/cli/kcl/fmt](https://www.kcl-lang.io/docs/tools/cli/kcl/fmt)                             |
| kdlfmt               | [https://github.com/hougesen/kdlfmt](https://github.com/hougesen/kdlfmt)                                                     |
| ktfmt                | [https://github.com/facebook/ktfmt](https://github.com/facebook/ktfmt)                                                       |
| ktlint               | [https://github.com/pinterest/ktlint](https://github.com/pinterest/ktlint)                                                   |
| leptosfmt            | [https://github.com/bram209/leptosfmt](https://github.com/bram209/leptosfmt)                                                 |
| liquidsoap-prettier  | [https://github.com/savonet/liquidsoap-prettier](https://github.com/savonet/liquidsoap-prettier)                             |
| luaformatter         | [https://github.com/Koihik/LuaFormatter](https://github.com/Koihik/LuaFormatter)                                             |
| markdownlint         | [https://github.com/davidanson/markdownlint](https://github.com/davidanson/markdownlint)                                     |
| markuplint           | [https://markuplint.dev](https://markuplint.dev)                                                                             |
| mdformat             | [https://github.com/executablebooks/mdformat](https://github.com/executablebooks/mdformat)                                   |
| misspell             | [https://github.com/client9/misspell/](https://github.com/client9/misspell/)                                                 |
| mix_format           | [https://hexdocs.pm/mix/main/Mix.Tasks.Format.html](https://hexdocs.pm/mix/main/Mix.Tasks.Format.html)                       |
| nickel_format        | [https://nickel-lang.org](https://nickel-lang.org)                                                                           |
| nimpretty            | [https://github.com/nim-lang/nim](https://github.com/nim-lang/nim)                                                           |
| nixfmt               | [https://github.com/serokell/nixfmt](https://github.com/serokell/nixfmt)                                                     |
| nixpkgs-fmt          | [https://github.com/nix-community/nixpkgs-fmt](https://github.com/nix-community/nixpkgs-fmt)                                 |
| npm-groovy-lint      | [https://github.com/nvuillam/npm-groovy-lint](https://github.com/nvuillam/npm-groovy-lint)                                   |
| ocamlformat          | [https://github.com/ocaml-ppx/ocamlformat](https://github.com/ocaml-ppx/ocamlformat)                                         |
| ocp-indent           | [https://github.com/OCamlPro/ocp-indent](https://github.com/OCamlPro/ocp-indent)                                             |
| ormolu               | [https://hackage.haskell.org/package/ormolu](https://hackage.haskell.org/package/ormolu)                                     |
| oxlint               | [https://oxc.rs](https://oxc.rs)                                                                                             |
| perltidy             | [https://github.com/perltidy/perltidy](https://github.com/perltidy/perltidy)                                                 |
| pg_format            | [https://github.com/darold/pgFormatter](https://github.com/darold/pgFormatter)                                               |
| php-cs-fixer         | [https://github.com/PHP-CS-Fixer/PHP-CS-Fixer](https://github.com/PHP-CS-Fixer/PHP-CS-Fixer)                                 |
| phpcbf               | [https://phpqa.io/projects/phpcbf.html](https://phpqa.io/projects/phpcbf.html)                                               |
| phpinsights          | [https://phpinsights.com/](https://phpinsights.com/)                                                                         |
| pint                 | [https://github.com/laravel/pint](https://github.com/laravel/pint)                                                           |
| prettier             | [https://github.com/prettier/prettier](https://github.com/prettier/prettier)                                                 |
| pretty-php           | [https://github.com/lkrms/pretty-php](https://github.com/lkrms/pretty-php)                                                   |
| prisma               | [https://prisma.io/](https://prisma.io/)                                                                                     |
| puppet-lint          | [https://github.com/puppetlabs/puppet-lint](https://github.com/puppetlabs/puppet-lint)                                       |
| purs-tidy            | [https://github.com/natefaubion/purescript-tidy](https://github.com/natefaubion/purescript-tidy)                             |
| pycln                | [https://github.com/hadialqattan/pycln](https://github.com/hadialqattan/pycln)                                               |
| pyink                | [https://github.com/google/pyink](https://github.com/google/pyink)                                                           |
| raco_fmt             | [https://docs.racket-lang.org/fmt/](https://docs.racket-lang.org/fmt/)                                                       |
| rescript_format      | [https://rescript-lang.org/](https://rescript-lang.org/)                                                                     |
| roc_format           | [https://github.com/roc-lang/roc](https://github.com/roc-lang/roc)                                                           |
| rstfmt               | [https://github.com/dzhu/rstfmt](https://github.com/dzhu/rstfmt)                                                             |
| rubocop              | [https://github.com/rubocop/rubocop](https://github.com/rubocop/rubocop)                                                     |
| rubyfmt              | [https://github.com/fables-tales/rubyfmt](https://github.com/fables-tales/rubyfmt)                                           |
| ruff                 | [https://docs.astral.sh/ruff/formatter/](https://docs.astral.sh/ruff/formatter/)                                             |
| ruff_check           | [https://docs.astral.sh/ruff/linter/](https://docs.astral.sh/ruff/linter/)                                                   |
| rufo                 | [https://github.com/ruby-formatter/rufo](https://github.com/ruby-formatter/rufo)                                             |
| rune_fmt             | [https://github.com/rune-rs/rune](https://github.com/rune-rs/rune)                                                           |
| rustfmt              | [https://github.com/rust-lang/rustfmt](https://github.com/rust-lang/rustfmt)                                                 |
| rustywind            | [https://github.com/avencera/rustywind](https://github.com/avencera/rustywind)                                               |
| scalafmt             | [https://github.com/scalameta/scalafmt](https://github.com/scalameta/scalafmt)                                               |
| shfmt                | [https://github.com/mvdan/sh](https://github.com/mvdan/sh)                                                                   |
| smlfmt               | [https://github.com/shwestrick/smlfmt](https://github.com/shwestrick/smlfmt)                                                 |
| snakefmt             | [https://github.com/snakemake/snakefmt](https://github.com/snakemake/snakefmt)                                               |
| sql-formatter        | [https://github.com/sql-formatter-org/sql-formatter](https://github.com/sql-formatter-org/sql-formatter)                     |
| sqlfluff             | [https://github.com/sqlfluff/sqlfluff](https://github.com/sqlfluff/sqlfluff)                                                 |
| sqlfmt               | [https://sqlfmt.com](https://sqlfmt.com)                                                                                     |
| standardjs           | [https://standardjs.com/](https://standardjs.com/)                                                                           |
| standardrb           | [https://github.com/standardrb/standard](https://github.com/standardrb/standard)                                             |
| stylelint            | [https://github.com/stylelint/stylelint](https://github.com/stylelint/stylelint)                                             |
| stylish-haskell      | [https://github.com/haskell/stylish-haskell](https://github.com/haskell/stylish-haskell)                                     |
| stylua               | [https://github.com/JohnnyMorganz/StyLua](https://github.com/JohnnyMorganz/StyLua)                                           |
| swift-format         | [https://github.com/apple/swift-format](https://github.com/apple/swift-format)                                               |
| swiftformat          | [https://github.com/nicklockwood/SwiftFormat](https://github.com/nicklockwood/SwiftFormat)                                   |
| taplo                | [https://github.com/tamasfe/taplo](https://github.com/tamasfe/taplo)                                                         |
| templ                | [https://templ.guide](https://templ.guide)                                                                                   |
| terraform_fmt        | [https://www.terraform.io/docs/cli/commands/fmt.html](https://www.terraform.io/docs/cli/commands/fmt.html)                   |
| tlint                | [https://github.com/tighten/tlint](https://github.com/tighten/tlint)                                                         |
| tofu_fmt             | [https://opentofu.org/docs/cli/commands/fmt/](https://opentofu.org/docs/cli/commands/fmt/)                                   |
| ts-standard          | [https://github.com/standard/ts-standard](https://github.com/standard/ts-standard)                                           |
| twig-cs-fixer        | [https://github.com/VincentLanglet/Twig-CS-Fixer](https://github.com/VincentLanglet/Twig-CS-Fixer)                           |
| typos                | [https://github.com/crate-ci/typos](https://github.com/crate-ci/typos)                                                       |
| uiua_fmt             | [https://github.com/uiua-lang/uiua](https://github.com/uiua-lang/uiua)                                                       |
| usort                | [https://github.com/facebook/usort](https://github.com/facebook/usort)                                                       |
| veryl_fmt            | [https://github.com/veryl-lang/veryl](https://github.com/veryl-lang/veryl)                                                   |
| vlang_fmt            | [https://vlang.io](https://vlang.io)                                                                                         |
| xmlformat            | [https://github.com/pamoller/xmlformatter](https://github.com/pamoller/xmlformatter)                                         |
| xmllint              | [http://xmlsoft.org/xmllint.html](http://xmlsoft.org/xmllint.html)                                                           |
| xo                   | [http://github.com/xojs/xo](http://github.com/xojs/xo)                                                                       |
| yamlfix              | [https://github.com/lyz-code/yamlfix](https://github.com/lyz-code/yamlfix)                                                   |
| yamlfmt              | [https://github.com/google/yamlfmt](https://github.com/google/yamlfmt)                                                       |
| yapf                 | [https://github.com/google/yapf](https://github.com/google/yapf)                                                             |
| yew-fmt              | [https://github.com/its-the-shrimp/yew-fmt](https://github.com/its-the-shrimp/yew-fmt)                                       |
| zigfmt               | [https://ziglang.org/](https://ziglang.org/)                                                                                 |
| zprint               | [https://github.com/kkinnear/zprint](https://github.com/kkinnear/zprint)                                                     |

<!-- END_SECTION:supported-tools -->

## Acknowledgement

mdsf was inspired by the amazing neovim formatting plugin [conform.nvim](https://github.com/stevearc/conform.nvim).

## Alternatives to mdsf

- [conform.nvim](https://github.com/stevearc/conform.nvim) using `injected` mode.
- [mdformat](https://github.com/executablebooks/mdformat).

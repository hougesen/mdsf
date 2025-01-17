///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
pub mod actionlint;
pub mod alejandra;
pub mod ameba;
pub mod ansible_lint;
pub mod asmfmt;
pub mod astyle;
pub mod auto_optional;
pub mod autocorrect;
pub mod autoflake;
pub mod autopep_8;
pub mod beancount_black;
pub mod beautysh;
pub mod bibtex_tidy;
pub mod bicep_format;
pub mod biome_check;
pub mod biome_format;
pub mod biome_lint;
pub mod black;
pub mod blade_formatter;
pub mod blue;
pub mod bpfmt;
pub mod brittany;
pub mod brunette;
pub mod bsfmt;
pub mod bslint;
pub mod buf_format;
pub mod buildifier;
pub mod cabal_fmt;
pub mod cabal_format;
pub mod cabal_prettify;
pub mod caddy_fmt;
pub mod caramel_fmt;
pub mod clang_format;
pub mod clang_tidy;
pub mod cljfmt_fix;
pub mod cljstyle;
pub mod cmake_format;
pub mod codespell;
pub mod crlfmt;
pub mod crystal_format;
pub mod csharpier;
pub mod css_beautify;
pub mod csscomb;
pub mod csslint;
pub mod curlylint;
pub mod d_2_fmt;
pub mod dart_fix;
pub mod dart_format;
pub mod dcm_fix;
pub mod dcm_format;
pub mod deadnix;
pub mod deno_fmt;
pub mod deno_lint;
pub mod dfmt;
pub mod dhall;
pub mod djade;
pub mod djlint;
pub mod docformatter;
pub mod dockfmt;
pub mod docstrfmt;
pub mod doctoc;
pub mod dotenv_linter_fix;
pub mod dprint_fmt;
pub mod easy_coding_standard;
pub mod efmt;
pub mod elm_format;
pub mod erb_formatter;
pub mod erlfmt;
pub mod eslint;
pub mod fantomas;
pub mod fish_indent;
pub mod fixjson;
pub mod floskell;
pub mod fnlfmt;
pub mod forge_fmt;
pub mod fourmolu;
pub mod fprettify;
pub mod futhark_fmt;
pub mod gci;
pub mod gdformat;
pub mod gersemi;
pub mod gleam_format;
pub mod gluon_fmt;
pub mod gofmt;
pub mod gofumpt;
pub mod goimports;
pub mod goimports_reviser;
pub mod golines;
pub mod google_java_format;
pub mod grain_format;
pub mod hadolint;
pub mod haml_lint;
pub mod hclfmt;
pub mod hfmt;
pub mod hindent;
pub mod hlint;
pub mod html_beautify;
pub mod htmlbeautifier;
pub mod htmlhint;
pub mod imba_fmt;
pub mod inko_fmt;
pub mod isort;
pub mod joker;
pub mod js_beautify;
pub mod json_5_format;
pub mod jsona_format;
pub mod jsona_lint;
pub mod jsonlint;
pub mod jsonnet_lint;
pub mod jsonnetfmt;
pub mod juliaformatter_jl;
pub mod just;
pub mod kcl_fmt;
pub mod kdlfmt;
pub mod kdoc_formatter;
pub mod ktfmt;
pub mod ktlint;
pub mod kulala_fmt;
pub mod leptosfmt;
pub mod liquidsoap_prettier;
pub mod luacheck;
pub mod luaformatter;
pub mod mado_check;
pub mod markdownfmt;
pub mod markdownlint;
pub mod markdownlint_cli_2;
pub mod markuplint;
pub mod mdformat;
pub mod mdslw;
pub mod meson_fmt;
pub mod misspell;
pub mod mix_format;
pub mod mojo_format;
pub mod mypy;
pub mod nginxbeautifier;
pub mod nginxfmt;
pub mod nickel_format;
pub mod nimpretty;
pub mod nixfmt;
pub mod nixpkgs_fmt;
pub mod nomad_fmt;
pub mod nph;
pub mod npm_groovy_lint;
pub mod nufmt;
pub mod ocamlformat;
pub mod ocp_indent;
pub mod odinfmt;
pub mod oelint_adv;
pub mod opa_fmt;
pub mod ormolu;
pub mod oxlint;
pub mod packer_fix;
pub mod packer_fmt;
pub mod perltidy;
pub mod pg_format;
pub mod php_cs_fixer_fix;
pub mod phpcbf;
pub mod phpinsights_fix;
pub mod pint;
pub mod prettier;
pub mod pretty_php;
pub mod prettypst;
pub mod prisma_format;
pub mod protolint;
pub mod ptop;
pub mod puppet_lint;
pub mod purs_tidy;
pub mod purty;
pub mod pycln;
pub mod pycodestyle;
pub mod pyink;
pub mod pyment;
pub mod qmlfmt;
pub mod quick_lint_js;
pub mod raco_fmt;
pub mod refmt;
pub mod reformat_gherkin;
pub mod regal_fix;
pub mod regal_lint;
pub mod reorder_python_imports;
pub mod rescript_format;
pub mod roc_format;
pub mod rstfmt;
pub mod rubocop;
pub mod rubyfmt;
pub mod ruff_check;
pub mod ruff_format;
pub mod rufo;
pub mod rune_fmt;
pub mod rustfmt;
pub mod rustywind;
pub mod salt_lint;
pub mod scalafmt;
pub mod scalariform;
pub mod selene;
pub mod shellcheck;
pub mod shellharden;
pub mod shfmt;
pub mod sleek;
pub mod smlfmt;
pub mod snakefmt;
pub mod solhint;
pub mod sql_formatter;
pub mod sqlfluff_fix;
pub mod sqlfluff_format;
pub mod sqlfmt;
pub mod sqruff;
pub mod standardjs;
pub mod standardrb;
pub mod statix_check;
pub mod statix_fix;
pub mod stylefmt;
pub mod stylelint;
pub mod stylish_haskell;
pub mod stylua;
pub mod superhtml_fmt;
pub mod swift_format;
pub mod swiftformat;
pub mod taplo;
pub mod templ_fmt;
pub mod terraform_fmt;
pub mod terragrunt_hclfmt;
pub mod tex_fmt;
pub mod tlint_format;
pub mod tofu_fmt;
pub mod toml_sort;
pub mod topiary;
pub mod ts_standard;
pub mod tsqllint;
pub mod twig_cs_fixer_lint;
pub mod typos;
pub mod typstfmt;
pub mod typstyle;
pub mod ufmt;
pub mod uiua_fmt;
pub mod unimport;
pub mod usort;
pub mod v_fmt;
pub mod vacuum_lint;
pub mod veryl_fmt;
pub mod vhdl_style_guide;
pub mod wa_fmt;
pub mod wfindent;
pub mod xmlformat;
pub mod xmllint;
pub mod xo;
pub mod yamlfix;
pub mod yamlfmt;
pub mod yapf;
pub mod yew_fmt;
pub mod zig_fmt;
pub mod ziggy_fmt;
pub mod zprint;

#[derive(serde::Serialize, serde::Deserialize, Hash)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum Tooling {
    #[serde(rename = "actionlint")]
    /// `actionlint $PATH`
    Actionlint,

    #[serde(rename = "alejandra")]
    /// `alejandra --quiet $PATH`
    Alejandra,

    #[serde(rename = "ameba")]
    /// `ameba --fix $PATH`
    Ameba,

    #[serde(rename = "ansible-lint")]
    /// `ansible-lint $PATH`
    AnsibleLint,

    #[serde(rename = "asmfmt")]
    /// `asmfmt -w $PATH`
    Asmfmt,

    #[serde(rename = "astyle")]
    /// `astyle --quiet $PATH`
    Astyle,

    #[serde(rename = "auto-optional")]
    /// `auto-optional $PATH`
    AutoOptional,

    #[serde(rename = "autocorrect")]
    /// `autocorrect --fix $PATH`
    Autocorrect,

    #[serde(rename = "autoflake")]
    /// `autoflake --quiet --in-place $PATH`
    Autoflake,

    #[serde(rename = "autopep8")]
    /// `autopep8 --in-place $PATH`
    Autopep8,

    #[serde(rename = "beancount-black")]
    /// `bean-black $PATH`
    BeancountBlack,

    #[serde(rename = "beautysh")]
    /// `beautysh $PATH`
    Beautysh,

    #[serde(rename = "bibtex-tidy")]
    /// `bibtex-tidy -m $PATH`
    BibtexTidy,

    #[serde(rename = "bicep:format")]
    /// `bicep format $PATH`
    BicepFormat,

    #[serde(rename = "biome:check")]
    /// `biome check --write $PATH`
    BiomeCheck,

    #[serde(rename = "biome:format")]
    /// `biome format --write $PATH`
    BiomeFormat,

    #[serde(rename = "biome:lint")]
    /// `biome lint --write $PATH`
    BiomeLint,

    #[serde(rename = "black")]
    /// `black --quiet $PATH`
    Black,

    #[serde(rename = "blade-formatter")]
    /// `blade-formatter --write $PATH`
    BladeFormatter,

    #[serde(rename = "blue")]
    /// `blue --quiet $PATH`
    Blue,

    #[serde(rename = "bpfmt")]
    /// `bpfmt -w $PATH`
    Bpfmt,

    #[serde(rename = "brittany")]
    /// `brittany --write-mode=inplace $PATH`
    Brittany,

    #[serde(rename = "brunette")]
    /// `brunette --quiet $PATH`
    Brunette,

    #[serde(rename = "bsfmt")]
    /// `bsfmt $PATH --write`
    Bsfmt,

    #[serde(rename = "bslint")]
    /// `bslint --fix $PATH`
    Bslint,

    #[serde(rename = "buf:format")]
    /// `buf format --write $PATH`
    BufFormat,

    #[serde(rename = "buildifier")]
    /// `buildifier $PATH`
    Buildifier,

    #[serde(rename = "cabal-fmt")]
    /// `cabal-fmt --inplace $PATH`
    CabalFmt,

    #[serde(rename = "cabal-prettify")]
    /// `cabal-prettify $PATH`
    CabalPrettify,

    #[serde(rename = "cabal:format")]
    /// `cabal format $PATH`
    CabalFormat,

    #[serde(rename = "caddy:fmt")]
    /// `caddy fmt $PATH -w`
    CaddyFmt,

    #[serde(rename = "caramel:fmt")]
    /// `caramel fmt $PATH`
    CaramelFmt,

    #[serde(rename = "clang-format")]
    /// `clang-format -i $PATH`
    ClangFormat,

    #[serde(rename = "clang-tidy")]
    /// `clang-tidy --fix $PATH`
    ClangTidy,

    #[serde(rename = "cljfmt:fix")]
    /// `cljfmt fix $PATH`
    CljfmtFix,

    #[serde(rename = "cljstyle")]
    /// `cljstyle fix $PATH`
    Cljstyle,

    #[serde(rename = "cmake-format")]
    /// `cmake-format -i $PATH`
    CmakeFormat,

    #[serde(rename = "codespell")]
    /// `codespell $PATH --check-hidden --write-changes`
    Codespell,

    #[serde(rename = "crlfmt")]
    /// `crlfmt -w $PATH`
    Crlfmt,

    #[serde(rename = "crystal:format")]
    /// `crystal tool format $PATH`
    CrystalFormat,

    #[serde(rename = "csharpier")]
    /// `dotnet csharpier $PATH`
    Csharpier,

    #[serde(rename = "css-beautify")]
    /// `css-beautify -r --type css -f $PATH`
    CssBeautify,

    #[serde(rename = "csscomb")]
    /// `csscomb -t $PATH`
    Csscomb,

    #[serde(rename = "csslint")]
    /// `csslint --quiet $PATH`
    Csslint,

    #[serde(rename = "curlylint")]
    /// `curlylint -q $PATH`
    Curlylint,

    #[serde(rename = "d2:fmt")]
    /// `d2 fmt $PATH`
    D2Fmt,

    #[serde(rename = "dart:fix")]
    /// `dart fix --apply $PATH`
    DartFix,

    #[serde(rename = "dart:format")]
    /// `dart format $PATH`
    DartFormat,

    #[serde(rename = "dcm:fix")]
    /// `dcm fix $PATH`
    DcmFix,

    #[serde(rename = "dcm:format")]
    /// `dcm format $PATH`
    DcmFormat,

    #[serde(rename = "deadnix")]
    /// `deadnix -q --edit $PATH`
    Deadnix,

    #[serde(rename = "deno:fmt")]
    /// `deno fmt --quiet $PATH`
    DenoFmt,

    #[serde(rename = "deno:lint")]
    /// `deno lint --fix $PATH`
    DenoLint,

    #[serde(rename = "dfmt")]
    /// `dfmt -i $PATH`
    Dfmt,

    #[serde(rename = "dhall")]
    /// `dhall format $PATH`
    Dhall,

    #[serde(rename = "djade")]
    /// `djade $PATH`
    Djade,

    #[serde(rename = "djlint")]
    /// `djlint $PATH --reformat`
    Djlint,

    #[serde(rename = "docformatter")]
    /// `docformatter --in-place $PATH`
    Docformatter,

    #[serde(rename = "dockfmt")]
    /// `dockfmt fmt -w $PATH`
    Dockfmt,

    #[serde(rename = "docstrfmt")]
    /// `docstrfmt $PATH`
    Docstrfmt,

    #[serde(rename = "doctoc")]
    /// `doctoc $PATH`
    Doctoc,

    #[serde(rename = "dotenv-linter:fix")]
    /// `dotenv-linter fix $PATH`
    DotenvLinterFix,

    #[serde(rename = "dprint:fmt")]
    /// `dprint fmt $PATH`
    DprintFmt,

    #[serde(rename = "easy-coding-standard")]
    /// `ecs check $PATH --fix --no-interaction`
    EasyCodingStandard,

    #[serde(rename = "efmt")]
    /// `efmt -w $PATH`
    Efmt,

    #[serde(rename = "elm-format")]
    /// `elm-format --elm-version=0.19 --yes $PATH`
    ElmFormat,

    #[serde(rename = "erb-formatter")]
    /// `erb-format $PATH --write`
    ErbFormatter,

    #[serde(rename = "erlfmt")]
    /// `erlfmt -w $PATH_STRING`
    Erlfmt,

    #[serde(rename = "eslint")]
    /// `eslint --fix $PATH`
    Eslint,

    #[serde(rename = "fantomas")]
    /// `fantomas $PATH`
    Fantomas,

    #[serde(rename = "fish_indent")]
    /// `fish_indent -w $PATH`
    FishIndent,

    #[serde(rename = "fixjson")]
    /// `fixjson -w $PATH`
    Fixjson,

    #[serde(rename = "floskell")]
    /// `floskell $PATH`
    Floskell,

    #[serde(rename = "fnlfmt")]
    /// `fnlfmt $PATH`
    Fnlfmt,

    #[serde(rename = "forge:fmt")]
    /// `forge fmt $PATH`
    ForgeFmt,

    #[serde(rename = "fourmolu")]
    /// `fourmolu -i $PATH`
    Fourmolu,

    #[serde(rename = "fprettify")]
    /// `fprettify $PATH`
    Fprettify,

    #[serde(rename = "futhark:fmt")]
    /// `futhark fmt $PATH`
    FutharkFmt,

    #[serde(rename = "gci")]
    /// `gci write --skip-generated --skip-vender $PATH`
    Gci,

    #[serde(rename = "gdformat")]
    /// `gdformat $PATH`
    Gdformat,

    #[serde(rename = "gersemi")]
    /// `gersemi -i -q $PATH`
    Gersemi,

    #[serde(rename = "gleam:format")]
    /// `gleam format $PATH`
    GleamFormat,

    #[serde(rename = "gluon:fmt")]
    /// `gluon fmt $PATH`
    GluonFmt,

    #[serde(rename = "gofmt")]
    /// `gofmt -w $PATH`
    Gofmt,

    #[serde(rename = "gofumpt")]
    /// `gofumpt -w $PATH`
    Gofumpt,

    #[serde(rename = "goimports")]
    /// `goimports -w $PATH`
    Goimports,

    #[serde(rename = "goimports-reviser")]
    /// `goimports-reviser -format $PATH`
    GoimportsReviser,

    #[serde(rename = "golines")]
    /// `golines -w $PATH`
    Golines,

    #[serde(rename = "google-java-format")]
    /// `google-java-format -i $PATH`
    GoogleJavaFormat,

    #[serde(rename = "grain:format")]
    /// `grain format $PATH -o $PATH`
    GrainFormat,

    #[serde(rename = "hadolint")]
    /// `hadolint $PATH`
    Hadolint,

    #[serde(rename = "haml-lint")]
    /// `haml-lint --auto-correct $PATH`
    HamlLint,

    #[serde(rename = "hclfmt")]
    /// `hclfmt -w $PATH`
    Hclfmt,

    #[serde(rename = "hfmt")]
    /// `hfmt -w $PATH`
    Hfmt,

    #[serde(rename = "hindent")]
    /// `hindent $PATH`
    Hindent,

    #[serde(rename = "hlint")]
    /// `hlint --refactor -i $PATH`
    Hlint,

    #[serde(rename = "html-beautify")]
    /// `html-beautify -r --type html -f $PATH`
    HtmlBeautify,

    #[serde(rename = "htmlbeautifier")]
    /// `htmlbeautifier $PATH`
    Htmlbeautifier,

    #[serde(rename = "htmlhint")]
    /// `htmlhint $PATH`
    Htmlhint,

    #[serde(rename = "imba:fmt")]
    /// `imba fmt -f $PATH`
    ImbaFmt,

    #[serde(rename = "inko:fmt")]
    /// `inko fmt $PATH`
    InkoFmt,

    #[serde(rename = "isort")]
    /// `isort --quiet $PATH`
    Isort,

    #[serde(rename = "joker")]
    /// `joker --format --write $PATH`
    Joker,

    #[serde(rename = "js-beautify")]
    /// `js-beautify -r --type js -f $PATH`
    JsBeautify,

    #[serde(rename = "json5format")]
    /// `json5format -r $PATH`
    Json5Format,

    #[serde(rename = "jsona:format")]
    /// `jsona format $PATH`
    JsonaFormat,

    #[serde(rename = "jsona:lint")]
    /// `jsona lint $PATH`
    JsonaLint,

    #[serde(rename = "jsonlint")]
    /// `jsonlint -i $PATH`
    Jsonlint,

    #[serde(rename = "jsonnet-lint")]
    /// `jsonnet-lint $PATH`
    JsonnetLint,

    #[serde(rename = "jsonnetfmt")]
    /// `jsonnetfmt -i $PATH`
    Jsonnetfmt,

    #[serde(rename = "juliaformatter.jl")]
    /// `julia -E using JuliaFormatter;format_file(\"{$PATH_STRING}\")`
    JuliaformatterJl,

    #[serde(rename = "just")]
    /// `just --fmt --unstable --justfile $PATH`
    Just,

    #[serde(rename = "kcl:fmt")]
    /// `kcl fmt $PATH`
    KclFmt,

    #[serde(rename = "kdlfmt")]
    /// `kdlfmt format $PATH`
    Kdlfmt,

    #[serde(rename = "kdoc-formatter")]
    /// `kdoc-formatter --quiet $PATH`
    KdocFormatter,

    #[serde(rename = "ktfmt")]
    /// `ktfmt --format --log-level=error $PATH`
    Ktfmt,

    #[serde(rename = "ktlint")]
    /// `ktlint --format --log-level=error $PATH`
    Ktlint,

    #[serde(rename = "kulala-fmt")]
    /// `kulala-fmt $PATH`
    KulalaFmt,

    #[serde(rename = "leptosfmt")]
    /// `leptosfmt --quiet $PATH`
    Leptosfmt,

    #[serde(rename = "liquidsoap-prettier")]
    /// `liquidsoap-prettier --write $PATH`
    LiquidsoapPrettier,

    #[serde(rename = "luacheck")]
    /// `luacheck $PATH`
    Luacheck,

    #[serde(rename = "luaformatter")]
    /// `lua-format -i $PATH`
    Luaformatter,

    #[serde(rename = "mado:check")]
    /// `mado check $PATH`
    MadoCheck,

    #[serde(rename = "markdownfmt")]
    /// `markdownfmt -w $PATH`
    Markdownfmt,

    #[serde(rename = "markdownlint")]
    /// `markdownlint --fix $PATH`
    Markdownlint,

    #[serde(rename = "markdownlint-cli2")]
    /// `markdownlint-cli2 --fix $PATH`
    MarkdownlintCli2,

    #[serde(rename = "markuplint")]
    /// `markuplint --fix $PATH`
    Markuplint,

    #[serde(rename = "mdformat")]
    /// `mdformat $PATH`
    Mdformat,

    #[serde(rename = "mdslw")]
    /// `mdslw $PATH`
    Mdslw,

    #[serde(rename = "meson:fmt")]
    /// `meson fmt -i $PATH`
    MesonFmt,

    #[serde(rename = "misspell")]
    /// `misspell -w $PATH`
    Misspell,

    #[serde(rename = "mix:format")]
    /// `mix format $PATH`
    MixFormat,

    #[serde(rename = "mojo:format")]
    /// `mojo format -q $PATH`
    MojoFormat,

    #[serde(rename = "mypy")]
    /// `mypy $PATH`
    Mypy,

    #[serde(rename = "nginxbeautifier")]
    /// `nginxbeautifier $PATH`
    Nginxbeautifier,

    #[serde(rename = "nginxfmt")]
    /// `nginxfmt $PATH`
    Nginxfmt,

    #[serde(rename = "nickel:format")]
    /// `nickel format $PATH`
    NickelFormat,

    #[serde(rename = "nimpretty")]
    /// `nimpretty $PATH`
    Nimpretty,

    #[serde(rename = "nixfmt")]
    /// `nixfmt $PATH`
    Nixfmt,

    #[serde(rename = "nixpkgs-fmt")]
    /// `nixpkgs-fmt $PATH`
    NixpkgsFmt,

    #[serde(rename = "nomad:fmt")]
    /// `nomad fmt $PATH`
    NomadFmt,

    #[serde(rename = "nph")]
    /// `nph $PATH`
    Nph,

    #[serde(rename = "npm-groovy-lint")]
    /// `npm-groovy-lint --format $PATH`
    NpmGroovyLint,

    #[serde(rename = "nufmt")]
    /// `nufmt $PATH`
    Nufmt,

    #[serde(rename = "ocamlformat")]
    /// `ocamlformat --ignore-invalid-option --inplace --enable-outside-detected-project $PATH`
    Ocamlformat,

    #[serde(rename = "ocp-indent")]
    /// `ocp-indent --inplace $PATH`
    OcpIndent,

    #[serde(rename = "odinfmt")]
    /// `odinfmt -w $PATH`
    Odinfmt,

    #[serde(rename = "oelint-adv")]
    /// `oelint-adv --fix --nobackup --quiet $PATH`
    OelintAdv,

    #[serde(rename = "opa:fmt")]
    /// `opa fmt $PATH -w`
    OpaFmt,

    #[serde(rename = "ormolu")]
    /// `ormolu --mode inplace $PATH`
    Ormolu,

    #[serde(rename = "oxlint")]
    /// `oxlint --fix $PATH`
    Oxlint,

    #[serde(rename = "packer:fix")]
    /// `packer fix $PATH`
    PackerFix,

    #[serde(rename = "packer:fmt")]
    /// `packer fmt $PATH`
    PackerFmt,

    #[serde(rename = "perltidy")]
    /// `perltidy -b $PATH`
    Perltidy,

    #[serde(rename = "pg_format")]
    /// `pg_format --inplace $PATH`
    PgFormat,

    #[serde(rename = "php-cs-fixer:fix")]
    /// `php-cs-fixer fix $PATH`
    PhpCsFixerFix,

    #[serde(rename = "phpcbf")]
    /// `phpcbf $PATH`
    Phpcbf,

    #[serde(rename = "phpinsights:fix")]
    /// `phpinsights fix $PATH --no-interaction --quiet`
    PhpinsightsFix,

    #[serde(rename = "pint")]
    /// `pint $PATH`
    Pint,

    #[serde(rename = "prettier")]
    /// `prettier --embedded-language-formatting off --log-level error --write $PATH`
    Prettier,

    #[serde(rename = "pretty-php")]
    /// `pretty-php $PATH`
    PrettyPhp,

    #[serde(rename = "prettypst")]
    /// `prettypst $PATH`
    Prettypst,

    #[serde(rename = "prisma:format")]
    /// `prisma format --schema={$PATH_STRING}`
    PrismaFormat,

    #[serde(rename = "protolint")]
    /// `protolint lint -fix $PATH`
    Protolint,

    #[serde(rename = "ptop")]
    /// `ptop $PATH $PATH`
    Ptop,

    #[serde(rename = "puppet-lint")]
    /// `puppet-lint --fix $PATH`
    PuppetLint,

    #[serde(rename = "purs-tidy")]
    /// `purs-tidy format-in-place $PATH`
    PursTidy,

    #[serde(rename = "purty")]
    /// `purty --write $PATH`
    Purty,

    #[serde(rename = "pycln")]
    /// `pycln --no-gitignore --quiet $PATH`
    Pycln,

    #[serde(rename = "pycodestyle")]
    /// `pycodestyle $PATH`
    Pycodestyle,

    #[serde(rename = "pyink")]
    /// `pyink --quiet $PATH`
    Pyink,

    #[serde(rename = "pyment")]
    /// `pyment -w $PATH`
    Pyment,

    #[serde(rename = "qmlfmt")]
    /// `qmlfmt -w $PATH`
    Qmlfmt,

    #[serde(rename = "quick-lint-js")]
    /// `quick-lint-js $PATH`
    QuickLintJs,

    #[serde(rename = "raco:fmt")]
    /// `raco fmt -i $PATH`
    RacoFmt,

    #[serde(rename = "refmt")]
    /// `refmt --in-place $PATH`
    Refmt,

    #[serde(rename = "reformat-gherkin")]
    /// `reformat-gherkin $PATH`
    ReformatGherkin,

    #[serde(rename = "regal:fix")]
    /// `regal fix $PATH`
    RegalFix,

    #[serde(rename = "regal:lint")]
    /// `regal lint $PATH`
    RegalLint,

    #[serde(rename = "reorder-python-imports")]
    /// `reorder-python-imports $PATH`
    ReorderPythonImports,

    #[serde(rename = "rescript:format")]
    /// `rescript format $PATH`
    RescriptFormat,

    #[serde(rename = "roc:format")]
    /// `roc format $PATH`
    RocFormat,

    #[serde(rename = "rstfmt")]
    /// `rstfmt $PATH`
    Rstfmt,

    #[serde(rename = "rubocop")]
    /// `rubocop --fix-layout --autocorrect --format quiet $PATH`
    Rubocop,

    #[serde(rename = "rubyfmt")]
    /// `rubyfmt -i $PATH`
    Rubyfmt,

    #[serde(rename = "ruff:check")]
    /// `ruff check --fix --quiet $PATH`
    RuffCheck,

    #[serde(rename = "ruff:format")]
    /// `ruff format --quiet $PATH`
    RuffFormat,

    #[serde(rename = "rufo")]
    /// `rufo --simple-exit $PATH`
    Rufo,

    #[serde(rename = "rune:fmt")]
    /// `rune fmt $PATH`
    RuneFmt,

    #[serde(rename = "rustfmt")]
    /// `rustfmt --edition 2021 --quiet $PATH`
    Rustfmt,

    #[serde(rename = "rustywind")]
    /// `rustywind --write $PATH`
    Rustywind,

    #[serde(rename = "salt-lint")]
    /// `salt-lint $PATH`
    SaltLint,

    #[serde(rename = "scalafmt")]
    /// `scalafmt --quiet --mode any $PATH`
    Scalafmt,

    #[serde(rename = "scalariform")]
    /// `scalariform $PATH`
    Scalariform,

    #[serde(rename = "selene")]
    /// `selene $PATH`
    Selene,

    #[serde(rename = "shellcheck")]
    /// `shellcheck $PATH`
    Shellcheck,

    #[serde(rename = "shellharden")]
    /// `shellharden --transform --replace $PATH`
    Shellharden,

    #[serde(rename = "shfmt")]
    /// `shfmt --write $PATH`
    Shfmt,

    #[serde(rename = "sleek")]
    /// `sleek $PATH`
    Sleek,

    #[serde(rename = "smlfmt")]
    /// `smlfmt --force $PATH`
    Smlfmt,

    #[serde(rename = "snakefmt")]
    /// `snakefmt $PATH`
    Snakefmt,

    #[serde(rename = "solhint")]
    /// `solhint --quiet --fix --noPrompt $PATH`
    Solhint,

    #[serde(rename = "sql-formatter")]
    /// `sql-formatter --fix $PATH`
    SqlFormatter,

    #[serde(rename = "sqlfluff:fix")]
    /// `sqlfluff fix --dialect ansi $PATH`
    SqlfluffFix,

    #[serde(rename = "sqlfluff:format")]
    /// `sqlfluff format --dialect ansi $PATH`
    SqlfluffFormat,

    #[serde(rename = "sqlfmt")]
    /// `sqlfmt $PATH`
    Sqlfmt,

    #[serde(rename = "sqruff")]
    /// `sqruff fix $PATH`
    Sqruff,

    #[serde(rename = "standardjs")]
    /// `standard --fix $PATH`
    Standardjs,

    #[serde(rename = "standardrb")]
    /// `standardrb --fix $PATH`
    Standardrb,

    #[serde(rename = "statix:check")]
    /// `statix check $PATH`
    StatixCheck,

    #[serde(rename = "statix:fix")]
    /// `statix fix $PATH`
    StatixFix,

    #[serde(rename = "stylefmt")]
    /// `stylefmt $PATH`
    Stylefmt,

    #[serde(rename = "stylelint")]
    /// `stylelint --fix $PATH`
    Stylelint,

    #[serde(rename = "stylish-haskell")]
    /// `stylish-haskell --inplace $PATH`
    StylishHaskell,

    #[serde(rename = "stylua")]
    /// `stylua --verify $PATH`
    Stylua,

    #[serde(rename = "superhtml:fmt")]
    /// `superhtml fmt $PATH`
    SuperhtmlFmt,

    #[serde(rename = "swift-format")]
    /// `swift-format --in-place $PATH`
    SwiftFormat,

    #[serde(rename = "swiftformat")]
    /// `swiftformat --quiet $PATH`
    Swiftformat,

    #[serde(rename = "taplo")]
    /// `taplo format $PATH`
    Taplo,

    #[serde(rename = "templ:fmt")]
    /// `templ fmt $PATH`
    TemplFmt,

    #[serde(rename = "terraform:fmt")]
    /// `terraform fmt -write=true $PATH`
    TerraformFmt,

    #[serde(rename = "terragrunt:hclfmt")]
    /// `terragrunt hclfmt --terragrunt-hclfmt-file $PATH`
    TerragruntHclfmt,

    #[serde(rename = "tex-fmt")]
    /// `tex-fmt $PATH`
    TexFmt,

    #[serde(rename = "tlint:format")]
    /// `tlint format $PATH`
    TlintFormat,

    #[serde(rename = "tofu:fmt")]
    /// `tofu fmt -write=true $PATH`
    TofuFmt,

    #[serde(rename = "toml-sort")]
    /// `toml-sort -i $PATH`
    TomlSort,

    #[serde(rename = "topiary")]
    /// `topiary format $PATH`
    Topiary,

    #[serde(rename = "ts-standard")]
    /// `ts-standard --fix $PATH`
    TsStandard,

    #[serde(rename = "tsqllint")]
    /// `tsqllint --fix $PATH`
    Tsqllint,

    #[serde(rename = "twig-cs-fixer:lint")]
    /// `twig-cs-fixer lint $PATH --fix --no-interaction --quiet`
    TwigCsFixerLint,

    #[serde(rename = "typos")]
    /// `typos -w --no-ignore --hidden $PATH`
    Typos,

    #[serde(rename = "typstfmt")]
    /// `typstfmt $PATH`
    Typstfmt,

    #[serde(rename = "typstyle")]
    /// `typstyle -i $PATH`
    Typstyle,

    #[serde(rename = "ufmt")]
    /// `ufmt format $PATH`
    Ufmt,

    #[serde(rename = "uiua:fmt")]
    /// `uiua fmt $PATH`
    UiuaFmt,

    #[serde(rename = "unimport")]
    /// `unimport -r $PATH`
    Unimport,

    #[serde(rename = "usort")]
    /// `usort format $PATH`
    Usort,

    #[serde(rename = "v:fmt")]
    /// `v fmt -w $PATH`
    VFmt,

    #[serde(rename = "vacuum:lint")]
    /// `vacuum lint $PATH`
    VacuumLint,

    #[serde(rename = "veryl:fmt")]
    /// `veryl fmt $PATH`
    VerylFmt,

    #[serde(rename = "vhdl-style-guide")]
    /// `vsg -f $PATH --fix`
    VhdlStyleGuide,

    #[serde(rename = "wa:fmt")]
    /// `wa fmt $PATH`
    WaFmt,

    #[serde(rename = "wfindent")]
    /// `wfindent $PATH`
    Wfindent,

    #[serde(rename = "xmlformat")]
    /// `xmlformat --overwrite $PATH`
    Xmlformat,

    #[serde(rename = "xmllint")]
    /// `xmllint --format $PATH --output $PATH`
    Xmllint,

    #[serde(rename = "xo")]
    /// `xo --fix $PATH`
    Xo,

    #[serde(rename = "yamlfix")]
    /// `yamlfix $PATH`
    Yamlfix,

    #[serde(rename = "yamlfmt")]
    /// `yamlfmt -quiet $PATH`
    Yamlfmt,

    #[serde(rename = "yapf")]
    /// `yapf --in-place $PATH`
    Yapf,

    #[serde(rename = "yew-fmt")]
    /// `yew-fmt --edition 2021 --quiet $PATH`
    YewFmt,

    #[serde(rename = "zig:fmt")]
    /// `zig fmt $PATH`
    ZigFmt,

    #[serde(rename = "ziggy:fmt")]
    /// `ziggy fmt $PATH`
    ZiggyFmt,

    #[serde(rename = "zprint")]
    /// `zprint -w $PATH`
    Zprint,
}

impl Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    pub fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), crate::error::MdsfError> {
        match self {
            Self::Actionlint => actionlint::run(snippet_path),
            Self::Alejandra => alejandra::run(snippet_path),
            Self::Ameba => ameba::run(snippet_path),
            Self::AnsibleLint => ansible_lint::run(snippet_path),
            Self::Asmfmt => asmfmt::run(snippet_path),
            Self::Astyle => astyle::run(snippet_path),
            Self::AutoOptional => auto_optional::run(snippet_path),
            Self::Autocorrect => autocorrect::run(snippet_path),
            Self::Autoflake => autoflake::run(snippet_path),
            Self::Autopep8 => autopep_8::run(snippet_path),
            Self::BeancountBlack => beancount_black::run(snippet_path),
            Self::Beautysh => beautysh::run(snippet_path),
            Self::BibtexTidy => bibtex_tidy::run(snippet_path),
            Self::BicepFormat => bicep_format::run(snippet_path),
            Self::BiomeCheck => biome_check::run(snippet_path),
            Self::BiomeFormat => biome_format::run(snippet_path),
            Self::BiomeLint => biome_lint::run(snippet_path),
            Self::Black => black::run(snippet_path),
            Self::BladeFormatter => blade_formatter::run(snippet_path),
            Self::Blue => blue::run(snippet_path),
            Self::Bpfmt => bpfmt::run(snippet_path),
            Self::Brittany => brittany::run(snippet_path),
            Self::Brunette => brunette::run(snippet_path),
            Self::Bsfmt => bsfmt::run(snippet_path),
            Self::Bslint => bslint::run(snippet_path),
            Self::BufFormat => buf_format::run(snippet_path),
            Self::Buildifier => buildifier::run(snippet_path),
            Self::CabalFmt => cabal_fmt::run(snippet_path),
            Self::CabalFormat => cabal_format::run(snippet_path),
            Self::CabalPrettify => cabal_prettify::run(snippet_path),
            Self::CaddyFmt => caddy_fmt::run(snippet_path),
            Self::CaramelFmt => caramel_fmt::run(snippet_path),
            Self::ClangFormat => clang_format::run(snippet_path),
            Self::ClangTidy => clang_tidy::run(snippet_path),
            Self::CljfmtFix => cljfmt_fix::run(snippet_path),
            Self::Cljstyle => cljstyle::run(snippet_path),
            Self::CmakeFormat => cmake_format::run(snippet_path),
            Self::Codespell => codespell::run(snippet_path),
            Self::Crlfmt => crlfmt::run(snippet_path),
            Self::CrystalFormat => crystal_format::run(snippet_path),
            Self::Csharpier => csharpier::run(snippet_path),
            Self::CssBeautify => css_beautify::run(snippet_path),
            Self::Csscomb => csscomb::run(snippet_path),
            Self::Csslint => csslint::run(snippet_path),
            Self::Curlylint => curlylint::run(snippet_path),
            Self::D2Fmt => d_2_fmt::run(snippet_path),
            Self::DartFix => dart_fix::run(snippet_path),
            Self::DartFormat => dart_format::run(snippet_path),
            Self::DcmFix => dcm_fix::run(snippet_path),
            Self::DcmFormat => dcm_format::run(snippet_path),
            Self::Deadnix => deadnix::run(snippet_path),
            Self::DenoFmt => deno_fmt::run(snippet_path),
            Self::DenoLint => deno_lint::run(snippet_path),
            Self::Dfmt => dfmt::run(snippet_path),
            Self::Dhall => dhall::run(snippet_path),
            Self::Djade => djade::run(snippet_path),
            Self::Djlint => djlint::run(snippet_path),
            Self::Docformatter => docformatter::run(snippet_path),
            Self::Dockfmt => dockfmt::run(snippet_path),
            Self::Docstrfmt => docstrfmt::run(snippet_path),
            Self::Doctoc => doctoc::run(snippet_path),
            Self::DotenvLinterFix => dotenv_linter_fix::run(snippet_path),
            Self::DprintFmt => dprint_fmt::run(snippet_path),
            Self::EasyCodingStandard => easy_coding_standard::run(snippet_path),
            Self::Efmt => efmt::run(snippet_path),
            Self::ElmFormat => elm_format::run(snippet_path),
            Self::ErbFormatter => erb_formatter::run(snippet_path),
            Self::Erlfmt => erlfmt::run(snippet_path),
            Self::Eslint => eslint::run(snippet_path),
            Self::Fantomas => fantomas::run(snippet_path),
            Self::FishIndent => fish_indent::run(snippet_path),
            Self::Fixjson => fixjson::run(snippet_path),
            Self::Floskell => floskell::run(snippet_path),
            Self::Fnlfmt => fnlfmt::run(snippet_path),
            Self::ForgeFmt => forge_fmt::run(snippet_path),
            Self::Fourmolu => fourmolu::run(snippet_path),
            Self::Fprettify => fprettify::run(snippet_path),
            Self::FutharkFmt => futhark_fmt::run(snippet_path),
            Self::Gci => gci::run(snippet_path),
            Self::Gdformat => gdformat::run(snippet_path),
            Self::Gersemi => gersemi::run(snippet_path),
            Self::GleamFormat => gleam_format::run(snippet_path),
            Self::GluonFmt => gluon_fmt::run(snippet_path),
            Self::Gofmt => gofmt::run(snippet_path),
            Self::Gofumpt => gofumpt::run(snippet_path),
            Self::Goimports => goimports::run(snippet_path),
            Self::GoimportsReviser => goimports_reviser::run(snippet_path),
            Self::Golines => golines::run(snippet_path),
            Self::GoogleJavaFormat => google_java_format::run(snippet_path),
            Self::GrainFormat => grain_format::run(snippet_path),
            Self::Hadolint => hadolint::run(snippet_path),
            Self::HamlLint => haml_lint::run(snippet_path),
            Self::Hclfmt => hclfmt::run(snippet_path),
            Self::Hfmt => hfmt::run(snippet_path),
            Self::Hindent => hindent::run(snippet_path),
            Self::Hlint => hlint::run(snippet_path),
            Self::HtmlBeautify => html_beautify::run(snippet_path),
            Self::Htmlbeautifier => htmlbeautifier::run(snippet_path),
            Self::Htmlhint => htmlhint::run(snippet_path),
            Self::ImbaFmt => imba_fmt::run(snippet_path),
            Self::InkoFmt => inko_fmt::run(snippet_path),
            Self::Isort => isort::run(snippet_path),
            Self::Joker => joker::run(snippet_path),
            Self::JsBeautify => js_beautify::run(snippet_path),
            Self::Json5Format => json_5_format::run(snippet_path),
            Self::JsonaFormat => jsona_format::run(snippet_path),
            Self::JsonaLint => jsona_lint::run(snippet_path),
            Self::Jsonlint => jsonlint::run(snippet_path),
            Self::JsonnetLint => jsonnet_lint::run(snippet_path),
            Self::Jsonnetfmt => jsonnetfmt::run(snippet_path),
            Self::JuliaformatterJl => juliaformatter_jl::run(snippet_path),
            Self::Just => just::run(snippet_path),
            Self::KclFmt => kcl_fmt::run(snippet_path),
            Self::Kdlfmt => kdlfmt::run(snippet_path),
            Self::KdocFormatter => kdoc_formatter::run(snippet_path),
            Self::Ktfmt => ktfmt::run(snippet_path),
            Self::Ktlint => ktlint::run(snippet_path),
            Self::KulalaFmt => kulala_fmt::run(snippet_path),
            Self::Leptosfmt => leptosfmt::run(snippet_path),
            Self::LiquidsoapPrettier => liquidsoap_prettier::run(snippet_path),
            Self::Luacheck => luacheck::run(snippet_path),
            Self::Luaformatter => luaformatter::run(snippet_path),
            Self::MadoCheck => mado_check::run(snippet_path),
            Self::Markdownfmt => markdownfmt::run(snippet_path),
            Self::Markdownlint => markdownlint::run(snippet_path),
            Self::MarkdownlintCli2 => markdownlint_cli_2::run(snippet_path),
            Self::Markuplint => markuplint::run(snippet_path),
            Self::Mdformat => mdformat::run(snippet_path),
            Self::Mdslw => mdslw::run(snippet_path),
            Self::MesonFmt => meson_fmt::run(snippet_path),
            Self::Misspell => misspell::run(snippet_path),
            Self::MixFormat => mix_format::run(snippet_path),
            Self::MojoFormat => mojo_format::run(snippet_path),
            Self::Mypy => mypy::run(snippet_path),
            Self::Nginxbeautifier => nginxbeautifier::run(snippet_path),
            Self::Nginxfmt => nginxfmt::run(snippet_path),
            Self::NickelFormat => nickel_format::run(snippet_path),
            Self::Nimpretty => nimpretty::run(snippet_path),
            Self::Nixfmt => nixfmt::run(snippet_path),
            Self::NixpkgsFmt => nixpkgs_fmt::run(snippet_path),
            Self::NomadFmt => nomad_fmt::run(snippet_path),
            Self::Nph => nph::run(snippet_path),
            Self::NpmGroovyLint => npm_groovy_lint::run(snippet_path),
            Self::Nufmt => nufmt::run(snippet_path),
            Self::Ocamlformat => ocamlformat::run(snippet_path),
            Self::OcpIndent => ocp_indent::run(snippet_path),
            Self::Odinfmt => odinfmt::run(snippet_path),
            Self::OelintAdv => oelint_adv::run(snippet_path),
            Self::OpaFmt => opa_fmt::run(snippet_path),
            Self::Ormolu => ormolu::run(snippet_path),
            Self::Oxlint => oxlint::run(snippet_path),
            Self::PackerFix => packer_fix::run(snippet_path),
            Self::PackerFmt => packer_fmt::run(snippet_path),
            Self::Perltidy => perltidy::run(snippet_path),
            Self::PgFormat => pg_format::run(snippet_path),
            Self::PhpCsFixerFix => php_cs_fixer_fix::run(snippet_path),
            Self::Phpcbf => phpcbf::run(snippet_path),
            Self::PhpinsightsFix => phpinsights_fix::run(snippet_path),
            Self::Pint => pint::run(snippet_path),
            Self::Prettier => prettier::run(snippet_path),
            Self::PrettyPhp => pretty_php::run(snippet_path),
            Self::Prettypst => prettypst::run(snippet_path),
            Self::PrismaFormat => prisma_format::run(snippet_path),
            Self::Protolint => protolint::run(snippet_path),
            Self::Ptop => ptop::run(snippet_path),
            Self::PuppetLint => puppet_lint::run(snippet_path),
            Self::PursTidy => purs_tidy::run(snippet_path),
            Self::Purty => purty::run(snippet_path),
            Self::Pycln => pycln::run(snippet_path),
            Self::Pycodestyle => pycodestyle::run(snippet_path),
            Self::Pyink => pyink::run(snippet_path),
            Self::Pyment => pyment::run(snippet_path),
            Self::Qmlfmt => qmlfmt::run(snippet_path),
            Self::QuickLintJs => quick_lint_js::run(snippet_path),
            Self::RacoFmt => raco_fmt::run(snippet_path),
            Self::Refmt => refmt::run(snippet_path),
            Self::ReformatGherkin => reformat_gherkin::run(snippet_path),
            Self::RegalFix => regal_fix::run(snippet_path),
            Self::RegalLint => regal_lint::run(snippet_path),
            Self::ReorderPythonImports => reorder_python_imports::run(snippet_path),
            Self::RescriptFormat => rescript_format::run(snippet_path),
            Self::RocFormat => roc_format::run(snippet_path),
            Self::Rstfmt => rstfmt::run(snippet_path),
            Self::Rubocop => rubocop::run(snippet_path),
            Self::Rubyfmt => rubyfmt::run(snippet_path),
            Self::RuffCheck => ruff_check::run(snippet_path),
            Self::RuffFormat => ruff_format::run(snippet_path),
            Self::Rufo => rufo::run(snippet_path),
            Self::RuneFmt => rune_fmt::run(snippet_path),
            Self::Rustfmt => rustfmt::run(snippet_path),
            Self::Rustywind => rustywind::run(snippet_path),
            Self::SaltLint => salt_lint::run(snippet_path),
            Self::Scalafmt => scalafmt::run(snippet_path),
            Self::Scalariform => scalariform::run(snippet_path),
            Self::Selene => selene::run(snippet_path),
            Self::Shellcheck => shellcheck::run(snippet_path),
            Self::Shellharden => shellharden::run(snippet_path),
            Self::Shfmt => shfmt::run(snippet_path),
            Self::Sleek => sleek::run(snippet_path),
            Self::Smlfmt => smlfmt::run(snippet_path),
            Self::Snakefmt => snakefmt::run(snippet_path),
            Self::Solhint => solhint::run(snippet_path),
            Self::SqlFormatter => sql_formatter::run(snippet_path),
            Self::SqlfluffFix => sqlfluff_fix::run(snippet_path),
            Self::SqlfluffFormat => sqlfluff_format::run(snippet_path),
            Self::Sqlfmt => sqlfmt::run(snippet_path),
            Self::Sqruff => sqruff::run(snippet_path),
            Self::Standardjs => standardjs::run(snippet_path),
            Self::Standardrb => standardrb::run(snippet_path),
            Self::StatixCheck => statix_check::run(snippet_path),
            Self::StatixFix => statix_fix::run(snippet_path),
            Self::Stylefmt => stylefmt::run(snippet_path),
            Self::Stylelint => stylelint::run(snippet_path),
            Self::StylishHaskell => stylish_haskell::run(snippet_path),
            Self::Stylua => stylua::run(snippet_path),
            Self::SuperhtmlFmt => superhtml_fmt::run(snippet_path),
            Self::SwiftFormat => swift_format::run(snippet_path),
            Self::Swiftformat => swiftformat::run(snippet_path),
            Self::Taplo => taplo::run(snippet_path),
            Self::TemplFmt => templ_fmt::run(snippet_path),
            Self::TerraformFmt => terraform_fmt::run(snippet_path),
            Self::TerragruntHclfmt => terragrunt_hclfmt::run(snippet_path),
            Self::TexFmt => tex_fmt::run(snippet_path),
            Self::TlintFormat => tlint_format::run(snippet_path),
            Self::TofuFmt => tofu_fmt::run(snippet_path),
            Self::TomlSort => toml_sort::run(snippet_path),
            Self::Topiary => topiary::run(snippet_path),
            Self::TsStandard => ts_standard::run(snippet_path),
            Self::Tsqllint => tsqllint::run(snippet_path),
            Self::TwigCsFixerLint => twig_cs_fixer_lint::run(snippet_path),
            Self::Typos => typos::run(snippet_path),
            Self::Typstfmt => typstfmt::run(snippet_path),
            Self::Typstyle => typstyle::run(snippet_path),
            Self::Ufmt => ufmt::run(snippet_path),
            Self::UiuaFmt => uiua_fmt::run(snippet_path),
            Self::Unimport => unimport::run(snippet_path),
            Self::Usort => usort::run(snippet_path),
            Self::VFmt => v_fmt::run(snippet_path),
            Self::VacuumLint => vacuum_lint::run(snippet_path),
            Self::VerylFmt => veryl_fmt::run(snippet_path),
            Self::VhdlStyleGuide => vhdl_style_guide::run(snippet_path),
            Self::WaFmt => wa_fmt::run(snippet_path),
            Self::Wfindent => wfindent::run(snippet_path),
            Self::Xmlformat => xmlformat::run(snippet_path),
            Self::Xmllint => xmllint::run(snippet_path),
            Self::Xo => xo::run(snippet_path),
            Self::Yamlfix => yamlfix::run(snippet_path),
            Self::Yamlfmt => yamlfmt::run(snippet_path),
            Self::Yapf => yapf::run(snippet_path),
            Self::YewFmt => yew_fmt::run(snippet_path),
            Self::ZigFmt => zig_fmt::run(snippet_path),
            Self::ZiggyFmt => ziggy_fmt::run(snippet_path),
            Self::Zprint => zprint::run(snippet_path),
        }
    }
}

impl AsRef<str> for Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    fn as_ref(&self) -> &str {
        match self {
            Self::Actionlint => "actionlint",
            Self::Alejandra => "alejandra",
            Self::Ameba => "ameba",
            Self::AnsibleLint => "ansible_lint",
            Self::Asmfmt => "asmfmt",
            Self::Astyle => "astyle",
            Self::AutoOptional => "auto_optional",
            Self::Autocorrect => "autocorrect",
            Self::Autoflake => "autoflake",
            Self::Autopep8 => "autopep_8",
            Self::BeancountBlack => "beancount_black",
            Self::Beautysh => "beautysh",
            Self::BibtexTidy => "bibtex_tidy",
            Self::BicepFormat => "bicep_format",
            Self::BiomeCheck => "biome_check",
            Self::BiomeFormat => "biome_format",
            Self::BiomeLint => "biome_lint",
            Self::Black => "black",
            Self::BladeFormatter => "blade_formatter",
            Self::Blue => "blue",
            Self::Bpfmt => "bpfmt",
            Self::Brittany => "brittany",
            Self::Brunette => "brunette",
            Self::Bsfmt => "bsfmt",
            Self::Bslint => "bslint",
            Self::BufFormat => "buf_format",
            Self::Buildifier => "buildifier",
            Self::CabalFmt => "cabal_fmt",
            Self::CabalFormat => "cabal_format",
            Self::CabalPrettify => "cabal_prettify",
            Self::CaddyFmt => "caddy_fmt",
            Self::CaramelFmt => "caramel_fmt",
            Self::ClangFormat => "clang_format",
            Self::ClangTidy => "clang_tidy",
            Self::CljfmtFix => "cljfmt_fix",
            Self::Cljstyle => "cljstyle",
            Self::CmakeFormat => "cmake_format",
            Self::Codespell => "codespell",
            Self::Crlfmt => "crlfmt",
            Self::CrystalFormat => "crystal_format",
            Self::Csharpier => "csharpier",
            Self::CssBeautify => "css_beautify",
            Self::Csscomb => "csscomb",
            Self::Csslint => "csslint",
            Self::Curlylint => "curlylint",
            Self::D2Fmt => "d_2_fmt",
            Self::DartFix => "dart_fix",
            Self::DartFormat => "dart_format",
            Self::DcmFix => "dcm_fix",
            Self::DcmFormat => "dcm_format",
            Self::Deadnix => "deadnix",
            Self::DenoFmt => "deno_fmt",
            Self::DenoLint => "deno_lint",
            Self::Dfmt => "dfmt",
            Self::Dhall => "dhall",
            Self::Djade => "djade",
            Self::Djlint => "djlint",
            Self::Docformatter => "docformatter",
            Self::Dockfmt => "dockfmt",
            Self::Docstrfmt => "docstrfmt",
            Self::Doctoc => "doctoc",
            Self::DotenvLinterFix => "dotenv_linter_fix",
            Self::DprintFmt => "dprint_fmt",
            Self::EasyCodingStandard => "easy_coding_standard",
            Self::Efmt => "efmt",
            Self::ElmFormat => "elm_format",
            Self::ErbFormatter => "erb_formatter",
            Self::Erlfmt => "erlfmt",
            Self::Eslint => "eslint",
            Self::Fantomas => "fantomas",
            Self::FishIndent => "fish_indent",
            Self::Fixjson => "fixjson",
            Self::Floskell => "floskell",
            Self::Fnlfmt => "fnlfmt",
            Self::ForgeFmt => "forge_fmt",
            Self::Fourmolu => "fourmolu",
            Self::Fprettify => "fprettify",
            Self::FutharkFmt => "futhark_fmt",
            Self::Gci => "gci",
            Self::Gdformat => "gdformat",
            Self::Gersemi => "gersemi",
            Self::GleamFormat => "gleam_format",
            Self::GluonFmt => "gluon_fmt",
            Self::Gofmt => "gofmt",
            Self::Gofumpt => "gofumpt",
            Self::Goimports => "goimports",
            Self::GoimportsReviser => "goimports_reviser",
            Self::Golines => "golines",
            Self::GoogleJavaFormat => "google_java_format",
            Self::GrainFormat => "grain_format",
            Self::Hadolint => "hadolint",
            Self::HamlLint => "haml_lint",
            Self::Hclfmt => "hclfmt",
            Self::Hfmt => "hfmt",
            Self::Hindent => "hindent",
            Self::Hlint => "hlint",
            Self::HtmlBeautify => "html_beautify",
            Self::Htmlbeautifier => "htmlbeautifier",
            Self::Htmlhint => "htmlhint",
            Self::ImbaFmt => "imba_fmt",
            Self::InkoFmt => "inko_fmt",
            Self::Isort => "isort",
            Self::Joker => "joker",
            Self::JsBeautify => "js_beautify",
            Self::Json5Format => "json_5_format",
            Self::JsonaFormat => "jsona_format",
            Self::JsonaLint => "jsona_lint",
            Self::Jsonlint => "jsonlint",
            Self::JsonnetLint => "jsonnet_lint",
            Self::Jsonnetfmt => "jsonnetfmt",
            Self::JuliaformatterJl => "juliaformatter_jl",
            Self::Just => "just",
            Self::KclFmt => "kcl_fmt",
            Self::Kdlfmt => "kdlfmt",
            Self::KdocFormatter => "kdoc_formatter",
            Self::Ktfmt => "ktfmt",
            Self::Ktlint => "ktlint",
            Self::KulalaFmt => "kulala_fmt",
            Self::Leptosfmt => "leptosfmt",
            Self::LiquidsoapPrettier => "liquidsoap_prettier",
            Self::Luacheck => "luacheck",
            Self::Luaformatter => "luaformatter",
            Self::MadoCheck => "mado_check",
            Self::Markdownfmt => "markdownfmt",
            Self::Markdownlint => "markdownlint",
            Self::MarkdownlintCli2 => "markdownlint_cli_2",
            Self::Markuplint => "markuplint",
            Self::Mdformat => "mdformat",
            Self::Mdslw => "mdslw",
            Self::MesonFmt => "meson_fmt",
            Self::Misspell => "misspell",
            Self::MixFormat => "mix_format",
            Self::MojoFormat => "mojo_format",
            Self::Mypy => "mypy",
            Self::Nginxbeautifier => "nginxbeautifier",
            Self::Nginxfmt => "nginxfmt",
            Self::NickelFormat => "nickel_format",
            Self::Nimpretty => "nimpretty",
            Self::Nixfmt => "nixfmt",
            Self::NixpkgsFmt => "nixpkgs_fmt",
            Self::NomadFmt => "nomad_fmt",
            Self::Nph => "nph",
            Self::NpmGroovyLint => "npm_groovy_lint",
            Self::Nufmt => "nufmt",
            Self::Ocamlformat => "ocamlformat",
            Self::OcpIndent => "ocp_indent",
            Self::Odinfmt => "odinfmt",
            Self::OelintAdv => "oelint_adv",
            Self::OpaFmt => "opa_fmt",
            Self::Ormolu => "ormolu",
            Self::Oxlint => "oxlint",
            Self::PackerFix => "packer_fix",
            Self::PackerFmt => "packer_fmt",
            Self::Perltidy => "perltidy",
            Self::PgFormat => "pg_format",
            Self::PhpCsFixerFix => "php_cs_fixer_fix",
            Self::Phpcbf => "phpcbf",
            Self::PhpinsightsFix => "phpinsights_fix",
            Self::Pint => "pint",
            Self::Prettier => "prettier",
            Self::PrettyPhp => "pretty_php",
            Self::Prettypst => "prettypst",
            Self::PrismaFormat => "prisma_format",
            Self::Protolint => "protolint",
            Self::Ptop => "ptop",
            Self::PuppetLint => "puppet_lint",
            Self::PursTidy => "purs_tidy",
            Self::Purty => "purty",
            Self::Pycln => "pycln",
            Self::Pycodestyle => "pycodestyle",
            Self::Pyink => "pyink",
            Self::Pyment => "pyment",
            Self::Qmlfmt => "qmlfmt",
            Self::QuickLintJs => "quick_lint_js",
            Self::RacoFmt => "raco_fmt",
            Self::Refmt => "refmt",
            Self::ReformatGherkin => "reformat_gherkin",
            Self::RegalFix => "regal_fix",
            Self::RegalLint => "regal_lint",
            Self::ReorderPythonImports => "reorder_python_imports",
            Self::RescriptFormat => "rescript_format",
            Self::RocFormat => "roc_format",
            Self::Rstfmt => "rstfmt",
            Self::Rubocop => "rubocop",
            Self::Rubyfmt => "rubyfmt",
            Self::RuffCheck => "ruff_check",
            Self::RuffFormat => "ruff_format",
            Self::Rufo => "rufo",
            Self::RuneFmt => "rune_fmt",
            Self::Rustfmt => "rustfmt",
            Self::Rustywind => "rustywind",
            Self::SaltLint => "salt_lint",
            Self::Scalafmt => "scalafmt",
            Self::Scalariform => "scalariform",
            Self::Selene => "selene",
            Self::Shellcheck => "shellcheck",
            Self::Shellharden => "shellharden",
            Self::Shfmt => "shfmt",
            Self::Sleek => "sleek",
            Self::Smlfmt => "smlfmt",
            Self::Snakefmt => "snakefmt",
            Self::Solhint => "solhint",
            Self::SqlFormatter => "sql_formatter",
            Self::SqlfluffFix => "sqlfluff_fix",
            Self::SqlfluffFormat => "sqlfluff_format",
            Self::Sqlfmt => "sqlfmt",
            Self::Sqruff => "sqruff",
            Self::Standardjs => "standardjs",
            Self::Standardrb => "standardrb",
            Self::StatixCheck => "statix_check",
            Self::StatixFix => "statix_fix",
            Self::Stylefmt => "stylefmt",
            Self::Stylelint => "stylelint",
            Self::StylishHaskell => "stylish_haskell",
            Self::Stylua => "stylua",
            Self::SuperhtmlFmt => "superhtml_fmt",
            Self::SwiftFormat => "swift_format",
            Self::Swiftformat => "swiftformat",
            Self::Taplo => "taplo",
            Self::TemplFmt => "templ_fmt",
            Self::TerraformFmt => "terraform_fmt",
            Self::TerragruntHclfmt => "terragrunt_hclfmt",
            Self::TexFmt => "tex_fmt",
            Self::TlintFormat => "tlint_format",
            Self::TofuFmt => "tofu_fmt",
            Self::TomlSort => "toml_sort",
            Self::Topiary => "topiary",
            Self::TsStandard => "ts_standard",
            Self::Tsqllint => "tsqllint",
            Self::TwigCsFixerLint => "twig_cs_fixer_lint",
            Self::Typos => "typos",
            Self::Typstfmt => "typstfmt",
            Self::Typstyle => "typstyle",
            Self::Ufmt => "ufmt",
            Self::UiuaFmt => "uiua_fmt",
            Self::Unimport => "unimport",
            Self::Usort => "usort",
            Self::VFmt => "v_fmt",
            Self::VacuumLint => "vacuum_lint",
            Self::VerylFmt => "veryl_fmt",
            Self::VhdlStyleGuide => "vhdl_style_guide",
            Self::WaFmt => "wa_fmt",
            Self::Wfindent => "wfindent",
            Self::Xmlformat => "xmlformat",
            Self::Xmllint => "xmllint",
            Self::Xo => "xo",
            Self::Yamlfix => "yamlfix",
            Self::Yamlfmt => "yamlfmt",
            Self::Yapf => "yapf",
            Self::YewFmt => "yew_fmt",
            Self::ZigFmt => "zig_fmt",
            Self::ZiggyFmt => "ziggy_fmt",
            Self::Zprint => "zprint",
        }
    }
}

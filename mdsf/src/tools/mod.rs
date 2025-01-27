///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::execution::run_tools;

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
pub mod kcl_lint;
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

#[derive(serde::Serialize, serde::Deserialize, Hash, Clone, Copy)]
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
    /// `gci write --skip-generated --skip-vendor $PATH`
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

    #[serde(rename = "kcl:lint")]
    /// `kcl lint $PATH`
    KclLint,

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
        timeout: u64,
    ) -> Result<(bool, Option<String>), crate::error::MdsfError> {
        match self {
            Self::Actionlint => run_tools(
                &actionlint::COMMANDS,
                snippet_path,
                actionlint::set_arguments,
                timeout,
            ),
            Self::Alejandra => run_tools(
                &alejandra::COMMANDS,
                snippet_path,
                alejandra::set_arguments,
                timeout,
            ),
            Self::Ameba => run_tools(
                &ameba::COMMANDS,
                snippet_path,
                ameba::set_arguments,
                timeout,
            ),
            Self::AnsibleLint => run_tools(
                &ansible_lint::COMMANDS,
                snippet_path,
                ansible_lint::set_arguments,
                timeout,
            ),
            Self::Asmfmt => run_tools(
                &asmfmt::COMMANDS,
                snippet_path,
                asmfmt::set_arguments,
                timeout,
            ),
            Self::Astyle => run_tools(
                &astyle::COMMANDS,
                snippet_path,
                astyle::set_arguments,
                timeout,
            ),
            Self::AutoOptional => run_tools(
                &auto_optional::COMMANDS,
                snippet_path,
                auto_optional::set_arguments,
                timeout,
            ),
            Self::Autocorrect => run_tools(
                &autocorrect::COMMANDS,
                snippet_path,
                autocorrect::set_arguments,
                timeout,
            ),
            Self::Autoflake => run_tools(
                &autoflake::COMMANDS,
                snippet_path,
                autoflake::set_arguments,
                timeout,
            ),
            Self::Autopep8 => run_tools(
                &autopep_8::COMMANDS,
                snippet_path,
                autopep_8::set_arguments,
                timeout,
            ),
            Self::BeancountBlack => run_tools(
                &beancount_black::COMMANDS,
                snippet_path,
                beancount_black::set_arguments,
                timeout,
            ),
            Self::Beautysh => run_tools(
                &beautysh::COMMANDS,
                snippet_path,
                beautysh::set_arguments,
                timeout,
            ),
            Self::BibtexTidy => run_tools(
                &bibtex_tidy::COMMANDS,
                snippet_path,
                bibtex_tidy::set_arguments,
                timeout,
            ),
            Self::BicepFormat => run_tools(
                &bicep_format::COMMANDS,
                snippet_path,
                bicep_format::set_arguments,
                timeout,
            ),
            Self::BiomeCheck => run_tools(
                &biome_check::COMMANDS,
                snippet_path,
                biome_check::set_arguments,
                timeout,
            ),
            Self::BiomeFormat => run_tools(
                &biome_format::COMMANDS,
                snippet_path,
                biome_format::set_arguments,
                timeout,
            ),
            Self::BiomeLint => run_tools(
                &biome_lint::COMMANDS,
                snippet_path,
                biome_lint::set_arguments,
                timeout,
            ),
            Self::Black => run_tools(
                &black::COMMANDS,
                snippet_path,
                black::set_arguments,
                timeout,
            ),
            Self::BladeFormatter => run_tools(
                &blade_formatter::COMMANDS,
                snippet_path,
                blade_formatter::set_arguments,
                timeout,
            ),
            Self::Blue => run_tools(&blue::COMMANDS, snippet_path, blue::set_arguments, timeout),
            Self::Bpfmt => run_tools(
                &bpfmt::COMMANDS,
                snippet_path,
                bpfmt::set_arguments,
                timeout,
            ),
            Self::Brittany => run_tools(
                &brittany::COMMANDS,
                snippet_path,
                brittany::set_arguments,
                timeout,
            ),
            Self::Brunette => run_tools(
                &brunette::COMMANDS,
                snippet_path,
                brunette::set_arguments,
                timeout,
            ),
            Self::Bsfmt => run_tools(
                &bsfmt::COMMANDS,
                snippet_path,
                bsfmt::set_arguments,
                timeout,
            ),
            Self::Bslint => run_tools(
                &bslint::COMMANDS,
                snippet_path,
                bslint::set_arguments,
                timeout,
            ),
            Self::BufFormat => run_tools(
                &buf_format::COMMANDS,
                snippet_path,
                buf_format::set_arguments,
                timeout,
            ),
            Self::Buildifier => run_tools(
                &buildifier::COMMANDS,
                snippet_path,
                buildifier::set_arguments,
                timeout,
            ),
            Self::CabalFmt => run_tools(
                &cabal_fmt::COMMANDS,
                snippet_path,
                cabal_fmt::set_arguments,
                timeout,
            ),
            Self::CabalFormat => run_tools(
                &cabal_format::COMMANDS,
                snippet_path,
                cabal_format::set_arguments,
                timeout,
            ),
            Self::CabalPrettify => run_tools(
                &cabal_prettify::COMMANDS,
                snippet_path,
                cabal_prettify::set_arguments,
                timeout,
            ),
            Self::CaddyFmt => run_tools(
                &caddy_fmt::COMMANDS,
                snippet_path,
                caddy_fmt::set_arguments,
                timeout,
            ),
            Self::CaramelFmt => run_tools(
                &caramel_fmt::COMMANDS,
                snippet_path,
                caramel_fmt::set_arguments,
                timeout,
            ),
            Self::ClangFormat => run_tools(
                &clang_format::COMMANDS,
                snippet_path,
                clang_format::set_arguments,
                timeout,
            ),
            Self::ClangTidy => run_tools(
                &clang_tidy::COMMANDS,
                snippet_path,
                clang_tidy::set_arguments,
                timeout,
            ),
            Self::CljfmtFix => run_tools(
                &cljfmt_fix::COMMANDS,
                snippet_path,
                cljfmt_fix::set_arguments,
                timeout,
            ),
            Self::Cljstyle => run_tools(
                &cljstyle::COMMANDS,
                snippet_path,
                cljstyle::set_arguments,
                timeout,
            ),
            Self::CmakeFormat => run_tools(
                &cmake_format::COMMANDS,
                snippet_path,
                cmake_format::set_arguments,
                timeout,
            ),
            Self::Codespell => run_tools(
                &codespell::COMMANDS,
                snippet_path,
                codespell::set_arguments,
                timeout,
            ),
            Self::Crlfmt => run_tools(
                &crlfmt::COMMANDS,
                snippet_path,
                crlfmt::set_arguments,
                timeout,
            ),
            Self::CrystalFormat => run_tools(
                &crystal_format::COMMANDS,
                snippet_path,
                crystal_format::set_arguments,
                timeout,
            ),
            Self::Csharpier => run_tools(
                &csharpier::COMMANDS,
                snippet_path,
                csharpier::set_arguments,
                timeout,
            ),
            Self::CssBeautify => run_tools(
                &css_beautify::COMMANDS,
                snippet_path,
                css_beautify::set_arguments,
                timeout,
            ),
            Self::Csscomb => run_tools(
                &csscomb::COMMANDS,
                snippet_path,
                csscomb::set_arguments,
                timeout,
            ),
            Self::Csslint => run_tools(
                &csslint::COMMANDS,
                snippet_path,
                csslint::set_arguments,
                timeout,
            ),
            Self::Curlylint => run_tools(
                &curlylint::COMMANDS,
                snippet_path,
                curlylint::set_arguments,
                timeout,
            ),
            Self::D2Fmt => run_tools(
                &d_2_fmt::COMMANDS,
                snippet_path,
                d_2_fmt::set_arguments,
                timeout,
            ),
            Self::DartFix => run_tools(
                &dart_fix::COMMANDS,
                snippet_path,
                dart_fix::set_arguments,
                timeout,
            ),
            Self::DartFormat => run_tools(
                &dart_format::COMMANDS,
                snippet_path,
                dart_format::set_arguments,
                timeout,
            ),
            Self::DcmFix => run_tools(
                &dcm_fix::COMMANDS,
                snippet_path,
                dcm_fix::set_arguments,
                timeout,
            ),
            Self::DcmFormat => run_tools(
                &dcm_format::COMMANDS,
                snippet_path,
                dcm_format::set_arguments,
                timeout,
            ),
            Self::Deadnix => run_tools(
                &deadnix::COMMANDS,
                snippet_path,
                deadnix::set_arguments,
                timeout,
            ),
            Self::DenoFmt => run_tools(
                &deno_fmt::COMMANDS,
                snippet_path,
                deno_fmt::set_arguments,
                timeout,
            ),
            Self::DenoLint => run_tools(
                &deno_lint::COMMANDS,
                snippet_path,
                deno_lint::set_arguments,
                timeout,
            ),
            Self::Dfmt => run_tools(&dfmt::COMMANDS, snippet_path, dfmt::set_arguments, timeout),
            Self::Dhall => run_tools(
                &dhall::COMMANDS,
                snippet_path,
                dhall::set_arguments,
                timeout,
            ),
            Self::Djade => run_tools(
                &djade::COMMANDS,
                snippet_path,
                djade::set_arguments,
                timeout,
            ),
            Self::Djlint => run_tools(
                &djlint::COMMANDS,
                snippet_path,
                djlint::set_arguments,
                timeout,
            ),
            Self::Docformatter => run_tools(
                &docformatter::COMMANDS,
                snippet_path,
                docformatter::set_arguments,
                timeout,
            ),
            Self::Dockfmt => run_tools(
                &dockfmt::COMMANDS,
                snippet_path,
                dockfmt::set_arguments,
                timeout,
            ),
            Self::Docstrfmt => run_tools(
                &docstrfmt::COMMANDS,
                snippet_path,
                docstrfmt::set_arguments,
                timeout,
            ),
            Self::Doctoc => run_tools(
                &doctoc::COMMANDS,
                snippet_path,
                doctoc::set_arguments,
                timeout,
            ),
            Self::DotenvLinterFix => run_tools(
                &dotenv_linter_fix::COMMANDS,
                snippet_path,
                dotenv_linter_fix::set_arguments,
                timeout,
            ),
            Self::DprintFmt => run_tools(
                &dprint_fmt::COMMANDS,
                snippet_path,
                dprint_fmt::set_arguments,
                timeout,
            ),
            Self::EasyCodingStandard => run_tools(
                &easy_coding_standard::COMMANDS,
                snippet_path,
                easy_coding_standard::set_arguments,
                timeout,
            ),
            Self::Efmt => run_tools(&efmt::COMMANDS, snippet_path, efmt::set_arguments, timeout),
            Self::ElmFormat => run_tools(
                &elm_format::COMMANDS,
                snippet_path,
                elm_format::set_arguments,
                timeout,
            ),
            Self::ErbFormatter => run_tools(
                &erb_formatter::COMMANDS,
                snippet_path,
                erb_formatter::set_arguments,
                timeout,
            ),
            Self::Erlfmt => run_tools(
                &erlfmt::COMMANDS,
                snippet_path,
                erlfmt::set_arguments,
                timeout,
            ),
            Self::Eslint => run_tools(
                &eslint::COMMANDS,
                snippet_path,
                eslint::set_arguments,
                timeout,
            ),
            Self::Fantomas => run_tools(
                &fantomas::COMMANDS,
                snippet_path,
                fantomas::set_arguments,
                timeout,
            ),
            Self::FishIndent => run_tools(
                &fish_indent::COMMANDS,
                snippet_path,
                fish_indent::set_arguments,
                timeout,
            ),
            Self::Fixjson => run_tools(
                &fixjson::COMMANDS,
                snippet_path,
                fixjson::set_arguments,
                timeout,
            ),
            Self::Floskell => run_tools(
                &floskell::COMMANDS,
                snippet_path,
                floskell::set_arguments,
                timeout,
            ),
            Self::Fnlfmt => run_tools(
                &fnlfmt::COMMANDS,
                snippet_path,
                fnlfmt::set_arguments,
                timeout,
            ),
            Self::ForgeFmt => run_tools(
                &forge_fmt::COMMANDS,
                snippet_path,
                forge_fmt::set_arguments,
                timeout,
            ),
            Self::Fourmolu => run_tools(
                &fourmolu::COMMANDS,
                snippet_path,
                fourmolu::set_arguments,
                timeout,
            ),
            Self::Fprettify => run_tools(
                &fprettify::COMMANDS,
                snippet_path,
                fprettify::set_arguments,
                timeout,
            ),
            Self::FutharkFmt => run_tools(
                &futhark_fmt::COMMANDS,
                snippet_path,
                futhark_fmt::set_arguments,
                timeout,
            ),
            Self::Gci => run_tools(&gci::COMMANDS, snippet_path, gci::set_arguments, timeout),
            Self::Gdformat => run_tools(
                &gdformat::COMMANDS,
                snippet_path,
                gdformat::set_arguments,
                timeout,
            ),
            Self::Gersemi => run_tools(
                &gersemi::COMMANDS,
                snippet_path,
                gersemi::set_arguments,
                timeout,
            ),
            Self::GleamFormat => run_tools(
                &gleam_format::COMMANDS,
                snippet_path,
                gleam_format::set_arguments,
                timeout,
            ),
            Self::GluonFmt => run_tools(
                &gluon_fmt::COMMANDS,
                snippet_path,
                gluon_fmt::set_arguments,
                timeout,
            ),
            Self::Gofmt => run_tools(
                &gofmt::COMMANDS,
                snippet_path,
                gofmt::set_arguments,
                timeout,
            ),
            Self::Gofumpt => run_tools(
                &gofumpt::COMMANDS,
                snippet_path,
                gofumpt::set_arguments,
                timeout,
            ),
            Self::Goimports => run_tools(
                &goimports::COMMANDS,
                snippet_path,
                goimports::set_arguments,
                timeout,
            ),
            Self::GoimportsReviser => run_tools(
                &goimports_reviser::COMMANDS,
                snippet_path,
                goimports_reviser::set_arguments,
                timeout,
            ),
            Self::Golines => run_tools(
                &golines::COMMANDS,
                snippet_path,
                golines::set_arguments,
                timeout,
            ),
            Self::GoogleJavaFormat => run_tools(
                &google_java_format::COMMANDS,
                snippet_path,
                google_java_format::set_arguments,
                timeout,
            ),
            Self::GrainFormat => run_tools(
                &grain_format::COMMANDS,
                snippet_path,
                grain_format::set_arguments,
                timeout,
            ),
            Self::Hadolint => run_tools(
                &hadolint::COMMANDS,
                snippet_path,
                hadolint::set_arguments,
                timeout,
            ),
            Self::HamlLint => run_tools(
                &haml_lint::COMMANDS,
                snippet_path,
                haml_lint::set_arguments,
                timeout,
            ),
            Self::Hclfmt => run_tools(
                &hclfmt::COMMANDS,
                snippet_path,
                hclfmt::set_arguments,
                timeout,
            ),
            Self::Hfmt => run_tools(&hfmt::COMMANDS, snippet_path, hfmt::set_arguments, timeout),
            Self::Hindent => run_tools(
                &hindent::COMMANDS,
                snippet_path,
                hindent::set_arguments,
                timeout,
            ),
            Self::Hlint => run_tools(
                &hlint::COMMANDS,
                snippet_path,
                hlint::set_arguments,
                timeout,
            ),
            Self::HtmlBeautify => run_tools(
                &html_beautify::COMMANDS,
                snippet_path,
                html_beautify::set_arguments,
                timeout,
            ),
            Self::Htmlbeautifier => run_tools(
                &htmlbeautifier::COMMANDS,
                snippet_path,
                htmlbeautifier::set_arguments,
                timeout,
            ),
            Self::Htmlhint => run_tools(
                &htmlhint::COMMANDS,
                snippet_path,
                htmlhint::set_arguments,
                timeout,
            ),
            Self::ImbaFmt => run_tools(
                &imba_fmt::COMMANDS,
                snippet_path,
                imba_fmt::set_arguments,
                timeout,
            ),
            Self::InkoFmt => run_tools(
                &inko_fmt::COMMANDS,
                snippet_path,
                inko_fmt::set_arguments,
                timeout,
            ),
            Self::Isort => run_tools(
                &isort::COMMANDS,
                snippet_path,
                isort::set_arguments,
                timeout,
            ),
            Self::Joker => run_tools(
                &joker::COMMANDS,
                snippet_path,
                joker::set_arguments,
                timeout,
            ),
            Self::JsBeautify => run_tools(
                &js_beautify::COMMANDS,
                snippet_path,
                js_beautify::set_arguments,
                timeout,
            ),
            Self::Json5Format => run_tools(
                &json_5_format::COMMANDS,
                snippet_path,
                json_5_format::set_arguments,
                timeout,
            ),
            Self::JsonaFormat => run_tools(
                &jsona_format::COMMANDS,
                snippet_path,
                jsona_format::set_arguments,
                timeout,
            ),
            Self::JsonaLint => run_tools(
                &jsona_lint::COMMANDS,
                snippet_path,
                jsona_lint::set_arguments,
                timeout,
            ),
            Self::Jsonlint => run_tools(
                &jsonlint::COMMANDS,
                snippet_path,
                jsonlint::set_arguments,
                timeout,
            ),
            Self::JsonnetLint => run_tools(
                &jsonnet_lint::COMMANDS,
                snippet_path,
                jsonnet_lint::set_arguments,
                timeout,
            ),
            Self::Jsonnetfmt => run_tools(
                &jsonnetfmt::COMMANDS,
                snippet_path,
                jsonnetfmt::set_arguments,
                timeout,
            ),
            Self::JuliaformatterJl => run_tools(
                &juliaformatter_jl::COMMANDS,
                snippet_path,
                juliaformatter_jl::set_arguments,
                timeout,
            ),
            Self::Just => run_tools(&just::COMMANDS, snippet_path, just::set_arguments, timeout),
            Self::KclFmt => run_tools(
                &kcl_fmt::COMMANDS,
                snippet_path,
                kcl_fmt::set_arguments,
                timeout,
            ),
            Self::KclLint => run_tools(
                &kcl_lint::COMMANDS,
                snippet_path,
                kcl_lint::set_arguments,
                timeout,
            ),
            Self::Kdlfmt => run_tools(
                &kdlfmt::COMMANDS,
                snippet_path,
                kdlfmt::set_arguments,
                timeout,
            ),
            Self::KdocFormatter => run_tools(
                &kdoc_formatter::COMMANDS,
                snippet_path,
                kdoc_formatter::set_arguments,
                timeout,
            ),
            Self::Ktfmt => run_tools(
                &ktfmt::COMMANDS,
                snippet_path,
                ktfmt::set_arguments,
                timeout,
            ),
            Self::Ktlint => run_tools(
                &ktlint::COMMANDS,
                snippet_path,
                ktlint::set_arguments,
                timeout,
            ),
            Self::KulalaFmt => run_tools(
                &kulala_fmt::COMMANDS,
                snippet_path,
                kulala_fmt::set_arguments,
                timeout,
            ),
            Self::Leptosfmt => run_tools(
                &leptosfmt::COMMANDS,
                snippet_path,
                leptosfmt::set_arguments,
                timeout,
            ),
            Self::LiquidsoapPrettier => run_tools(
                &liquidsoap_prettier::COMMANDS,
                snippet_path,
                liquidsoap_prettier::set_arguments,
                timeout,
            ),
            Self::Luacheck => run_tools(
                &luacheck::COMMANDS,
                snippet_path,
                luacheck::set_arguments,
                timeout,
            ),
            Self::Luaformatter => run_tools(
                &luaformatter::COMMANDS,
                snippet_path,
                luaformatter::set_arguments,
                timeout,
            ),
            Self::MadoCheck => run_tools(
                &mado_check::COMMANDS,
                snippet_path,
                mado_check::set_arguments,
                timeout,
            ),
            Self::Markdownfmt => run_tools(
                &markdownfmt::COMMANDS,
                snippet_path,
                markdownfmt::set_arguments,
                timeout,
            ),
            Self::Markdownlint => run_tools(
                &markdownlint::COMMANDS,
                snippet_path,
                markdownlint::set_arguments,
                timeout,
            ),
            Self::MarkdownlintCli2 => run_tools(
                &markdownlint_cli_2::COMMANDS,
                snippet_path,
                markdownlint_cli_2::set_arguments,
                timeout,
            ),
            Self::Markuplint => run_tools(
                &markuplint::COMMANDS,
                snippet_path,
                markuplint::set_arguments,
                timeout,
            ),
            Self::Mdformat => run_tools(
                &mdformat::COMMANDS,
                snippet_path,
                mdformat::set_arguments,
                timeout,
            ),
            Self::Mdslw => run_tools(
                &mdslw::COMMANDS,
                snippet_path,
                mdslw::set_arguments,
                timeout,
            ),
            Self::MesonFmt => run_tools(
                &meson_fmt::COMMANDS,
                snippet_path,
                meson_fmt::set_arguments,
                timeout,
            ),
            Self::Misspell => run_tools(
                &misspell::COMMANDS,
                snippet_path,
                misspell::set_arguments,
                timeout,
            ),
            Self::MixFormat => run_tools(
                &mix_format::COMMANDS,
                snippet_path,
                mix_format::set_arguments,
                timeout,
            ),
            Self::MojoFormat => run_tools(
                &mojo_format::COMMANDS,
                snippet_path,
                mojo_format::set_arguments,
                timeout,
            ),
            Self::Mypy => run_tools(&mypy::COMMANDS, snippet_path, mypy::set_arguments, timeout),
            Self::Nginxbeautifier => run_tools(
                &nginxbeautifier::COMMANDS,
                snippet_path,
                nginxbeautifier::set_arguments,
                timeout,
            ),
            Self::Nginxfmt => run_tools(
                &nginxfmt::COMMANDS,
                snippet_path,
                nginxfmt::set_arguments,
                timeout,
            ),
            Self::NickelFormat => run_tools(
                &nickel_format::COMMANDS,
                snippet_path,
                nickel_format::set_arguments,
                timeout,
            ),
            Self::Nimpretty => run_tools(
                &nimpretty::COMMANDS,
                snippet_path,
                nimpretty::set_arguments,
                timeout,
            ),
            Self::Nixfmt => run_tools(
                &nixfmt::COMMANDS,
                snippet_path,
                nixfmt::set_arguments,
                timeout,
            ),
            Self::NixpkgsFmt => run_tools(
                &nixpkgs_fmt::COMMANDS,
                snippet_path,
                nixpkgs_fmt::set_arguments,
                timeout,
            ),
            Self::NomadFmt => run_tools(
                &nomad_fmt::COMMANDS,
                snippet_path,
                nomad_fmt::set_arguments,
                timeout,
            ),
            Self::Nph => run_tools(&nph::COMMANDS, snippet_path, nph::set_arguments, timeout),
            Self::NpmGroovyLint => run_tools(
                &npm_groovy_lint::COMMANDS,
                snippet_path,
                npm_groovy_lint::set_arguments,
                timeout,
            ),
            Self::Nufmt => run_tools(
                &nufmt::COMMANDS,
                snippet_path,
                nufmt::set_arguments,
                timeout,
            ),
            Self::Ocamlformat => run_tools(
                &ocamlformat::COMMANDS,
                snippet_path,
                ocamlformat::set_arguments,
                timeout,
            ),
            Self::OcpIndent => run_tools(
                &ocp_indent::COMMANDS,
                snippet_path,
                ocp_indent::set_arguments,
                timeout,
            ),
            Self::Odinfmt => run_tools(
                &odinfmt::COMMANDS,
                snippet_path,
                odinfmt::set_arguments,
                timeout,
            ),
            Self::OelintAdv => run_tools(
                &oelint_adv::COMMANDS,
                snippet_path,
                oelint_adv::set_arguments,
                timeout,
            ),
            Self::OpaFmt => run_tools(
                &opa_fmt::COMMANDS,
                snippet_path,
                opa_fmt::set_arguments,
                timeout,
            ),
            Self::Ormolu => run_tools(
                &ormolu::COMMANDS,
                snippet_path,
                ormolu::set_arguments,
                timeout,
            ),
            Self::Oxlint => run_tools(
                &oxlint::COMMANDS,
                snippet_path,
                oxlint::set_arguments,
                timeout,
            ),
            Self::PackerFix => run_tools(
                &packer_fix::COMMANDS,
                snippet_path,
                packer_fix::set_arguments,
                timeout,
            ),
            Self::PackerFmt => run_tools(
                &packer_fmt::COMMANDS,
                snippet_path,
                packer_fmt::set_arguments,
                timeout,
            ),
            Self::Perltidy => run_tools(
                &perltidy::COMMANDS,
                snippet_path,
                perltidy::set_arguments,
                timeout,
            ),
            Self::PgFormat => run_tools(
                &pg_format::COMMANDS,
                snippet_path,
                pg_format::set_arguments,
                timeout,
            ),
            Self::PhpCsFixerFix => run_tools(
                &php_cs_fixer_fix::COMMANDS,
                snippet_path,
                php_cs_fixer_fix::set_arguments,
                timeout,
            ),
            Self::Phpcbf => run_tools(
                &phpcbf::COMMANDS,
                snippet_path,
                phpcbf::set_arguments,
                timeout,
            ),
            Self::PhpinsightsFix => run_tools(
                &phpinsights_fix::COMMANDS,
                snippet_path,
                phpinsights_fix::set_arguments,
                timeout,
            ),
            Self::Pint => run_tools(&pint::COMMANDS, snippet_path, pint::set_arguments, timeout),
            Self::Prettier => run_tools(
                &prettier::COMMANDS,
                snippet_path,
                prettier::set_arguments,
                timeout,
            ),
            Self::PrettyPhp => run_tools(
                &pretty_php::COMMANDS,
                snippet_path,
                pretty_php::set_arguments,
                timeout,
            ),
            Self::Prettypst => run_tools(
                &prettypst::COMMANDS,
                snippet_path,
                prettypst::set_arguments,
                timeout,
            ),
            Self::PrismaFormat => run_tools(
                &prisma_format::COMMANDS,
                snippet_path,
                prisma_format::set_arguments,
                timeout,
            ),
            Self::Protolint => run_tools(
                &protolint::COMMANDS,
                snippet_path,
                protolint::set_arguments,
                timeout,
            ),
            Self::Ptop => run_tools(&ptop::COMMANDS, snippet_path, ptop::set_arguments, timeout),
            Self::PuppetLint => run_tools(
                &puppet_lint::COMMANDS,
                snippet_path,
                puppet_lint::set_arguments,
                timeout,
            ),
            Self::PursTidy => run_tools(
                &purs_tidy::COMMANDS,
                snippet_path,
                purs_tidy::set_arguments,
                timeout,
            ),
            Self::Purty => run_tools(
                &purty::COMMANDS,
                snippet_path,
                purty::set_arguments,
                timeout,
            ),
            Self::Pycln => run_tools(
                &pycln::COMMANDS,
                snippet_path,
                pycln::set_arguments,
                timeout,
            ),
            Self::Pycodestyle => run_tools(
                &pycodestyle::COMMANDS,
                snippet_path,
                pycodestyle::set_arguments,
                timeout,
            ),
            Self::Pyink => run_tools(
                &pyink::COMMANDS,
                snippet_path,
                pyink::set_arguments,
                timeout,
            ),
            Self::Pyment => run_tools(
                &pyment::COMMANDS,
                snippet_path,
                pyment::set_arguments,
                timeout,
            ),
            Self::Qmlfmt => run_tools(
                &qmlfmt::COMMANDS,
                snippet_path,
                qmlfmt::set_arguments,
                timeout,
            ),
            Self::QuickLintJs => run_tools(
                &quick_lint_js::COMMANDS,
                snippet_path,
                quick_lint_js::set_arguments,
                timeout,
            ),
            Self::RacoFmt => run_tools(
                &raco_fmt::COMMANDS,
                snippet_path,
                raco_fmt::set_arguments,
                timeout,
            ),
            Self::Refmt => run_tools(
                &refmt::COMMANDS,
                snippet_path,
                refmt::set_arguments,
                timeout,
            ),
            Self::ReformatGherkin => run_tools(
                &reformat_gherkin::COMMANDS,
                snippet_path,
                reformat_gherkin::set_arguments,
                timeout,
            ),
            Self::RegalFix => run_tools(
                &regal_fix::COMMANDS,
                snippet_path,
                regal_fix::set_arguments,
                timeout,
            ),
            Self::RegalLint => run_tools(
                &regal_lint::COMMANDS,
                snippet_path,
                regal_lint::set_arguments,
                timeout,
            ),
            Self::ReorderPythonImports => run_tools(
                &reorder_python_imports::COMMANDS,
                snippet_path,
                reorder_python_imports::set_arguments,
                timeout,
            ),
            Self::RescriptFormat => run_tools(
                &rescript_format::COMMANDS,
                snippet_path,
                rescript_format::set_arguments,
                timeout,
            ),
            Self::RocFormat => run_tools(
                &roc_format::COMMANDS,
                snippet_path,
                roc_format::set_arguments,
                timeout,
            ),
            Self::Rstfmt => run_tools(
                &rstfmt::COMMANDS,
                snippet_path,
                rstfmt::set_arguments,
                timeout,
            ),
            Self::Rubocop => run_tools(
                &rubocop::COMMANDS,
                snippet_path,
                rubocop::set_arguments,
                timeout,
            ),
            Self::Rubyfmt => run_tools(
                &rubyfmt::COMMANDS,
                snippet_path,
                rubyfmt::set_arguments,
                timeout,
            ),
            Self::RuffCheck => run_tools(
                &ruff_check::COMMANDS,
                snippet_path,
                ruff_check::set_arguments,
                timeout,
            ),
            Self::RuffFormat => run_tools(
                &ruff_format::COMMANDS,
                snippet_path,
                ruff_format::set_arguments,
                timeout,
            ),
            Self::Rufo => run_tools(&rufo::COMMANDS, snippet_path, rufo::set_arguments, timeout),
            Self::RuneFmt => run_tools(
                &rune_fmt::COMMANDS,
                snippet_path,
                rune_fmt::set_arguments,
                timeout,
            ),
            Self::Rustfmt => run_tools(
                &rustfmt::COMMANDS,
                snippet_path,
                rustfmt::set_arguments,
                timeout,
            ),
            Self::Rustywind => run_tools(
                &rustywind::COMMANDS,
                snippet_path,
                rustywind::set_arguments,
                timeout,
            ),
            Self::SaltLint => run_tools(
                &salt_lint::COMMANDS,
                snippet_path,
                salt_lint::set_arguments,
                timeout,
            ),
            Self::Scalafmt => run_tools(
                &scalafmt::COMMANDS,
                snippet_path,
                scalafmt::set_arguments,
                timeout,
            ),
            Self::Scalariform => run_tools(
                &scalariform::COMMANDS,
                snippet_path,
                scalariform::set_arguments,
                timeout,
            ),
            Self::Selene => run_tools(
                &selene::COMMANDS,
                snippet_path,
                selene::set_arguments,
                timeout,
            ),
            Self::Shellcheck => run_tools(
                &shellcheck::COMMANDS,
                snippet_path,
                shellcheck::set_arguments,
                timeout,
            ),
            Self::Shellharden => run_tools(
                &shellharden::COMMANDS,
                snippet_path,
                shellharden::set_arguments,
                timeout,
            ),
            Self::Shfmt => run_tools(
                &shfmt::COMMANDS,
                snippet_path,
                shfmt::set_arguments,
                timeout,
            ),
            Self::Sleek => run_tools(
                &sleek::COMMANDS,
                snippet_path,
                sleek::set_arguments,
                timeout,
            ),
            Self::Smlfmt => run_tools(
                &smlfmt::COMMANDS,
                snippet_path,
                smlfmt::set_arguments,
                timeout,
            ),
            Self::Snakefmt => run_tools(
                &snakefmt::COMMANDS,
                snippet_path,
                snakefmt::set_arguments,
                timeout,
            ),
            Self::Solhint => run_tools(
                &solhint::COMMANDS,
                snippet_path,
                solhint::set_arguments,
                timeout,
            ),
            Self::SqlFormatter => run_tools(
                &sql_formatter::COMMANDS,
                snippet_path,
                sql_formatter::set_arguments,
                timeout,
            ),
            Self::SqlfluffFix => run_tools(
                &sqlfluff_fix::COMMANDS,
                snippet_path,
                sqlfluff_fix::set_arguments,
                timeout,
            ),
            Self::SqlfluffFormat => run_tools(
                &sqlfluff_format::COMMANDS,
                snippet_path,
                sqlfluff_format::set_arguments,
                timeout,
            ),
            Self::Sqlfmt => run_tools(
                &sqlfmt::COMMANDS,
                snippet_path,
                sqlfmt::set_arguments,
                timeout,
            ),
            Self::Sqruff => run_tools(
                &sqruff::COMMANDS,
                snippet_path,
                sqruff::set_arguments,
                timeout,
            ),
            Self::Standardjs => run_tools(
                &standardjs::COMMANDS,
                snippet_path,
                standardjs::set_arguments,
                timeout,
            ),
            Self::Standardrb => run_tools(
                &standardrb::COMMANDS,
                snippet_path,
                standardrb::set_arguments,
                timeout,
            ),
            Self::StatixCheck => run_tools(
                &statix_check::COMMANDS,
                snippet_path,
                statix_check::set_arguments,
                timeout,
            ),
            Self::StatixFix => run_tools(
                &statix_fix::COMMANDS,
                snippet_path,
                statix_fix::set_arguments,
                timeout,
            ),
            Self::Stylefmt => run_tools(
                &stylefmt::COMMANDS,
                snippet_path,
                stylefmt::set_arguments,
                timeout,
            ),
            Self::Stylelint => run_tools(
                &stylelint::COMMANDS,
                snippet_path,
                stylelint::set_arguments,
                timeout,
            ),
            Self::StylishHaskell => run_tools(
                &stylish_haskell::COMMANDS,
                snippet_path,
                stylish_haskell::set_arguments,
                timeout,
            ),
            Self::Stylua => run_tools(
                &stylua::COMMANDS,
                snippet_path,
                stylua::set_arguments,
                timeout,
            ),
            Self::SuperhtmlFmt => run_tools(
                &superhtml_fmt::COMMANDS,
                snippet_path,
                superhtml_fmt::set_arguments,
                timeout,
            ),
            Self::SwiftFormat => run_tools(
                &swift_format::COMMANDS,
                snippet_path,
                swift_format::set_arguments,
                timeout,
            ),
            Self::Swiftformat => run_tools(
                &swiftformat::COMMANDS,
                snippet_path,
                swiftformat::set_arguments,
                timeout,
            ),
            Self::Taplo => run_tools(
                &taplo::COMMANDS,
                snippet_path,
                taplo::set_arguments,
                timeout,
            ),
            Self::TemplFmt => run_tools(
                &templ_fmt::COMMANDS,
                snippet_path,
                templ_fmt::set_arguments,
                timeout,
            ),
            Self::TerraformFmt => run_tools(
                &terraform_fmt::COMMANDS,
                snippet_path,
                terraform_fmt::set_arguments,
                timeout,
            ),
            Self::TerragruntHclfmt => run_tools(
                &terragrunt_hclfmt::COMMANDS,
                snippet_path,
                terragrunt_hclfmt::set_arguments,
                timeout,
            ),
            Self::TexFmt => run_tools(
                &tex_fmt::COMMANDS,
                snippet_path,
                tex_fmt::set_arguments,
                timeout,
            ),
            Self::TlintFormat => run_tools(
                &tlint_format::COMMANDS,
                snippet_path,
                tlint_format::set_arguments,
                timeout,
            ),
            Self::TofuFmt => run_tools(
                &tofu_fmt::COMMANDS,
                snippet_path,
                tofu_fmt::set_arguments,
                timeout,
            ),
            Self::TomlSort => run_tools(
                &toml_sort::COMMANDS,
                snippet_path,
                toml_sort::set_arguments,
                timeout,
            ),
            Self::Topiary => run_tools(
                &topiary::COMMANDS,
                snippet_path,
                topiary::set_arguments,
                timeout,
            ),
            Self::TsStandard => run_tools(
                &ts_standard::COMMANDS,
                snippet_path,
                ts_standard::set_arguments,
                timeout,
            ),
            Self::Tsqllint => run_tools(
                &tsqllint::COMMANDS,
                snippet_path,
                tsqllint::set_arguments,
                timeout,
            ),
            Self::TwigCsFixerLint => run_tools(
                &twig_cs_fixer_lint::COMMANDS,
                snippet_path,
                twig_cs_fixer_lint::set_arguments,
                timeout,
            ),
            Self::Typos => run_tools(
                &typos::COMMANDS,
                snippet_path,
                typos::set_arguments,
                timeout,
            ),
            Self::Typstfmt => run_tools(
                &typstfmt::COMMANDS,
                snippet_path,
                typstfmt::set_arguments,
                timeout,
            ),
            Self::Typstyle => run_tools(
                &typstyle::COMMANDS,
                snippet_path,
                typstyle::set_arguments,
                timeout,
            ),
            Self::Ufmt => run_tools(&ufmt::COMMANDS, snippet_path, ufmt::set_arguments, timeout),
            Self::UiuaFmt => run_tools(
                &uiua_fmt::COMMANDS,
                snippet_path,
                uiua_fmt::set_arguments,
                timeout,
            ),
            Self::Unimport => run_tools(
                &unimport::COMMANDS,
                snippet_path,
                unimport::set_arguments,
                timeout,
            ),
            Self::Usort => run_tools(
                &usort::COMMANDS,
                snippet_path,
                usort::set_arguments,
                timeout,
            ),
            Self::VFmt => run_tools(
                &v_fmt::COMMANDS,
                snippet_path,
                v_fmt::set_arguments,
                timeout,
            ),
            Self::VacuumLint => run_tools(
                &vacuum_lint::COMMANDS,
                snippet_path,
                vacuum_lint::set_arguments,
                timeout,
            ),
            Self::VerylFmt => run_tools(
                &veryl_fmt::COMMANDS,
                snippet_path,
                veryl_fmt::set_arguments,
                timeout,
            ),
            Self::VhdlStyleGuide => run_tools(
                &vhdl_style_guide::COMMANDS,
                snippet_path,
                vhdl_style_guide::set_arguments,
                timeout,
            ),
            Self::WaFmt => run_tools(
                &wa_fmt::COMMANDS,
                snippet_path,
                wa_fmt::set_arguments,
                timeout,
            ),
            Self::Wfindent => run_tools(
                &wfindent::COMMANDS,
                snippet_path,
                wfindent::set_arguments,
                timeout,
            ),
            Self::Xmlformat => run_tools(
                &xmlformat::COMMANDS,
                snippet_path,
                xmlformat::set_arguments,
                timeout,
            ),
            Self::Xmllint => run_tools(
                &xmllint::COMMANDS,
                snippet_path,
                xmllint::set_arguments,
                timeout,
            ),
            Self::Xo => run_tools(&xo::COMMANDS, snippet_path, xo::set_arguments, timeout),
            Self::Yamlfix => run_tools(
                &yamlfix::COMMANDS,
                snippet_path,
                yamlfix::set_arguments,
                timeout,
            ),
            Self::Yamlfmt => run_tools(
                &yamlfmt::COMMANDS,
                snippet_path,
                yamlfmt::set_arguments,
                timeout,
            ),
            Self::Yapf => run_tools(&yapf::COMMANDS, snippet_path, yapf::set_arguments, timeout),
            Self::YewFmt => run_tools(
                &yew_fmt::COMMANDS,
                snippet_path,
                yew_fmt::set_arguments,
                timeout,
            ),
            Self::ZigFmt => run_tools(
                &zig_fmt::COMMANDS,
                snippet_path,
                zig_fmt::set_arguments,
                timeout,
            ),
            Self::ZiggyFmt => run_tools(
                &ziggy_fmt::COMMANDS,
                snippet_path,
                ziggy_fmt::set_arguments,
                timeout,
            ),
            Self::Zprint => run_tools(
                &zprint::COMMANDS,
                snippet_path,
                zprint::set_arguments,
                timeout,
            ),
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
            Self::KclLint => "kcl_lint",
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

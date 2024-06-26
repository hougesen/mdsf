use std::{ffi::OsStr, io::Write, process::Command, str::FromStr};

use tempfile::NamedTempFile;
use which::which;

use crate::{
    config::MdsfConfig,
    error::MdsfError,
    generated::{self, language_to_ext},
    terminal::{
        print_binary_not_in_path, print_error_formatting, print_formatter_info,
        print_formatter_time,
    },
    LineInfo, DEBUG,
};

mod alejandra;
mod asmfmt;
mod astyle;
mod auto_optional;
mod autocorrect;
mod autoflake;
mod autopep8;
mod beautysh;
mod bicep_format;
mod biome;
mod black;
mod blade_formatter;
mod blue;
mod bpfmt;
mod bsfmt;
mod buf;
mod buildifier;
mod cabal_format;
mod caramel;
mod clang_format;
mod clang_tidy;
mod cljstyle;
mod codespell;
mod crlfmt;
mod crystal_format;
mod csharpier;
mod d2;
mod dart;
mod dcm;
mod deno;
mod dfmt;
mod djlint;
mod docformatter;
mod docstrfmt;
mod dotenv_linter;
mod dprint;
mod efmt;
mod elm_format;
mod erb_formatter;
mod erlfmt;
mod eslint;
mod fantomas;
mod findent;
mod fish_indent;
mod fixjson;
mod fnlfmt;
mod forge_fmt;
mod fourmolu;
mod fprettify;
mod gci;
mod gdformat;
mod gersemi;
mod gleam_format;
mod gluon;
mod gofmt;
mod gofumpt;
mod goimports;
mod goimports_reviser;
mod golines;
mod google_java_format;
mod grain;
mod haml_lint;
mod hindent;
mod htmlbeautifier;
mod imba;
mod isort;
mod joker;
mod jsona;
mod juliaformatter_jl;
mod just_fmt;
mod kcl_fmt;
mod kdlfmt;
mod ktfmt;
mod ktlint;
mod leptosfmt;
mod luaformatter;
mod markdownlint;
mod markuplint;
mod mdformat;
mod misspell;
mod mix_format;
mod nickel;
mod nimpretty;
mod nixfmt;
mod nixpkgs_fmt;
mod npm_groovy_lint;
mod ocamlformat;
mod ocp_indent;
mod ormolu;
mod oxlint;
mod perltidy;
mod pg_format;
mod prettier;
mod puppet_lint;
mod purs_tidy;
mod pyink;
mod raco;
mod rescript_format;
mod roc_format;
mod rstfmt;
mod rubocop;
mod rubyfmt;
mod ruff;
mod rufo;
mod rune;
mod rustfmt;
mod rustywind;
mod scalafmt;
mod shfmt;
mod smlfmt;
mod snakefmt;
mod sql_formatter;
mod sqlfluff;
mod sqlfmt;
mod standardjs;
mod standardrb;
mod stylelint;
mod stylish_haskell;
mod stylua;
mod swift_format;
mod swiftformat;
mod taplo;
mod templ;
mod terraform_fmt;
mod tofu_fmt;
mod ts_standard;
mod typos;
mod uiua;
mod usort;
mod v;
mod veryl;
mod xmlformat;
mod xmllint;
mod xo;
mod yamlfix;
mod yamlfmt;
mod yapf;
mod yew_fmt;
mod zigfmt;
mod zprint;

#[inline]
fn setup_temp_dir() -> std::io::Result<()> {
    std::fs::create_dir_all(".mdsf-cache")?;

    std::fs::write(
        ".mdsf-cache/.gitignore",
        "Automatically created by mdsf.\n*\n",
    )?;

    Ok(())
}

#[inline]
pub fn setup_snippet(code: &str, file_ext: &str) -> std::io::Result<NamedTempFile> {
    let mut b = tempfile::Builder::new();

    b.rand_bytes(12).suffix(file_ext).prefix(
        // ktlint wants PascalCase file names
        if file_ext == language_to_ext("kotlin") {
            "MdsfFile"
        } else {
            "mdsf"
        },
    );

    if !std::path::PathBuf::from_str(".mdsf-cache/.gitignore")
        .is_ok_and(|p| p.try_exists().unwrap_or_default())
    {
        setup_temp_dir()?;
    }

    let mut f = b.tempfile_in(".mdsf-cache")?;

    f.write_all(code.as_bytes())?;
    f.flush()?;

    Ok(f)
}

#[inline]
pub fn read_snippet(file_path: &std::path::Path) -> std::io::Result<String> {
    std::fs::read_to_string(file_path)
}

#[inline]
fn handle_post_execution(
    result: std::io::Result<std::process::Output>,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    match result {
        Ok(output) => {
            if output.status.success() {
                read_snippet(snippet_path)
                    .map(|code| (false, Some(code)))
                    .map_err(MdsfError::from)
            } else {
                Err(MdsfError::FormatterError(
                    String::from_utf8_lossy(&output.stderr).to_string(),
                ))
            }
        }
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                Ok((true, None))
            } else {
                Err(MdsfError::from(err))
            }
        }
    }
}

fn spawn_command(cmd: &mut Command) -> std::io::Result<std::process::Output> {
    if !DEBUG.load(core::sync::atomic::Ordering::Relaxed) {
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());
    }

    cmd.output()
}

#[inline]
pub fn execute_command(
    cmd: &mut Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let binary_name = cmd.get_program();

    if !binary_in_path(binary_name) {
        return Err(MdsfError::MissingBinary(
            binary_name.to_string_lossy().to_string(),
        ));
    }

    handle_post_execution(spawn_command(cmd), snippet_path)
}

#[inline]
pub fn format_snippet(config: &MdsfConfig, info: &LineInfo, code: &str) -> String {
    if let Some(formatters) = config.languages.get(info.language) {
        if let Ok(snippet) = setup_snippet(
            code,
            config
                .custom_file_extensions
                .get(info.language)
                .map_or_else(
                    || generated::language_to_ext(info.language),
                    |s| {
                        if s.is_empty() {
                            info.language
                        } else {
                            s
                        }
                    },
                ),
        ) {
            let snippet_path = snippet.path();

            if let Ok(Some(formatted_code)) = formatters.format(snippet_path, info) {
                let mut f = formatted_code.trim().to_owned();

                f.push('\n');

                return f;
            }
        }
    }

    code.to_owned()
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Hash)]
#[cfg_attr(test, derive(PartialEq, Eq))]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum MdsfFormatter<T>
where
    T: core::fmt::Display,
{
    Single(T),
    Multiple(Vec<MdsfFormatter<T>>),
}

#[inline]
pub fn binary_in_path(binary_name: &OsStr) -> bool {
    which(binary_name).is_ok()
}

#[derive(serde::Serialize, serde::Deserialize, Hash)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub enum Tooling {
    #[doc = "https://github.com/kamadorueda/alejandra"]
    #[serde(rename = "alejandra")]
    Alejandra,

    #[doc = "https://github.com/klauspost/asmfmt"]
    #[serde(rename = "asmfmt")]
    Asmfmt,

    #[doc = "https://astyle.sourceforge.net"]
    #[serde(rename = "astyle")]
    Astyle,

    #[doc = "https://pypi.org/project/auto-optional/"]
    #[serde(rename = "auto-optional")]
    AutoOptional,

    #[doc = "https://github.com/huacnlee/autocorrect"]
    #[serde(rename = "autocorrect")]
    Autocorrect,

    #[doc = "https://github.com/pycqa/autoflake"]
    #[serde(rename = "autoflake")]
    Autoflake,

    #[doc = "https://pypi.org/project/autopep8/"]
    #[serde(rename = "autopep8")]
    Autopep8,

    #[doc = "https://pypi.org/project/beautysh/"]
    #[serde(rename = "beautysh")]
    Beautysh,

    #[doc = "https://github.com/Azure/bicep"]
    #[serde(rename = "bicep_format")]
    BicepFormat,

    #[doc = "https://biomejs.dev"]
    #[serde(rename = "biome")]
    Biome,

    #[doc = "https://biomejs.dev"]
    #[serde(rename = "biome_lint")]
    BiomeLint,

    #[doc = "https://biomejs.dev"]
    #[serde(rename = "biome_check")]
    BiomeCheck,

    #[doc = "https://github.com/psf/black"]
    #[serde(rename = "black")]
    Black,

    #[doc = "https://github.com/shufo/blade-formatter"]
    #[serde(rename = "blade-formatter")]
    BladeFormatter,

    #[doc = "https://blue.readthedocs.io/en/latest/"]
    #[serde(rename = "blue")]
    Blue,

    #[doc = "https://source.android.com/docs/setup/reference/androidbp"]
    #[serde(rename = "bpfmt")]
    Bpfmt,

    #[doc = "https://github.com/rokucommunity/brighterscript-formatter"]
    #[serde(rename = "bsfmt")]
    Bsfmt,

    #[doc = "https://buf.build/docs/reference/cli/buf/format"]
    #[serde(rename = "buf")]
    Buf,

    #[doc = "https://github.com/bazelbuild/buildtools"]
    #[serde(rename = "buildifier")]
    Buildifier,

    #[doc = "https://www.haskell.org/cabal/"]
    #[serde(rename = "cabal_format")]
    CabalFormat,

    #[doc = "https://caramel.run"]
    #[serde(rename = "caramel_fmt")]
    CaramelFmt,

    #[doc = "https://docs.kernel.org/process/clang-format.html"]
    #[serde(rename = "clang-format")]
    ClangFormat,

    #[doc = "https://clang.llvm.org/extra/clang-tidy"]
    #[serde(rename = "clang-tidy")]
    ClangTidy,

    #[doc = "https://github.com/greglook/cljstyle"]
    #[serde(rename = "cljstyle")]
    Cljstyle,

    #[doc = "https://github.com/codespell-project/codespell"]
    #[serde(rename = "codespell")]
    Codespell,

    #[doc = "https://github.com/cockroachdb/crlfmt"]
    #[serde(rename = "crlfmt")]
    CrlFmt,

    #[doc = "https://crystal-lang.org/"]
    #[serde(rename = "crystal_format")]
    CrystalFormat,

    #[doc = "https://csharpier.com/"]
    #[serde(rename = "csharpier")]
    CSharpier,

    #[doc = "https://d2lang.com/"]
    #[serde(rename = "d2")]
    D2,

    #[doc = "https://dart.dev/tools/dart-format"]
    #[serde(rename = "dart_format")]
    DartFormat,

    #[doc = "https://dart.dev/tools/dart-fix"]
    #[serde(rename = "dart_fix")]
    DartFix,

    #[doc = "https://dcm.dev/docs/cli/fix/"]
    #[serde(rename = "dcm_fix")]
    DcmFix,

    #[doc = "https://dcm.dev/docs/cli/format/"]
    #[serde(rename = "dcm_format")]
    DcmFormat,

    #[doc = "https://docs.deno.com/runtime/manual/tools/formatter"]
    #[serde(rename = "deno_fmt")]
    DenoFmt,

    #[doc = "https://docs.deno.com/runtime/manual/tools/linter"]
    #[serde(rename = "deno_lint")]
    DenoLint,

    #[doc = "https://github.com/dlang-community/dfmt"]
    #[serde(rename = "dfmt")]
    DFmt,

    #[doc = "https://www.djlint.com/"]
    #[serde(rename = "djlint")]
    DjLint,

    #[doc = "https://pypi.org/project/docformatter/"]
    #[serde(rename = "docformatter")]
    Docformatter,

    #[doc = "https://pypi.org/project/docstrfmt/"]
    #[serde(rename = "docstrfmt")]
    Docstrfmt,

    #[doc = "https://github.com/dotenv-linter/dotenv-linter"]
    #[serde(rename = "dotenv-linter")]
    DotenvLinter,

    #[doc = "https://dprint.dev"]
    #[serde(rename = "dprint")]
    Dprint,

    #[doc = "https://github.com/sile/efmt"]
    #[serde(rename = "efmt")]
    Efmt,

    #[doc = "https://github.com/avh4/elm-format"]
    #[serde(rename = "elm-format")]
    ElmFormat,

    #[doc = "https://github.com/nebulab/erb-formatter"]
    #[serde(rename = "erb-formatter")]
    ErbFormatter,

    #[doc = "https://github.com/WhatsApp/erlfmt"]
    #[serde(rename = "erlfmt")]
    Erlfmt,

    #[doc = "https://eslint.org"]
    #[serde(rename = "eslint")]
    Eslint,

    #[doc = "https://github.com/fsprojects/fantomas"]
    #[serde(rename = "fantomas")]
    Fantomas,

    #[doc = "https://pypi.org/project/findent/"]
    #[serde(rename = "findent")]
    Findent,

    #[doc = "https://fishshell.com/docs/current/cmds/fish_indent.html"]
    #[serde(rename = "fish_indent")]
    FishIndent,

    #[doc = "https://github.com/rhysd/fixjson"]
    #[serde(rename = "fixjson")]
    Fixjson,

    #[doc = "https://git.sr.ht/~technomancy/fnlfmt"]
    #[serde(rename = "fnlfmt")]
    Fnlfmt,

    #[doc = "https://docs.rs/forge-fmt/latest/forge_fmt/"]
    #[serde(rename = "forge_fmt")]
    ForgeFmt,

    #[doc = "https://hackage.haskell.org/package/fourmolu"]
    #[serde(rename = "fourmolu")]
    Fourmolu,

    #[doc = "https://github.com/fortran-lang/fprettify"]
    #[serde(rename = "fprettify")]
    Fprettify,

    #[doc = "https://github.com/daixiang0/gci"]
    #[serde(rename = "gci")]
    GCI,

    #[doc = "https://github.com/scony/godot-gdscript-toolkit"]
    #[serde(rename = "gdformat")]
    Gdformat,

    #[doc = "https://github.com/blankspruce/gersemi"]
    #[serde(rename = "gersemi")]
    Gersemi,

    #[doc = "https://gleam.run/"]
    #[serde(rename = "gleam_format")]
    GleamFormat,

    #[doc = "https://github.com/gluon-lang/gluon"]
    #[serde(rename = "gluon_fmt")]
    GluonFmt,

    #[doc = "https://pkg.go.dev/cmd/gofmt"]
    #[serde(rename = "gofmt")]
    GoFmt,

    #[doc = "https://github.com/mvdan/gofumpt"]
    #[serde(rename = "gofumpt")]
    GoFumpt,

    #[doc = "https://pkg.go.dev/golang.org/x/tools/cmd/goimports"]
    #[serde(rename = "goimports")]
    GoImports,

    #[doc = "https://github.com/incu6us/goimports-reviser"]
    #[serde(rename = "goimports-reviser")]
    GoImportsReviser,

    #[doc = "https://github.com/segmentio/golines"]
    #[serde(rename = "golines")]
    GoLines,

    #[doc = "https://github.com/google/google-java-format"]
    #[serde(rename = "google-java-format")]
    GoogleJavaFormat,

    #[doc = "https://grain-lang.org"]
    #[serde(rename = "grain_format")]
    GrainFormat,

    #[doc = "https://github.com/sds/haml-lint"]
    #[serde(rename = "haml-lint")]
    HamlLint,

    #[doc = "https://hackage.haskell.org/package/hindent"]
    #[serde(rename = "hindent")]
    HIndent,

    #[doc = "https://github.com/threedaymonk/htmlbeautifier"]
    #[serde(rename = "htmlbeautifier")]
    Htmlbeautifier,

    #[doc = "https://github.com/imba/imba"]
    #[serde(rename = "imba_fmt")]
    ImbaFmt,

    #[doc = "https://pycqa.github.io/isort/"]
    #[serde(rename = "isort")]
    Isort,

    #[doc = "https://github.com/candid82/joker"]
    #[serde(rename = "joker")]
    Joker,

    #[doc = "https://github.com/domluna/JuliaFormatter.jl"]
    #[serde(rename = "juliaformatter.jl")]
    JuliaFormatterJl,

    #[doc = "https://github.com/casey/just"]
    #[serde(rename = "just_fmt")]
    JustFmt,

    #[doc = "https://github.com/jsona/jsona"]
    #[serde(rename = "jsona_format")]
    JsonaFormat,

    #[doc = "https://www.kcl-lang.io/docs/tools/cli/kcl/fmt"]
    #[serde(rename = "kcl_fmt")]
    KclFmt,

    #[doc = "https://github.com/hougesen/kdlfmt"]
    #[serde(rename = "kdlfmt")]
    Kdlfmt,

    #[doc = "https://github.com/facebook/ktfmt"]
    #[serde(rename = "ktfmt")]
    Ktfmt,

    #[doc = "https://github.com/pinterest/ktlint"]
    #[serde(rename = "ktlint")]
    Ktlint,

    #[doc = "https://github.com/bram209/leptosfmt"]
    #[serde(rename = "leptosfmt")]
    LeptosFmt,

    #[doc = "https://github.com/Koihik/LuaFormatter"]
    #[serde(rename = "luaformatter")]
    LuaFormatter,

    #[doc = "https://github.com/davidanson/markdownlint"]
    #[serde(rename = "markdownlint")]
    Markdownlint,

    #[doc = "https://markuplint.dev"]
    #[serde(rename = "markuplint")]
    Markuplint,

    #[doc = "https://github.com/executablebooks/mdformat"]
    #[serde(rename = "mdformat")]
    MdFormat,

    #[doc = "https://github.com/client9/misspell/"]
    #[serde(rename = "misspell")]
    Misspell,

    #[doc = "https://hexdocs.pm/mix/main/Mix.Tasks.Format.html"]
    #[serde(rename = "mix_format")]
    MixFormat,

    #[doc = "https://nickel-lang.org"]
    #[serde(rename = "nickel_format")]
    NickelFormat,

    #[doc = "https://github.com/nim-lang/nim"]
    #[serde(rename = "nimpretty")]
    Nimpretty,

    #[doc = "https://github.com/serokell/nixfmt"]
    #[serde(rename = "nixfmt")]
    Nixfmt,

    #[doc = "https://github.com/nix-community/nixpkgs-fmt"]
    #[serde(rename = "nixpkgs-fmt")]
    NixpkgsFmt,

    #[doc = "https://github.com/nvuillam/npm-groovy-lint"]
    #[serde(rename = "npm-groovy-lint")]
    NpmGroovyLint,

    #[doc = "https://github.com/ocaml-ppx/ocamlformat"]
    #[serde(rename = "ocamlformat")]
    OCamlFormat,

    #[doc = "https://github.com/OCamlPro/ocp-indent"]
    #[serde(rename = "ocp-indent")]
    OcpIndent,

    #[doc = "https://hackage.haskell.org/package/ormolu"]
    #[serde(rename = "ormolu")]
    Ormolu,

    #[doc = "https://oxc.rs"]
    #[serde(rename = "oxlint")]
    Oxlint,

    #[doc = "https://github.com/perltidy/perltidy"]
    #[serde(rename = "perltidy")]
    PerlTidy,

    #[doc = "https://github.com/darold/pgFormatter"]
    #[serde(rename = "pg_format")]
    PgFormat,

    #[doc = "https://github.com/prettier/prettier"]
    #[serde(rename = "prettier")]
    Prettier,

    #[doc = "https://github.com/puppetlabs/puppet-lint"]
    #[serde(rename = "puppet-lint")]
    PuppetLint,

    #[doc = "https://github.com/natefaubion/purescript-tidy"]
    #[serde(rename = "purs-tidy")]
    PursTidy,

    #[doc = "https://github.com/google/pyink"]
    #[serde(rename = "pyink")]
    PyInk,

    #[doc = "https://docs.racket-lang.org/fmt/"]
    #[serde(rename = "raco_fmt")]
    RacoFmt,

    #[doc = "https://rescript-lang.org/"]
    #[serde(rename = "rescript_format")]
    ReScriptFormat,

    #[doc = "https://github.com/roc-lang/roc"]
    #[serde(rename = "roc_format")]
    RocFormat,

    #[doc = "https://github.com/dzhu/rstfmt"]
    #[serde(rename = "rstfmt")]
    RstFmt,

    #[doc = "https://github.com/rubocop/rubocop"]
    #[serde(rename = "rubocop")]
    RuboCop,

    #[doc = "https://github.com/fables-tales/rubyfmt"]
    #[serde(rename = "rubyfmt")]
    RubyFmt,

    #[doc = "https://docs.astral.sh/ruff/formatter/"]
    #[serde(rename = "ruff")]
    Ruff,

    #[doc = "https://docs.astral.sh/ruff/linter/"]
    #[serde(rename = "ruff_check")]
    RuffCheck,

    #[doc = "https://github.com/ruby-formatter/rufo"]
    #[serde(rename = "rufo")]
    Rufo,

    #[doc = "https://github.com/rune-rs/rune"]
    #[serde(rename = "rune_fmt")]
    RuneFmt,

    #[doc = "https://github.com/rust-lang/rustfmt"]
    #[serde(rename = "rustfmt")]
    RustFmt,

    #[doc = "https://github.com/avencera/rustywind"]
    #[serde(rename = "rustywind")]
    Rustywind,

    #[doc = "https://github.com/scalameta/scalafmt"]
    #[serde(rename = "scalafmt")]
    Scalafmt,

    #[doc = "https://github.com/mvdan/sh"]
    #[serde(rename = "shfmt")]
    Shfmt,

    #[doc = "https://github.com/shwestrick/smlfmt"]
    #[serde(rename = "smlfmt")]
    Smlfmt,

    #[doc = "https://github.com/snakemake/snakefmt"]
    #[serde(rename = "snakefmt")]
    Snakefmt,

    #[doc = "https://github.com/sql-formatter-org/sql-formatter"]
    #[serde(rename = "sql-formatter")]
    SQLFormatter,

    #[doc = "https://github.com/sqlfluff/sqlfluff"]
    #[serde(rename = "sqlfluff")]
    Sqlfluff,

    #[doc = "https://sqlfmt.com"]
    #[serde(rename = "sqlfmt")]
    Sqlfmt,

    #[doc = "https://standardjs.com/"]
    #[serde(rename = "standardjs")]
    Standardjs,

    #[doc = "https://github.com/standardrb/standard"]
    #[serde(rename = "standardrb")]
    Standardrb,

    #[doc = "https://github.com/stylelint/stylelint"]
    #[serde(rename = "stylelint")]
    StyleLint,

    #[doc = "https://github.com/haskell/stylish-haskell"]
    #[serde(rename = "stylish-haskell")]
    StylishHaskell,

    #[doc = "https://github.com/JohnnyMorganz/StyLua"]
    #[serde(rename = "stylua")]
    Stylua,

    #[doc = "https://github.com/apple/swift-format"]
    #[serde(rename = "swift-format")]
    AppleSwiftFormat,

    #[doc = "https://github.com/nicklockwood/SwiftFormat"]
    #[serde(rename = "swiftformat")]
    NicklockwoodSwiftFormat,

    #[doc = "https://github.com/tamasfe/taplo"]
    #[serde(rename = "taplo")]
    Taplo,

    #[doc = "https://templ.guide"]
    #[serde(rename = "templ")]
    Templ,

    #[doc = "https://www.terraform.io/docs/cli/commands/fmt.html"]
    #[serde(rename = "terraform_fmt")]
    TerraformFmt,

    #[doc = "https://opentofu.org/docs/cli/commands/fmt/"]
    #[serde(rename = "tofu_fmt")]
    TofuFmt,

    #[doc = "https://github.com/standard/ts-standard"]
    #[serde(rename = "ts-standard")]
    TsStandard,

    #[doc = "https://github.com/crate-ci/typos"]
    #[serde(rename = "typos")]
    Typos,

    #[doc = "https://github.com/uiua-lang/uiua"]
    #[serde(rename = "uiua_fmt")]
    UiuaFmt,

    #[doc = "https://github.com/facebook/usort"]
    #[serde(rename = "usort")]
    Usort,

    #[doc = "https://vlang.io"]
    #[serde(rename = "vlang_fmt")]
    VlangFmt,

    #[doc = "https://github.com/veryl-lang/veryl"]
    #[serde(rename = "veryl_fmt")]
    VerylFmt,

    #[doc = "https://github.com/pamoller/xmlformatter"]
    #[serde(rename = "xmlformat")]
    XmlFormat,

    #[doc = "http://xmlsoft.org/xmllint.html"]
    #[serde(rename = "xmllint")]
    XmlLint,

    #[doc = "http://github.com/xojs/xo"]
    #[serde(rename = "xo")]
    Xo,

    #[doc = "https://github.com/lyz-code/yamlfix"]
    #[serde(rename = "yamlfix")]
    YamlFix,

    #[doc = "https://github.com/google/yamlfmt"]
    #[serde(rename = "yamlfmt")]
    YamlFmt,

    #[doc = "https://github.com/google/yapf"]
    #[serde(rename = "yapf")]
    Yapf,

    #[doc = "https://github.com/its-the-shrimp/yew-fmt"]
    #[serde(rename = "yew-fmt")]
    YewFmt,

    #[doc = "https://ziglang.org/"]
    #[serde(rename = "zigfmt")]
    ZigFmt,

    #[doc = "https://github.com/kkinnear/zprint"]
    #[serde(rename = "zprint")]
    Zprint,
}

impl Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Alejandra => alejandra::run(snippet_path),
            Self::AppleSwiftFormat => swift_format::run(snippet_path),
            Self::Asmfmt => asmfmt::run(snippet_path),
            Self::Astyle => astyle::run(snippet_path),
            Self::AutoOptional => auto_optional::run(snippet_path),
            Self::Autocorrect => autocorrect::run(snippet_path),
            Self::Autoflake => autoflake::run(snippet_path),
            Self::Autopep8 => autopep8::run(snippet_path),
            Self::Beautysh => beautysh::run(snippet_path),
            Self::BicepFormat => bicep_format::run(snippet_path),
            Self::Biome => biome::run_format(snippet_path),
            Self::BiomeCheck => biome::run_check(snippet_path),
            Self::BiomeLint => biome::run_lint(snippet_path),
            Self::Black => black::run(snippet_path),
            Self::BladeFormatter => blade_formatter::run(snippet_path),
            Self::Blue => blue::run(snippet_path),
            Self::Bpfmt => bpfmt::run(snippet_path),
            Self::Bsfmt => bsfmt::run(snippet_path),
            Self::Buf => buf::run(snippet_path),
            Self::Buildifier => buildifier::run(snippet_path),
            Self::CSharpier => csharpier::run(snippet_path),
            Self::CabalFormat => cabal_format::run(snippet_path),
            Self::CaramelFmt => caramel::run_fmt(snippet_path),
            Self::ClangFormat => clang_format::run(snippet_path),
            Self::ClangTidy => clang_tidy::run(snippet_path),
            Self::Cljstyle => cljstyle::run(snippet_path),
            Self::Codespell => codespell::run(snippet_path),
            Self::CrlFmt => crlfmt::run(snippet_path),
            Self::CrystalFormat => crystal_format::run(snippet_path),
            Self::D2 => d2::run(snippet_path),
            Self::DFmt => dfmt::run(snippet_path),
            Self::DartFormat => dart::run_format(snippet_path),
            Self::DartFix => dart::run_fix(snippet_path),
            Self::DcmFix => dcm::run_fix(snippet_path),
            Self::DcmFormat => dcm::run_format(snippet_path),
            Self::DenoFmt => deno::run_fmt(snippet_path),
            Self::DenoLint => deno::run_lint(snippet_path),
            Self::DjLint => djlint::run(snippet_path),
            Self::Docformatter => docformatter::run(snippet_path),
            Self::Docstrfmt => docstrfmt::run(snippet_path),
            Self::DotenvLinter => dotenv_linter::run(snippet_path),
            Self::Dprint => dprint::run(snippet_path),
            Self::Efmt => efmt::run(snippet_path),
            Self::ElmFormat => elm_format::run(snippet_path),
            Self::ErbFormatter => erb_formatter::run(snippet_path),
            Self::Erlfmt => erlfmt::run(snippet_path),
            Self::Eslint => eslint::run(snippet_path),
            Self::Fantomas => fantomas::run(snippet_path),
            Self::Findent => findent::run(snippet_path),
            Self::FishIndent => fish_indent::run(snippet_path),
            Self::Fixjson => fixjson::run(snippet_path),
            Self::Fnlfmt => fnlfmt::run(snippet_path),
            Self::ForgeFmt => forge_fmt::run(snippet_path),
            Self::Fourmolu => fourmolu::run(snippet_path),
            Self::Fprettify => fprettify::run(snippet_path),
            Self::GCI => gci::run(snippet_path),
            Self::Gdformat => gdformat::run(snippet_path),
            Self::Gersemi => gersemi::run(snippet_path),
            Self::GleamFormat => gleam_format::run(snippet_path),
            Self::GluonFmt => gluon::run_fmt(snippet_path),
            Self::GoFmt => gofmt::run(snippet_path),
            Self::GoFumpt => gofumpt::run(snippet_path),
            Self::GoImports => goimports::run(snippet_path),
            Self::GoImportsReviser => goimports_reviser::run(snippet_path),
            Self::GoLines => golines::run(snippet_path),
            Self::GoogleJavaFormat => google_java_format::run(snippet_path),
            Self::GrainFormat => grain::run_format(snippet_path),
            Self::HIndent => hindent::run(snippet_path),
            Self::HamlLint => haml_lint::run(snippet_path),
            Self::Htmlbeautifier => htmlbeautifier::run(snippet_path),
            Self::ImbaFmt => imba::run_fmt(snippet_path),
            Self::Isort => isort::run(snippet_path),
            Self::Joker => joker::run(snippet_path),
            Self::JuliaFormatterJl => juliaformatter_jl::run(snippet_path),
            Self::JustFmt => just_fmt::run(snippet_path),
            Self::JsonaFormat => jsona::run_format(snippet_path),
            Self::KclFmt => kcl_fmt::run(snippet_path),
            Self::Kdlfmt => kdlfmt::run(snippet_path),
            Self::Ktfmt => ktfmt::run(snippet_path),
            Self::Ktlint => ktlint::run(snippet_path),
            Self::LeptosFmt => leptosfmt::run(snippet_path),
            Self::LuaFormatter => luaformatter::run(snippet_path),
            Self::Markdownlint => markdownlint::run(snippet_path),
            Self::Markuplint => markuplint::run(snippet_path),
            Self::MdFormat => mdformat::run(snippet_path),
            Self::Misspell => misspell::run(snippet_path),
            Self::MixFormat => mix_format::run(snippet_path),
            Self::NickelFormat => nickel::run_format(snippet_path),
            Self::NicklockwoodSwiftFormat => swiftformat::run(snippet_path),
            Self::Nimpretty => nimpretty::run(snippet_path),
            Self::Nixfmt => nixfmt::run(snippet_path),
            Self::NixpkgsFmt => nixpkgs_fmt::run(snippet_path),
            Self::NpmGroovyLint => npm_groovy_lint::run(snippet_path),
            Self::OCamlFormat => ocamlformat::run(snippet_path),
            Self::OcpIndent => ocp_indent::run(snippet_path),
            Self::Ormolu => ormolu::run(snippet_path),
            Self::Oxlint => oxlint::run(snippet_path),
            Self::PerlTidy => perltidy::run(snippet_path),
            Self::PgFormat => pg_format::run(snippet_path),
            Self::Prettier => prettier::run(snippet_path),
            Self::PuppetLint => puppet_lint::run(snippet_path),
            Self::PursTidy => purs_tidy::run(snippet_path),
            Self::PyInk => pyink::run(snippet_path),
            Self::RacoFmt => raco::run_fmt(snippet_path),
            Self::ReScriptFormat => rescript_format::run(snippet_path),
            Self::RocFormat => roc_format::run(snippet_path),
            Self::RstFmt => rstfmt::run(snippet_path),
            Self::RuboCop => rubocop::run(snippet_path),
            Self::RubyFmt => rubyfmt::run(snippet_path),
            Self::Ruff => ruff::run_format(snippet_path),
            Self::RuffCheck => ruff::run_check(snippet_path),
            Self::Rufo => rufo::run(snippet_path),
            Self::RuneFmt => rune::run_fmt(snippet_path),
            Self::RustFmt => rustfmt::run(snippet_path),
            Self::Rustywind => rustywind::run(snippet_path),
            Self::SQLFormatter => sql_formatter::run(snippet_path),
            Self::Scalafmt => scalafmt::run(snippet_path),
            Self::Shfmt => shfmt::run(snippet_path),
            Self::Smlfmt => smlfmt::run(snippet_path),
            Self::Snakefmt => snakefmt::run(snippet_path),
            Self::Sqlfluff => sqlfluff::run(snippet_path),
            Self::Sqlfmt => sqlfmt::run(snippet_path),
            Self::Standardjs => standardjs::run(snippet_path),
            Self::Standardrb => standardrb::run(snippet_path),
            Self::StyleLint => stylelint::run(snippet_path),
            Self::StylishHaskell => stylish_haskell::run(snippet_path),
            Self::Stylua => stylua::run(snippet_path),
            Self::Taplo => taplo::run(snippet_path),
            Self::Templ => templ::run(snippet_path),
            Self::TerraformFmt => terraform_fmt::run(snippet_path),
            Self::TofuFmt => tofu_fmt::run(snippet_path),
            Self::TsStandard => ts_standard::run(snippet_path),
            Self::Typos => typos::run(snippet_path),
            Self::UiuaFmt => uiua::run_fmt(snippet_path),
            Self::Usort => usort::run(snippet_path),
            Self::VlangFmt => v::run_fmt(snippet_path),
            Self::VerylFmt => veryl::run_fmt(snippet_path),
            Self::XmlFormat => xmlformat::run(snippet_path),
            Self::XmlLint => xmllint::run(snippet_path),
            Self::Xo => xo::run(snippet_path),
            Self::YamlFix => yamlfix::run(snippet_path),
            Self::YamlFmt => yamlfmt::run(snippet_path),
            Self::Yapf => yapf::run(snippet_path),
            Self::YewFmt => yew_fmt::run(snippet_path),
            Self::ZigFmt => zigfmt::run(snippet_path),
            Self::Zprint => zprint::run(snippet_path),
        }
    }
}

impl AsRef<str> for Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    fn as_ref(&self) -> &str {
        match self {
            Self::Alejandra => "alejandra",
            Self::AppleSwiftFormat => "swift-format",
            Self::Asmfmt => "asmfmt",
            Self::Astyle => "astyle",
            Self::AutoOptional => "auto-optional",
            Self::Autocorrect => "autocorrect",
            Self::Autoflake => "autoflake",
            Self::Autopep8 => "autopep8",
            Self::Beautysh => "beautysh",
            Self::BicepFormat => "bicep_format",
            Self::Biome => "biome",
            Self::BiomeCheck => "biome_check",
            Self::BiomeLint => "biome_lint",
            Self::Black => "black",
            Self::BladeFormatter => "blade-formatter",
            Self::Blue => "blue",
            Self::Bpfmt => "bpfmt",
            Self::Bsfmt => "bsfmt",
            Self::Buf => "buf",
            Self::Buildifier => "buildifier",
            Self::CSharpier => "csharpier",
            Self::CabalFormat => "cabal_format",
            Self::CaramelFmt => "caramel_fmt",
            Self::ClangFormat => "clang-format",
            Self::ClangTidy => "clang-tidy",
            Self::Cljstyle => "cljstyle",
            Self::Codespell => "codespell",
            Self::CrlFmt => "crlfmt",
            Self::CrystalFormat => "crystal_format",
            Self::D2 => "d2",
            Self::DFmt => "dfmt",
            Self::DartFormat => "dart_format",
            Self::DartFix => "dart_fix",
            Self::DenoFmt => "deno_fmt",
            Self::DenoLint => "deno_lint",
            Self::DjLint => "djlint",
            Self::DcmFormat => "dcm_format",
            Self::DcmFix => "dcm_fix",
            Self::Docformatter => "docformatter",
            Self::Docstrfmt => "docstrfmt",
            Self::DotenvLinter => "dotenv-linter",
            Self::Dprint => "dprint",
            Self::Efmt => "efmt",
            Self::ElmFormat => "elm-format",
            Self::ErbFormatter => "erb-formatter",
            Self::Erlfmt => "erlfmt",
            Self::Eslint => "eslint",
            Self::Fantomas => "fantomas",
            Self::Findent => "findent",
            Self::FishIndent => "fish_indent",
            Self::Fixjson => "fixjson",
            Self::Fnlfmt => "fnlfmt",
            Self::ForgeFmt => "forge_fmt",
            Self::Fourmolu => "fourmolu",
            Self::Fprettify => "fprettify",
            Self::GCI => "gci",
            Self::Gdformat => "gdformat",
            Self::Gersemi => "gersemi",
            Self::GleamFormat => "gleam_format",
            Self::GluonFmt => "gluon_fmt",
            Self::GoFmt => "gofmt",
            Self::GoFumpt => "gofumpt",
            Self::GoImports => "goimports",
            Self::GoImportsReviser => "goimports-reviser",
            Self::GoLines => "golines",
            Self::GoogleJavaFormat => "google-java-format",
            Self::GrainFormat => "grain_format",
            Self::HIndent => "hundent",
            Self::HamlLint => "haml-lint",
            Self::Htmlbeautifier => "htmlbeautifier",
            Self::ImbaFmt => "imba_fmt",
            Self::Isort => "isort",
            Self::Joker => "joker",
            Self::JuliaFormatterJl => "juliaformatter.jl",
            Self::JustFmt => "just_fmt",
            Self::JsonaFormat => "jsona_format",
            Self::KclFmt => "kcl_fmt",
            Self::Kdlfmt => "kdlfmt",
            Self::Ktfmt => "ktfmt",
            Self::Ktlint => "ktlint",
            Self::LeptosFmt => "leptosfmt",
            Self::LuaFormatter => "luaformatter",
            Self::Markdownlint => "markdownlint",
            Self::Markuplint => "markuplint",
            Self::MdFormat => "mdformat",
            Self::Misspell => "misspell",
            Self::MixFormat => "mix_format",
            Self::NickelFormat => "nickel_format",
            Self::NicklockwoodSwiftFormat => "swiftformat",
            Self::Nimpretty => "nimpretty",
            Self::Nixfmt => "nixfmt",
            Self::NixpkgsFmt => "nixpkgs-fmt",
            Self::NpmGroovyLint => "npm-groovy-lint",
            Self::OCamlFormat => "ocamlformat",
            Self::OcpIndent => "ocpindent",
            Self::Ormolu => "ormolu",
            Self::Oxlint => "oxlint",
            Self::PerlTidy => "perltidy",
            Self::PgFormat => "pg_format",
            Self::Prettier => "prettier",
            Self::PuppetLint => "puppet-lint",
            Self::PursTidy => "purs-tidy",
            Self::PyInk => "pyink",
            Self::RacoFmt => "raco_fmt",
            Self::ReScriptFormat => "rescript_format",
            Self::RocFormat => "roc_format",
            Self::RstFmt => "rstfmt",
            Self::RuboCop => "rubocop",
            Self::RubyFmt => "rubyfmt",
            Self::Ruff => "ruff",
            Self::RuffCheck => "ruff_check",
            Self::Rufo => "rufo",
            Self::RuneFmt => "rune_fmt",
            Self::RustFmt => "rustfmt",
            Self::Rustywind => "rustywind",
            Self::SQLFormatter => "sql-formatter",
            Self::Scalafmt => "scalafmt",
            Self::Shfmt => "shfmt",
            Self::Smlfmt => "smlfmt",
            Self::Snakefmt => "snakefmt",
            Self::Sqlfluff => "sqlfluff",
            Self::Sqlfmt => "sqlfmt",
            Self::Standardjs => "standardjs",
            Self::Standardrb => "standardrb",
            Self::StyleLint => "stylelint",
            Self::StylishHaskell => "stylish-haskell",
            Self::Stylua => "stylua",
            Self::Taplo => "taplo",
            Self::Templ => "templ",
            Self::TerraformFmt => "terraform_fmt",
            Self::TofuFmt => "tofu_fmt",
            Self::TsStandard => "ts-standard",
            Self::Typos => "typos",
            Self::UiuaFmt => "uiua_fmt",
            Self::Usort => "usort",
            Self::VlangFmt => "vlang_fmt",
            Self::VerylFmt => "veryl_fmt",
            Self::XmlFormat => "xmlformat",
            Self::XmlLint => "xmllint",
            Self::Xo => "xo",
            Self::YamlFix => "yamlfix",
            Self::YamlFmt => "yamlfmt",
            Self::Yapf => "yapf",
            Self::YewFmt => "yew-fmt",
            Self::ZigFmt => "zigfmt",
            Self::Zprint => "zprint",
        }
    }
}

impl core::fmt::Display for Tooling {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Default for MdsfFormatter<Tooling> {
    fn default() -> Self {
        Self::Multiple(Vec::new())
    }
}

impl MdsfFormatter<Tooling> {
    #[inline]
    pub fn format(
        &self,
        snippet_path: &std::path::Path,
        info: &LineInfo,
    ) -> Result<Option<String>, MdsfError> {
        Self::format_multiple(self, snippet_path, info, false)
            .map(|(_should_continue, output)| output)
    }

    #[inline]
    pub fn format_multiple(
        formatter: &MdsfFormatter<Tooling>,
        snippet_path: &std::path::Path,
        info: &LineInfo,
        nested: bool,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match formatter {
            Self::Single(f) => {
                let formatter_name: &str = f.as_ref();

                print_formatter_info(formatter_name, info);

                let time = std::time::Instant::now();

                let r = f.format_snippet(snippet_path);

                print_formatter_time(formatter_name, info, time.elapsed());

                if let Err(e) = &r {
                    if let MdsfError::MissingBinary(binary) = e {
                        print_binary_not_in_path(
                            info.filename,
                            if formatter_name == binary {
                                formatter_name.to_string()
                            } else {
                                format!("{binary} ({formatter_name})")
                            }
                            .as_str(),
                        );

                        return Ok((false, None));
                    } else if let MdsfError::FormatterError(stderr) = e {
                        print_error_formatting(formatter_name, info, stderr);
                        return Ok((false, None));
                    }
                }

                r
            }

            Self::Multiple(formatters) => {
                let mut r = Ok((true, None));

                for f in formatters {
                    r = Self::format_multiple(f, snippet_path, info, true);

                    if r.as_ref()
                        .is_ok_and(|(should_continue, _code)| !should_continue)
                        && nested
                    {
                        break;
                    }
                }

                r
            }
        }
    }
}

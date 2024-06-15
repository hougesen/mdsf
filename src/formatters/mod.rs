use std::{ffi::OsStr, io::Write, process::Command};

use tempfile::NamedTempFile;
use which::which;

use crate::{
    config::MdsfConfig,
    error::MdsfError,
    formatters::{
        alejandra::format_using_alejandra, asmfmt::format_using_asmfmt,
        astyle::format_using_astyle, auto_optional::format_using_auto_optional,
        autocorrect::format_using_autocorrect, autoflake::format_using_autoflake,
        autopep8::format_using_autopep8, beautysh::format_using_beautysh,
        bicep_format::format_using_bicep_format, biome::format_using_biome,
        black::format_using_black, blade_formatter::format_using_blade_formatter,
        blue::format_using_blue, bpfmt::format_using_bpfmt, buf::format_using_buf,
        buildifier::format_using_buildifier, cabal_format::format_using_cabal_format,
        clang_format::format_using_clang_format, cljstyle::format_using_cljstyle,
        codespell::format_using_codespell, crlfmt::format_using_crlfmt,
        crystal_format::format_using_crystal_format, csharpier::format_using_csharpier,
        d2::format_using_d2, dart_format::format_using_dart_format,
        deno_fmt::format_using_deno_fmt, dfmt::format_using_dfmt, djlint::format_using_djlint,
        docstrfmt::format_using_docstrfmt, dprint::format_using_dprint, efmt::format_using_efmt,
        elm_format::format_using_elm_format, erb_formatter::format_using_erb_formatter,
        erlfmt::format_using_erlfmt, eslint::format_using_eslint, fantomas::format_using_fantomas,
        findent::format_using_findent, fish_indent::format_using_fish_indent,
        fixjson::format_using_fixjson, fnlfmt::format_using_fnlfmt,
        forge_fmt::format_using_forge_fmt, fourmolu::format_using_fourmolu,
        fprettify::format_using_fprettify, gci::format_using_gci, gdformat::format_using_gdformat,
        gersemi::format_using_gersemi, gleam_format::format_using_gleam_format,
        gofmt::format_using_gofmt, gofumpt::format_using_gofumpt,
        goimports::format_using_goimports, goimports_reviser::format_using_goimports_reviser,
        golines::format_using_golines, google_java_format::format_using_google_java_format,
        hindent::format_using_hindent, htmlbeautifier::format_using_htmlbeautifier,
        isort::format_using_isort, joker::format_using_joker,
        juliaformatter_jl::format_using_juliaformatter_jl, just_fmt::format_using_just_fmt,
        kcl_fmt::format_using_kcl_fmt, kdlfmt::format_using_kdlfmt, ktfmt::format_using_ktfmt,
        ktlint::format_using_ktlint, leptosfmt::format_using_leptosfmt,
        luaformatter::format_using_luaformatter, markdownlint::format_using_markdownlint,
        mdformat::format_using_mdformat, misspell::format_using_misspell,
        mix_format::format_using_mix_format, nimpretty::format_using_nimpretty,
        nixfmt::format_using_nixfmt, nixpkgs_fmt::format_using_nixpkgs_fmt,
        npm_groovy_lint::format_using_npm_groovy_lint, ocamlformat::format_using_ocamlformat,
        ocp_indent::format_using_ocp_indent, ormolu::format_using_ormolu,
        perltidy::format_using_perltidy, prettier::format_using_prettier,
        puppet_lint::format_using_puppet_lint, purs_tidy::format_using_purs_tidy,
        pyink::format_using_pyink, rescript_format::format_using_rescript_format,
        roc_format::format_using_roc_format, rstfmt::format_using_rstfmt,
        rubocop::format_using_rubocop, rubyfmt::format_using_rubyfmt, ruff::format_using_ruff,
        rufo::format_using_rufo, rustfmt::format_using_rustfmt, scalafmt::format_using_scalafmt,
        shfmt::format_using_shfmt, sql_formatter::format_using_sql_formatter,
        sqlfluff::format_using_sqlfluff, standardjs::format_using_standardjs,
        standardrb::format_using_standardrb, stylelint::format_using_stylelint,
        stylish_haskell::format_using_stylish_haskell, stylua::format_using_stylua,
        swift_format::format_using_swift_format, swiftformat::format_using_swiftformat,
        taplo::format_using_taplo, terraform_fmt::format_using_terraform_fmt,
        tofu_fmt::format_using_tofu_fmt, typos::format_using_typos, usort::format_using_usort,
        xmlformat::format_using_xmlformat, xmllint::format_using_xmllint,
        yamlfix::format_using_yamlfix, yamlfmt::format_using_yamlfmt, yapf::format_using_yapf,
        yew_fmt::format_using_yew_fmt, zigfmt::format_using_zigfmt, zprint::format_using_zprint,
    },
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
mod buf;
mod buildifier;
mod cabal_format;
mod clang_format;
mod cljstyle;
mod codespell;
mod crlfmt;
mod crystal_format;
mod csharpier;
mod d2;
mod dart_format;
mod deno_fmt;
mod dfmt;
mod djlint;
mod docstrfmt;
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
mod gofmt;
mod gofumpt;
mod goimports;
mod goimports_reviser;
mod golines;
mod google_java_format;
mod hindent;
mod htmlbeautifier;
mod isort;
mod joker;
mod juliaformatter_jl;
mod just_fmt;
mod kcl_fmt;
mod kdlfmt;
mod ktfmt;
mod ktlint;
mod leptosfmt;
mod luaformatter;
mod markdownlint;
mod mdformat;
mod misspell;
mod mix_format;
mod nimpretty;
mod nixfmt;
mod nixpkgs_fmt;
mod npm_groovy_lint;
mod ocamlformat;
mod ocp_indent;
mod ormolu;
mod perltidy;
mod prettier;
mod puppet_lint;
mod purs_tidy;
mod pyink;
mod rescript_format;
mod roc_format;
mod rstfmt;
mod rubocop;
mod rubyfmt;
mod ruff;
mod rufo;
mod rustfmt;
mod scalafmt;
mod shfmt;
mod sql_formatter;
mod sqlfluff;
mod standardjs;
mod standardrb;
mod stylelint;
mod stylish_haskell;
mod stylua;
mod swift_format;
mod swiftformat;
mod taplo;
mod terraform_fmt;
mod tofu_fmt;
mod typos;
mod usort;
mod xmlformat;
mod xmllint;
mod yamlfix;
mod yamlfmt;
mod yapf;
mod yew_fmt;
mod zigfmt;
mod zprint;

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

    let mut f = if file_ext == ".cs" || file_ext == ".proto" {
        std::fs::create_dir_all(".mdsf-cache").ok();
        b.tempfile_in(".mdsf-cache")
    } else {
        b.tempfile()
    }?;

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
    result: std::io::Result<bool>,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    match result {
        Ok(true) => read_snippet(snippet_path)
            .map(|code| (false, Some(code)))
            .map_err(MdsfError::from),

        Ok(false) => Err(MdsfError::FormatterError),

        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                Ok((true, None))
            } else {
                Err(MdsfError::from(err))
            }
        }
    }
}

fn spawn_command(cmd: &mut Command) -> std::io::Result<bool> {
    if !DEBUG.load(core::sync::atomic::Ordering::Relaxed) {
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());
    }

    Ok(cmd.spawn()?.wait()?.success())
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
        if let Ok(snippet) = setup_snippet(code, &generated::language_to_ext(info.language)) {
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

#[derive(serde::Serialize, serde::Deserialize)]
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

    #[doc = "https://buf.build/docs/reference/cli/buf/format"]
    #[serde(rename = "buf")]
    Buf,

    #[doc = "https://github.com/bazelbuild/buildtools"]
    #[serde(rename = "buildifier")]
    Buildifier,

    #[doc = "https://www.haskell.org/cabal/"]
    #[serde(rename = "cabal_format")]
    CabalFormat,

    #[doc = "https://docs.kernel.org/process/clang-format.html"]
    #[serde(rename = "clang-format")]
    ClangFormat,

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

    #[doc = "https://dart.dev/tools/dart-format"]
    #[serde(rename = "deno_fmt")]
    DenoFmt,

    #[doc = "https://github.com/dlang-community/dfmt"]
    #[serde(rename = "dfmt")]
    DFmt,

    #[doc = "https://www.djlint.com/"]
    #[serde(rename = "djlint")]
    DjLint,

    #[doc = "https://pypi.org/project/docstrfmt/"]
    #[serde(rename = "docstrfmt")]
    Docstrfmt,

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

    #[doc = "https://hackage.haskell.org/package/hindent"]
    #[serde(rename = "hindent")]
    HIndent,

    #[doc = "https://github.com/threedaymonk/htmlbeautifier"]
    #[serde(rename = "htmlbeautifier")]
    Htmlbeautifier,

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

    #[doc = "https://github.com/executablebooks/mdformat"]
    #[serde(rename = "mdformat")]
    MdFormat,

    #[doc = "https://github.com/client9/misspell/"]
    #[serde(rename = "misspell")]
    Misspell,

    #[doc = "https://hexdocs.pm/mix/main/Mix.Tasks.Format.html"]
    #[serde(rename = "mix_format")]
    MixFormat,

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

    #[doc = "https://github.com/perltidy/perltidy"]
    #[serde(rename = "perltidy")]
    PerlTidy,

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

    #[doc = "https://docs.astral.sh/ruff/"]
    #[serde(rename = "ruff")]
    Ruff,

    #[doc = "https://github.com/ruby-formatter/rufo"]
    #[serde(rename = "rufo")]
    Rufo,

    #[doc = "https://github.com/rust-lang/rustfmt"]
    #[serde(rename = "rustfmt")]
    RustFmt,

    #[doc = "https://github.com/scalameta/scalafmt"]
    #[serde(rename = "scalafmt")]
    Scalafmt,

    #[doc = "https://github.com/mvdan/sh"]
    #[serde(rename = "shfmt")]
    Shfmt,

    #[doc = "https://github.com/sql-formatter-org/sql-formatter"]
    #[serde(rename = "sql-formatter")]
    SQLFormatter,

    #[doc = "https://github.com/sqlfluff/sqlfluff"]
    #[serde(rename = "sqlfluff")]
    Sqlfluff,

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

    #[doc = "https://www.terraform.io/docs/cli/commands/fmt.html"]
    #[serde(rename = "terraform_fmt")]
    TerraformFmt,

    #[doc = "https://opentofu.org/docs/cli/commands/fmt/"]
    #[serde(rename = "tofu_fmt")]
    TofuFmt,

    #[doc = "https://github.com/crate-ci/typos"]
    #[serde(rename = "typos")]
    Typos,

    #[doc = "https://github.com/facebook/usort"]
    #[serde(rename = "usort")]
    Usort,

    #[doc = "https://github.com/pamoller/xmlformatter"]
    #[serde(rename = "xmlformat")]
    XmlFormat,

    #[doc = "http://xmlsoft.org/xmllint.html"]
    #[serde(rename = "xmllint")]
    XmlLint,

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
            Self::Alejandra => format_using_alejandra(snippet_path),
            Self::AppleSwiftFormat => format_using_swift_format(snippet_path),
            Self::Asmfmt => format_using_asmfmt(snippet_path),
            Self::Astyle => format_using_astyle(snippet_path),
            Self::AutoOptional => format_using_auto_optional(snippet_path),
            Self::Autocorrect => format_using_autocorrect(snippet_path),
            Self::Autoflake => format_using_autoflake(snippet_path),
            Self::Autopep8 => format_using_autopep8(snippet_path),
            Self::Beautysh => format_using_beautysh(snippet_path),
            Self::BicepFormat => format_using_bicep_format(snippet_path),
            Self::Biome => format_using_biome(snippet_path),
            Self::Black => format_using_black(snippet_path),
            Self::BladeFormatter => format_using_blade_formatter(snippet_path),
            Self::Blue => format_using_blue(snippet_path),
            Self::Bpfmt => format_using_bpfmt(snippet_path),
            Self::Buf => format_using_buf(snippet_path),
            Self::Buildifier => format_using_buildifier(snippet_path),
            Self::CSharpier => format_using_csharpier(snippet_path),
            Self::CabalFormat => format_using_cabal_format(snippet_path),
            Self::ClangFormat => format_using_clang_format(snippet_path),
            Self::Cljstyle => format_using_cljstyle(snippet_path),
            Self::Codespell => format_using_codespell(snippet_path),
            Self::CrlFmt => format_using_crlfmt(snippet_path),
            Self::CrystalFormat => format_using_crystal_format(snippet_path),
            Self::DFmt => format_using_dfmt(snippet_path),
            Self::D2 => format_using_d2(snippet_path),
            Self::DartFormat => format_using_dart_format(snippet_path),
            Self::DenoFmt => format_using_deno_fmt(snippet_path),
            Self::DjLint => format_using_djlint(snippet_path),
            Self::Docstrfmt => format_using_docstrfmt(snippet_path),
            Self::Dprint => format_using_dprint(snippet_path),
            Self::Efmt => format_using_efmt(snippet_path),
            Self::ElmFormat => format_using_elm_format(snippet_path),
            Self::ErbFormatter => format_using_erb_formatter(snippet_path),
            Self::Erlfmt => format_using_erlfmt(snippet_path),
            Self::Eslint => format_using_eslint(snippet_path),
            Self::Fantomas => format_using_fantomas(snippet_path),
            Self::Findent => format_using_findent(snippet_path),
            Self::FishIndent => format_using_fish_indent(snippet_path),
            Self::Fixjson => format_using_fixjson(snippet_path),
            Self::Fnlfmt => format_using_fnlfmt(snippet_path),
            Self::ForgeFmt => format_using_forge_fmt(snippet_path),
            Self::Fourmolu => format_using_fourmolu(snippet_path),
            Self::Fprettify => format_using_fprettify(snippet_path),
            Self::GCI => format_using_gci(snippet_path),
            Self::Gdformat => format_using_gdformat(snippet_path),
            Self::Gersemi => format_using_gersemi(snippet_path),
            Self::GleamFormat => format_using_gleam_format(snippet_path),
            Self::GoFmt => format_using_gofmt(snippet_path),
            Self::GoFumpt => format_using_gofumpt(snippet_path),
            Self::GoImports => format_using_goimports(snippet_path),
            Self::GoImportsReviser => format_using_goimports_reviser(snippet_path),
            Self::GoLines => format_using_golines(snippet_path),
            Self::GoogleJavaFormat => format_using_google_java_format(snippet_path),
            Self::HIndent => format_using_hindent(snippet_path),
            Self::Htmlbeautifier => format_using_htmlbeautifier(snippet_path),
            Self::Isort => format_using_isort(snippet_path),
            Self::Joker => format_using_joker(snippet_path),
            Self::JuliaFormatterJl => format_using_juliaformatter_jl(snippet_path),
            Self::JustFmt => format_using_just_fmt(snippet_path),
            Self::KclFmt => format_using_kcl_fmt(snippet_path),
            Self::Kdlfmt => format_using_kdlfmt(snippet_path),
            Self::Ktfmt => format_using_ktfmt(snippet_path),
            Self::Ktlint => format_using_ktlint(snippet_path),
            Self::LeptosFmt => format_using_leptosfmt(snippet_path),
            Self::LuaFormatter => format_using_luaformatter(snippet_path),
            Self::Markdownlint => format_using_markdownlint(snippet_path),
            Self::MdFormat => format_using_mdformat(snippet_path),
            Self::Misspell => format_using_misspell(snippet_path),
            Self::MixFormat => format_using_mix_format(snippet_path),
            Self::NicklockwoodSwiftFormat => format_using_swiftformat(snippet_path),
            Self::Nimpretty => format_using_nimpretty(snippet_path),
            Self::Nixfmt => format_using_nixfmt(snippet_path),
            Self::NixpkgsFmt => format_using_nixpkgs_fmt(snippet_path),
            Self::NpmGroovyLint => format_using_npm_groovy_lint(snippet_path),
            Self::OCamlFormat => format_using_ocamlformat(snippet_path),
            Self::OcpIndent => format_using_ocp_indent(snippet_path),
            Self::Ormolu => format_using_ormolu(snippet_path),
            Self::PerlTidy => format_using_perltidy(snippet_path),
            Self::Prettier => format_using_prettier(snippet_path),
            Self::PuppetLint => format_using_puppet_lint(snippet_path),
            Self::PursTidy => format_using_purs_tidy(snippet_path),
            Self::PyInk => format_using_pyink(snippet_path),
            Self::ReScriptFormat => format_using_rescript_format(snippet_path),
            Self::RocFormat => format_using_roc_format(snippet_path),
            Self::RstFmt => format_using_rstfmt(snippet_path),
            Self::RuboCop => format_using_rubocop(snippet_path),
            Self::RubyFmt => format_using_rubyfmt(snippet_path),
            Self::Ruff => format_using_ruff(snippet_path),
            Self::Rufo => format_using_rufo(snippet_path),
            Self::RustFmt => format_using_rustfmt(snippet_path),
            Self::SQLFormatter => format_using_sql_formatter(snippet_path),
            Self::Scalafmt => format_using_scalafmt(snippet_path),
            Self::Shfmt => format_using_shfmt(snippet_path),
            Self::Sqlfluff => format_using_sqlfluff(snippet_path),
            Self::Standardjs => format_using_standardjs(snippet_path),
            Self::Standardrb => format_using_standardrb(snippet_path),
            Self::StyleLint => format_using_stylelint(snippet_path),
            Self::StylishHaskell => format_using_stylish_haskell(snippet_path),
            Self::Stylua => format_using_stylua(snippet_path),
            Self::Taplo => format_using_taplo(snippet_path),
            Self::TerraformFmt => format_using_terraform_fmt(snippet_path),
            Self::TofuFmt => format_using_tofu_fmt(snippet_path),
            Self::Typos => format_using_typos(snippet_path),
            Self::Usort => format_using_usort(snippet_path),
            Self::XmlFormat => format_using_xmlformat(snippet_path),
            Self::XmlLint => format_using_xmllint(snippet_path),
            Self::YamlFix => format_using_yamlfix(snippet_path),
            Self::YamlFmt => format_using_yamlfmt(snippet_path),
            Self::Yapf => format_using_yapf(snippet_path),
            Self::YewFmt => format_using_yew_fmt(snippet_path),
            Self::ZigFmt => format_using_zigfmt(snippet_path),
            Self::Zprint => format_using_zprint(snippet_path),
        }
    }
}

impl core::fmt::Display for Tooling {
    #[allow(clippy::too_many_lines)]
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Alejandra => write!(f, "alejandra"),
            Self::AppleSwiftFormat => write!(f, "swift-format"),
            Self::Asmfmt => write!(f, "asmfmt"),
            Self::Astyle => write!(f, "astyle"),
            Self::AutoOptional => write!(f, "auto-optional"),
            Self::Autocorrect => write!(f, "autocorrect"),
            Self::Autoflake => write!(f, "autoflake"),
            Self::Autopep8 => write!(f, "autopep8"),
            Self::Beautysh => write!(f, "beautysh"),
            Self::BicepFormat => write!(f, "bicep_format"),
            Self::Biome => write!(f, "biome"),
            Self::Black => write!(f, "black"),
            Self::BladeFormatter => write!(f, "blade-formatter"),
            Self::Blue => write!(f, "blue"),
            Self::Bpfmt => write!(f, "bpfmt"),
            Self::Buf => write!(f, "buf"),
            Self::Buildifier => write!(f, "buildifier"),
            Self::CSharpier => write!(f, "csharpier"),
            Self::CabalFormat => write!(f, "cabal_format"),
            Self::ClangFormat => write!(f, "clang-format"),
            Self::Cljstyle => write!(f, "cljstyle"),
            Self::Codespell => write!(f, "codespell"),
            Self::CrlFmt => write!(f, "crlfmt"),
            Self::CrystalFormat => write!(f, "crystal_format"),
            Self::DFmt => write!(f, "dfmt"),
            Self::D2 => write!(f, "d2"),
            Self::DartFormat => write!(f, "dart_format"),
            Self::DenoFmt => write!(f, "deno_fmt"),
            Self::DjLint => write!(f, "djlint"),
            Self::Docstrfmt => write!(f, "docstrfmt"),
            Self::Dprint => write!(f, "dprint"),
            Self::Efmt => write!(f, "efmt"),
            Self::ElmFormat => write!(f, "elm-format"),
            Self::ErbFormatter => write!(f, "erb-formatter"),
            Self::Erlfmt => write!(f, "erlfmt"),
            Self::Eslint => write!(f, "eslint"),
            Self::Fantomas => write!(f, "fantomas"),
            Self::Findent => write!(f, "findent"),
            Self::FishIndent => write!(f, "fish_indent"),
            Self::Fixjson => write!(f, "fixjson"),
            Self::Fnlfmt => write!(f, "fnlfmt"),
            Self::ForgeFmt => write!(f, "forge_fmt"),
            Self::Fourmolu => write!(f, "fourmolu"),
            Self::Fprettify => write!(f, "fprettify"),
            Self::GCI => write!(f, "gci"),
            Self::Gdformat => write!(f, "gdformat"),
            Self::Gersemi => write!(f, "gersemi"),
            Self::GleamFormat => write!(f, "gleam_format"),
            Self::GoFmt => write!(f, "gofmt"),
            Self::GoFumpt => write!(f, "gofumpt"),
            Self::GoImports => write!(f, "goimports"),
            Self::GoImportsReviser => write!(f, "goimports-reviser"),
            Self::GoLines => write!(f, "golines"),
            Self::GoogleJavaFormat => write!(f, "google-java-format"),
            Self::HIndent => write!(f, "hundent"),
            Self::Htmlbeautifier => write!(f, "htmlbeautifier"),
            Self::Isort => write!(f, "isort"),
            Self::Joker => write!(f, "joker"),
            Self::JuliaFormatterJl => write!(f, "juliaformatter.jl"),
            Self::JustFmt => write!(f, "just_fmt"),
            Self::KclFmt => write!(f, "kcl_fmt"),
            Self::Kdlfmt => write!(f, "kdlfmt"),
            Self::Ktfmt => write!(f, "ktfmt"),
            Self::Ktlint => write!(f, "ktlint"),
            Self::LeptosFmt => write!(f, "leptosfmt"),
            Self::LuaFormatter => write!(f, "luaformatter"),
            Self::Markdownlint => write!(f, "markdownlint"),
            Self::MdFormat => write!(f, "mdformat"),
            Self::Misspell => write!(f, "misspell"),
            Self::MixFormat => write!(f, "mix_format"),
            Self::NicklockwoodSwiftFormat => write!(f, "swiftformat"),
            Self::Nimpretty => write!(f, "nimpretty"),
            Self::Nixfmt => write!(f, "nixfmt"),
            Self::NixpkgsFmt => write!(f, "nixpkgs-fmt"),
            Self::NpmGroovyLint => write!(f, "npm-groovy-lint"),
            Self::OCamlFormat => write!(f, "ocamlformat"),
            Self::OcpIndent => write!(f, "ocpindent"),
            Self::Ormolu => write!(f, "ormolu"),
            Self::PerlTidy => write!(f, "perltidy"),
            Self::Prettier => write!(f, "prettier"),
            Self::PuppetLint => write!(f, "puppet-lint"),
            Self::PursTidy => write!(f, "purs-tidy"),
            Self::PyInk => write!(f, "pyink"),
            Self::ReScriptFormat => write!(f, "rescript_format"),
            Self::RocFormat => write!(f, "roc_format"),
            Self::RstFmt => write!(f, "rstfmt"),
            Self::RuboCop => write!(f, "rubocop"),
            Self::RubyFmt => write!(f, "rubyfmt"),
            Self::Ruff => write!(f, "ruff"),
            Self::Rufo => write!(f, "rufo"),
            Self::RustFmt => write!(f, "rustfmt"),
            Self::SQLFormatter => write!(f, "sql-formatter"),
            Self::Scalafmt => write!(f, "scalafmt"),
            Self::Shfmt => write!(f, "shfmt"),
            Self::Sqlfluff => write!(f, "qqlfluff"),
            Self::Standardjs => write!(f, "standardjs"),
            Self::Standardrb => write!(f, "standardrb"),
            Self::StyleLint => write!(f, "stylelint"),
            Self::StylishHaskell => write!(f, "stylish-haskell"),
            Self::Stylua => write!(f, "stylua"),
            Self::Taplo => write!(f, "taplo"),
            Self::TerraformFmt => write!(f, "terraform_fmt"),
            Self::TofuFmt => write!(f, "tofu_fmt"),
            Self::Typos => write!(f, "typos"),
            Self::Usort => write!(f, "usort"),
            Self::XmlFormat => write!(f, "xmlformat"),
            Self::XmlLint => write!(f, "xmllint"),
            Self::YamlFix => write!(f, "yamlfix"),
            Self::YamlFmt => write!(f, "yamlfmt"),
            Self::Yapf => write!(f, "yapf"),
            Self::YewFmt => write!(f, "yew-fmt"),
            Self::ZigFmt => write!(f, "zigfmt"),
            Self::Zprint => write!(f, "zprint"),
        }
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
                let formatter_name = f.to_string();

                print_formatter_info(&formatter_name, info);

                let time = std::time::Instant::now();

                let r = f.format_snippet(snippet_path);

                print_formatter_time(&formatter_name, info, time.elapsed());

                if let Err(e) = &r {
                    if let MdsfError::MissingBinary(binary) = e {
                        print_binary_not_in_path(
                            if &formatter_name == binary {
                                formatter_name
                            } else {
                                format!("{binary} ({formatter_name})")
                            }
                            .as_str(),
                        );

                        return Ok((false, None));
                    } else if matches!(e, MdsfError::FormatterError) {
                        print_error_formatting(&formatter_name, info);
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

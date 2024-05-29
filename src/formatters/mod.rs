use std::{ffi::OsStr, io::Write, process::Command};

use schemars::JsonSchema;
use tempfile::NamedTempFile;
use which::which;

use crate::{config::MdsfConfig, error::MdsfError, languages::Language, LineInfo, DEBUG};

pub mod alejandra;
pub mod autopep8;
pub mod beautysh;
pub mod biome;
pub mod black;
pub mod blade_formatter;
pub mod blue;
pub mod buf;
pub mod cabal_format;
pub mod clang_format;
pub mod cljstyle;
pub mod crlfmt;
pub mod crystal_format;
pub mod csharpier;
pub mod dart_format;
pub mod deno_fmt;
pub mod dfmt;
pub mod djlint;
pub mod efmt;
pub mod elm_format;
pub mod erb_formatter;
pub mod erlfmt;
pub mod fantomas;
pub mod fourmolu;
pub mod fprettify;
pub mod gleam_format;
pub mod gofmt;
pub mod gofumpt;
pub mod goimports;
pub mod google_java_format;
pub mod hindent;
pub mod isort;
pub mod joker;
pub mod juliaformatter_jl;
pub mod just_fmt;
pub mod kcl_fmt;
pub mod ktfmt;
pub mod ktlint;
pub mod leptosfmt;
pub mod luaformatter;
pub mod mdformat;
pub mod mix_format;
pub mod nimpretty;
pub mod nixfmt;
pub mod nixpkgs_fmt;
pub mod npm_groovy_lint;
pub mod ocamlformat;
pub mod ocp_indent;
pub mod ormolu;
pub mod perltidy;
pub mod prettier;
pub mod purs_tidy;
pub mod rescript_format;
pub mod roc_format;
pub mod rstfmt;
pub mod rubocop;
pub mod rubyfmt;
pub mod ruff;
pub mod rufo;
pub mod rustfmt;
pub mod scalafmt;
pub mod shfmt;
pub mod sql_formatter;
pub mod sqlfluff;
pub mod standardjs;
pub mod standardrb;
pub mod stylelint;
pub mod stylish_haskell;
pub mod stylua;
pub mod swift_format;
pub mod swiftformat;
pub mod taplo;
pub mod terraform_fmt;
pub mod tofu_fmt;
pub mod usort;
pub mod xmlformat;
pub mod xmllint;
pub mod yamlfix;
pub mod yamlfmt;
pub mod yapf;
pub mod yew_fmt;
pub mod zigfmt;

#[inline]
pub fn setup_snippet(code: &str, file_ext: &str) -> std::io::Result<NamedTempFile> {
    let mut b = tempfile::Builder::new();

    b.rand_bytes(12).suffix(file_ext).prefix(
        // ktlint wants PascalCase file names
        if file_ext == Language::Kotlin.to_file_ext() {
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
    if let Ok(snippet) = setup_snippet(code, info.language.to_file_ext()) {
        let snippet_path = snippet.path();

        if let Ok(Some(formatted_code)) = match info.language {
            Language::Blade => config.blade.format(snippet_path, info),
            Language::C => config.c.format(snippet_path, info),
            Language::Cabal => config.cabal.format(snippet_path, info),
            Language::Clojure => config.clojure.format(snippet_path, info),
            Language::Cpp => config.cpp.format(snippet_path, info),
            Language::Crystal => config.crystal.format(snippet_path, info),
            Language::CSharp => config.csharp.format(snippet_path, info),
            Language::Css(_flavor) => config.css.format(snippet_path, info),
            Language::D => config.d.format(snippet_path, info),
            Language::Dart => config.dart.format(snippet_path, info),
            Language::Elixir => config.elixir.format(snippet_path, info),
            Language::Elm => config.elm.format(snippet_path, info),
            Language::Erb => config.erb.format(snippet_path, info),
            Language::Erlang => config.erlang.format(snippet_path, info),
            Language::Fortran => config.fortran.format(snippet_path, info),
            Language::FSharp => config.fsharp.format(snippet_path, info),
            Language::Gleam => config.gleam.format(snippet_path, info),
            Language::Go => config.go.format(snippet_path, info),
            Language::GraphQL => config.graphql.format(snippet_path, info),
            Language::Groovy => config.groovy.format(snippet_path, info),
            Language::Handlebars => config.handlebars.format(snippet_path, info),
            Language::Haskell => config.haskell.format(snippet_path, info),
            Language::Hcl => config.hcl.format(snippet_path, info),
            Language::Html => config.html.format(snippet_path, info),
            Language::Java => config.java.format(snippet_path, info),
            Language::JavaScript(_flavor) => config.javascript.format(snippet_path, info),
            Language::Json(_flavor) => config.json.format(snippet_path, info),
            Language::Julia => config.julia.format(snippet_path, info),
            Language::Just => config.just.format(snippet_path, info),
            Language::Kcl => config.kcl.format(snippet_path, info),
            Language::Kotlin => config.kotlin.format(snippet_path, info),
            Language::Lua => config.lua.format(snippet_path, info),
            Language::Markdown => config.markdown.format(snippet_path, info),
            Language::Mustache => config.mustache.format(snippet_path, info),
            Language::Nim => config.nim.format(snippet_path, info),
            Language::Nix => config.nix.format(snippet_path, info),
            Language::Nunjucks => config.nunjucks.format(snippet_path, info),
            Language::ObjectiveC => config.objective_c.format(snippet_path, info),
            Language::OCaml => config.ocaml.format(snippet_path, info),
            Language::Perl => config.perl.format(snippet_path, info),
            Language::Protobuf => config.protobuf.format(snippet_path, info),
            Language::PureScript => config.purescript.format(snippet_path, info),
            Language::Python => config.python.format(snippet_path, info),
            Language::ReScript => config.rescript.format(snippet_path, info),
            Language::ReStructuredText => config.restructuredtext.format(snippet_path, info),
            Language::Roc => config.roc.format(snippet_path, info),
            Language::Ruby => config.ruby.format(snippet_path, info),
            Language::Rust => config.rust.format(snippet_path, info),
            Language::Scala => config.scala.format(snippet_path, info),
            Language::Shell(_flavor) => config.shell.format(snippet_path, info),
            Language::Sql => config.sql.format(snippet_path, info),
            Language::Swift => config.swift.format(snippet_path, info),
            Language::Toml => config.toml.format(snippet_path, info),
            Language::TypeScript(_flavor) => config.typescript.format(snippet_path, info),
            Language::Vue => config.vue.format(snippet_path, info),
            Language::Xml => config.xml.format(snippet_path, info),
            Language::Yaml => config.yaml.format(snippet_path, info),
            Language::Zig => config.zig.format(snippet_path, info),
        } {
            let mut f = formatted_code.trim().to_owned();

            f.push('\n');

            return f;
        }
    }

    code.to_owned()
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
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

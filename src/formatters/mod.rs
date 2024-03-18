use std::{io::Write, process::Command};

use schemars::JsonSchema;
use tempfile::NamedTempFile;

use crate::{config::MdsfConfig, languages::Language};

pub mod autopep8;
pub mod biome;
pub mod black;
pub mod blade_formatter;
pub mod blue;
pub mod clang_format;
pub mod crystal_format;
pub mod dart_format;
pub mod deno_format;
pub mod elm_format;
pub mod gleam_format;
pub mod gofmt;
pub mod gofumpt;
pub mod isort;
pub mod just_fmt;
pub mod mix_format;
pub mod nimpretty;
pub mod prettier;
pub mod roc_format;
pub mod rubocop;
pub mod ruff;
pub mod rustfmt;
pub mod shfmt;
pub mod sql_formatter;
pub mod sqlfluff;
pub mod stylua;
pub mod taplo;
pub mod usort;
pub mod yapf;
pub mod zigfmt;

#[inline]
pub fn setup_snippet(code: &str, file_ext: &str) -> std::io::Result<NamedTempFile> {
    let mut f = tempfile::Builder::new()
        .rand_bytes(12)
        .suffix(file_ext)
        .tempfile()?;

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
) -> std::io::Result<(bool, Option<String>)> {
    if let Err(err) = result {
        if err.kind() == std::io::ErrorKind::NotFound {
            return Ok((true, None));
        }

        return Err(err);
    }

    if matches!(result, Ok(true)) {
        return read_snippet(snippet_path).map(|code| (false, Some(code)));
    }

    Ok((false, None))
}

fn spawn_command(cmd: &mut Command) -> std::io::Result<bool> {
    #[cfg(not(test))]
    cmd.stdout(std::process::Stdio::null());
    #[cfg(not(test))]
    cmd.stderr(std::process::Stdio::null());

    Ok(cmd.spawn()?.wait()?.success())
}

#[inline]
pub fn execute_command(
    cmd: &mut Command,
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    handle_post_execution(spawn_command(cmd), snippet_path)
}

#[inline]
pub fn format_snippet(config: &MdsfConfig, language: &Language, code: &str) -> String {
    if let Ok(snippet) = setup_snippet(code, language.to_file_ext()) {
        let snippet_path = snippet.path();

        if let Ok(Some(formatted_code)) = match language {
            Language::Blade => config.blade.format(snippet_path),
            Language::C => config.c.format(snippet_path),
            Language::CSharp => config.csharp.format(snippet_path),
            Language::Cpp => config.cpp.format(snippet_path),
            Language::Crystal => config.crystal.format(snippet_path),
            Language::Css => config.css.format(snippet_path),
            Language::Dart => config.dart.format(snippet_path),
            Language::Elm => config.elm.format(snippet_path),
            Language::Elixir => config.elixir.format(snippet_path),
            Language::Gleam => config.gleam.format(snippet_path),
            Language::Go => config.go.format(snippet_path),
            Language::GraphQL => config.graphql.format(snippet_path),
            Language::Html => config.html.format(snippet_path),
            Language::Java => config.java.format(snippet_path),
            Language::JavaScript => config.javascript.format(snippet_path),
            Language::Json => config.json.format(snippet_path),
            Language::Just => config.just.format(snippet_path),
            Language::Lua => config.lua.format(snippet_path),
            Language::Markdown => config.markdown.format(snippet_path),
            Language::Nim => config.nim.format(snippet_path),
            Language::ObjectiveC => config.objective_c.format(snippet_path),
            Language::Protobuf => config.protobuf.format(snippet_path),
            Language::Python => config.python.format(snippet_path),
            Language::Roc => config.roc.format(snippet_path),
            Language::Ruby => config.ruby.format(snippet_path),
            Language::Rust => config.rust.format(snippet_path),
            Language::Shell => config.shell.format(snippet_path),
            Language::Sql => config.sql.format(snippet_path),
            Language::Toml => config.toml.format(snippet_path),
            Language::TypeScript => config.typescript.format(snippet_path),
            Language::Vue => config.vue.format(snippet_path),
            Language::Yaml => config.yaml.format(snippet_path),
            Language::Zig => config.zig.format(snippet_path),
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
pub enum MdsfFormatter<T> {
    Single(T),
    Multiple(Vec<MdsfFormatter<T>>),
}

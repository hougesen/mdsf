use std::{
    io::Write,
    process::{Command, Stdio},
};

use tempfile::NamedTempFile;

use crate::{
    config::MdsfConfig,
    languages::{Language, LanguageFormatter},
};

pub mod biome;
pub mod gleam_format;
pub mod mix_format;
pub mod nimpretty;
pub mod prettier;
pub mod ruff;
pub mod rustfmt;
pub mod stylua;
pub mod taplo;
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
pub fn execute_command(cmd: &mut Command) -> std::io::Result<bool> {
    Ok(cmd
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?
        .success())
}

#[inline]
pub fn format_snippet(config: &MdsfConfig, language: &Language, code: &str) -> String {
    if let Ok(snippet) = setup_snippet(code, language.to_file_ext()) {
        let snippet_path = snippet.path();

        if let Ok(Some(formatted_code)) = match language {
            Language::Css => config.css.format(snippet_path),
            Language::Elixir => config.elixir.format(snippet_path),
            Language::Gleam => config.gleam.format(snippet_path),
            Language::Html => config.html.format(snippet_path),
            Language::JavaScript => config.javascript.format(snippet_path),
            Language::Json => config.json.format(snippet_path),
            Language::Lua => config.lua.format(snippet_path),
            Language::Markdown => config.markdown.format(snippet_path),
            Language::Nim => config.nim.format(snippet_path),
            Language::Python => config.python.format(snippet_path),
            Language::Rust => config.rust.format(snippet_path),
            Language::Toml => config.toml.format(snippet_path),
            Language::TypeScript => config.typescript.format(snippet_path),
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

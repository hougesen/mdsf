use std::{io::Write, process::Command};

use tempfile::NamedTempFile;

use crate::languages::Language;

use self::{
    nimpretty::format_using_nimpretty, ruff::format_using_ruff, rustfmt::format_using_rustfmt,
    stylua::format_using_stylua, zigfmt::format_using_zigfmt,
};

mod nimpretty;
mod ruff;
mod rustfmt;
mod stylua;
mod zigfmt;

pub fn setup_snippet(code: &str) -> std::io::Result<NamedTempFile> {
    let mut f = NamedTempFile::new()?;

    f.write_all(code.as_bytes())?;
    f.flush()?;

    Ok(f)
}

pub fn read_snippet(file_path: &std::path::Path) -> std::io::Result<String> {
    std::fs::read_to_string(file_path)
}

pub fn execute_command(cmd: &mut Command) -> std::io::Result<bool> {
    Ok(cmd.spawn()?.wait()?.success())
}

pub fn format_snippet(language: &str, code: String) -> String {
    let mut f = if let Some(l) = Language::from_str(language) {
        if let Ok(snippet) = setup_snippet(&code) {
            let snippet_path = snippet.path();

            if let Ok(Some(formatted_code)) = match l {
                Language::Lua => format_using_stylua(snippet_path),
                Language::Nim => format_using_nimpretty(snippet_path),
                Language::Python => format_using_ruff(snippet_path),
                Language::Rust => format_using_rustfmt(snippet_path),
                Language::Zig => format_using_zigfmt(snippet_path),
            } {
                return formatted_code;
            }
        }

        code
    } else {
        code
    }
    .trim()
    .to_owned();

    f.push('\n');

    f
}

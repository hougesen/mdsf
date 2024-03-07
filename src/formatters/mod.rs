use std::{
    io::Write,
    process::{Command, Stdio},
};

use tempfile::NamedTempFile;

use crate::languages::Language;

use self::{
    biome::format_using_biome, nimpretty::format_using_nimpretty, ruff::format_using_ruff,
    rustfmt::format_using_rustfmt, stylua::format_using_stylua, taplo::format_using_taplo,
    zigfmt::format_using_zigfmt,
};

mod biome;
mod nimpretty;
mod ruff;
mod rustfmt;
mod stylua;
mod taplo;
mod zigfmt;

pub fn setup_snippet(code: &str, file_ext: &str) -> std::io::Result<NamedTempFile> {
    let mut f = tempfile::Builder::new()
        .rand_bytes(12)
        .suffix(file_ext)
        .tempfile()?;

    f.write_all(code.as_bytes())?;
    f.flush()?;

    Ok(f)
}

pub fn read_snippet(file_path: &std::path::Path) -> std::io::Result<String> {
    std::fs::read_to_string(file_path)
}

pub fn execute_command(cmd: &mut Command) -> std::io::Result<bool> {
    Ok(cmd.stdout(Stdio::null()).spawn()?.wait()?.success())
}

pub fn format_snippet(language: &str, code: String) -> String {
    let mut f = if let Some(l) = Language::from_str(language) {
        if let Ok(snippet) = setup_snippet(&code, l.to_file_ext()) {
            let snippet_path = snippet.path();

            if let Ok(Some(formatted_code)) = match l {
                Language::JavaScript => format_using_biome(snippet_path),
                Language::Json => format_using_biome(snippet_path),
                Language::Lua => format_using_stylua(snippet_path),
                Language::Nim => format_using_nimpretty(snippet_path),
                Language::Python => format_using_ruff(snippet_path),
                Language::Rust => format_using_rustfmt(snippet_path),
                Language::Toml => format_using_taplo(snippet_path),
                Language::TypeScript => format_using_biome(snippet_path),
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

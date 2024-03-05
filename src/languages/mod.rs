use std::{io::Write, process::Command};

use tempfile::NamedTempFile;

use self::{lua::format_using_stylua, rust::format_using_rustfmt};

mod lua;
mod rust;

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
    let mut f = if let Ok(Some(formatted_code)) = match language {
        "rust" => format_using_rustfmt(&code),
        "lua" => format_using_stylua(&code),
        _ => Ok(None),
    } {
        return formatted_code;
    } else {
        code
    }
    .trim()
    .to_string();

    f.push('\n');

    f
}

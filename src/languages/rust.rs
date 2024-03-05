use crate::languages::read_snippet;

use super::{execute_command, setup_snippet};

pub fn format_using_rustfmt(code: &str) -> std::io::Result<Option<String>> {
    let file = setup_snippet(code)?;
    let fp = file.path();

    let mut cmd = std::process::Command::new("rustfmt");

    cmd.arg(fp);

    if execute_command(&mut cmd)? {
        return read_snippet(fp).map(Some);
    }

    Ok(None)
}

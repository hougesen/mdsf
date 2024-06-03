use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_fish_indent(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("fish_indent");

    cmd.arg("-w").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

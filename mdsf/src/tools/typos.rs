///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-w");
    cmd.arg("--no-ignore");
    cmd.arg("--hidden");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("typos")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_typos {
    #[test_with::executable(typos)]
    fn test_typos_python_cba663e4f5e54b7f() {
        let input = r#"anouncement"#;

        let output = r#"announcement"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::Typos.test_format_snippet(input, output, &file_ext);
    }
}

///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("lint");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("erg")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_erg_lint {
    #[test_with::executable(erg)]
    fn test_erg_lint_erg_802e6b757d972583() {
        let input = r#"add(a, b) = a + b

print!(add(1, 2))
"#;

        let output = r#"add(a, b) = a + b

print!(add(1, 2))
"#;

        let file_ext = crate::fttype::get_file_extension(".erg");

        crate::tools::Tooling::ErgLint.test_format_snippet(input, output, &file_ext);
    }
}

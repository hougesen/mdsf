///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("tool");
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("crystal")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_crystal_format {
    #[test_with::executable(crystal)]
    fn test_crystal_format_crystal_e0f2d532cd984bee() {
        let input = r#"def add(a, b)  return a + b end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let file_ext = crate::fttype::get_file_extension("crystal");

        crate::tools::Tooling::CrystalFormat.test_format_snippet(input, output, &file_ext);
    }
}

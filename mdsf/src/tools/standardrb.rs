///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 2] = [
    CommandType::Direct("standardrb"),
    CommandType::GemExec("standardrb"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_standardrb {
    #[test_with::executable(standardrb || gem)]
    fn test_standardrb_ruby_bec6c50c1664b6ed() {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  a + b
end
"#;

        let file_ext = crate::fttype::get_file_extension("ruby");

        crate::tools::Tooling::Standardrb.test_format_snippet(input, output, &file_ext);
    }
}

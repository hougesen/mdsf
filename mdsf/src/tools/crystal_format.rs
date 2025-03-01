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

#[cfg(test)]
mod test_crystal_format {
    const TIMEOUT: u64 = 0;
    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(crystal)]
    fn test_crystal_format_crystal_e0f2d532cd984bee() {
        let input = r#"def add(a, b)  return a + b end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let file_ext = crate::fttype::get_file_extension("crystal");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            false,
            DEBUG_ENABLED,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

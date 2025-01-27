///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("gleam")];

#[cfg(test)]
mod test_gleam_format {
    #[test_with::executable(gleam)]
    fn test_gleam_format_gleam_d23656d11ef3a81d() {
        let input = r#"pub fn add(a:Int,b:Int)->Int{a+b}"#;

        let output = r#"pub fn add(a: Int, b: Int) -> Int {
  a + b
}
"#;

        let file_ext = crate::fttype::get_file_extension("gleam");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result =
            crate::execution::run_tools(&super::COMMANDS, snippet.path(), super::set_arguments, 0)
                .expect("it to be successful")
                .1
                .expect("it to be some");

        assert_eq!(result, output);
    }
}

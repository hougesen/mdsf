///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fix-layout");
    cmd.arg("--autocorrect");
    cmd.arg("--format");
    cmd.arg("quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("rubocop")];

#[cfg(test)]
mod test_rubocop {
    #[test_with::executable(rubocop)]
    fn test_rubocop_ruby_d2b8a6db3c8eee1c() {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let file_ext = crate::fttype::get_file_extension("ruby");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            0,
            false,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

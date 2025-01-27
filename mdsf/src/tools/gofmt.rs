///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_arguments(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("gofmt")];

#[cfg(test)]
mod test_gofmt {
    #[test_with::executable(gofmt)]
    fn test_gofmt_go_3b56f602fe22977b() {
        let input = r#"package main

   func add(a int , b int  ) int {
                return a + b
       }

    "#;

        let output = r#"package main

func add(a int, b int) int {
	return a + b
}
"#;

        let file_ext = crate::fttype::get_file_extension("go");

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

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
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("gofumpt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_gofumpt {
    #[test_with::executable(gofumpt)]
    fn test_gofumpt_go_3b56f602fe22977b() {
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

        crate::tools::Tooling::Gofumpt.test_format_snippet(input, output, &file_ext);
    }
}

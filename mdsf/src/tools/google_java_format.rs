///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_google_java_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("google-java-format")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_google_java_format_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path, timeout);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
}

#[cfg(test)]
mod test_google_java_format {
    #[test_with::executable(google-java-format)]
    fn test_google_java_format_java_4950b011328e8048() {
        let input = r#"class HelloWorld {
    public static void main(String[] args) {
                System.out.println("Hello");
                System.out.println("World!");
                 }
}"#;
        let output = Some(
            r#"class HelloWorld {
  public static void main(String[] args) {
    System.out.println("Hello");
    System.out.println("World!");
  }
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("java");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::google_java_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

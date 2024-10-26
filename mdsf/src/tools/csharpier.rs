use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_csharpier_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("csharpier");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("dotnet")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_csharpier_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

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
mod test_csharpier {
    #[test_with::executable(dotnet)]
    fn test_csharpier_csharp_257c6494c2fdb601() {
        let input = r#"namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                var c=a+b ;
                                                        return c ;
                                                    }
                                                 }
                                                 } "#;
        let output = r#"namespace Mdsf
{
    class Adder
    {
        public static int add(int a, int b)
        {
            var c = a + b;
            return c;
        }
    }
}
"#;
        let file_ext = crate::fttype::get_file_extension("csharp");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::csharpier::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}

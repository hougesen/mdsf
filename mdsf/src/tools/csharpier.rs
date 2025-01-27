///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_csharpier_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("csharpier");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("dotnet")];

    crate::execution::run_tools(&commands, file_path, timeout, set_csharpier_args)
}

#[cfg(test)]
mod test_csharpier {
    #[test_with::executable(dotnet)]
    fn test_csharpier_csharp_4f321bc3873fc142() {
        let input = r#"namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                var c=a+b ;
                                                        return c ;
                                                    }
                                                 }
                                                 } "#;
        let output = Some(
            r#"namespace Mdsf
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
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("csharp");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::csharpier::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

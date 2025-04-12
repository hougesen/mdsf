///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("csharpier");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("dotnet")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_csharpier {
    #[test_with::executable(dotnet)]
    fn test_csharpier_csharp_a79aa94ad2d86b6c() {
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

        crate::tools::Tooling::Csharpier.test_format_snippet(input, output, &file_ext);
    }
}

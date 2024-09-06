use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("dotnet").build();

    cmd.arg("csharpier").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_csharpier {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(dotnet)]
    fn it_should_format_csharp() {
        let input = "namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                var c=a+b ;
                                                        return c ;
                                                    }
                                                 }
                                                 } ";

        let expected_output = "namespace Mdsf
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
";

        let snippet =
            setup_snippet(input, language_to_ext("csharp")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}

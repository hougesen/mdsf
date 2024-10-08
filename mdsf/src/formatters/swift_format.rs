use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("swift-format").build();

    cmd.arg("--in-place").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_swift_format {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(swift_format)]
    fn it_should_format_swift() {
        let input = " func add(a:Int ,b:Int)->Int {
    return a + b
    }";

        let expected_output = "func add(a: Int, b: Int) -> Int {
    return a + b
}
";

        let snippet = setup_snippet(input, &get_file_extension("swift"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

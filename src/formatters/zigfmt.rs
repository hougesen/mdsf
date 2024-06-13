use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_zigfmt(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("zig");

    cmd.arg("fmt");

    cmd.arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_zigfmt {
    use crate::{
        formatters::{setup_snippet, zigfmt::format_using_zigfmt},
        generated::language_to_ext,
    };

    #[test_with::executable(zig)]
    fn it_should_format_zig() {
        let input = "
    fn     add   (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }
    ";

        let expected_output = "fn add(a: i32, b: i32) i32 {
    return a + b;
}
";

        let snippet =
            setup_snippet(input, &language_to_ext("zig")).expect("it to create a snippet file");

        let output = format_using_zigfmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

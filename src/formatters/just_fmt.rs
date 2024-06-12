use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_just_fmt(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("just");

    cmd.arg("--fmt")
        // TODO: remove once it is stabilized
        .arg("--unstable")
        .arg("--justfile")
        .arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_just_fmt {
    use crate::{
        formatters::{just_fmt::format_using_just_fmt, setup_snippet},
        generated::language_to_ext,
    };

    #[test_with::executable(just)]
    #[test]
    fn it_should_format_just() {
        let input = "build:
                cargo build
                cargo build --release
            ";

        let expected_output = "build:
    cargo build
    cargo build --release
";

        let snippet =
            setup_snippet(input, &language_to_ext("just")).expect("it to create a snippet file");

        let output = format_using_just_fmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

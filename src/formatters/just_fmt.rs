use super::execute_command;

#[inline]
pub fn format_using_just_fmt(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
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
        languages::Language,
    };

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

        let snippet = setup_snippet(input, Language::Gleam.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_just_fmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

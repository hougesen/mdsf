use super::{execute_command, read_snippet};

#[inline]
pub fn format_using_zigfmt(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    let mut cmd = std::process::Command::new("zig");

    cmd.arg("fmt");

    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}

#[cfg(test)]
mod test_zigfmt {
    use crate::{
        formatters::{setup_snippet, zigfmt::format_using_zigfmt},
        languages::Language,
    };

    #[test]
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
            setup_snippet(input, Language::Zig.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_zigfmt(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

use super::{execute_command, read_snippet};

#[inline]
pub fn format_using_gleam_format(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    // TODO: use installed biome instead
    let mut cmd = std::process::Command::new("gleam");

    // Incase the use hasn't installed biome
    cmd.arg("format").arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}

#[cfg(test)]
mod test_gleam_format {
    use crate::{
        formatters::{gleam_format::format_using_gleam_format, setup_snippet},
        languages::Language,
    };

    #[test]
    fn it_should_format_gleam() {
        let input = "pub fn add(a:Int,b:Int)->Int{a+b}";
        let expected_output = "pub fn add(a: Int, b: Int) -> Int {
  a + b
}
";

        let snippet = setup_snippet(input, Language::Gleam.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_gleam_format(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

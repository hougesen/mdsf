use super::execute_command;

#[inline]
pub fn format_using_buf(snippet_path: &std::path::Path) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("buf");

    cmd.arg("format").arg("--write").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_buf {
    use crate::{formatters::setup_snippet, languages::Language};

    use super::format_using_buf;

    #[test_with::executable(buf)]
    #[test]
    fn it_should_format_protobuf() {
        let input = "service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }";

        let expected_output = "service SearchService {
  rpc Search(SearchRequest) returns (SearchResponse);
}
";

        let snippet = setup_snippet(input, Language::Protobuf.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_buf(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

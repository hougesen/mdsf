///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("buf"),
    CommandType::Direct("buf"),
    CommandType::Npm("@bufbuild/buf"),
    CommandType::Pnpm("@bufbuild/buf"),
    CommandType::Bun("@bufbuild/buf"),
    CommandType::Deno("@bufbuild/buf"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_buf_format {
    #[test_with::executable(buf || npx || pnpm || deno || bunx)]
    fn test_buf_format_protobuf_10af516c8a015ab5() {
        let input = r#"service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }"#;

        let output = r#"service SearchService {
  rpc Search(SearchRequest) returns (SearchResponse);
}
"#;

        let file_ext = crate::fttype::get_file_extension("protobuf");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::BufFormat
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}

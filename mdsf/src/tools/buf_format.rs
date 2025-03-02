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

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("buf"),
    CommandType::Direct("buf"),
    CommandType::Npm("@bufbuild/buf"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_buf_format {
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(npx)]
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

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

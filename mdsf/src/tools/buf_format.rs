///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_buf_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg("--write");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("buf"),
        CommandType::Direct("buf"),
        CommandType::Npm("@bufbuild/buf"),
    ];

    crate::execution::run_tools(&commands, file_path, timeout, set_buf_format_args)
}

#[cfg(test)]
mod test_buf_format {
    #[test_with::executable(npx)]
    fn test_buf_format_protobuf_bbae42b1d9a7bf36() {
        let input = r#"service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }"#;
        let output = Some(
            r#"service SearchService {
  rpc Search(SearchRequest) returns (SearchResponse);
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("protobuf");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::buf_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

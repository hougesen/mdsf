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

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("buf"),
    CommandType::Direct("buf"),
    CommandType::Npm("@bufbuild/buf"),
    CommandType::Pnpm("@bufbuild/buf"),
    CommandType::Bun("@bufbuild/buf"),
    CommandType::Deno("@bufbuild/buf"),
    CommandType::Yarn("@bufbuild/buf"),
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

        crate::tools::Tooling::BufFormat.test_format_snippet(input, output, &file_ext);
    }
}

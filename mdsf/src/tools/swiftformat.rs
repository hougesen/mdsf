///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("swiftformat")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_swiftformat {
    #[test_with::executable(swiftformat)]
    fn test_swiftformat_swift_5717762df3975151() {
        let input = r#" func add(a:Int ,b:Int)->Int {
    return a + b
    }"#;

        let output = r#"func add(a: Int, b: Int) -> Int {
    return a + b
}
"#;

        let file_ext = crate::fttype::get_file_extension("swift");

        crate::tools::Tooling::Swiftformat.test_format_snippet(input, output, &file_ext);
    }
}

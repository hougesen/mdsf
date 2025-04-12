///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-r");
    cmd.arg("--type");
    cmd.arg("js");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("js-beautify"),
    CommandType::Direct("js-beautify"),
    CommandType::Npm("js-beautify"),
    CommandType::Pnpm("js-beautify"),
    CommandType::Bun("js-beautify"),
    CommandType::Deno("js-beautify"),
    CommandType::Yarn("js-beautify"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_js_beautify {
    #[test_with::executable(js-beautify || npx || pnpm || deno || bunx)]
    fn test_js_beautify_javascript_151bf21bc63609e8() {
        let input = r#"function add (a,b){return a +b }"#;

        let output = r#"function add(a, b) {
    return a + b
}"#;

        let file_ext = crate::fttype::get_file_extension("javascript");

        crate::tools::Tooling::JsBeautify.test_format_snippet(input, output, &file_ext);
    }
}

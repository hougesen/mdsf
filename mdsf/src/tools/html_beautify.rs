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
    cmd.arg("html");
    cmd.arg("-f");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("html-beautify"),
    CommandType::Direct("html-beautify"),
    CommandType::Npm("js-beautify"),
    CommandType::Pnpm("js-beautify"),
    CommandType::Bun("js-beautify"),
    CommandType::Deno("js-beautify"),
    CommandType::Yarn("js-beautify"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_html_beautify {
    #[test_with::executable(html-beautify || npx || pnpm || deno || bunx)]
    fn test_html_beautify_html_63850f31f2ef5caf() {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;

        let output = r#"<div>
    <p>
        Mads was here
    </p>
</div>"#;

        let file_ext = crate::fttype::get_file_extension("html");

        crate::tools::Tooling::HtmlBeautify.test_format_snippet(input, output, &file_ext);
    }
}

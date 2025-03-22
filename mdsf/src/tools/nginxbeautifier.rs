///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("nginxbeautifier"),
    CommandType::Direct("nginxbeautifier"),
    CommandType::Npm("nginxbeautifier"),
    CommandType::Pnpm("nginxbeautifier"),
    CommandType::Bun("nginxbeautifier"),
    CommandType::Deno("nginxbeautifier"),
    CommandType::Yarn("nginxbeautifier"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_nginxbeautifier {
    #[test_with::executable(nginxbeautifier || npx || pnpm || deno || bunx)]
    fn test_nginxbeautifier_conf_5c2a2e0d4f44354f() {
        let input = r#"server {
    listen 80;
        listen [::]:80;
       server_name example.com;
    }
"#;

        let output = r#"server {
	listen 80;
	listen [::]:80;
	server_name example.com;
}
"#;

        let file_ext = crate::fttype::get_file_extension(".conf");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Nginxbeautifier
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

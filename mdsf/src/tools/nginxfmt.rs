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

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("nginxfmt"),
    CommandType::Uv("nginxfmt", "nginxfmt"),
    CommandType::Pipx("nginxfmt"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_nginxfmt {
    #[test_with::executable(nginxfmt || pipx || uv)]
    fn test_nginxfmt_conf_2e651ac1789b7182() {
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

        let result = crate::tools::Tooling::Nginxfmt
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

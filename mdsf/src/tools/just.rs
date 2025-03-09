///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--fmt");
    cmd.arg("--unstable");
    cmd.arg("--justfile");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 9] = [
    CommandType::NodeModules("just"),
    CommandType::Direct("just"),
    CommandType::Npm("rust-just"),
    CommandType::Pnpm("rust-just"),
    CommandType::Bun("rust-just"),
    CommandType::Deno("rust-just"),
    CommandType::Yarn("rust-just"),
    CommandType::Uv("rust-just"),
    CommandType::Pipx("rust-just"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_just {
    #[test_with::executable(just || npx || pnpm || deno || bunx || pipx || uv)]
    fn test_just_just_ef70afaf3ede68b9() {
        let input = r#"build:
                cargo build
                cargo build --release
            "#;

        let output = r#"build:
    cargo build
    cargo build --release
"#;

        let file_ext = crate::fttype::get_file_extension("just");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Just
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

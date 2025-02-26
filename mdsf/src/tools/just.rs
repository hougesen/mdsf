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

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("just"),
    CommandType::Direct("just"),
    CommandType::Npm("rust-just"),
];

#[cfg(test)]
mod test_just {
    #[test_with::executable(npx)]
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

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            0,
            false,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--edition");
    cmd.arg("2021");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("rustfmt")];

#[cfg(test)]
mod test_rustfmt {
    #[test_with::executable(rustfmt)]
    fn test_rustfmt_rust_70ad564760e773e9() {
        let input = r#"pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    "#;

        let output = r#"pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let file_ext = crate::fttype::get_file_extension("rust");

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

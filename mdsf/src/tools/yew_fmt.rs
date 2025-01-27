///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_yew_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--edition");
    cmd.arg("2021");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("yew-fmt")];

    crate::execution::run_tools(&commands, file_path, timeout, set_yew_fmt_args)
}

#[cfg(test)]
mod test_yew_fmt {
    #[test_with::executable(yew-fmt)]
    fn test_yew_fmt_rust_760026fac349dc23() {
        let input = r#"pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    "#;
        let output = Some(
            r#"pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("rust");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::yew_fmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

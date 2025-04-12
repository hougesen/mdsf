///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-w");
    cmd.arg("-n");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("dockerfmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_dockerfmt {
    #[test_with::executable(dockerfmt)]
    fn test_dockerfmt_dockerfile_bb70f2ad30df0302() {
        let input = r#"FROM          ubuntu:latest
 RUN   echo   "Hello world"
"#;

        let output = r#"FROM ubuntu:latest
RUN echo "Hello world"
"#;

        let file_ext = crate::fttype::get_file_extension(".Dockerfile");

        crate::tools::Tooling::Dockerfmt.test_format_snippet(input, output, &file_ext);
    }
}

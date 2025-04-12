///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 2] = [CommandType::Direct("dfmt"), CommandType::Dub("dfmt")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_dfmt {
    #[test_with::executable(dfmt || dub)]
    fn test_dfmt_d_768f677c0817bc61() {
        let input = r#"int add(int a,int b){return a + b;}
"#;

        let output = r#"int add(int a, int b)
{
    return a + b;
}
"#;

        let file_ext = crate::fttype::get_file_extension("d");

        crate::tools::Tooling::Dfmt.test_format_snippet(input, output, &file_ext);
    }
}

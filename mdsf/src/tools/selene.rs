///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--no-summary");
    cmd.arg("--quiet");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("selene")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_selene {
    #[test_with::executable(selene)]
    fn test_selene_lua_e4a3734aedc452ef() {
        let input = r#"function add(a, b)
	return a + b
end

return add
"#;

        let output = r#"function add(a, b)
	return a + b
end

return add
"#;

        let file_ext = crate::fttype::get_file_extension("lua");

        crate::tools::Tooling::Selene.test_format_snippet(input, output, &file_ext);
    }
}

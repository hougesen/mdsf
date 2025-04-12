///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--verify");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("stylua"),
    CommandType::Direct("stylua"),
    CommandType::Npm("@johnnymorganz/stylua-bin"),
    CommandType::Pnpm("@johnnymorganz/stylua-bin"),
    CommandType::Bun("@johnnymorganz/stylua-bin"),
    CommandType::Deno("@johnnymorganz/stylua-bin"),
    CommandType::Yarn("@johnnymorganz/stylua-bin"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_stylua {
    #[test_with::executable(stylua || npx || pnpm || deno || bunx)]
    fn test_stylua_lua_ab45775f0dc2fcca() {
        let input = r#"

        local               function        add (                                       a , b
)

return              a +b


end

    "#;

        let output = r#"local function add(a, b)
	return a + b
end
"#;

        let file_ext = crate::fttype::get_file_extension("lua");

        crate::tools::Tooling::Stylua.test_format_snippet(input, output, &file_ext);
    }
}

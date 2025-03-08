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

pub const COMMANDS: [CommandType; 6] = [
    CommandType::NodeModules("stylua"),
    CommandType::Direct("stylua"),
    CommandType::Npm("@johnnymorganz/stylua-bin"),
    CommandType::Pnpm("@johnnymorganz/stylua-bin"),
    CommandType::Bun("@johnnymorganz/stylua-bin"),
    CommandType::Deno("@johnnymorganz/stylua-bin"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_stylua {
    #[test_with::executable(npx)]
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

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Stylua
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

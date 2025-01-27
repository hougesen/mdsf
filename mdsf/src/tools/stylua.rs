///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_stylua_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--verify");
    cmd.arg(file_path);
    cmd
}

const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("stylua"),
    CommandType::Direct("stylua"),
    CommandType::Npm("@johnnymorganz/stylua-bin"),
];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_stylua_args)
}

#[cfg(test)]
mod test_stylua {
    #[test_with::executable(npx)]
    fn test_stylua_lua_aaecbc44cc18ece0() {
        let input = r#"

        local               function        add (                                       a , b
)

return              a +b


end

    "#;
        let output = Some(
            r#"local function add(a, b)
	return a + b
end
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("lua");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::stylua::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

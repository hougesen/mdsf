///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_stylua_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--verify");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("stylua"),
        CommandType::Direct("stylua"),
        CommandType::Npm("@johnnymorganz/stylua-bin"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_stylua_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path, timeout);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
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

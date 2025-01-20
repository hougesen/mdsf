use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_luaformatter_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("lua-format")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_luaformatter_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

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
mod test_luaformatter {
    #[test_with::executable(lua-format)]
    fn test_luaformatter_lua_df0e81b2c9a1a835() {
        let input = r#"

        local               function        add (                                       a , b
)
local c=a+b
return    c


end
    "#;
        let output = r#"local function add(a, b)
    local c = a + b
    return c

end
"#;
        let file_ext = crate::fttype::get_file_extension("lua");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::luaformatter::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}

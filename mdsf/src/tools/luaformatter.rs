///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_luaformatter_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("lua-format")];

    crate::execution::run_tools(&commands, file_path, timeout, set_luaformatter_args)
}

#[cfg(test)]
mod test_luaformatter {
    #[test_with::executable(lua-format)]
    fn test_luaformatter_lua_b025fac1e27e4d1a() {
        let input = r#"

        local               function        add (                                       a , b
)
local c=a+b
return    c


end
    "#;
        let output = Some(
            r#"local function add(a, b)
    local c = a + b
    return c

end
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("lua");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::luaformatter::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

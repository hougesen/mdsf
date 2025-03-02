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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("lua-format")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_luaformatter {
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

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

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

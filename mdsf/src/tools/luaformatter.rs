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

        crate::tools::Tooling::Luaformatter.test_format_snippet(input, output, &file_ext);
    }
}

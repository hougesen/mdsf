use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_luaformatter(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("lua-format");

    cmd.arg("-i").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_luaformatter {
    use crate::{
        formatters::{luaformatter::format_using_luaformatter, setup_snippet},
        languages::Language,
    };

    #[test_with::executable(lua-format)]
    #[test]
    fn it_should_format_lua() {
        let input = "

        local               function        add (                                       a , b
)
local c=a+b
return    c


end
    ";

        let expected_output = "local function add(a, b)
    local c = a + b
    return c

end
";

        let snippet =
            setup_snippet(input, Language::Lua.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_luaformatter(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

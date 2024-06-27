use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("lua-format");

    cmd.arg("-i").arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[cfg(test)]
mod test_luaformatter {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(lua-format)]
    async fn it_should_format_lua() {
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

        let snippet = setup_snippet(input, language_to_ext("lua"))
            .await
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

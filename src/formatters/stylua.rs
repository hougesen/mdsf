use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_stylua_args(cmd: &mut tokio::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("--verify");
    cmd.arg(snippet_path);
}

#[inline]
async fn invoke_stylua(
    mut cmd: tokio::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_stylua_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_stylua(tokio::process::Command::new("stylua"), snippet_path).await
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_stylua(setup_npm_script("@johnnymorganz/stylua-bin"), snippet_path).await
}

#[cfg(test)]
mod test_stylua {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(stylua)]
    async fn it_should_format_lua() {
        let input = "

        local               function        add (                                       a , b
)

return              a +b


end

    ";

        let expected_output = "local function add(a, b)\n\treturn a + b\nend\n";

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

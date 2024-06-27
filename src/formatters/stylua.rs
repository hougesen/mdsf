use super::execute_command;
use crate::{error::MdsfError, runners::setup_npm_script};

#[inline]
fn set_stylua_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--verify").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_stylua(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_stylua_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) = invoke_stylua(std::process::Command::new("stylua"), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_stylua(setup_npm_script("@johnnymorganz/stylua-bin"), snippet_path)
}

#[cfg(test)]
mod test_stylua {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(stylua)]
    fn it_should_format_lua() {
        let input = "

        local               function        add (                                       a , b
)

return              a +b


end

    ";

        let expected_output = "local function add(a, b)\n\treturn a + b\nend\n";

        let snippet =
            setup_snippet(input, language_to_ext("lua")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

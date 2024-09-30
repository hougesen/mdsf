use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

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
    if let Ok(path_result) = invoke_stylua(CommandType::Direct("stylua").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_stylua(CommandType::NodeModules("stylua").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_stylua(
        CommandType::Npm("@johnnymorganz/stylua-bin").build(),
        snippet_path,
    )
}

#[cfg(test)]
mod test_stylua {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

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
            setup_snippet(input, &get_file_extension("lua")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

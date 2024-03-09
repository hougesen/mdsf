use super::execute_command;

#[inline]
pub fn format_using_stylua(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("stylua");

    cmd.arg("--verify");
    cmd.arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_stylua {
    use crate::{
        formatters::{setup_snippet, stylua::format_using_stylua},
        languages::Language,
    };

    #[test]
    fn it_should_format_lua() {
        let input = "

        local               function        add (                                       a , b
)

return              a +b


end

    ";

        let expected_output = "local function add(a, b)\n\treturn a + b\nend\n";

        let snippet =
            setup_snippet(input, Language::Lua.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_stylua(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

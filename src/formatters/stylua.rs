use super::{execute_command, read_snippet};

#[inline]
pub fn format_using_stylua(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    let mut cmd = std::process::Command::new("stylua");

    cmd.arg("--verify");
    cmd.arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
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
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_biome_format_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format").arg("--write").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_biome_format(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_biome_format_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run_format(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_biome_format(CommandType::NodeModules("biome").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_biome_format(CommandType::Direct("biome").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_biome_format(CommandType::Npm("@biomejs/biome").build(), snippet_path)
}

#[inline]
fn set_biome_lint_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("lint").arg("--write").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_biome_lint(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_biome_lint_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run_lint(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_biome_lint(CommandType::NodeModules("biome").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_biome_lint(CommandType::Direct("biome").build(), snippet_path) {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_biome_lint(CommandType::Npm("@biomejs/biome").build(), snippet_path)
}

#[inline]
fn set_biome_check_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("check").arg("--write").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_biome_check(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_biome_check_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run_check(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    if let Ok(path_result) =
        invoke_biome_check(CommandType::NodeModules("biome").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    if let Ok(path_result) = invoke_biome_check(CommandType::Direct("biome").build(), snippet_path)
    {
        if !path_result.0 {
            return Ok(path_result);
        }
    }

    invoke_biome_check(CommandType::Npm("@biomejs/biome").build(), snippet_path)
}

#[cfg(test)]
mod test_biome {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(npx)]
    fn it_should_format_json() {
        let input = "
              {
              \"key\": \"value\",
  \"key2\": [
      \"value2\",
      \"value3\",
      1
            , null]
 }
  ";

        let expected_output = "{
\t\"key\": \"value\",
\t\"key2\": [\"value2\", \"value3\", 1, null]
}
";

        let snippet =
            setup_snippet(input, language_to_ext("json")).expect("it to create a snippet file");

        let output = super::run_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(npx)]
    fn it_should_format_javascript() {
        let input = "
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            ";

        let expected_output = "async function asyncAddition(a, b) {
\treturn a + b;
}
";

        let snippet = setup_snippet(input, language_to_ext("javascript"))
            .expect("it to create a snippet file");

        let output = super::run_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(npx)]
    fn it_should_format_typescript() {
        let input = "
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            ";

        let expected_output =
            "async function asyncAddition(a: number, b: number): Promise<number> {
\treturn a + b;
}
";

        let snippet = setup_snippet(input, language_to_ext("typescript"))
            .expect("it to create a snippet file");

        let output = super::run_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

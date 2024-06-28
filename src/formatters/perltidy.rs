use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_perltidy_args(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-b").arg(snippet_path);

    cmd
}

#[inline]
fn invoke_perltidy(
    cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    execute_command(set_perltidy_args(cmd, snippet_path), snippet_path)
}

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    invoke_perltidy(CommandType::Direct("perltidy").build(), snippet_path)
}

#[cfg(test)]
mod test_perltidy {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(perltidy)]
    fn it_should_format_perl() {
        let input = r#"$_= <<'EOL';
   $url = new URI::URL "http://www/";   die if $url eq "xXx";
EOL
LOOP:{print(" digits"),redo LOOP if/\G\d+\b[,.;]?\s*/gc;print(" lowercase"),
redo LOOP if/\G[a-z]+\b[,.;]?\s*/gc;print(" UPPERCASE"),redo LOOP
if/\G[A-Z]+\b[,.;]?\s*/gc;print(" Capitalized"),
redo LOOP if/\G[A-Z][a-z]+\b[,.;]?\s*/gc;
print(" MiXeD"),redo LOOP if/\G[A-Za-z]+\b[,.;]?\s*/gc;print(
" alphanumeric"),redo LOOP if/\G[A-Za-z0-9]+\b[,.;]?\s*/gc;print(" line-noise"
),redo LOOP if/\G[^A-Za-z0-9]+/gc;print". That's all!\n";}"#;

        let expected_output = r#"$_ = <<'EOL';
   $url = new URI::URL "http://www/";   die if $url eq "xXx";
EOL
LOOP: {
    print(" digits"),    redo LOOP if /\G\d+\b[,.;]?\s*/gc;
    print(" lowercase"), redo LOOP if /\G[a-z]+\b[,.;]?\s*/gc;
    print(" UPPERCASE"), redo LOOP
      if /\G[A-Z]+\b[,.;]?\s*/gc;
    print(" Capitalized"),  redo LOOP if /\G[A-Z][a-z]+\b[,.;]?\s*/gc;
    print(" MiXeD"),        redo LOOP if /\G[A-Za-z]+\b[,.;]?\s*/gc;
    print(" alphanumeric"), redo LOOP if /\G[A-Za-z0-9]+\b[,.;]?\s*/gc;
    print(" line-noise"),   redo LOOP if /\G[^A-Za-z0-9]+/gc;
    print ". That's all!\n";
}
"#;

        let snippet =
            setup_snippet(input, language_to_ext("perl")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}

use super::execute_command;
use crate::error::MdsfError;

#[inline]
fn set_perltidy_args(cmd: &mut std::process::Command, snippet_path: &std::path::Path) {
    cmd.arg("-b");
    cmd.arg(snippet_path);
}

#[inline]
fn invoke_perltidy(
    mut cmd: std::process::Command,
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    set_perltidy_args(&mut cmd, snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[inline]
pub fn format_using_perltidy(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    invoke_perltidy(std::process::Command::new("perltidy"), snippet_path)
}

#[cfg(test)]
mod test_perltidy {
    use crate::{
        formatters::{perltidy::format_using_perltidy, setup_snippet},
        generated::language_to_ext,
    };

    #[test_with::executable(perltidy)]
    #[test]
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
            setup_snippet(input, &language_to_ext("perl")).expect("it to create a snippet file");

        let output = format_using_perltidy(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}

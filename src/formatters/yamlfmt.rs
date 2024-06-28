use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("yamlfmt").build();

    cmd.arg("-quiet").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_yamlfmt {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(yamlfmt)]
    fn it_should_format_yaml() {
        let input = "


version:                                                                             2
updates:
  - package-ecosystem:                    \"cargo\"
    directory:  \"/\"
    schedule:
      interval:     \"monthly\"
    assignees:
      -     \"hougesen\"
    open-pull-requests-limit:       25

  - package-ecosystem:                              \"github-actions\"
    directory:          \"/\"
    schedule:
        interval:          \"monthly\"
    assignees:
        - \"hougesen\"
    open-pull-requests-limit: 25


        ";

        let expected_output = "version: 2
updates:
  - package-ecosystem: \"cargo\"
    directory: \"/\"
    schedule:
      interval: \"monthly\"
    assignees:
      - \"hougesen\"
    open-pull-requests-limit: 25
  - package-ecosystem: \"github-actions\"
    directory: \"/\"
    schedule:
      interval: \"monthly\"
    assignees:
      - \"hougesen\"
    open-pull-requests-limit: 25
";

        let snippet =
            setup_snippet(input, language_to_ext("yaml")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}

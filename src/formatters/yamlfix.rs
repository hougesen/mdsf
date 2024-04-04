use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_yamlfix(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("yamlfix");

    let mut cmd = std::process::Command::new("yamlfix");

    cmd.arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_yamlfix {
    use super::format_using_yamlfix;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(yamlfix)]
    #[test]
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

        let expected_output = "---
version: 2
updates:
  - package-ecosystem: cargo
    directory: /
    schedule:
      interval: monthly
    assignees: [hougesen]
    open-pull-requests-limit: 25
  - package-ecosystem: github-actions
    directory: /
    schedule:
      interval: monthly
    assignees: [hougesen]
    open-pull-requests-limit: 25
";

        let snippet = setup_snippet(input, Language::Yaml.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_yamlfix(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}
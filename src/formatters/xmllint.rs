use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("xmllint").build();

    cmd.arg("--format")
        .arg(snippet_path)
        .arg("--output")
        .arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_xmllint {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(xmllint)]
    fn it_should_format_xml() {
        let input = "
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>";

        let expected_output = r#"<?xml version="1.0"?>
<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>
"#;

        let snippet =
            setup_snippet(input, language_to_ext("xml")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

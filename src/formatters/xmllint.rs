use super::execute_command;

#[inline]
pub fn format_using_xmllint(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("xmllint");

    cmd.arg("--format")
        .arg(snippet_path)
        .arg("--output")
        .arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_xmllint {
    use super::format_using_xmllint;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(xmllint)]
    #[test]
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
            setup_snippet(input, Language::Xml.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_xmllint(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

use super::execute_command;
use crate::terminal::print_formatter_info;

#[inline]
pub fn format_using_xmlformat(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    print_formatter_info("xmlformat");

    let mut cmd = std::process::Command::new("xmlformat");

    cmd.arg("--overwrite").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_xmlformat {
    use super::format_using_xmlformat;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(xmlformat)]
    #[test]
    fn it_should_format_xml() {
        let input = "
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>";

        let expected_output = "<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>";

        let snippet =
            setup_snippet(input, Language::Xml.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_xmlformat(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}

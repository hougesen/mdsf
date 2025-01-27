///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_xmllint_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--format");
    cmd.arg(file_path);
    cmd.arg("--output");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("xmllint")];

    crate::execution::run_tools(&commands, file_path, timeout, set_xmllint_args)
}

#[cfg(test)]
mod test_xmllint {
    #[test_with::executable(xmllint)]
    fn test_xmllint_xml_8a39bd2662133a88() {
        let input = r#"
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>"#;
        let output = Some(
            r#"<?xml version="1.0"?>
<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("xml");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::xmllint::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

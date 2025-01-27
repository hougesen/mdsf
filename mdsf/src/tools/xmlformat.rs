///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_xmlformat_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--overwrite");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("xmlformat")];

    crate::execution::run_tools(&commands, file_path, timeout, set_xmlformat_args)
}

#[cfg(test)]
mod test_xmlformat {
    #[test_with::executable(xmlformat)]
    fn test_xmlformat_xml_34659067ca1d8b7c() {
        let input = r#"
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>"#;
        let output = Some(
            r#"<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>"#
                .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("xml");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::xmlformat::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

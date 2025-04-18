///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--overwrite");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("xmlformat"),
    CommandType::Uv("xmlformatter", "xmlformat"),
    CommandType::Pipx("xmlformatter"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_xmlformat {
    #[test_with::executable(xmlformat || pipx || uv)]
    fn test_xmlformat_xml_5e39abb678e63c0b() {
        let input = r#"
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>"#;

        let output = r#"<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>"#;

        let file_ext = crate::fttype::get_file_extension("xml");

        crate::tools::Tooling::Xmlformat.test_format_snippet(input, output, &file_ext);
    }
}

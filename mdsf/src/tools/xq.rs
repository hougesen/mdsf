///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(cmd: std::process::Command, _file_path: &std::path::Path) -> std::process::Command {
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("xq")];

pub const IS_STDIN: bool = true;

#[cfg(test)]
mod test_xq {
    #[test_with::executable(xq)]
    fn test_xq_xml_1289078d9c0aa8a3() {
        let input = r#"<?xml version="1.0"?> <catalog>    <book id="bk112">       <author>Galos, Mike</author>       <title>Visual Studio 7: A Comprehensive Guide</title>       <genre>Computer</genre>       <price>49.95</price>       <publish_date>2001-04-16</publish_date>       <description>Microsoft Visual Studio 7 is explored in depth,       looking at how Visual Basic, Visual C++, C#, and ASP+ are        integrated into a comprehensive development        environment.</description>    </book> </catalog>"#;

        let output = r#"<?xml version="1.0"?>
<catalog>
  <book id="bk112">
    <author>Galos, Mike</author>
    <title>Visual Studio 7: A Comprehensive Guide</title>
    <genre>Computer</genre>
    <price>49.95</price>
    <publish_date>2001-04-16</publish_date>
    <description>Microsoft Visual Studio 7 is explored in depth,       looking at how Visual Basic, Visual C++, C#, and ASP+ are        integrated into a comprehensive development        environment.</description>
  </book>
</catalog>
"#;

        let file_ext = crate::fttype::get_file_extension("xml");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Xq
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}

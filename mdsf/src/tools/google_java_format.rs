///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("google-java-format")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_google_java_format {
    #[test_with::executable(google-java-format)]
    fn test_google_java_format_java_9d3ffaedafc37e65() {
        let input = r#"class HelloWorld {
    public static void main(String[] args) {
                System.out.println("Hello");
                System.out.println("World!");
                 }
}"#;

        let output = r#"class HelloWorld {
  public static void main(String[] args) {
    System.out.println("Hello");
    System.out.println("World!");
  }
}
"#;

        let file_ext = crate::fttype::get_file_extension("java");

        crate::tools::Tooling::GoogleJavaFormat.test_format_snippet(input, output, &file_ext);
    }
}

///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_clang_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-i");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("clang-format")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_clang_format_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path, timeout);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
}

#[cfg(test)]
mod test_clang_format {
    #[test_with::executable(clang-format)]
    fn test_clang_format_java_3a8693a8a31c89e1() {
        let input = r#"class HelloWorld {
    public static void main(String[] args) {
                System.out.println("Hello");
                System.out.println("World!");
                 }
}"#;
        let output = Some(
            r#"class HelloWorld {
  public static void main(String[] args) {
    System.out.println("Hello");
    System.out.println("World!");
  }
}"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("java");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::clang_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_protobuf_230c199e5d2e6168() {
        let input = r#"service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }"#;
        let output = Some(
            r#"service SearchService { rpc Search(SearchRequest) returns (SearchResponse); }"#
                .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("protobuf");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::clang_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_objective_c_dee0cca41b3fee5d() {
        let input = r#"int add(int a,int b){
            a - a ;
       return a + b;
    }"#;
        let output = Some(
            r#"int add(int a, int b) {
  a - a;
  return a + b;
}"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("objective-c");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::clang_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_c_f77ec2be9551eaf5() {
        let input = r#"int add(int a,int b){
                a-b;
       return a + b;
    }"#;
        let output = Some(
            r#"int add(int a, int b) {
  a - b;
  return a + b;
}"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("c");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::clang_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_cpp_fc6a259c1f521059() {
        let input = r#"int add(int a,int b){
                 a-b;
       return a + b;
    }"#;
        let output = Some(
            r#"int add(int a, int b) {
  a - b;
  return a + b;
}"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("cpp");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::clang_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_csharp_30d7743cc539319d() {
        let input = r#"namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                a-b ;
                                                        return a + b;
                                                    }
                                                 }
                                                 } "#;
        let output = Some(
            r#"namespace Mdsf {
class Adder {
  public static int add(int a, int b) {
    a - b;
    return a + b;
  }
}
}"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("csharp");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::clang_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_json_aaecdc564d205c23() {
        let input = r#"              {
              "key": "value",
  "key2": ["value2", "value3", 1            , null]
 }  "#;
        let output = Some(
            r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("json");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::clang_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_javascript_96407481e0b6425() {
        let input = r#"    async function asyncAddition(  a,b) {
            a * b;
        return a+b
    }            "#;
        let output = Some(
            r#"async function asyncAddition(a, b) {
  a * b;
  return a + b
}"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("javascript");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::clang_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

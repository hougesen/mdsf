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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("clang-format")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_clang_format {
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(clang-format)]
    fn test_clang_format_java_c4fcc280a3a8aac0() {
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
}"#;

        let file_ext = crate::fttype::get_file_extension("java");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_protobuf_7be6def196942f83() {
        let input = r#"service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }"#;

        let output =
            r#"service SearchService { rpc Search(SearchRequest) returns (SearchResponse); }"#;

        let file_ext = crate::fttype::get_file_extension("protobuf");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_objective_c_3d56455568c6e83f() {
        let input = r#"int add(int a,int b){
            a - a ;
       return a + b;
    }"#;

        let output = r#"int add(int a, int b) {
  a - a;
  return a + b;
}"#;

        let file_ext = crate::fttype::get_file_extension("objective-c");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_c_bb10810bd7d8a71() {
        let input = r#"int add(int a,int b){
                a-b;
       return a + b;
    }"#;

        let output = r#"int add(int a, int b) {
  a - b;
  return a + b;
}"#;

        let file_ext = crate::fttype::get_file_extension("c");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_cpp_8a39c61364dbbe50() {
        let input = r#"int add(int a,int b){
                 a-b;
       return a + b;
    }"#;

        let output = r#"int add(int a, int b) {
  a - b;
  return a + b;
}"#;

        let file_ext = crate::fttype::get_file_extension("cpp");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_csharp_8ebf20c1ddcd1aeb() {
        let input = r#"namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                a-b ;
                                                        return a + b;
                                                    }
                                                 }
                                                 } "#;

        let output = r#"namespace Mdsf {
class Adder {
  public static int add(int a, int b) {
    a - b;
    return a + b;
  }
}
}"#;

        let file_ext = crate::fttype::get_file_extension("csharp");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_json_574b008e140f1be6() {
        let input = r#"              {
              "key": "value",
  "key2": ["value2", "value3", 1            , null]
 }  "#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}"#;

        let file_ext = crate::fttype::get_file_extension("json");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }

    #[test_with::executable(clang-format)]
    fn test_clang_format_javascript_d6184d76490772e9() {
        let input = r#"    async function asyncAddition(  a,b) {
            a * b;
        return a+b
    }            "#;

        let output = r#"async function asyncAddition(a, b) {
  a * b;
  return a + b
}"#;

        let file_ext = crate::fttype::get_file_extension("javascript");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

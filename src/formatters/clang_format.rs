use super::execute_command;

#[inline]
pub fn format_using_clang_format(
    snippet_path: &std::path::Path,
) -> std::io::Result<(bool, Option<String>)> {
    let mut cmd = std::process::Command::new("clang-format");

    cmd.arg("-i").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_clang_format {
    use crate::{
        formatters::{clang_format::format_using_clang_format, setup_snippet},
        languages::Language,
    };

    #[test_with::executable(clang-format)]
    #[test]
    fn it_should_format_c() {
        let input = "int add(int a,int b){
                a-b;
       return a + b;
    }";

        let expected_output = "int add(int a, int b) {
  a - b;
  return a + b;
}";

        let snippet =
            setup_snippet(input, Language::C.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_clang_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn it_should_format_cpp() {
        let input = "int add(int a,int b){
                 a-b;
       return a + b;
    }";

        let expected_output = "int add(int a, int b) {
  a - b;
  return a + b;
}";

        let snippet =
            setup_snippet(input, Language::Cpp.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_clang_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn it_should_format_csharp() {
        let input = "namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                a-b ;
                                                        return a + b;
                                                    }
                                                 }
                                                 } ";

        let expected_output =
         "namespace Mdsf {\nclass Adder {\n  public static int add(int a, int b) {\n    a - b;\n    return a + b;\n  }\n}\n}";

        let snippet = setup_snippet(input, Language::CSharp.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_clang_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn it_should_format_javascript() {
        let input = "    async function asyncAddition(  a,b) {
            a * b;
        return a+b
    }            ";

        let expected_output = "async function asyncAddition(a, b) {\n  a * b;\n  return a + b\n}";

        let snippet = setup_snippet(input, Language::JavaScript.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_clang_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn it_should_format_json() {
        let input = "              {
              \"key\": \"value\",
  \"key2\": [      \"value2\", \"value3\", 1            , null]
 }  ";

        let expected_output =
            "{ \"key\" : \"value\", \"key2\" : [ \"value2\", \"value3\", 1, null ] }";

        let snippet = setup_snippet(input, Language::Json.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_clang_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn it_should_format_objective_c() {
        let input = "int add(int a,int b){
            a - a ;
       return a + b;
    }";

        let expected_output = "int add(int a, int b) {
  a - a;
  return a + b;
}";

        let snippet = setup_snippet(input, Language::ObjectiveC.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_clang_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn it_should_format_protobuf() {
        let input = "service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }";

        let expected_output =
            "service SearchService { rpc Search(SearchRequest) returns (SearchResponse); }";

        let snippet = setup_snippet(input, Language::Protobuf.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_clang_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }

    #[test_with::executable(clang-format)]
    #[test]
    fn it_should_format_java() {
        let input = "class HelloWorld {
    public static void main(String[] args) {
                System.out.println(\"Hello\");
                System.out.println(\"World!\");
                 }
}";

        let expected_output = "class HelloWorld {
  public static void main(String[] args) {
    System.out.println(\"Hello\");
    System.out.println(\"World!\");
  }
}";

        let snippet = setup_snippet(input, Language::Java.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_clang_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

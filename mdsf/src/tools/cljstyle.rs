///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fix");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("cljstyle")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_cljstyle {
    #[test_with::executable(cljstyle)]
    fn test_cljstyle_clojure_92fbb2f42ebeeb2e() {
        let input = r#"(  ns
 foo.bar.baz  "some doc"
    (:require (foo.bar [abc :as abc]
        def))
    (:use foo.bar.qux)
    (:import foo.bar.qux.Foo
      ;; Need this for the thing
      foo.bar.qux.Bar)
    )

(defn hello "says hi" (
      [] (hello "world")
  ) ([name]
  ( println "Hello," name  )
  ))"#;

        let output = r#"(ns foo.bar.baz
  "some doc"
  (:require
    [foo.bar.abc :as abc]
    [foo.bar.def]
    [foo.bar.qux :refer :all])
  (:import
    (foo.bar.qux
      ;; Need this for the thing
      Bar
      Foo)))


(defn hello
  "says hi"
  ([] (hello "world"))
  ([name]
   (println "Hello," name)))
"#;

        let file_ext = crate::fttype::get_file_extension("clojure");

        crate::tools::Tooling::Cljstyle.test_format_snippet(input, output, &file_ext);
    }
}

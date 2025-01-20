///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_cljstyle_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fix");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("cljstyle")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_cljstyle_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

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
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::cljstyle::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}

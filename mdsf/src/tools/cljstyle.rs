///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_cljstyle_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("fix");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("cljstyle")];

    crate::execution::run_tools(&commands, file_path, timeout, set_cljstyle_args)
}

#[cfg(test)]
mod test_cljstyle {
    #[test_with::executable(cljstyle)]
    fn test_cljstyle_clojure_958a889bbccbb51b() {
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
        let output = Some(
            r#"(ns foo.bar.baz
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
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("clojure");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::cljstyle::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

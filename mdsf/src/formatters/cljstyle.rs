use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("cljstyle").build();

    cmd.arg("fix").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_cljstyle {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(cljstyle)]
    fn it_should_format_clojure() {
        let input = "(  ns
 foo.bar.baz  \"some doc\"
    (:require (foo.bar [abc :as abc]
        def))
    (:use foo.bar.qux)
    (:import foo.bar.qux.Foo
      ;; Need this for the thing
      foo.bar.qux.Bar)
    )

(defn hello \"says hi\" (
      [] (hello \"world\")
  ) ([name]
  ( println \"Hello,\" name  )
  ))";

        let expected_output = "(ns foo.bar.baz
  \"some doc\"
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
  \"says hi\"
  ([] (hello \"world\"))
  ([name]
   (println \"Hello,\" name)))
";

        let snippet = setup_snippet(input, &get_file_extension("clojure"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}

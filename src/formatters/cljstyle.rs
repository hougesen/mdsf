use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_cljstyle(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("cljstyle");

    cmd.arg("fix").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_cljstyle {
    use super::format_using_cljstyle;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(cljstyle)]
    #[test]
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

        let snippet = setup_snippet(input, Language::Clojure.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_cljstyle(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}

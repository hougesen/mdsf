use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub async fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = tokio::process::Command::new("cljstyle");

    cmd.arg("fix").arg(snippet_path);

    execute_command(&mut cmd, snippet_path).await
}

#[cfg(test)]
mod test_cljstyle {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[tokio::test]
    #[test_with::executable(cljstyle)]
    async fn it_should_format_clojure() {
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

        let snippet = setup_snippet(input, language_to_ext("clojure"))
            .await
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .await
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}

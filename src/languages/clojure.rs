use schemars::JsonSchema;

use crate::formatters::{cljstyle::format_using_cljstyle, MdsfFormatter};

use super::{Lang, LanguageFormatter};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub enum Clojure {
    #[default]
    #[serde(rename = "cljstyle")]
    Cljstyle,
}

impl Default for Lang<Clojure> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Clojure>::default(),
        }
    }
}

impl Default for MdsfFormatter<Clojure> {
    #[inline]
    fn default() -> Self {
        Self::Single(Clojure::Cljstyle)
    }
}

impl LanguageFormatter for Clojure {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> std::io::Result<(bool, Option<String>)> {
        match self {
            Self::Cljstyle => format_using_cljstyle(snippet_path),
        }
    }
}

#[cfg(test)]
mod test_c {
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
    };

    use super::Clojure;

    const INPUT: &str = "(  ns
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

    const EXTENSION: &str = crate::languages::Language::Clojure.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Clojure>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(Lang::<Clojure> {
            enabled: false,
            formatter: MdsfFormatter::Single(Clojure::default()),
        }
        .format(snippet_path)
        .expect("it to not fail")
        .is_none());
    }

    #[test_with::executable(cljstyle)]
    #[test]
    fn test_cljstyle() {
        let l = Lang::<Clojure> {
            enabled: true,
            formatter: MdsfFormatter::Single(Clojure::Cljstyle),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

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

        assert_eq!(output, expected_output);
    }
}

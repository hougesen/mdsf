use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{cljstyle::format_using_cljstyle, joker::format_using_joker, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Clojure {
    #[default]
    #[serde(rename = "cljstyle")]
    Cljstyle,
    #[serde(rename = "joker")]
    Joker,
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
        Self::Multiple(vec![Self::Multiple(vec![
            Self::Single(Clojure::Cljstyle),
            Self::Single(Clojure::Joker),
        ])])
    }
}

impl LanguageFormatter for Clojure {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Cljstyle => format_using_cljstyle(snippet_path),
            Self::Joker => format_using_joker(snippet_path),
        }
    }
}

impl core::fmt::Display for Clojure {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Cljstyle => write!(f, "cljstyle"),
            Self::Joker => write!(f, "joker"),
        }
    }
}

#[cfg(test)]
mod test_clojure {
    use super::Clojure;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

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
        .format(snippet_path, &LineInfo::fake())
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
            .format(snippet_path, &LineInfo::fake())
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

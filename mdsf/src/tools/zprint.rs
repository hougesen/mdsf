///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_zprint_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("-w");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("zprint")];

    crate::execution::run_tools(&commands, file_path, timeout, set_zprint_args)
}

#[cfg(test)]
mod test_zprint {
    #[test_with::executable(zprint)]
    fn test_zprint_clojure_c7eb78c2d8154947() {
        let input = r#"(defn change-start-column [new-start-column style-vec [inline-comment-index
  start-column spaces-before :as comment-vec]] (if (zero? inline-comment-index)
  style-vec (let [delta-spaces (- new-start-column start-column) new-spaces
  (+ spaces-before delta-spaces) previous-element-index (dec
  inline-comment-index) [s c e :as previous-element] (nth style-vec
  previous-element-index) new-previous-element (cond (= e :indent) [(str "
"
  (blanks new-spaces)) c e] (= e :whitespace) [(str (blanks new-spaces))
  c e 26] :else nil)] (assoc style-vec previous-element-index
  new-previous-element))))"#;
        let output = Some(
            r#"(defn change-start-column
  [new-start-column style-vec
   [inline-comment-index start-column spaces-before :as comment-vec]]
  (if (zero? inline-comment-index)
    style-vec
    (let [delta-spaces (- new-start-column start-column)
          new-spaces (+ spaces-before delta-spaces)
          previous-element-index (dec inline-comment-index)
          [s c e :as previous-element] (nth style-vec previous-element-index)
          new-previous-element
            (cond (= e :indent) [(str "
" (blanks new-spaces)) c e]
                  (= e :whitespace) [(str (blanks new-spaces)) c e 26]
                  :else nil)]
      (assoc style-vec previous-element-index new-previous-element))))"#
                .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("clojure");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::zprint::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

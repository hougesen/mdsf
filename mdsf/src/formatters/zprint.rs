use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("zprint").build();

    cmd.arg("-w").arg(file_path);

    execute_command(cmd, file_path)
}

#[cfg(test)]
mod test_zprint {
    use crate::{formatters::setup_snippet, fttype::get_file_extension};

    #[test_with::executable(zprint)]
    fn it_should_format_clojure() {
        let input = "(defn change-start-column [new-start-column style-vec [inline-comment-index
  start-column spaces-before :as comment-vec]] (if (zero? inline-comment-index)
  style-vec (let [delta-spaces (- new-start-column start-column) new-spaces
  (+ spaces-before delta-spaces) previous-element-index (dec
  inline-comment-index) [s c e :as previous-element] (nth style-vec
  previous-element-index) new-previous-element (cond (= e :indent) [(str \"\n\"
  (blanks new-spaces)) c e] (= e :whitespace) [(str (blanks new-spaces))
  c e 26] :else nil)] (assoc style-vec previous-element-index
  new-previous-element))))";

        let expected_output = "(defn change-start-column
  [new-start-column style-vec
   [inline-comment-index start-column spaces-before :as comment-vec]]
  (if (zero? inline-comment-index)
    style-vec
    (let [delta-spaces (- new-start-column start-column)
          new-spaces (+ spaces-before delta-spaces)
          previous-element-index (dec inline-comment-index)
          [s c e :as previous-element] (nth style-vec previous-element-index)
          new-previous-element
            (cond (= e :indent) [(str \"\n\" (blanks new-spaces)) c e]
                  (= e :whitespace) [(str (blanks new-spaces)) c e 26]
                  :else nil)]
      (assoc style-vec previous-element-index new-previous-element))))";

        let snippet = setup_snippet(input, &get_file_extension("clojure"))
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        // NOTE: we are trimming since I do not remember if the output ends with a blank line
        assert_eq!(expected_output, output.trim());
    }
}

use super::{execute_command, read_snippet};

pub fn format_using_biome(file_path: &std::path::Path) -> std::io::Result<Option<String>> {
    // TODO: use installed biome instead
    let mut cmd = std::process::Command::new("npx");

    // Incase the use hasn't installed biome
    cmd.arg("--yes")
        .arg("@biomejs/biome")
        .arg("format")
        .arg("--write")
        .arg(file_path);

    if execute_command(&mut cmd)? {
        return read_snippet(file_path).map(Some);
    }

    Ok(None)
}

#[cfg(test)]
mod test_biome {
    use crate::{
        formatters::{biome::format_using_biome, setup_snippet},
        languages::Language,
    };

    #[test]
    fn it_should_format_json() {
        let input = r#"          
              {
    // comments are allowed
              "key": "value", 
  "key2": [
      "value2", 
      "value3", 
      1
            , null]
 }
  "#;

        let expected_output = "{
\t// comments are allowed
\t\"key\": \"value\",
\t\"key2\": [\"value2\", \"value3\", 1, null]
}
";

        let snippet = setup_snippet(input, Language::Json.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_biome(snippet.path())
            .expect("it to be succesful")
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

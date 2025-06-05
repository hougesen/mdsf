mod generated_file_types;

#[inline]
pub fn get_file_extension(language: &str) -> String {
    let lowercase = language.to_lowercase();

    if let Some(s) = generated_file_types::language_to_ext(&lowercase) {
        return s.to_string();
    }

    if let Some(ext) = fallback_file_extension(&lowercase) {
        return ext.to_string();
    }

    if language.starts_with('.') {
        return language.to_string();
    }

    format!(".{language}")
}

#[inline]
fn fallback_file_extension(language: &str) -> Option<&'static str> {
    match language {
        "beancount" | "beanhub" => Some(".bean"),
        "kcl" => Some(".k"),
        "superhtml" => Some(".shtml"),
        _ => None,
    }
}

#[cfg(test)]
mod test_get_file_extension {
    use crate::filetype::get_file_extension;

    #[test]
    fn test_non_generated_fallback() {
        for (output, inputs) in [
            (".bean", vec!["beancount", "beanhub"]),
            (".k", vec!["kcl"]),
            (".shtml", vec!["superhtml"]),
        ] {
            for input in inputs {
                assert_eq!(output, get_file_extension(input));
            }
        }
    }

    #[test]
    fn test_generated_file_type() {
        let input = "rust";

        assert_eq!(".rs", get_file_extension(input));
    }

    #[test]
    fn test_input_starts_without_dot() {
        let input = "custom_file_extension";

        assert_eq!(format!(".{input}"), get_file_extension(input));
    }

    #[test]
    fn test_input_starts_with_dot() {
        let input = ".custom_file_extension";

        assert_eq!(input, get_file_extension(input));
    }
}

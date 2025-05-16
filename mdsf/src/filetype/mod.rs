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
        "bean" | "beancount" | "beanhub" => Some(".bean"),
        "htm" => Some(".htm"),
        "hurl" => Some(".hurl"),
        "kcl" => Some(".k"),
        "shtml" | "superhtml" => Some(".shtml"),
        _ => None,
    }
}

#[cfg(test)]
mod test_get_file_extension {
    use crate::filetype::get_file_extension;

    #[test]
    fn test_input_starts_with_dot() {
        let input = ".custom_file_extension";

        assert_eq!(input, get_file_extension(input));
    }
}

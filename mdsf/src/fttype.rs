use crate::generated;

#[inline]
pub fn get_file_extension(language: &str) -> String {
    let lowercase = language.to_lowercase();

    if let Some(s) = generated::language_to_ext(&lowercase) {
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
        "kcl" => Some(".k"),
        "htm" => Some(".htm"),
        "shtml" => Some(".shtml"),
        "superhtml" => Some(".shtml"),
        _ => None,
    }
}

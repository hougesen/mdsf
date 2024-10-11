use crate::generated;

#[inline]
pub fn get_file_extension(language: &str) -> String {
    if let Some(s) = generated::language_to_ext(language) {
        return s.to_string();
    }

    if let Some(ext) = fallback_file_extension(language) {
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
        "kcl" => Some(".k"),
        "htm" => Some(".htm"),
        "shtml" => Some(".shtml"),
        "superhtml" => Some(".shtml"),
        _ => None,
    }
}

use crate::generated;

#[inline]
pub fn get_file_extension(language: &str) -> String {
    let s = generated::language_to_ext(language);

    if s.is_empty() {
        fallback_file_extension(language)
    } else {
        s.to_string()
    }
}

#[inline]
fn fallback_file_extension(language: &str) -> String {
    match language {
        "kcl" => ".k".to_string(),
        _ => {
            if language.starts_with('.') {
                language.to_string()
            } else {
                format!(".{language}")
            }
        }
    }
}

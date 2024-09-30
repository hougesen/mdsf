pub fn fallback_file_extension(language: &str) -> String {
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

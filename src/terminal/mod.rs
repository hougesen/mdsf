use console::style;
use log::{debug, error, info, trace, warn};

use crate::{error::MdsfError, runners::JavaScriptRuntime, LineInfo};

pub mod logging;

#[inline]
pub fn print_error(error: &MdsfError) {
    error!("{error}");
}

#[inline]
pub fn print_formatter_info(formatter: &str, info: &LineInfo) {
    debug!(
        "{}:{} to :{} {} block using {formatter}",
        info.filename.display(),
        info.start,
        info.end,
        info.language
    );
}

#[inline]
pub fn print_formatter_time(formatter: &str, info: &LineInfo, duration: core::time::Duration) {
    trace!(
        "{}:{} to :{} {} took {}ms to format using {formatter}",
        info.filename.display(),
        info.start,
        info.end,
        info.language,
        duration.as_millis()
    );
}

#[inline]
pub fn print_unchanged_file(path: &std::path::Path, dur: core::time::Duration, cached: bool) {
    info!(
        "{}",
        style(format!(
            "{} finished in {}ms (unchanged){}",
            path.display(),
            dur.as_millis(),
            if cached { " (cached)" } else { "" }
        ))
        .dim()
    );
}

#[inline]
pub fn print_changed_file(path: &std::path::Path, dur: core::time::Duration, cached: bool) {
    info!(
        "{} finished in {}ms{}",
        path.display(),
        dur.as_millis(),
        if cached { " (cached)" } else { "" }
    );
}

#[inline]
pub fn print_changed_file_error(path: &std::path::Path) {
    error!("{} has changes", path.display());
}

#[inline]
pub fn print_config_not_found(path: &std::path::Path) {
    warn!("No config was found at {}", path.display());
}

#[inline]
pub fn print_unknown_javascript_runtime(value: u8, fallback: JavaScriptRuntime) {
    warn!("Unknown JavaScript runtime value '{value}'; Using {fallback:?} instead");
}

#[inline]
pub fn print_binary_not_in_path(path: &std::path::Path, binary_name: &str) {
    warn!("{} {binary_name} not found in path", path.display());
}

#[inline]
pub fn print_error_formatting(formatter_name: &str, info: &LineInfo, stderr: &str) {
    warn!(
        "{}:{} to :{} error formatting using {formatter_name}{}",
        info.filename.display(),
        info.start,
        info.end,
        wrap_text(stderr.trim())
    );
}

#[inline]
pub fn warn_unknown_language(language_name: &str, filename: &std::path::Path) {
    warn!(
        "{} no formatter configured for '{language_name}'",
        filename.display()
    );
}

#[inline]
pub fn print_error_reading_file(path: &std::path::Path, error: &std::io::Error) {
    error!("{} error reading file - {error}", path.display());
}

#[inline]
pub fn print_error_saving_file(path: &std::path::Path, error: &std::io::Error) {
    error!("{} error saving output - {error}", path.display());
}

#[inline]
fn wrap_text(input: &str) -> String {
    if input.trim().is_empty() {
        return String::new();
    }

    let mut lines = Vec::new();

    let mut line_length = 0;
    for line in input.lines() {
        let trimmed_line = line.trim_end();

        if trimmed_line.len() > line_length {
            line_length = trimmed_line.len();
        }

        lines.push(trimmed_line);
    }

    if lines.is_empty() || line_length == 0 {
        return String::new();
    }

    let break_line = format!(
        "{:->width$}",
        "",
        width = terminal_size::terminal_size().map_or(line_length, |(w, _h)| usize::from(w.0))
    );

    lines.insert(0, &break_line);
    lines.insert(0, "");
    lines.push(&break_line);

    lines.join("\n")
}

#[cfg(test)]
mod test_wrap_text {
    use crate::terminal::wrap_text;

    #[test]
    fn it_should_ignore_empty_text() {
        assert_eq!(wrap_text(""), "");
        assert_eq!(wrap_text("              "), "");
        assert_eq!(wrap_text(" \n  \n \n          "), "");
    }
}

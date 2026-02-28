use log::{debug, error, info, trace, warn};
use owo_colors::OwoColorize;

pub mod logging;

#[inline]
pub fn print_error(error: &crate::error::MdsfError) {
    error!("{error}");
}

#[inline]
pub fn print_tool_info(tool: &str, info: &crate::LineInfo) {
    debug!(
        "{}:{} to :{} {} block using {tool}",
        info.filename.display(),
        info.start,
        info.end,
        info.language
    );
}

#[inline]
pub fn print_tool_time(tool: &str, info: &crate::LineInfo, duration: core::time::Duration) {
    trace!(
        "{}:{} to :{} {} took {}ms to run using {tool}",
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
        format!(
            "{} finished in {}ms (unchanged){}",
            path.display(),
            dur.as_millis(),
            if cached { " (cached)" } else { "" }
        )
        .if_supports_color(owo_colors::Stream::Stderr, |text| text.dimmed())
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
pub fn print_binary_not_in_path(path: &std::path::Path, binary_name: &str, error: bool) {
    if error {
        error!("{} {binary_name} not found in path", path.display());
    } else {
        warn!("{} {binary_name} not found in path", path.display());
    }
}

#[inline]
pub fn print_error_running_tool(tool_name: &str, info: &crate::LineInfo, stderr: &str) {
    warn!(
        "{}:{} to :{} error running {tool_name}{}",
        info.filename.display(),
        info.start,
        info.end,
        wrap_text(stderr.trim())
    );
}

#[inline]
pub fn print_unknown_language(language: &str, path: &std::path::Path, error: bool) {
    if error {
        error!("{} no tool configured for '{language}'", path.display());
    } else {
        warn!("{} no tool configured for '{language}'", path.display());
    }
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
pub fn print_config_schema_version_mismatch(version: &str) {
    let cli_version = env!("CARGO_PKG_VERSION");

    warn!(
        "Your mdsf.json schema version ({version}) does not match the CLI version ({cli_version})"
    );
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

    lines.join(crate::config::Newline::Lf.as_str())
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

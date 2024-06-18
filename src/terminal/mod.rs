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
pub fn print_unchanged_file(path: &std::path::Path, dur: core::time::Duration) {
    info!(
        "{}",
        style(format!(
            "{} finished in {}ms (unchanged)",
            path.display(),
            dur.as_millis()
        ))
        .dim()
    );
}

#[inline]
pub fn print_changed_file(path: &std::path::Path, dur: core::time::Duration) {
    info!("{} finished in {}ms", path.display(), dur.as_millis());
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
pub fn print_binary_not_in_path(binary_name: &str) {
    warn!("{binary_name} not found in path");
}

#[inline]
pub fn print_error_formatting(formatter_name: &str, info: &LineInfo) {
    warn!(
        "{}:{} to :{} error formatting using {formatter_name}",
        info.filename.display(),
        info.start,
        info.end
    );
}

#[inline]
pub fn warn_unknown_language(language_name: &str, filename: &std::path::Path) {
    warn!(
        "{} no formatter configured for '{language_name}'",
        filename.display()
    );
}

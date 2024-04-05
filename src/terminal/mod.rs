use log::{debug, error, info, warn};

use crate::{error::MdsfError, runners::JavaScriptRuntime, LineInfo};

pub mod logging;

#[inline]
pub fn print_error(error: &MdsfError) {
    error!("{error}");
}

#[inline]
pub fn print_formatter_info(formatter: &str, info: &LineInfo) {
    debug!(
        "{} formatting '{}' block from :{} to :{} using {formatter}",
        info.filename.display(),
        info.language,
        info.start,
        info.end
    );
}

#[inline]
pub fn print_unchanged_file(path: &std::path::Path, dur: core::time::Duration) {
    info!("{} {}ms (unchanged)", path.display(), dur.as_millis());
}

#[inline]
pub fn print_changed_line(path: &std::path::Path, dur: core::time::Duration) {
    info!("{} {}ms", path.display(), dur.as_millis());
}

#[inline]
pub fn print_config_not_found() {
    warn!("Using default config since no config was found");
}

#[inline]
pub fn print_unknown_javascript_runtime(value: u8, fallback: JavaScriptRuntime) {
    warn!("Unknown JavaScript runtime value '{value}'; Using {fallback:?} instead");
}

#[inline]
pub fn print_binary_not_in_path(binary_name: &str) {
    warn!("'{binary_name}' not found in path");
}

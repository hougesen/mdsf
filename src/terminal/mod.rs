use crate::{error::MdsfError, languages::Language, runners::JavaScriptRuntime, DEBUG};

#[cfg(target_os = "windows")]
const GREY_TEXT: &str = "";
#[cfg(not(target_os = "windows"))]
const GREY_TEXT: &str = "\u{1b}[2m";

#[cfg(target_os = "windows")]
const DEBUG_TEXT_DECORATION: &str = "";
#[cfg(not(target_os = "windows"))]
const DEBUG_TEXT_DECORATION: &str = "\u{1b}[33m";

#[cfg(target_os = "windows")]
const ERROR_TEXT_DECORATION: &str = "";
#[cfg(not(target_os = "windows"))]
const ERROR_TEXT_DECORATION: &str = "\u{1b}[31m";

#[cfg(target_os = "windows")]
const TEXT_RESET: &str = "";
#[cfg(not(target_os = "windows"))]
const TEXT_RESET: &str = "\u{1b}[0m";

#[inline]
pub fn print_error(error: &MdsfError) {
    println!("{ERROR_TEXT_DECORATION}{error}{TEXT_RESET}");
}

#[inline]
pub fn print_debug<T: core::fmt::Display>(args: T) {
    if DEBUG.load(core::sync::atomic::Ordering::Relaxed) {
        println!("{DEBUG_TEXT_DECORATION}{args}{TEXT_RESET}");
    }
}

#[inline]
pub fn print_file_info(filename: &std::path::Path) {
    print_debug(format!("Formatting '{}'", filename.display()));
}

#[inline]
pub fn print_line_info(language: Language, start: usize, end: usize) {
    print_debug(format!(
        "Formatting '{language}' block from :{start} to :{end}"
    ));
}

#[inline]
pub fn print_formatter_info(formatter: &str) {
    print_debug(format!("Using formatter: '{formatter}'"));
}

#[inline]
pub fn print_unchanged_file(path: &std::path::Path, dur: core::time::Duration) {
    println!(
        "{GREY_TEXT}{} {}ms (unchanged){TEXT_RESET}",
        path.display(),
        dur.as_millis()
    );
}

#[inline]
pub fn print_changed_line(path: &std::path::Path, dur: core::time::Duration) {
    println!("{} {}ms", path.display(), dur.as_millis());
}

#[inline]
pub fn print_config_info(maybe_config_path: Option<&std::path::Path>) {
    if let Some(config_path) = maybe_config_path {
        print_debug(format!(
            "{DEBUG_TEXT_DECORATION}Using config found at '{}'{TEXT_RESET}",
            config_path.display()
        ));
    } else {
        print_debug("Using default config since no config was found");
    }
}

#[inline]
pub fn print_unknown_javascript_runtime(value: u8, fallback: JavaScriptRuntime) {
    print_debug(format!(
        "Unknown JavaScript runtime value '{value}'; Using {fallback:?} instead"
    ));
}

#[inline]
pub fn print_javascript_runtime(runtime: JavaScriptRuntime) {
    print_debug(format!("Using JavaScript runtime '{runtime}'"));
}

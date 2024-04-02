use crate::{languages::Language, DEBUG};

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
pub fn print_debug_file_info(filename: &std::path::Path) {
    if DEBUG.load(core::sync::atomic::Ordering::Relaxed) {
        println!(
            "{DEBUG_TEXT_DECORATION}Formatting '{}'{TEXT_RESET}",
            filename.display()
        );
    }
}

#[inline]
pub fn print_debug_line_info(language: Language, start: usize, end: usize) {
    if DEBUG.load(core::sync::atomic::Ordering::Relaxed) {
        println!("{DEBUG_TEXT_DECORATION}Formatting '{language}' block from :{start} to :{end}{TEXT_RESET}");
    }
}

#[inline]
pub fn print_debug_formatter_info(formatter: &str) {
    if DEBUG.load(core::sync::atomic::Ordering::Relaxed) {
        println!("{DEBUG_TEXT_DECORATION}Using formatter: '{formatter}'{TEXT_RESET}");
    }
}

#[inline]
pub fn write_unchanged_line(path: &std::path::Path, dur: core::time::Duration) {
    println!(
        "{GREY_TEXT}{} {}ms (unchanged){TEXT_RESET}",
        path.display(),
        dur.as_millis()
    );
}

#[inline]
pub fn write_changed_line(path: &std::path::Path, dur: core::time::Duration) {
    println!("{} {}ms", path.display(), dur.as_millis());
}

#[inline]
pub fn print_file_not_found(path: &std::path::Path) {
    println!(
        "{ERROR_TEXT_DECORATION}No file or directory with the name \"{}\" found{TEXT_RESET}",
        path.display(),
    );
}

use std::io::Write;

use console::style;
use log::LevelFilter;

use crate::cli::LogLevel;

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
        }
    }
}

pub fn setup_logger(log_level: LogLevel) {
    env_logger::Builder::from_env("MDSF_LOG")
        .filter(None, LevelFilter::Off)
        .filter_module("mdsf::terminal", log_level.into())
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .format_level(false)
        .format(move |buf, record| {
            let x = 0;

            match record.level() {
                log::Level::Error => writeln!(
                    buf,
                    "{}{} {}",
                    style("error").red().bold(),
                    style(":").bold(),
                    record.args()
                ),
                log::Level::Warn => writeln!(
                    buf,
                    "{}{} {}",
                    style("warn").yellow().bold(),
                    style(":").bold(),
                    record.args()
                ),
                log::Level::Info => writeln!(buf, "{}", record.args()),
                log::Level::Debug => writeln!(buf, "{}", style(format!("{}", record.args())).dim()),
                log::Level::Trace => writeln!(buf, "{}", style(format!("{}", record.args())).dim()),
            }
            /*
                    let tag = match record.level() {
                        log::Level::Error => style("e").red(),
                        log::Level::Warn => style("w").yellow(),
                        log::Level::Info => style("i").green(),
                        log::Level::Debug => style("d").cyan(),
                        log::Level::Trace => style("t").magenta(),
                    }
                    .bold();

                    writeln!(buf, "{}{} {}", tag, style(":").bold(), record.args())
            */
        })
        .init();
}

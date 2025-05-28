use std::io::Write;

use log::LevelFilter;
use owo_colors::OwoColorize as _;

use crate::cli::LogLevel;

impl From<LogLevel> for LevelFilter {
    #[inline]
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => Self::Off,
            LogLevel::Error => Self::Error,
            LogLevel::Warn => Self::Warn,
            LogLevel::Info => Self::Info,
            LogLevel::Debug => Self::Debug,
            LogLevel::Trace => Self::Trace,
        }
    }
}

#[inline]
fn force_colors() -> bool {
    std::env::var("GITHUB_ACTIONS").is_ok()
}

#[inline]
pub fn setup_logger(log_level: LogLevel) {
    if force_colors() {
        owo_colors::set_override(true);
    }

    env_logger::Builder::from_env("MDSF_LOG")
        .filter(None, LevelFilter::Off)
        .filter_module("mdsf::terminal", log_level.into())
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .format_level(false)
        .format(move |buf, record| match record.level() {
            log::Level::Error => {
                writeln!(
                    buf,
                    "{}",
                    format!("{}", record.args())
                        .if_supports_color(owo_colors::Stream::Stdout, |text| text
                            .style(owo_colors::Style::new().bold().red()))
                )
            }
            log::Level::Warn => writeln!(
                buf,
                "{}",
                format!("{}", record.args())
                    .if_supports_color(owo_colors::Stream::Stdout, |text| text
                        .style(owo_colors::Style::new().yellow().bold()))
            ),
            log::Level::Info => writeln!(buf, "{}", record.args()),
            log::Level::Debug | log::Level::Trace => writeln!(
                buf,
                "{}",
                format!("{}", record.args())
                    .if_supports_color(owo_colors::Stream::Stdout, |text| text
                        .style(owo_colors::Style::new().dimmed()))
            ),
        })
        .init();
}

#[cfg(test)]
mod test_level_filter {
    use log::LevelFilter;

    use crate::cli::LogLevel;

    #[test]
    fn they_should_be_two_way() {
        assert_eq!(LevelFilter::from(LogLevel::Off), LevelFilter::Off);

        assert_eq!(LevelFilter::from(LogLevel::Error), LevelFilter::Error);

        assert_eq!(LevelFilter::from(LogLevel::Warn), LevelFilter::Warn);

        assert_eq!(LevelFilter::from(LogLevel::Info), LevelFilter::Info);

        assert_eq!(LevelFilter::from(LogLevel::Debug), LevelFilter::Debug);

        assert_eq!(LevelFilter::from(LogLevel::Trace), LevelFilter::Trace);
    }
}

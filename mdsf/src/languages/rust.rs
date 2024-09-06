use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    ("rust".to_string(), MdsfFormatter::Single(Tooling::RustFmt))
}

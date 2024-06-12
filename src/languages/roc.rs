use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    ("roc".to_string(), MdsfFormatter::Single(Tooling::RocFormat))
}

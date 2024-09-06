use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    ("kotlin".to_string(), MdsfFormatter::Single(Tooling::Ktfmt))
}

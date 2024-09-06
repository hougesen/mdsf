use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    ("nim".to_string(), MdsfFormatter::Single(Tooling::Nimpretty))
}

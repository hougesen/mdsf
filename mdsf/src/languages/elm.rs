use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    ("elm".to_string(), MdsfFormatter::Single(Tooling::ElmFormat))
}

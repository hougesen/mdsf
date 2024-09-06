use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "gleam".to_string(),
        MdsfFormatter::Single(Tooling::GleamFormat),
    )
}

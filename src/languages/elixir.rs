use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "elixir".to_string(),
        MdsfFormatter::Single(Tooling::MixFormat),
    )
}

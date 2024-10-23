use crate::{formatters::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    ("toml".to_string(), MdsfFormatter::Single(Tooling::Taplo))
}

use crate::{execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    ("yaml".to_string(), MdsfFormatter::Single(Tooling::Prettier))
}

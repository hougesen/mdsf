use crate::{formatters::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    ("nim".to_string(), MdsfFormatter::Single(Tooling::Nimpretty))
}

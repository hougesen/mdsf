use crate::{execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "dart".to_string(),
        MdsfFormatter::Single(Tooling::DartFormat),
    )
}

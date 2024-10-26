use crate::{execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "typescript".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::Prettier),
            MdsfFormatter::Single(Tooling::BiomeFormat),
            MdsfFormatter::Single(Tooling::DenoFmt),
        ])]),
    )
}

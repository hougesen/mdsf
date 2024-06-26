use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "javascript".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::Prettier),
            MdsfFormatter::Single(Tooling::Biome),
            MdsfFormatter::Single(Tooling::DenoFmt),
            MdsfFormatter::Single(Tooling::ClangFormat),
            MdsfFormatter::Single(Tooling::Standardjs),
        ])]),
    )
}

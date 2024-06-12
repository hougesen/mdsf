use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "yaml".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::Prettier),
            MdsfFormatter::Single(Tooling::YamlFmt),
            MdsfFormatter::Single(Tooling::YamlFix),
        ])]),
    )
}

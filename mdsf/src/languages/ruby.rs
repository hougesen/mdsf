use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "ruby".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::RuboCop),
            MdsfFormatter::Single(Tooling::Rufo),
            MdsfFormatter::Single(Tooling::RubyFmt),
            MdsfFormatter::Single(Tooling::Standardrb),
        ])]),
    )
}

use crate::{execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "ruby".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::Rubocop),
            MdsfFormatter::Single(Tooling::Rufo),
            MdsfFormatter::Single(Tooling::Rubyfmt),
            MdsfFormatter::Single(Tooling::Standardrb),
        ])]),
    )
}

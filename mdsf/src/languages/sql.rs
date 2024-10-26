use crate::{execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "sql".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::SqlFormatter),
            MdsfFormatter::Single(Tooling::SqlfluffFormat),
            MdsfFormatter::Single(Tooling::Sqlfmt),
        ])]),
    )
}

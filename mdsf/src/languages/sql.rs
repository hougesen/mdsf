use crate::{formatters::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "sql".to_string(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::SQLFormatter),
            MdsfFormatter::Single(Tooling::Sqlfluff),
        ])]),
    )
}

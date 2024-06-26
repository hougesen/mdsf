use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "go".to_string(),
        MdsfFormatter::Multiple(vec![
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(Tooling::GCI),
                MdsfFormatter::Single(Tooling::GoImportsReviser),
                MdsfFormatter::Single(Tooling::GoImports),
            ]),
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(Tooling::GoFumpt),
                MdsfFormatter::Single(Tooling::GoFmt),
                MdsfFormatter::Single(Tooling::CrlFmt),
            ]),
        ]),
    )
}

use crate::{execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "go".to_string(),
        MdsfFormatter::Multiple(vec![
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(Tooling::Gci),
                MdsfFormatter::Single(Tooling::GoimportsReviser),
                MdsfFormatter::Single(Tooling::Goimports),
            ]),
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(Tooling::Gofumpt),
                MdsfFormatter::Single(Tooling::Gofmt),
            ]),
        ]),
    )
}

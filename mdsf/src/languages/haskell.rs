use crate::{formatters::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (std::string::String, MdsfFormatter<Tooling>) {
    (
        "haskell".to_owned(),
        MdsfFormatter::Multiple(vec![MdsfFormatter::Multiple(vec![
            MdsfFormatter::Single(Tooling::Fourmolu),
            MdsfFormatter::Single(Tooling::Ormolu),
            MdsfFormatter::Single(Tooling::HIndent),
            MdsfFormatter::Single(Tooling::StylishHaskell),
        ])]),
    )
}

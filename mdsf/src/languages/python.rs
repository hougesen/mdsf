use crate::{execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "python".to_string(),
        MdsfFormatter::Multiple(vec![
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(Tooling::Usort),
                MdsfFormatter::Single(Tooling::Isort),
            ]),
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(Tooling::RuffFormat),
                MdsfFormatter::Single(Tooling::Blue),
                MdsfFormatter::Single(Tooling::Black),
                MdsfFormatter::Single(Tooling::Yapf),
                MdsfFormatter::Single(Tooling::Autopep8),
                MdsfFormatter::Single(Tooling::Pyink),
            ]),
        ]),
    )
}

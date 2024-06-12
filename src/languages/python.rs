use crate::formatters::{MdsfFormatter, Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<Tooling>) {
    (
        "ruby".to_string(),
        MdsfFormatter::Multiple(vec![
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(Tooling::Usort),
                MdsfFormatter::Single(Tooling::Isort),
            ]),
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(Tooling::Ruff),
                MdsfFormatter::Single(Tooling::Blue),
                MdsfFormatter::Single(Tooling::Black),
                MdsfFormatter::Single(Tooling::Yapf),
                MdsfFormatter::Single(Tooling::Autopep8),
                MdsfFormatter::Single(Tooling::PyInk),
            ]),
        ]),
    )
}

use crate::{config::MdsfTool, execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<MdsfTool>) {
    (
        "python".to_string(),
        MdsfFormatter::Multiple(vec![
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Usort)),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Isort)),
            ]),
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::RuffFormat)),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Blue)),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Black)),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Yapf)),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Autopep8)),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Pyink)),
            ]),
        ]),
    )
}

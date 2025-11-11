use crate::{config::MdsfTool, execution::MdsfToolWrapper, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfToolWrapper<MdsfTool>) {
    (
        "python".to_string(),
        MdsfToolWrapper::Multiple(vec![
            MdsfToolWrapper::Multiple(vec![
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Usort)),
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Isort)),
            ]),
            MdsfToolWrapper::Multiple(vec![
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::RuffFormat)),
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Blue)),
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Black)),
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Yapf)),
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Autopep8)),
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Pyink)),
            ]),
        ]),
    )
}

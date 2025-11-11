use crate::{config::MdsfTool, execution::MdsfToolWrapper, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfToolWrapper<MdsfTool>) {
    (
        "go".to_string(),
        MdsfToolWrapper::Multiple(vec![
            MdsfToolWrapper::Multiple(vec![
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Gci)),
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::GoimportsReviser)),
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Goimports)),
            ]),
            MdsfToolWrapper::Multiple(vec![
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Gofumpt)),
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Gofmt)),
            ]),
        ]),
    )
}

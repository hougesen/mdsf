use crate::{config::MdsfTool, execution::MdsfToolWrapper, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfToolWrapper<MdsfTool>) {
    (
        "csharp".to_string(),
        MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Csharpier)),
    )
}

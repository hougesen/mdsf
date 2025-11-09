use crate::{config::MdsfTool, execution::MdsfFormatter, tools::Tooling};

#[inline]
pub fn default_config() -> (String, MdsfFormatter<MdsfTool>) {
    (
        "go".to_string(),
        MdsfFormatter::Multiple(vec![
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Gci)),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::GoimportsReviser)),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Goimports)),
            ]),
            MdsfFormatter::Multiple(vec![
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Gofumpt)),
                MdsfFormatter::Single(MdsfTool::Preset(Tooling::Gofmt)),
            ]),
        ]),
    )
}

///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("fmt");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("kcl")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_kcl_fmt {
    #[test_with::executable(kcl)]
    fn test_kcl_fmt_kcl_709718d5d09b85cd() {
        let input = r#"apiVersion = "apps/v1"
kind = "Deployment"
metadata = {
    name =  "nginx"
                   labels.app = "nginx"
}
spec = {
    replicas    = 3
    selector.matchLabels = metadata.labels
    template.metadata.labels =                  metadata.labels
    template.spec.containers = [     {
        name = metadata.name
        image = "${metadata.name}:1.14.2"
        ports = [{                                                  containerPort = 80}]
    }]
}
"#;

        let output = r#"apiVersion = "apps/v1"
kind = "Deployment"
metadata = {
    name = "nginx"
    labels.app = "nginx"
}
spec = {
    replicas = 3
    selector.matchLabels = metadata.labels
    template.metadata.labels = metadata.labels
    template.spec.containers = [{
        name = metadata.name
        image = "${metadata.name}:1.14.2"
        ports = [{containerPort = 80}]
    }]
}
"#;

        let file_ext = crate::fttype::get_file_extension("kcl");

        crate::tools::Tooling::KclFmt.test_format_snippet(input, output, &file_ext);
    }
}

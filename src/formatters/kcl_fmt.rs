use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_kcl_fmt(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("kcl");

    cmd.arg("fmt").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_kcl_fmt {
    use super::format_using_kcl_fmt;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(kcl)]
    #[test]
    fn it_should_format_kcl() {
        let input = r#"apiVersion = "apps/v1"
kind = "Deployment"
metadata = {
    name = "nginx"
    labels.app = "nginx"
}
spec = {
    replicas = 3
    selector.matchLabels = metadata.labels
    template.metadata.labels = metadata.labels
    template.spec.containers = [
        {
            name = metadata.name
            image = "${metadata.name}:1.14.2"
            ports = [{ containerPort = 80 }]
        }
    ]
}
"#;

        let expected_output = r#"apiVersion = "apps/v1"
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

        let snippet =
            setup_snippet(input, Language::Kcl.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_kcl_fmt(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}
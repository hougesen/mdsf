use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("kcl");

    cmd.arg("fmt").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_kcl_fmt {
    use super::run;
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(kcl)]
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
            setup_snippet(input, language_to_ext("kcl")).expect("it to create a snippet file");

        let output = run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

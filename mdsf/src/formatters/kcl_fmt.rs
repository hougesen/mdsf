use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(snippet_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("kcl").build();

    cmd.arg("fmt").arg(snippet_path);

    execute_command(cmd, snippet_path)
}

#[cfg(test)]
mod test_kcl_fmt {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(kcl)]
    fn it_should_format_kcl() {
        let input = r#"import     math
mixin DeploymentMixin:
    service:str ="my-service"
schema DeploymentBase:
    name: str
    image  : str
schema Deployment[replicas] ( DeploymentBase )   :
    mixin[DeploymentMixin]
    replicas   : int   = replicas
    command: [str  ]
    labels: {str:  str}
deploy = Deployment(replicas = 3){}
"#;

        let expected_output = r#"import math

mixin DeploymentMixin:
    service: str = "my-service"

schema DeploymentBase:
    name: str
    image: str

schema Deployment[replicas](DeploymentBase):
    mixin [DeploymentMixin]
    replicas: int = replicas
    command: [str]
    labels: {str:str}

deploy = Deployment(replicas=3) {}
"#;

        let ft = language_to_ext("kcl");

        let snippet = setup_snippet(input, if ft.is_empty() { ".k" } else { ft })
            .expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

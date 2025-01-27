///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_prisma_format_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    let fps = file_path.to_string_lossy();
    cmd.arg(format!("--schema={fps}"));
    cmd
}

const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("prisma"),
    CommandType::Direct("prisma"),
    CommandType::Npm("prisma"),
];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_prisma_format_args)
}

#[cfg(test)]
mod test_prisma_format {
    #[test_with::executable(npx)]
    fn test_prisma_format_schema_9e1c2cd6551f36db() {
        let input = r#"datasource          db             {
  provider                  = "postgresql"
  url      =          env("DATABASE_URL")
  directUrl =                       env("DIRECT_DATABASE_URL")
}


"#;
        let output = Some(
            r#"datasource db {
  provider  = "postgresql"
  url       = env("DATABASE_URL")
  directUrl = env("DIRECT_DATABASE_URL")
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("schema");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::prisma_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

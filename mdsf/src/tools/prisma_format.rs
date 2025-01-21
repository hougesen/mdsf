///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_prisma_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    let fps = file_path.to_string_lossy();
    cmd.arg(format!("--schema={fps}"));
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("prisma"),
        CommandType::Direct("prisma"),
        CommandType::Npm("prisma"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_prisma_format_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
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
        let result = crate::tools::prisma_format::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

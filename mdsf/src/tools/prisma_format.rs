///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("format");
    let fps = file_path.to_string_lossy();
    cmd.arg(format!("--schema={fps}"));
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::NodeModules("prisma"),
    CommandType::Direct("prisma"),
    CommandType::Npm("prisma"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_prisma_format {
    const TIMEOUT: u64 = 0;

    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(npx)]
    fn test_prisma_format_schema_b6e70b1b6bb7472e() {
        let input = r#"datasource          db             {
  provider                  = "postgresql"
  url      =          env("DATABASE_URL")
  directUrl =                       env("DIRECT_DATABASE_URL")
}


"#;

        let output = r#"datasource db {
  provider  = "postgresql"
  url       = env("DATABASE_URL")
  directUrl = env("DIRECT_DATABASE_URL")
}
"#;

        let file_ext = crate::fttype::get_file_extension("schema");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            super::IS_STDIN,
            DEBUG_ENABLED,
            crate::runners::JavaScriptRuntime::default(),
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

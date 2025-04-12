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

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("prisma"),
    CommandType::Direct("prisma"),
    CommandType::Npm("prisma"),
    CommandType::Pnpm("prisma"),
    CommandType::Bun("prisma"),
    CommandType::Deno("prisma"),
    CommandType::Yarn("prisma"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_prisma_format {
    #[test_with::executable(prisma || npx || pnpm || deno || bunx)]
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

        crate::tools::Tooling::PrismaFormat.test_format_snippet(input, output, &file_ext);
    }
}

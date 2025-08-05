use super::path::setup_command_from_path;

#[inline]
pub fn setup_node_modules_command(binary_name: &str) -> std::process::Command {
    setup_command_from_path("./node_modules/.bin/", binary_name)
}

#[inline]
pub fn setup_npx_command(package_name: &str, executable_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("npx");

    // Auto install package
    cmd.arg("--yes");

    if package_name == executable_name {
        cmd.arg(package_name);
    } else {
        cmd.arg("--package");
        cmd.arg(package_name);
        cmd.arg(executable_name);
    }

    cmd
}

#[cfg(test)]
mod test_node {
    #[test_with::executable(npx)]
    #[test]
    fn it_can_execute_an_npm_package_script() {
        let input = "";

        let file_ext = crate::filetype::get_file_extension("json");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        crate::execution::run_tools(
            &[crate::runners::CommandType::Npm("prettier", "prettier")],
            snippet.path(),
            crate::tools::prettier::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::prettier::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                npx: true,
                ..Default::default()
            },
        )
        .expect("it to succeed");
    }

    #[test_with::executable(npx)]
    #[test]
    fn it_works_with_executable_name() {
        let input = r#"model Pet {  name: string;  age: int32;kind: "dog" | "cat" | "fish";}
"#;

        let output = r#"model Pet {
  name: string;
  age: int32;
  kind: "dog" | "cat" | "fish";
}
"#;

        let file_ext = crate::filetype::get_file_extension("typespec");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &[crate::runners::CommandType::Npm(
                "@typespec/compiler",
                "tsp",
            )],
            snippet.path(),
            crate::tools::tsp_format::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::tsp_format::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                npx: true,
                ..Default::default()
            },
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

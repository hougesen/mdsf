#[inline]
pub fn setup_yarn_dlx_command(package_name: &str, executable_name: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("yarn");

    cmd.arg("dlx");
    cmd.arg("--quiet");

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
mod test_yarn {
    #[test_with::executable(yarn)]
    #[test]
    fn it_can_execute_an_npm_package_script() {
        let input = "[1,2,3,4,5,6]";
        let output = "[1, 2, 3, 4, 5, 6]\n";

        let file_ext = crate::filetype::get_file_extension("json");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &[crate::runners::CommandType::Yarn("prettier", "prettier")],
            snippet.path(),
            crate::tools::prettier::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::prettier::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                yarn: true,
                ..Default::default()
            },
        )
        .expect("it to succeed")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }

    #[test_with::executable(yarn)]
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
            &[crate::runners::CommandType::Yarn(
                "@typespec/compiler",
                "tsp",
            )],
            snippet.path(),
            crate::tools::tsp_format::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::tsp_format::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                yarn: true,
                ..Default::default()
            },
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

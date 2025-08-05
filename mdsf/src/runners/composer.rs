use super::path::setup_command_from_path;

#[inline]
pub fn setup_php_vender_bin_command(binary_name: &str) -> std::process::Command {
    setup_command_from_path("./vendor/bin/", binary_name)
}

#[cfg(test)]
mod test_composer {
    #[test_with::executable(./vendor/bin/mago)]
    fn test_composer_path() {
        let input = r#"<?php
echo "Hello World!";
?>"#;

        let output = r#"<?php
echo 'Hello World!';
"#;

        let file_ext = crate::filetype::get_file_extension("php");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::execution::run_tools(
            &[crate::runners::CommandType::PhpVendor("mago")],
            snippet.path(),
            crate::tools::mago_format::set_args,
            crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
            crate::tools::mago_format::IS_STDIN,
            crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
            &crate::config::MdsfConfigRunners {
                bunx: false,
                deno: false,
                dotnet: false,
                dub: false,
                gem_exec: false,
                npx: false,
                pipx: false,
                pnpm: false,
                uv: false,
                yarn: false,
            },
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

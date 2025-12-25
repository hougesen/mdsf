use crate::{
    LineInfo,
    cli::{OnMissingLanguageDefinition, OnMissingToolBinary},
    config::MdsfConfig,
    error::{MdsfError, exit_with_error, set_exit_code_error},
    execution::format_snippet,
    terminal::print_unknown_language,
};

#[inline]
pub fn format_file(
    config: &MdsfConfig,
    filename: &std::path::Path,
    input: &str,
    timeout: u64,
    debug_enabled: bool,
    on_missing_tool_binary: OnMissingToolBinary,
    on_missing_language_definition: OnMissingLanguageDefinition,
) -> (bool, String) {
    let mut options = pulldown_cmark::Options::all();
    options.remove(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);

    let mut codeblock_language = None;

    let mut changes = Vec::new();

    for (event, range) in pulldown_cmark::Parser::new_ext(&input, options).into_offset_iter() {
        match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(
                pulldown_cmark::CodeBlockKind::Fenced(language),
            )) => {
                let language = language.to_string();

                // "*" is always ran
                // "_" is fallback formatters
                if config.languages.contains_key(&language)
                    || config.languages.contains_key("*")
                    || config.languages.contains_key("_")
                {
                    codeblock_language = Some(language);
                } else if !language.is_empty() {
                    match on_missing_language_definition {
                        OnMissingLanguageDefinition::Ignore => {
                            print_unknown_language(&language, filename, false);
                        }
                        OnMissingLanguageDefinition::Fail => {
                            print_unknown_language(&language, filename, true);

                            set_exit_code_error();
                        }
                        OnMissingLanguageDefinition::FailFast => exit_with_error(
                            &MdsfError::MissingLanguageDefinition(filename.to_path_buf(), language),
                        ),
                    };
                }
            }

            pulldown_cmark::Event::Text(input_code) => {
                if let Some(ref language) = codeblock_language {
                    let formatted = format_snippet(
                        config,
                        &LineInfo {
                            filename,
                            language,
                            start: range.start - 1,
                            end: range.end + 1,
                        },
                        &input_code,
                        timeout,
                        debug_enabled,
                        on_missing_tool_binary,
                    );

                    if input_code.into_string() != formatted {
                        changes.push((formatted, range));
                    }
                }
            }

            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::CodeBlock) => {
                codeblock_language = None;
            }

            _ => {}
        };
    }

    let mut output = input.to_string();

    for (codeblock, range) in changes.into_iter().rev() {
        output.replace_range(range, &codeblock);
    }

    if config.format_finished_document && !output.is_empty() {
        output = format_snippet(
            config,
            &LineInfo {
                filename,
                language: "markdown",
                start: 0,
                end: 0,
            },
            &output,
            timeout,
            debug_enabled,
            on_missing_tool_binary,
        );
    }

    output = config.newline.normalize(output);

    (output != input, output)
}

#[cfg(test)]
mod test {
    use crate::{
        config::{MdsfConfig, MdsfTool},
        execution::MdsfToolWrapper,
        parser::v2::format_file,
        testing::{
            DEFAULT_ON_MISSING_LANGUAGE_DEFINITION, DEFAULT_ON_MISSING_TOOL_BINARY,
            DEFAULT_TEST_DEBUG_ENABLED, DEFAULT_TEST_FORMATTER_TIMEOUT,
        },
        tools::Tooling,
    };

    #[test]
    fn it_should_format_the_code() {
        let input = "```rust
fn           add(
     a:
      i32, b: i32) -> i32 {
    a + b
}
```";

        let expected_output = "```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
";

        let config = MdsfConfig {
            languages: std::collections::BTreeMap::from_iter([(
                "rust".to_string(),
                MdsfToolWrapper::Single(MdsfTool::Preset(Tooling::Rustfmt)),
            )]),
            ..Default::default()
        };

        {
            let (modified, output) = format_file(
                &config,
                std::path::Path::new("."),
                input,
                DEFAULT_TEST_FORMATTER_TIMEOUT,
                DEFAULT_TEST_DEBUG_ENABLED,
                DEFAULT_ON_MISSING_TOOL_BINARY,
                DEFAULT_ON_MISSING_LANGUAGE_DEFINITION,
            );

            assert!(modified);

            assert_eq!(output, expected_output.trim());
        };
    }
}

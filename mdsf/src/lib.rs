use caching::CacheEntry;
use config::MdsfConfig;
use execution::format_snippet;
use parser::{indent_codeblock, parse_generic_codeblock, parse_go_codeblock, remove_go_package};
use terminal::{
    print_changed_file, print_changed_file_error, print_error_reading_file,
    print_error_saving_file, print_unchanged_file, warn_unknown_language,
};

pub mod caching;
pub mod cli;
pub mod config;
pub mod error;
pub mod execution;
pub mod fttype;
pub mod generated;
pub mod languages;
mod parser;
pub mod runners;
pub mod terminal;
mod tools;

static MDSF_PROJECT_DIR: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();

#[inline]
pub fn get_project_dir() -> &'static std::path::Path {
    let project_dir = MDSF_PROJECT_DIR.get_or_init(|| {
        directories::ProjectDirs::from("dk.mhouge.mdsf", "mdsf", "mdsf").map_or_else(
            || std::path::PathBuf::from(".mdsf/"),
            |dir| dir.cache_dir().to_path_buf(),
        )
    });

    let _ = std::fs::create_dir_all(project_dir);

    project_dir
}

#[inline]
pub fn format_file(
    config: &MdsfConfig,
    filename: &std::path::Path,
    input: &str,
    timeout: u64,
    debug_enabled: bool,
) -> (bool, String) {
    let mut output = String::with_capacity(input.len() + 128);

    let mut lines = input.lines().enumerate();

    while let Some((line_index, line)) = lines.next() {
        // TODO: implement support for code blocks with 4 `
        let trimmed_line = line.trim_start();

        if trimmed_line.starts_with("```") {
            let indentation = line.replace(trimmed_line, "");

            let language = trimmed_line
                .strip_prefix("```")
                .map(str::trim)
                .unwrap_or_default();

            // "*" is always ran
            // "_" is fallback formatters
            if config.languages.contains_key(language)
                || config.languages.contains_key("*")
                || config.languages.contains_key("_")
            {
                let is_go = language == "go" || language == "golang";

                let (is_snippet, code_snippet, snippet_lines) = if is_go {
                    parse_go_codeblock(&mut lines)
                } else {
                    parse_generic_codeblock(&mut lines)
                };

                if is_snippet {
                    let formatted = format_snippet(
                        config,
                        &LineInfo {
                            filename,
                            language,
                            start: line_index + 1,
                            end: line_index + snippet_lines + 1,
                        },
                        &code_snippet,
                        timeout,
                        debug_enabled,
                    );

                    let formatted = if is_go {
                        remove_go_package(formatted)
                    } else {
                        formatted
                    };

                    let formatted = formatted.trim().to_owned();

                    let formatted = indent_codeblock(&indentation, formatted);

                    output.push_str(line);

                    output.push('\n');

                    output.push_str(&formatted);

                    output.push('\n');
                    output.push_str(&indentation);
                    output.push_str("```");
                } else {
                    output.push_str(line);
                    output.push_str(&code_snippet);
                }
            } else {
                if !language.is_empty() {
                    warn_unknown_language(language, filename);
                }

                output.push_str(line);
            }
        } else {
            output.push_str(line);
        }

        output.push('\n');
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
        );
    }

    (output != input, output)
}

#[inline]
fn format_or_use_cache(
    config: &MdsfConfig,
    path: &std::path::Path,
    input: &str,
    cache_entry: Option<CacheEntry>,
    timeout: u64,
    debug_enabled: bool,
) -> (String, bool, bool) {
    if let Some(cached_value) = cache_entry.as_ref().and_then(caching::CacheEntry::get) {
        let modified = cached_value != input;

        return (cached_value, modified, true);
    }

    let (modified, output) = format_file(config, path, input, timeout, debug_enabled);

    if let Some(cache) = cache_entry {
        // We do not (currently) care if saving the cache fails.
        let _ = cache.set(&output);
    }

    (output, modified, false)
}

#[inline]
pub fn handle_file(
    config: &MdsfConfig,
    path: &std::path::Path,
    dry_run: bool,
    config_hash: Option<String>,
    timeout: u64,
    debug_enabled: bool,
) -> bool {
    let time = std::time::Instant::now();

    match std::fs::read_to_string(path) {
        Ok(input) => {
            if input.is_empty() {
                print_unchanged_file(path, time.elapsed(), false);

                return false;
            }

            let cache_entry = config_hash.map(|conf| CacheEntry::new(conf, path, &input));

            let (output, modified, cached) =
                format_or_use_cache(config, path, &input, cache_entry, timeout, debug_enabled);

            if modified && output != input {
                if dry_run {
                    print_changed_file_error(path);
                } else {
                    let write_result = std::fs::write(path, &output);

                    if let Err(write_error) = write_result {
                        print_error_saving_file(path, &write_error);

                        return false;
                    }
                }

                print_changed_file(path, time.elapsed(), cached);

                return true;
            }

            print_unchanged_file(path, time.elapsed(), cached);

            false
        }

        Err(error) => {
            print_error_reading_file(path, &error);

            false
        }
    }
}

pub struct LineInfo<'a> {
    pub filename: &'a std::path::Path,
    pub language: &'a str,
    pub start: usize,
    pub end: usize,
}

#[cfg(test)]
impl LineInfo<'_> {
    pub fn fake() -> Self {
        Self {
            filename: std::path::Path::new("."),
            language: "fakelang",
            start: 0,
            end: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::MdsfConfig,
        execution::{setup_snippet, MdsfFormatter},
        format_file,
        fttype::get_file_extension,
        handle_file,
        tools::Tooling,
    };

    const DEBUG_ENABLED: bool = true;

    const TIMEOUT: u64 = 0;

    const DRY_RUN: bool = false;

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

        let config = MdsfConfig::default();

        {
            let (modified, output) = format_file(
                &config,
                std::path::Path::new("."),
                input,
                TIMEOUT,
                DEBUG_ENABLED,
            );

            assert!(modified);

            assert_eq!(output, expected_output);
        };

        {
            let file =
                setup_snippet(input, &get_file_extension("markdown")).expect("it to create a file");

            assert!(handle_file(
                &config,
                file.path(),
                DRY_RUN,
                None,
                TIMEOUT,
                DEBUG_ENABLED
            ));

            let output = std::fs::read_to_string(file.path()).expect("it to return the string");

            assert_eq!(output, expected_output);
        };
    }

    #[test]
    fn it_should_format_the_codeblocks_that_start_with_whitespace() {
        let mut whitespaces = std::collections::HashSet::new();

        let mut spaces = String::new();
        let mut tabs = String::new();

        for _ in 0..10 {
            spaces.push(' ');
            tabs.push('\t');
            whitespaces.insert(spaces.clone());
            whitespaces.insert(tabs.clone());
        }

        for whitespace in whitespaces {
            let input = format!(
                "{whitespace}```rust
fn           add(
     a:
      i32, b: i32) -> i32 {{
    a + b
}}
{whitespace}```"
            );

            let expected_output = format!(
                "{whitespace}```rust
{whitespace}fn add(a: i32, b: i32) -> i32 {{
{whitespace}    a + b
{whitespace}}}
{whitespace}```
"
            );

            let config = MdsfConfig::default();

            {
                let (modified, output) = format_file(
                    &config,
                    std::path::Path::new("."),
                    &input,
                    TIMEOUT,
                    DEBUG_ENABLED,
                );

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(&input, &get_file_extension("markdown"))
                    .expect("it to create a file");

                assert!(handle_file(
                    &config,
                    file.path(),
                    DRY_RUN,
                    None,
                    TIMEOUT,
                    DEBUG_ENABLED
                ));

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        }
    }

    #[test]
    fn it_should_not_modify_outside_blocks() {
        let input = "# title

Let's play!

Have some fun...

I like \"mdsf\"

| Field            | Description            |
|------------------|------------------------|
| foo              |   foo field            |
| bar              |   bar field            |

```rust
fn           add(
     a:
      i32, b: i32) -> i32 {
    a + b
}
```


";

        let expected_output = "# title

Let's play!

Have some fun...

I like \"mdsf\"

| Field            | Description            |
|------------------|------------------------|
| foo              |   foo field            |
| bar              |   bar field            |

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```


";

        let config = MdsfConfig::default();

        let (modified, output) = format_file(
            &config,
            std::path::Path::new("."),
            input,
            TIMEOUT,
            DEBUG_ENABLED,
        );

        assert!(modified);

        assert_eq!(output, expected_output);
    }

    #[test_with::executable(gofmt)]
    #[test]
    fn it_should_support_go_with_package() {
        let input = "```go
package main

import (
\t\"errors\"
\t\"time\"
)

var err = errors.New(\"dddd\")

type Whatever struct {
     StartAt                time.Time  `json:\"start_at\"`
                End                time.Time  `json:\"end_at\"`
    Delete bool `json:-\"`
}
```";

        let expected_output = "```go
package main

import (
\t\"errors\"
\t\"time\"
)

var err = errors.New(\"dddd\")

type Whatever struct {
\tStartAt time.Time `json:\"start_at\"`
\tEnd     time.Time `json:\"end_at\"`
\tDelete  bool      `json:-\"`
}
```
";

        {
            let config = MdsfConfig::default();

            {
                let (modified, output) = format_file(
                    &config,
                    std::path::Path::new("."),
                    input,
                    TIMEOUT,
                    DEBUG_ENABLED,
                );

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, &get_file_extension("markdown"))
                    .expect("it to create a file");

                assert!(handle_file(
                    &config,
                    file.path(),
                    DRY_RUN,
                    None,
                    TIMEOUT,
                    DEBUG_ENABLED
                ));

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        };

        {
            let config = MdsfConfig {
                languages: std::collections::BTreeMap::from_iter([(
                    "go".to_string(),
                    MdsfFormatter::Single(Tooling::Gofmt),
                )]),
                ..MdsfConfig::default()
            };

            {
                let (modified, output) = format_file(
                    &config,
                    std::path::Path::new("."),
                    input,
                    TIMEOUT,
                    DEBUG_ENABLED,
                );

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, &get_file_extension("markdown"))
                    .expect("it to create a file");

                assert!(handle_file(
                    &config,
                    file.path(),
                    DRY_RUN,
                    None,
                    TIMEOUT,
                    DEBUG_ENABLED
                ));

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        }
    }

    #[test]
    fn it_should_add_go_package_if_missing() {
        let input = "```go
import (
\t\"errors\"
\t\"time\"
)

var err = errors.New(\"dddd\")

type Whatever struct {
     StartAt                time.Time  `json:\"start_at\"`
                End                time.Time  `json:\"end_at\"`
    Delete bool `json:-\"`
}
```";

        let expected_output = "```go
import (
\t\"errors\"
\t\"time\"
)

var err = errors.New(\"dddd\")

type Whatever struct {
\tStartAt time.Time `json:\"start_at\"`
\tEnd     time.Time `json:\"end_at\"`
\tDelete  bool      `json:-\"`
}
```
";

        {
            let config = MdsfConfig::default();

            {
                let (modified, output) = format_file(
                    &config,
                    std::path::Path::new("."),
                    input,
                    TIMEOUT,
                    DEBUG_ENABLED,
                );

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, &get_file_extension("markdown"))
                    .expect("it to create a file");

                assert!(handle_file(
                    &config,
                    file.path(),
                    DRY_RUN,
                    None,
                    TIMEOUT,
                    DEBUG_ENABLED
                ));

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        };

        {
            let config = MdsfConfig {
                languages: std::collections::BTreeMap::from_iter([(
                    "go".to_string(),
                    MdsfFormatter::Single(Tooling::Gofmt),
                )]),

                ..MdsfConfig::default()
            };

            {
                let (modified, output) = format_file(
                    &config,
                    std::path::Path::new("."),
                    input,
                    TIMEOUT,
                    DEBUG_ENABLED,
                );

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, &get_file_extension("markdown"))
                    .expect("it to create a file");

                assert!(handle_file(
                    &config,
                    file.path(),
                    DRY_RUN,
                    None,
                    TIMEOUT,
                    DEBUG_ENABLED
                ));

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        }
    }

    #[test]
    fn it_should_not_care_if_go_package_is_set() {
        let input = "With package name

```go



  package main

   func add(a int , b int  ) int {
                return a + b
       }


```


Without package name


```go




   func add(a int , b int  ) int {
                return a + b
       }


```

";

        let expected_output = "With package name

```go
package main

func add(a int, b int) int {
	return a + b
}
```


Without package name


```go
func add(a int, b int) int {
	return a + b
}
```

";

        {
            let config = MdsfConfig {
                languages: std::collections::BTreeMap::from_iter([(
                    "go".to_string(),
                    MdsfFormatter::Single(Tooling::Gofmt),
                )]),
                ..Default::default()
            };

            {
                let (modified, output) = format_file(
                    &config,
                    std::path::Path::new("."),
                    input,
                    TIMEOUT,
                    DEBUG_ENABLED,
                );

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, &get_file_extension("markdown"))
                    .expect("it to create a file");

                assert!(handle_file(
                    &config,
                    file.path(),
                    DRY_RUN,
                    None,
                    TIMEOUT,
                    DEBUG_ENABLED
                ));

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        };

        {
            let config = MdsfConfig {
                languages: std::collections::BTreeMap::from_iter([(
                    "go".to_string(),
                    MdsfFormatter::Single(Tooling::Gofmt),
                )]),
                ..MdsfConfig::default()
            };

            {
                let (modified, output) = format_file(
                    &config,
                    std::path::Path::new("."),
                    input,
                    TIMEOUT,
                    DEBUG_ENABLED,
                );

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, &get_file_extension("markdown"))
                    .expect("it to create a file");

                assert!(handle_file(
                    &config,
                    file.path(),
                    DRY_RUN,
                    None,
                    TIMEOUT,
                    DEBUG_ENABLED
                ));

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        }
    }
}

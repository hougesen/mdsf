use core::sync::atomic::AtomicBool;

use config::MdsfConfig;
use error::MdsfError;
use formatters::format_snippet;
use languages::Language;

pub mod cli;
pub mod config;
pub mod error;
pub mod formatters;
pub mod languages;
mod runners;

#[cfg(test)]
pub static DEBUG: AtomicBool = AtomicBool::new(true);
#[cfg(not(test))]
pub static DEBUG: AtomicBool = AtomicBool::new(false);

#[inline]
fn write_unchanged_line(path: &std::path::Path, dur: core::time::Duration) {
    #[cfg(target_os = "windows")]
    let pre = "";
    #[cfg(not(target_os = "windows"))]
    let pre = "\u{1b}[2m";

    #[cfg(target_os = "windows")]
    let post = "";
    #[cfg(not(target_os = "windows"))]
    let post = "\u{1b}[0m";

    println!(
        "{pre}{} {}ms (unchanged){post}",
        path.display(),
        dur.as_millis()
    );
}

#[inline]
fn write_changed_line(path: &std::path::Path, dur: core::time::Duration) {
    println!("{} {}ms", path.display(), dur.as_millis());
}

#[inline]
pub fn format_file(config: &MdsfConfig, path: &std::path::Path) -> Result<(), MdsfError> {
    let time = std::time::Instant::now();

    let input = std::fs::read_to_string(path)?;

    if input.is_empty() {
        let duration = time.elapsed();
        write_unchanged_line(path, duration);
        return Ok(());
    }

    let mut output = String::with_capacity(input.len() + 128);

    let mut modified = false;

    let mut lines = input.lines().enumerate();

    while let Some((_, line)) = lines.next() {
        // TODO: implement support for code blocks with 4 `
        if line.starts_with("```") {
            if let Some(language) = line.strip_prefix("```").and_then(Language::maybe_from_str) {
                let mut code_snippet = String::new();

                let mut is_snippet = false;

                for (_, subline) in lines.by_ref() {
                    if subline.trim_end() == "```" {
                        is_snippet = true;
                        break;
                    }

                    code_snippet.push_str(subline);

                    code_snippet.push('\n');
                }

                if is_snippet {
                    let formatted = format_snippet(config, &language, &code_snippet);

                    output.push_str(line);
                    output.push('\n');
                    output.push_str(formatted.trim());
                    output.push_str("\n```");

                    if formatted != code_snippet {
                        modified = true;
                    }
                } else {
                    output.push_str(line);
                    output.push_str(&code_snippet);
                }
            } else {
                output.push_str(line);
            }
        } else {
            output.push_str(line);
        }

        output.push('\n');
    }

    if config.markdown.enabled && !output.is_empty() {
        output = format_snippet(config, &Language::Markdown, &output);
        modified = true;
    }

    let duration = time.elapsed();

    if modified && output != input {
        std::fs::write(path, output)?;

        write_changed_line(path, duration);

        return Ok(());
    }

    write_unchanged_line(path, duration);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{config::MdsfConfig, format_file, formatters::setup_snippet};

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

        let file = setup_snippet(input, ".md").expect("it to create a file");

        let config = MdsfConfig::default();

        format_file(&config, file.path()).expect("it to format the file without errors");

        let output = std::fs::read_to_string(file.path()).expect("it to read the file");

        assert_eq!(output, expected_output);
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

        let file = setup_snippet(input, ".md").expect("it to create a file");

        let config = MdsfConfig::default();

        format_file(&config, file.path()).expect("it to format the file without errors");

        let output = std::fs::read_to_string(file.path()).expect("it to read the file");

        assert_eq!(output, expected_output);
    }
}

use core::{iter::Enumerate, str::Lines};

use regex::Regex;

const GO_TEMPORARY_PACKAGE_NAME: &str = "package mdsfformattertemporarynamespace\n";

#[inline]
pub fn parse_generic_codeblock(
    lines: &mut Enumerate<Lines>,
    indentation: &str,
) -> (bool, String, usize) {
    let mut code_snippet = String::new();

    let mut is_snippet = false;

    let mut snippet_lines = 0;

    for (_, subline) in lines.by_ref() {
        snippet_lines += 1;

        if subline.trim() == "```" {
            is_snippet = true;
            break;
        }

        code_snippet.push_str(dedent_line(subline, indentation));

        code_snippet.push(crate::config::LF_NEWLINE_CHAR);
    }

    (is_snippet, code_snippet, snippet_lines)
}

#[inline]
pub fn parse_go_codeblock(
    lines: &mut Enumerate<Lines>,
    indentation: &str,
) -> (bool, String, usize) {
    let (is_snippet, mut code_snippet, snippet_lines) = parse_generic_codeblock(lines, indentation);

    if is_snippet && !GO_PACKAGE_RE.is_match(&code_snippet) {
        code_snippet.insert_str(0, GO_TEMPORARY_PACKAGE_NAME);
    }

    (is_snippet, code_snippet, snippet_lines)
}

// TODO: check for multiline comments
pub static GO_PACKAGE_RE: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"^\s*package\s+\w").unwrap());

#[inline]
pub fn remove_go_package(snippet: String) -> String {
    if snippet.contains(GO_TEMPORARY_PACKAGE_NAME) {
        snippet.replace(GO_TEMPORARY_PACKAGE_NAME, "")
    } else {
        snippet
    }
}

#[inline]
fn dedent_line<'a>(line: &'a str, indentation: &str) -> &'a str {
    if indentation.is_empty() {
        line
    } else {
        line.strip_prefix(indentation)
            .unwrap_or_else(|| line.trim_start())
    }
}

#[inline]
pub fn indent_codeblock(indentation: &str, snippet: String) -> String {
    if indentation.is_empty() {
        snippet
    } else {
        snippet
            .lines()
            .map(|line| {
                if line.is_empty() {
                    line.to_owned()
                } else {
                    format!("{indentation}{line}")
                }
            })
            .collect::<Vec<_>>()
            .join(crate::config::Newline::Lf.as_str())
    }
}

#[cfg(test)]
mod test_parse_generic_codeblock {
    use crate::parser::parse_generic_codeblock;

    #[test]
    fn it_should_remove_the_codeblock_indentation() {
        let input = "    a = 1\n\n    b = 2\n    ```";

        let (is_snippet, snippet, snippet_lines) =
            parse_generic_codeblock(&mut input.lines().enumerate(), "    ");

        assert!(is_snippet);
        assert_eq!("a = 1\n\nb = 2\n", snippet);
        assert_eq!(4, snippet_lines);
    }

    #[test]
    fn it_should_keep_lines_of_unindented_codeblocks() {
        let input = "a = 1\n  b = 2\n```";

        let (is_snippet, snippet, snippet_lines) =
            parse_generic_codeblock(&mut input.lines().enumerate(), "");

        assert!(is_snippet);
        assert_eq!("a = 1\n  b = 2\n", snippet);
        assert_eq!(3, snippet_lines);
    }
}

#[cfg(test)]
mod test_indent_codeblock {
    use crate::parser::indent_codeblock;

    #[test]
    fn it_should_indent_every_line() {
        assert_eq!(
            "    a = 1\n    b = 2",
            indent_codeblock("    ", "a = 1\nb = 2".to_owned())
        );
    }

    #[test]
    fn it_should_not_indent_empty_lines() {
        assert_eq!(
            "    a = 1\n\n    b = 2",
            indent_codeblock("    ", "a = 1\n\nb = 2".to_owned())
        );
    }
}

#[cfg(test)]
mod test_go_package_re {
    use crate::parser::GO_PACKAGE_RE;

    #[test]
    fn it_should_match() {
        for s in [
            "package\tmdsf",
            "package mdsf ",
            "  package mdsf",
            "\n package mdsf",
            "\n package mdsf\t",
            "\n package    mdsf",
            "\n package \tmdsf",
            "\n package\tmdsf",
            "\n \tpackage\t\n\nmdsf\n",
        ] {
            assert!(GO_PACKAGE_RE.is_match(s), "'{s}' did not match");
        }
    }

    #[test]
    fn it_should_not_match() {
        for s in [
            "packageasd",
            "missing pkg name",
            "// package mdsf ",
            "//package mdsf",
            "//\tpackage mdsf",
        ] {
            assert!(!GO_PACKAGE_RE.is_match(s), "'{s}' matched");
        }
    }
}

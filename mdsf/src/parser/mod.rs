const GO_TEMPORARY_PACKAGE_NAME: &str = "package mdsfformattertemporarynamespace\n";

#[inline]
pub fn add_mdsf_go_package(snippet: String) -> String {
    if GO_PACKAGE_RE.is_match(&snippet) {
        snippet
    } else {
        let mut snippet = snippet;
        snippet.insert_str(0, GO_TEMPORARY_PACKAGE_NAME);
        snippet
    }
}

// TODO: check for multiline comments
pub static GO_PACKAGE_RE: std::sync::LazyLock<regex::Regex> =
    std::sync::LazyLock::new(|| regex::Regex::new(r"^\s*package\s+\w").unwrap());

#[inline]
pub fn remove_mdsf_go_package(snippet: String) -> String {
    if snippet.contains(GO_TEMPORARY_PACKAGE_NAME) {
        snippet.replace(GO_TEMPORARY_PACKAGE_NAME, "")
    } else {
        snippet
    }
}

#[inline]
pub fn indent_codeblock(indentation: &str, snippet: String) -> String {
    if indentation.is_empty() {
        snippet
    } else {
        snippet
            .lines()
            .map(|line| format!("{indentation}{line}"))
            .collect::<Vec<_>>()
            .join(crate::config::Newline::Lf.as_str())
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

///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("reorder-python-imports"),
    CommandType::Uv("reorder-python-imports", "reorder-python-imports"),
    CommandType::Pipx("reorder-python-imports"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_reorder_python_imports {
    #[test_with::executable(reorder-python-imports || pipx || uv)]
    fn test_reorder_python_imports_python_8ddc1587af0094c1() {
        let input = r#"import sys
import pyramid
import reorder_python_imports"#;

        let output = r#"import sys

import pyramid
import reorder_python_imports
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::ReorderPythonImports.test_format_snippet(input, output, &file_ext);
    }
}

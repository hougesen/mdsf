///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg("--quiet");
    cmd.arg("--in-place");
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("autoflake"),
    CommandType::Uv("autoflake", "autoflake"),
    CommandType::Pipx("autoflake"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_autoflake {
    #[test_with::executable(autoflake || pipx || uv)]
    fn test_autoflake_python_27cfd9b948e80d7f() {
        let input = r#"import math
import re
import os
import random
import multiprocessing
import grp, pwd, platform
import subprocess, sys


def foo():
    from abc import ABCMeta, WeakSet
    try:
        import multiprocessing
        print(multiprocessing.cpu_count())
    except ImportError as exception:
        print(sys.version)
    return math.pi
"#;

        let output = r#"import math
import sys


def foo():
    try:
        import multiprocessing
        print(multiprocessing.cpu_count())
    except ImportError as exception:
        print(sys.version)
    return math.pi
"#;

        let file_ext = crate::fttype::get_file_extension("python");

        crate::tools::Tooling::Autoflake.test_format_snippet(input, output, &file_ext);
    }
}

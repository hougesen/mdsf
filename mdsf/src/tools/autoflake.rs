///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_autoflake_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--quiet");
    cmd.arg("--in-place");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("autoflake")];

    crate::execution::run_tools(&commands, file_path, timeout, set_autoflake_args)
}

#[cfg(test)]
mod test_autoflake {
    #[test_with::executable(autoflake)]
    fn test_autoflake_python_a676d36968f04ba0() {
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
        let output = Some(
            r#"import math
import sys


def foo():
    try:
        import multiprocessing
        print(multiprocessing.cpu_count())
    except ImportError as exception:
        print(sys.version)
    return math.pi
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("python");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::autoflake::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

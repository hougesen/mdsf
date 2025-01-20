///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_autoflake_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("--quiet");
    cmd.arg("--in-place");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("autoflake")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_autoflake_args(cmd.build(), file_path);
        let execution_result = execute_command(cmd, file_path);

        if index == commands.len() - 1 {
            return execution_result;
        }

        if let Ok(r) = execution_result {
            if !r.0 {
                return Ok(r);
            }
        }
    }

    Ok((true, None))
}

#[cfg(test)]
mod test_autoflake {
    #[test_with::executable(autoflake)]
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
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::autoflake::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}

use super::execute_command;
use crate::{error::MdsfError, runners::CommandType};

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = CommandType::Direct("autoflake").build();

    cmd.arg("--quiet").arg("--in-place").arg(file_path);

    execute_command(cmd, file_path)
}

#[cfg(test)]
mod test_autoflake {
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(autoflake)]
    fn it_should_format_python() {
        let input = "import math
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
";

        let expected_output = "import math
import sys


def foo():
    try:
        import multiprocessing
        print(multiprocessing.cpu_count())
    except ImportError as exception:
        print(sys.version)
    return math.pi
";

        let snippet =
            setup_snippet(input, language_to_ext("python")).expect("it to create a snippet file");

        let output = super::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

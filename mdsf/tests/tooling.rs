//!
//! THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
//!

#[allow(dead_code)]
mod common;

#[cfg(test)]
mod test_actionlint {
    #[test_with::executable(actionlint)]
    fn test_actionlint_yaml_da8378e9384e0b1f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"name: action
on: push
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let output = r#"name: action
on: push
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let ft = "yaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Actionlint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(actionlint)]
    fn test_custom_tool_actionlint_yaml_da8378e9384e0b1f() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"name: action
on: push
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let output = r#"name: action
on: push
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let ft = "yaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "actionlint".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_air_format {
    #[test_with::executable(air)]
    fn test_air_format_r_b395a8aabbe68c56() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"data            |>
                  select(foo)

  foo         <- function(bar         =                               1, baz=2)                                 {
   list(bar,                 baz)
 }

"#;

        let output = r#"data |>
  select(foo)

foo <- function(bar = 1, baz = 2) {
  list(bar, baz)
}
"#;

        let ft = "r";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::AirFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(air)]
    fn test_custom_tool_air_r_b395a8aabbe68c56() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"data            |>
                  select(foo)

  foo         <- function(bar         =                               1, baz=2)                                 {
   list(bar,                 baz)
 }

"#;

        let output = r#"data |>
  select(foo)

foo <- function(bar = 1, baz = 2) {
  list(bar, baz)
}
"#;

        let ft = "r";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "air".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_alejandra {
    #[test_with::executable(alejandra)]
    fn test_alejandra_nix_f38bff8f20c2aa02() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{
            lib, buildPythonPackage, fetchFromGitHub, redis }:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let output = r#"{
  lib,
  buildPythonPackage,
  fetchFromGitHub,
  redis,
}:
buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [redis];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [maintainers.globin];
  };
}
"#;

        let ft = "nix";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Alejandra),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(alejandra)]
    fn test_custom_tool_alejandra_nix_f38bff8f20c2aa02() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"{
            lib, buildPythonPackage, fetchFromGitHub, redis }:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let output = r#"{
  lib,
  buildPythonPackage,
  fetchFromGitHub,
  redis,
}:
buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [redis];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [maintainers.globin];
  };
}
"#;

        let ft = "nix";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "alejandra".to_owned(),
                arguments: vec!["--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_alex {
    #[test_with::executable(alex || bunx || deno || npx || pnpm || yarn)]
    fn test_alex_markdown_33c15403c156c629() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"hello
"#;

        let output = r#"hello
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Alex),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(alex)]
    fn test_custom_tool_alex_markdown_33c15403c156c629() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"hello
"#;

        let output = r#"hello
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "alex".to_owned(),
                arguments: vec!["--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_auto_optional {
    #[test_with::executable(auto-optional || pipx || uv)]
    fn test_auto_optional_python_c43199b18f48026d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def foo(bar: str = None):
    pass
"#;

        let output = r#"from typing import Optional
def foo(bar: Optional[str] = None):
    pass
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::AutoOptional),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(auto-optional)]
    fn test_custom_tool_auto_optional_python_c43199b18f48026d()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def foo(bar: str = None):
    pass
"#;

        let output = r#"from typing import Optional
def foo(bar: Optional[str] = None):
    pass
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "auto-optional".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_autoflake {
    #[test_with::executable(autoflake || pipx || uv)]
    fn test_autoflake_python_27cfd9b948e80d7f() -> Result<(), Box<dyn core::error::Error>> {
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

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Autoflake),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(autoflake)]
    fn test_custom_tool_autoflake_python_27cfd9b948e80d7f()
    -> Result<(), Box<dyn core::error::Error>> {
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

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "autoflake".to_owned(),
                arguments: vec![
                    "--quiet".to_owned(),
                    "--in-place".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_autopep_8 {
    #[test_with::executable(autopep8 || pipx || uv)]
    fn test_autopep_8_python_a868b5ad9905fc3f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int,  b: int) -> int: return a+b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Autopep8),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(autopep8)]
    fn test_custom_tool_autopep_8_python_a868b5ad9905fc3f()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int,  b: int) -> int: return a+b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "autopep8".to_owned(),
                arguments: vec!["--in-place".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_bashate {
    #[test_with::executable(bashate || pipx || uv)]
    fn test_bashate_bash_1f0c485b85eb22b1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"# for loop examples

# pass
for i in $(seq 1 5); do
    echo $i
done

# pass
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

# fail E010
for i in $(seq 1 5); do
    echo $i
done

# fail E010
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

awk '{
    for (i = 1; i < 5; i++)
        print $i
}' < /dev/null
"#;

        let output = r#"# for loop examples

# pass
for i in $(seq 1 5); do
    echo $i
done

# pass
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

# fail E010
for i in $(seq 1 5); do
    echo $i
done

# fail E010
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

awk '{
    for (i = 1; i < 5; i++)
        print $i
}' < /dev/null
"#;

        let ft = "bash";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Bashate),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(bashate)]
    fn test_custom_tool_bashate_bash_1f0c485b85eb22b1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"# for loop examples

# pass
for i in $(seq 1 5); do
    echo $i
done

# pass
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

# fail E010
for i in $(seq 1 5); do
    echo $i
done

# fail E010
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

awk '{
    for (i = 1; i < 5; i++)
        print $i
}' < /dev/null
"#;

        let output = r#"# for loop examples

# pass
for i in $(seq 1 5); do
    echo $i
done

# pass
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

# fail E010
for i in $(seq 1 5); do
    echo $i
done

# fail E010
for (( i = 0 ; i < 5 ; i++ )); do
    echo $i
done

awk '{
    for (i = 1; i < 5; i++)
        print $i
}' < /dev/null
"#;

        let ft = "bash";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "bashate".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_beautysh {
    #[test_with::executable(beautysh)]
    fn test_beautysh_bash_a6831a7ad31bd0a6() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"#!/bin/bash

       add() {
    echo "$1" + "$2"
             }
"#;

        let output = r#"#!/bin/bash

add() {
    echo "$1" + "$2"
}
"#;

        let ft = "bash";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Beautysh),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(beautysh)]
    fn test_custom_tool_beautysh_bash_a6831a7ad31bd0a6() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"#!/bin/bash

       add() {
    echo "$1" + "$2"
             }
"#;

        let output = r#"#!/bin/bash

add() {
    echo "$1" + "$2"
}
"#;

        let ft = "bash";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "beautysh".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(beautysh)]
    fn test_beautysh_shell_f8c934ee37e2888() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"#!/bin/shell

       add() {
    echo "$1" + "$2"
             }
"#;

        let output = r#"#!/bin/shell

add() {
    echo "$1" + "$2"
}
"#;

        let ft = "shell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Beautysh),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(beautysh)]
    fn test_custom_tool_beautysh_shell_f8c934ee37e2888() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"#!/bin/shell

       add() {
    echo "$1" + "$2"
             }
"#;

        let output = r#"#!/bin/shell

add() {
    echo "$1" + "$2"
}
"#;

        let ft = "shell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "beautysh".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_biome_check {
    #[test_with::executable(biome || bunx || deno || npx || pnpm || yarn)]
    fn test_biome_check_typescript_8154bfdbd3b72275() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a: number, b: number): Promise<number> {
	return a + b;
}
"#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::BiomeCheck),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(biome)]
    fn test_custom_tool_biome_typescript_8154bfdbd3b72275()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a: number, b: number): Promise<number> {
	return a + b;
}
"#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "biome".to_owned(),
                arguments: vec!["check".to_owned(), "--write".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_biome_check_unsafe {
    #[test_with::executable(biome || bunx || deno || npx || pnpm || yarn)]
    fn test_biome_check_unsafe_typescript_6cf769686a46e14b()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            "#;

        let output = r#"async function _asyncAddition(a: number, b: number): Promise<number> {
	return a + b;
}
"#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::BiomeCheckUnsafe),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(biome)]
    fn test_custom_tool_biome_typescript_6cf769686a46e14b()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            "#;

        let output = r#"async function _asyncAddition(a: number, b: number): Promise<number> {
	return a + b;
}
"#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "biome".to_owned(),
                arguments: vec![
                    "check".to_owned(),
                    "--write".to_owned(),
                    "--unsafe".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_biome_format {
    #[test_with::executable(biome || bunx || deno || npx || pnpm || yarn)]
    fn test_biome_format_javascript_4845e9b01c23667f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
	return a + b;
}
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::BiomeFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(biome)]
    fn test_custom_tool_biome_javascript_4845e9b01c23667f()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
	return a + b;
}
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "biome".to_owned(),
                arguments: vec![
                    "format".to_owned(),
                    "--write".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(biome || bunx || deno || npx || pnpm || yarn)]
    fn test_biome_format_json_90a326e29048e3cd() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
	"key": "value",
	"key2": ["value2", "value3", 1, null]
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::BiomeFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(biome)]
    fn test_custom_tool_biome_json_90a326e29048e3cd() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
	"key": "value",
	"key2": ["value2", "value3", 1, null]
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "biome".to_owned(),
                arguments: vec![
                    "format".to_owned(),
                    "--write".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(biome || bunx || deno || npx || pnpm || yarn)]
    fn test_biome_format_typescript_8154bfdbd3b72275() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a: number, b: number): Promise<number> {
	return a + b;
}
"#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::BiomeFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(biome)]
    fn test_custom_tool_biome_typescript_8154bfdbd3b72275()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a:number,b:number
        ) :Promise<
number>
    {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a: number, b: number): Promise<number> {
	return a + b;
}
"#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "biome".to_owned(),
                arguments: vec![
                    "format".to_owned(),
                    "--write".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_biome_lint {
    #[test_with::executable(biome || bunx || deno || npx || pnpm || yarn)]
    fn test_biome_lint_javascript_3b1c1d6fd9c2e176() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"let variable = 0;
"#;

        let output = r#"const variable = 0;
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::BiomeLint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(biome)]
    fn test_custom_tool_biome_javascript_3b1c1d6fd9c2e176()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"let variable = 0;
"#;

        let output = r#"const variable = 0;
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "biome".to_owned(),
                arguments: vec!["lint".to_owned(), "--write".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_biome_lint_unsafe {
    #[test_with::executable(biome || bunx || deno || npx || pnpm || yarn)]
    fn test_biome_lint_unsafe_javascript_9165f2e512bbc53f()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"const hello = "hello";
const world = "world";

console.log("" + hello + world);
"#;

        let output = r#"const hello = "hello";
const world = "world";

console.log(`${hello}${world}`);
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::BiomeLintUnsafe),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(biome)]
    fn test_custom_tool_biome_javascript_9165f2e512bbc53f()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"const hello = "hello";
const world = "world";

console.log("" + hello + world);
"#;

        let output = r#"const hello = "hello";
const world = "world";

console.log(`${hello}${world}`);
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "biome".to_owned(),
                arguments: vec![
                    "lint".to_owned(),
                    "--write".to_owned(),
                    "--unsafe".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_black {
    #[test_with::executable(black || pipx || uv)]
    fn test_black_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Black),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(black)]
    fn test_custom_tool_black_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "black".to_owned(),
                arguments: vec!["--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_blade_formatter {
    #[test_with::executable(blade-formatter || bunx || deno || npx || pnpm || yarn)]
    fn test_blade_formatter_blade_9ddeaf972bfb08c1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"@extends('frontend.layouts.app')
@section('title') foo
@endsection
@section('content')
<section id="content">
<div class="container mod-users-pd-h">
    <div class="pf-user-header">
    <div></div>
    <p>@lang('users.index')</p>
    </div>
        <div class="pf-users-branch">
            <ul class="pf-users-branch__list">
                @foreach($users as $user)
        <li>
            <img src="{{ asset('img/frontend/icon/branch-arrow.svg') }}" alt="branch_arrow">
            {{ link_to_route("frontend.users.user.show",$users["name"],$users['_id']) }}
        </li>
        @endforeach
      </ul>
      <div class="pf-users-branch__btn">
      @can('create', App\Models\User::class)
            {!! link_to_route("frontend.users.user.create",__('users.create'),[1,2,3],['class' => 'btn']) !!}
            @endcan
        </div>
  </div>
    </div>
</section>
@endsection
@section('footer')
@stop"#;

        let output = r#"@extends('frontend.layouts.app')
@section('title') foo
@endsection
@section('content')
    <section id="content">
        <div class="container mod-users-pd-h">
            <div class="pf-user-header">
                <div></div>
                <p>@lang('users.index')</p>
            </div>
            <div class="pf-users-branch">
                <ul class="pf-users-branch__list">
                    @foreach ($users as $user)
                        <li>
                            <img src="{{ asset('img/frontend/icon/branch-arrow.svg') }}" alt="branch_arrow">
                            {{ link_to_route('frontend.users.user.show', $users['name'], $users['_id']) }}
                        </li>
                    @endforeach
                </ul>
                <div class="pf-users-branch__btn">
                    @can('create', App\Models\User::class)
                        {!! link_to_route('frontend.users.user.create', __('users.create'), [1, 2, 3], ['class' => 'btn']) !!}
                    @endcan
                </div>
            </div>
        </div>
    </section>
@endsection
@section('footer')
@stop
"#;

        let ft = "blade";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::BladeFormatter),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(blade-formatter)]
    fn test_custom_tool_blade_formatter_blade_9ddeaf972bfb08c1()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"@extends('frontend.layouts.app')
@section('title') foo
@endsection
@section('content')
<section id="content">
<div class="container mod-users-pd-h">
    <div class="pf-user-header">
    <div></div>
    <p>@lang('users.index')</p>
    </div>
        <div class="pf-users-branch">
            <ul class="pf-users-branch__list">
                @foreach($users as $user)
        <li>
            <img src="{{ asset('img/frontend/icon/branch-arrow.svg') }}" alt="branch_arrow">
            {{ link_to_route("frontend.users.user.show",$users["name"],$users['_id']) }}
        </li>
        @endforeach
      </ul>
      <div class="pf-users-branch__btn">
      @can('create', App\Models\User::class)
            {!! link_to_route("frontend.users.user.create",__('users.create'),[1,2,3],['class' => 'btn']) !!}
            @endcan
        </div>
  </div>
    </div>
</section>
@endsection
@section('footer')
@stop"#;

        let output = r#"@extends('frontend.layouts.app')
@section('title') foo
@endsection
@section('content')
    <section id="content">
        <div class="container mod-users-pd-h">
            <div class="pf-user-header">
                <div></div>
                <p>@lang('users.index')</p>
            </div>
            <div class="pf-users-branch">
                <ul class="pf-users-branch__list">
                    @foreach ($users as $user)
                        <li>
                            <img src="{{ asset('img/frontend/icon/branch-arrow.svg') }}" alt="branch_arrow">
                            {{ link_to_route('frontend.users.user.show', $users['name'], $users['_id']) }}
                        </li>
                    @endforeach
                </ul>
                <div class="pf-users-branch__btn">
                    @can('create', App\Models\User::class)
                        {!! link_to_route('frontend.users.user.create', __('users.create'), [1, 2, 3], ['class' => 'btn']) !!}
                    @endcan
                </div>
            </div>
        </div>
    </section>
@endsection
@section('footer')
@stop
"#;

        let ft = "blade";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "blade-formatter".to_owned(),
                arguments: vec!["--write".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_blue {
    #[test_with::executable(blue || pipx || uv)]
    fn test_blue_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Blue),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(blue)]
    fn test_custom_tool_blue_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "blue".to_owned(),
                arguments: vec!["--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_brunette {
    #[test_with::executable(brunette || pipx || uv)]
    fn test_brunette_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Brunette),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(brunette)]
    fn test_custom_tool_brunette_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "brunette".to_owned(),
                arguments: vec!["--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_buf_format {
    #[test_with::executable(buf || bunx || deno || npx || pnpm || yarn)]
    fn test_buf_format_protobuf_10af516c8a015ab5() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }"#;

        let output = r#"service SearchService {
  rpc Search(SearchRequest) returns (SearchResponse);
}
"#;

        let ft = "protobuf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::BufFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(buf)]
    fn test_custom_tool_buf_protobuf_10af516c8a015ab5() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }"#;

        let output = r#"service SearchService {
  rpc Search(SearchRequest) returns (SearchResponse);
}
"#;

        let ft = "protobuf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "buf".to_owned(),
                arguments: vec![
                    "format".to_owned(),
                    "--write".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_cabal_format {
    #[test_with::executable(cabal)]
    fn test_cabal_format_cabal_38e9e2aad5619a6a() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"cabal-version: 2.4
name: mdsf
version: 0

executable msdf
    default-language: Haskell2010
    hs-source-dirs: src
    main-is: Mdsf.hs
    build-depends: base >=4.11 && <4.13, pretty >=1.1.3.6 && <1.2, bytestring, Cabal ^>=2.5, containers ^>=0.5.11.0 || ^>=0.6.0.1
    other-extensions:
      DeriveFunctor FlexibleContexts ExistentialQuantification OverloadedStrings
      RankNTypes"#;

        let output = r#"cabal-version: 2.4
name:          mdsf
version:       0

executable msdf
    main-is:          Mdsf.hs
    hs-source-dirs:   src
    default-language: Haskell2010
    other-extensions:
        DeriveFunctor FlexibleContexts ExistentialQuantification
        OverloadedStrings RankNTypes

    build-depends:
        base >=4.11 && <4.13,
        pretty >=1.1.3.6 && <1.2,
        bytestring,
        Cabal ^>=2.5,
        containers ^>=0.5.11.0 || ^>=0.6.0.1
"#;

        let ft = "cabal";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::CabalFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(cabal)]
    fn test_custom_tool_cabal_cabal_38e9e2aad5619a6a() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"cabal-version: 2.4
name: mdsf
version: 0

executable msdf
    default-language: Haskell2010
    hs-source-dirs: src
    main-is: Mdsf.hs
    build-depends: base >=4.11 && <4.13, pretty >=1.1.3.6 && <1.2, bytestring, Cabal ^>=2.5, containers ^>=0.5.11.0 || ^>=0.6.0.1
    other-extensions:
      DeriveFunctor FlexibleContexts ExistentialQuantification OverloadedStrings
      RankNTypes"#;

        let output = r#"cabal-version: 2.4
name:          mdsf
version:       0

executable msdf
    main-is:          Mdsf.hs
    hs-source-dirs:   src
    default-language: Haskell2010
    other-extensions:
        DeriveFunctor FlexibleContexts ExistentialQuantification
        OverloadedStrings RankNTypes

    build-depends:
        base >=4.11 && <4.13,
        pretty >=1.1.3.6 && <1.2,
        bytestring,
        Cabal ^>=2.5,
        containers ^>=0.5.11.0 || ^>=0.6.0.1
"#;

        let ft = "cabal";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "cabal".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_clang_format {
    #[test_with::executable(clang-format || pipx || uv)]
    fn test_clang_format_c_bb10810bd7d8a71() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"int add(int a,int b){
                a-b;
       return a + b;
    }"#;

        let output = r#"int add(int a, int b) {
  a - b;
  return a + b;
}"#;

        let ft = "c";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ClangFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format)]
    fn test_custom_tool_clang_format_c_bb10810bd7d8a71() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"int add(int a,int b){
                a-b;
       return a + b;
    }"#;

        let output = r#"int add(int a, int b) {
  a - b;
  return a + b;
}"#;

        let ft = "c";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "clang-format".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format || pipx || uv)]
    fn test_clang_format_cpp_8a39c61364dbbe50() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"int add(int a,int b){
                 a-b;
       return a + b;
    }"#;

        let output = r#"int add(int a, int b) {
  a - b;
  return a + b;
}"#;

        let ft = "cpp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ClangFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format)]
    fn test_custom_tool_clang_format_cpp_8a39c61364dbbe50()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"int add(int a,int b){
                 a-b;
       return a + b;
    }"#;

        let output = r#"int add(int a, int b) {
  a - b;
  return a + b;
}"#;

        let ft = "cpp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "clang-format".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format || pipx || uv)]
    fn test_clang_format_csharp_8ebf20c1ddcd1aeb() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                a-b ;
                                                        return a + b;
                                                    }
                                                 }
                                                 } "#;

        let output = r#"namespace Mdsf {
class Adder {
  public static int add(int a, int b) {
    a - b;
    return a + b;
  }
}
}"#;

        let ft = "csharp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ClangFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format)]
    fn test_custom_tool_clang_format_csharp_8ebf20c1ddcd1aeb()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                a-b ;
                                                        return a + b;
                                                    }
                                                 }
                                                 } "#;

        let output = r#"namespace Mdsf {
class Adder {
  public static int add(int a, int b) {
    a - b;
    return a + b;
  }
}
}"#;

        let ft = "csharp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "clang-format".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format || pipx || uv)]
    fn test_clang_format_java_c4fcc280a3a8aac0() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"class HelloWorld {
    public static void main(String[] args) {
                System.out.println("Hello");
                System.out.println("World!");
                 }
}"#;

        let output = r#"class HelloWorld {
  public static void main(String[] args) {
    System.out.println("Hello");
    System.out.println("World!");
  }
}"#;

        let ft = "java";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ClangFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format)]
    fn test_custom_tool_clang_format_java_c4fcc280a3a8aac0()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"class HelloWorld {
    public static void main(String[] args) {
                System.out.println("Hello");
                System.out.println("World!");
                 }
}"#;

        let output = r#"class HelloWorld {
  public static void main(String[] args) {
    System.out.println("Hello");
    System.out.println("World!");
  }
}"#;

        let ft = "java";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "clang-format".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format || pipx || uv)]
    fn test_clang_format_javascript_d6184d76490772e9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"    async function asyncAddition(  a,b) {
            a * b;
        return a+b
    }            "#;

        let output = r#"async function asyncAddition(a, b) {
  a * b;
  return a + b
}"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ClangFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format)]
    fn test_custom_tool_clang_format_javascript_d6184d76490772e9()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"    async function asyncAddition(  a,b) {
            a * b;
        return a+b
    }            "#;

        let output = r#"async function asyncAddition(a, b) {
  a * b;
  return a + b
}"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "clang-format".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format || pipx || uv)]
    fn test_clang_format_json_574b008e140f1be6() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"              {
              "key": "value",
  "key2": ["value2", "value3", 1            , null]
 }  "#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ClangFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format)]
    fn test_custom_tool_clang_format_json_574b008e140f1be6()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"              {
              "key": "value",
  "key2": ["value2", "value3", 1            , null]
 }  "#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "clang-format".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format || pipx || uv)]
    fn test_clang_format_objective_c_3d56455568c6e83f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"int add(int a,int b){
            a - a ;
       return a + b;
    }"#;

        let output = r#"int add(int a, int b) {
  a - a;
  return a + b;
}"#;

        let ft = "objective-c";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ClangFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format)]
    fn test_custom_tool_clang_format_objective_c_3d56455568c6e83f()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"int add(int a,int b){
            a - a ;
       return a + b;
    }"#;

        let output = r#"int add(int a, int b) {
  a - a;
  return a + b;
}"#;

        let ft = "objective-c";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "clang-format".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format || pipx || uv)]
    fn test_clang_format_protobuf_7be6def196942f83() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }"#;

        let output =
            r#"service SearchService { rpc Search(SearchRequest) returns (SearchResponse); }"#;

        let ft = "protobuf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ClangFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(clang-format)]
    fn test_custom_tool_clang_format_protobuf_7be6def196942f83()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"service SearchService {
                              rpc Search (SearchRequest) returns (SearchResponse);
                               }"#;

        let output =
            r#"service SearchService { rpc Search(SearchRequest) returns (SearchResponse); }"#;

        let ft = "protobuf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "clang-format".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_cljstyle {
    #[test_with::executable(cljstyle)]
    fn test_cljstyle_clojure_92fbb2f42ebeeb2e() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"(  ns
 foo.bar.baz  "some doc"
    (:require (foo.bar [abc :as abc]
        def))
    (:use foo.bar.qux)
    (:import foo.bar.qux.Foo
      ;; Need this for the thing
      foo.bar.qux.Bar)
    )

(defn hello "says hi" (
      [] (hello "world")
  ) ([name]
  ( println "Hello," name  )
  ))"#;

        let output = r#"(ns foo.bar.baz
  "some doc"
  (:require
    [foo.bar.abc :as abc]
    [foo.bar.def]
    [foo.bar.qux :refer :all])
  (:import
    (foo.bar.qux
      ;; Need this for the thing
      Bar
      Foo)))


(defn hello
  "says hi"
  ([] (hello "world"))
  ([name]
   (println "Hello," name)))
"#;

        let ft = "clojure";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Cljstyle),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(cljstyle)]
    fn test_custom_tool_cljstyle_clojure_92fbb2f42ebeeb2e()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"(  ns
 foo.bar.baz  "some doc"
    (:require (foo.bar [abc :as abc]
        def))
    (:use foo.bar.qux)
    (:import foo.bar.qux.Foo
      ;; Need this for the thing
      foo.bar.qux.Bar)
    )

(defn hello "says hi" (
      [] (hello "world")
  ) ([name]
  ( println "Hello," name  )
  ))"#;

        let output = r#"(ns foo.bar.baz
  "some doc"
  (:require
    [foo.bar.abc :as abc]
    [foo.bar.def]
    [foo.bar.qux :refer :all])
  (:import
    (foo.bar.qux
      ;; Need this for the thing
      Bar
      Foo)))


(defn hello
  "says hi"
  ([] (hello "world"))
  ([name]
   (println "Hello," name)))
"#;

        let ft = "clojure";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "cljstyle".to_owned(),
                arguments: vec!["fix".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_coffeelint {
    #[test_with::executable(coffeelint || bunx || deno || npx || pnpm || yarn)]
    fn test_coffeelint_coffeescript_7b620f6d6e2ab16d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"add = (a, b) -> a + b"#;

        let output = r#"add = (a, b) -> a + b"#;

        let ft = "coffeescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Coffeelint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(coffeelint)]
    fn test_custom_tool_coffeelint_coffeescript_7b620f6d6e2ab16d()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"add = (a, b) -> a + b"#;

        let output = r#"add = (a, b) -> a + b"#;

        let ft = "coffeescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "coffeelint".to_owned(),
                arguments: vec!["-q".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_cppcheck {
    #[test_with::executable(cppcheck)]
    fn test_cppcheck_cpp_fd936e483242a65d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"int add(int a, int b) { return a + b; }
"#;

        let output = r#"int add(int a, int b) { return a + b; }
"#;

        let ft = "cpp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Cppcheck),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(cppcheck)]
    fn test_custom_tool_cppcheck_cpp_fd936e483242a65d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"int add(int a, int b) { return a + b; }
"#;

        let output = r#"int add(int a, int b) { return a + b; }
"#;

        let ft = "cpp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "cppcheck".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_cpplint {
    #[test_with::executable(cpplint || pipx || uv)]
    fn test_cpplint_cpp_5edac26b16656f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"// Copyright 2025 Mads Hougesen
int add(int a, int b) { return a + b; }
"#;

        let output = r#"// Copyright 2025 Mads Hougesen
int add(int a, int b) { return a + b; }
"#;

        let ft = "cpp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Cpplint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(cpplint)]
    fn test_custom_tool_cpplint_cpp_5edac26b16656f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"// Copyright 2025 Mads Hougesen
int add(int a, int b) { return a + b; }
"#;

        let output = r#"// Copyright 2025 Mads Hougesen
int add(int a, int b) { return a + b; }
"#;

        let ft = "cpp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "cpplint".to_owned(),
                arguments: vec!["--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_crystal_format {
    #[test_with::executable(crystal)]
    fn test_crystal_format_crystal_e0f2d532cd984bee() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add(a, b)  return a + b end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let ft = "crystal";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::CrystalFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(crystal)]
    fn test_custom_tool_crystal_crystal_e0f2d532cd984bee() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"def add(a, b)  return a + b end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let ft = "crystal";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "crystal".to_owned(),
                arguments: vec!["tool".to_owned(), "format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_csharpier {
    #[test_with::executable(csharpier)]
    fn test_csharpier_csharp_a79aa94ad2d86b6c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                var c=a+b ;
                                                        return c ;
                                                    }
                                                 }
                                                 } "#;

        let output = r#"namespace Mdsf
{
    class Adder
    {
        public static int add(int a, int b)
        {
            var c = a + b;
            return c;
        }
    }
}
"#;

        let ft = "csharp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Csharpier),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(csharpier)]
    fn test_custom_tool_csharpier_csharp_a79aa94ad2d86b6c()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"namespace Mdsf {
                        class Adder {
                                                    public static int add(int a,int b) {
                                var c=a+b ;
                                                        return c ;
                                                    }
                                                 }
                                                 } "#;

        let output = r#"namespace Mdsf
{
    class Adder
    {
        public static int add(int a, int b)
        {
            var c = a + b;
            return c;
        }
    }
}
"#;

        let ft = "csharp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "csharpier".to_owned(),
                arguments: vec!["format".to_owned(), "--write-stdout".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_css_beautify {
    #[test_with::executable(css-beautify || bunx || deno || npx || pnpm || yarn)]
    fn test_css_beautify_css_5ad41f26f69aea3e() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"h1   {color: blue;} p    {color: red;}"#;

        let output = r#"h1 {
    color: blue;
}

p {
    color: red;
}"#;

        let ft = "css";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::CssBeautify),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(css-beautify)]
    fn test_custom_tool_css_beautify_css_5ad41f26f69aea3e()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"h1   {color: blue;} p    {color: red;}"#;

        let output = r#"h1 {
    color: blue;
}

p {
    color: red;
}"#;

        let ft = "css";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "css-beautify".to_owned(),
                arguments: vec![
                    "-r".to_owned(),
                    "--type".to_owned(),
                    "css".to_owned(),
                    "-f".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_csscomb {
    #[test_with::executable(csscomb || bunx || deno || npx || pnpm || yarn)]
    fn test_csscomb_css_bed67a883a4a1aae() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"h1   {color: blue;}
p {color: red;}"#;

        let output = r#"h1
{
    color: blue;
}
p
{
    color: red;
}
"#;

        let ft = "css";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Csscomb),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(csscomb)]
    fn test_custom_tool_csscomb_css_bed67a883a4a1aae() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"h1   {color: blue;}
p {color: red;}"#;

        let output = r#"h1
{
    color: blue;
}
p
{
    color: red;
}
"#;

        let ft = "css";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "csscomb".to_owned(),
                arguments: vec!["-t".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_csslint {
    #[test_with::executable(csslint || bunx || deno || npx || pnpm || yarn)]
    fn test_csslint_css_9b7fd0554eb344f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"body {
  background: red;
}
"#;

        let output = r#"body {
  background: red;
}
"#;

        let ft = "css";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Csslint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(csslint)]
    fn test_custom_tool_csslint_css_9b7fd0554eb344f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"body {
  background: red;
}
"#;

        let output = r#"body {
  background: red;
}
"#;

        let ft = "css";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "csslint".to_owned(),
                arguments: vec!["--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_dart_format {
    #[test_with::executable(dart)]
    fn test_dart_format_dart_1e68d7619b4be391() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"class Adder {   int add(int a, int b) {     return a + b;   } }    "#;

        let output = r#"class Adder {
  int add(int a, int b) {
    return a + b;
  }
}
"#;

        let ft = "dart";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::DartFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(dart)]
    fn test_custom_tool_dart_dart_1e68d7619b4be391() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"class Adder {   int add(int a, int b) {     return a + b;   } }    "#;

        let output = r#"class Adder {
  int add(int a, int b) {
    return a + b;
  }
}
"#;

        let ft = "dart";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "dart".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_deno_fmt {
    #[test_with::executable(deno)]
    fn test_deno_fmt_javascript_d7445fa122fcd5cc() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(a,b){
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
  return a + b;
}
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::DenoFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(deno)]
    fn test_custom_tool_deno_javascript_d7445fa122fcd5cc() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"
    async function asyncAddition(a,b){
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
  return a + b;
}
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "deno".to_owned(),
                arguments: vec!["fmt".to_owned(), "--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(deno)]
    fn test_deno_fmt_json_d426a9ade74002d2() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::DenoFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(deno)]
    fn test_custom_tool_deno_json_d426a9ade74002d2() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "deno".to_owned(),
                arguments: vec!["fmt".to_owned(), "--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(deno)]
    fn test_deno_fmt_typescript_857476c85438ce71() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(                                a:       	number,b:number ) :Promise< number>
    {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a: number, b: number): Promise<number> {
  return a + b;
}
"#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::DenoFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(deno)]
    fn test_custom_tool_deno_typescript_857476c85438ce71() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"
    async function asyncAddition(                                a:       	number,b:number ) :Promise< number>
    {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a: number, b: number): Promise<number> {
  return a + b;
}
"#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "deno".to_owned(),
                arguments: vec!["fmt".to_owned(), "--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_dfmt {
    #[test_with::executable(dfmt || dub)]
    fn test_dfmt_d_768f677c0817bc61() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"int add(int a,int b){return a + b;}
"#;

        let output = r#"int add(int a, int b)
{
    return a + b;
}
"#;

        let ft = "d";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Dfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(dfmt)]
    fn test_custom_tool_dfmt_d_768f677c0817bc61() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"int add(int a,int b){return a + b;}
"#;

        let output = r#"int add(int a, int b)
{
    return a + b;
}
"#;

        let ft = "d";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "dfmt".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_dockerfmt {
    #[test_with::executable(dockerfmt)]
    fn test_dockerfmt_dockerfile_39458badf1770ecf() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"FROM          ubuntu:latest
 RUN   echo   "Hello world"
"#;

        let output = r#"FROM ubuntu:latest
RUN echo "Hello world"
"#;

        let ft = "dockerfile";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Dockerfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(dockerfmt)]
    fn test_custom_tool_dockerfmt_dockerfile_39458badf1770ecf()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"FROM          ubuntu:latest
 RUN   echo   "Hello world"
"#;

        let output = r#"FROM ubuntu:latest
RUN echo "Hello world"
"#;

        let ft = "dockerfile";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "dockerfmt".to_owned(),
                arguments: vec!["-w".to_owned(), "-n".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_dune_format_dune_file {
    #[test_with::executable(dune)]
    fn test_dune_format_dune_file_dune_455f0e69457a3786() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"(executable
           (name asd))"#;

        let output = r#"(executable
 (name asd))"#;

        let ft = "dune";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::DuneFormatDuneFile),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(dune)]
    fn test_custom_tool_dune_dune_455f0e69457a3786() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"(executable
           (name asd))"#;

        let output = r#"(executable
 (name asd))"#;

        let ft = "dune";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "dune".to_owned(),
                arguments: vec!["format-dune-file".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_dx_fmt {
    #[test_with::executable(dx)]
    fn test_dx_fmt_rust_c07936252118b5c6() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"fn add(a:i32,b:i32)->i32{a+b}"#;

        let output = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let ft = "rust";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::DxFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(dx)]
    fn test_custom_tool_dx_rust_c07936252118b5c6() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"fn add(a:i32,b:i32)->i32{a+b}"#;

        let output = r#"fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let ft = "rust";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "dx".to_owned(),
                arguments: vec![
                    "fmt".to_owned(),
                    "--all-code".to_owned(),
                    "--file".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_efmt {
    #[test_with::executable(efmt)]
    fn test_efmt_erlang_d4d88e49805fdb39() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
."#;

        let output = r#"what_is(Erlang) ->
    case Erlang of movie -> [hello(mike, joe, robert), credits]; language -> formatting_arguments end.
"#;

        let ft = "erlang";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Efmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(efmt)]
    fn test_custom_tool_efmt_erlang_d4d88e49805fdb39() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
."#;

        let output = r#"what_is(Erlang) ->
    case Erlang of movie -> [hello(mike, joe, robert), credits]; language -> formatting_arguments end.
"#;

        let ft = "erlang";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "efmt".to_owned(),
                arguments: vec!["-w".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_elm_format {
    #[test_with::executable(elm-format || bunx || deno || npx || pnpm || yarn)]
    fn test_elm_format_elm_4e120501af0177c4() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"import   Html       exposing   (text)


main =
      text              "Hello!"


  "#;

        let output = r#"module Main exposing (main)

import Html exposing (text)


main =
    text "Hello!"
"#;

        let ft = "elm";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ElmFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(elm-format)]
    fn test_custom_tool_elm_format_elm_4e120501af0177c4() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"import   Html       exposing   (text)


main =
      text              "Hello!"


  "#;

        let output = r#"module Main exposing (main)

import Html exposing (text)


main =
    text "Hello!"
"#;

        let ft = "elm";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "elm-format".to_owned(),
                arguments: vec![
                    "--elm-version=0.19".to_owned(),
                    "--yes".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_erg_lint {
    #[test_with::executable(erg)]
    fn test_erg_lint_erg_802e6b757d972583() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"add(a, b) = a + b

print!(add(1, 2))
"#;

        let output = r#"add(a, b) = a + b

print!(add(1, 2))
"#;

        let ft = ".erg";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ErgLint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(erg)]
    fn test_custom_tool_erg_erg_802e6b757d972583() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"add(a, b) = a + b

print!(add(1, 2))
"#;

        let output = r#"add(a, b) = a + b

print!(add(1, 2))
"#;

        let ft = ".erg";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "erg".to_owned(),
                arguments: vec!["lint".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_erlfmt {
    #[test_with::executable(erlfmt)]
    fn test_erlfmt_erlang_61f4ac26ad7484d2() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
."#;

        let output = r#"what_is(Erlang) ->
    case Erlang of
        movie -> [hello(mike, joe, robert), credits];
        language -> no_more_formatting_arguments
    end."#;

        let ft = "erlang";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Erlfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(erlfmt)]
    fn test_custom_tool_erlfmt_erlang_61f4ac26ad7484d2() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"what_is(Erlang) ->
case Erlang of movie->[hello(mike,joe,robert),credits]; language->formatting_arguments end
."#;

        let output = r#"what_is(Erlang) ->
    case Erlang of
        movie -> [hello(mike, joe, robert), credits];
        language -> no_more_formatting_arguments
    end."#;

        let ft = "erlang";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "erlfmt".to_owned(),
                arguments: vec!["-w".to_owned(), "'$PATH'".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_fantomas {
    #[test_with::executable(fantomas)]
    fn test_fantomas_fsharp_f3cb7f290d0660d3() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
let add a b  =  a +  b
            "#;

        let output = r#"let add a b = a + b
"#;

        let ft = "fsharp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Fantomas),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(fantomas)]
    fn test_custom_tool_fantomas_fsharp_f3cb7f290d0660d3() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"
let add a b  =  a +  b
            "#;

        let output = r#"let add a b = a + b
"#;

        let ft = "fsharp";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "fantomas".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_fixjson {
    #[test_with::executable(fixjson || bunx || deno || npx || pnpm || yarn)]
    fn test_fixjson_json_115ca7a7d8b2cc2b() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{     "fixjson": "fixjson"  }"#;

        let output = r#"{
  "fixjson": "fixjson"
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Fixjson),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(fixjson)]
    fn test_custom_tool_fixjson_json_115ca7a7d8b2cc2b() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{     "fixjson": "fixjson"  }"#;

        let output = r#"{
  "fixjson": "fixjson"
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "fixjson".to_owned(),
                arguments: vec!["-w".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_fortitude_check {
    #[test_with::executable(fortitude || pipx || uv)]
    fn test_fortitude_check_f_90_3b0b8d0e32ad7855() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let output = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let ft = ".f90";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::FortitudeCheck),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(fortitude)]
    fn test_custom_tool_fortitude_f_90_3b0b8d0e32ad7855() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let output = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let ft = ".f90";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "fortitude".to_owned(),
                arguments: vec![
                    "check".to_owned(),
                    "--quiet".to_owned(),
                    "--no-respect-gitignore".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_fortitude_check_fix {
    #[test_with::executable(fortitude || pipx || uv)]
    fn test_fortitude_check_fix_f_90_3b0b8d0e32ad7855() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let output = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let ft = ".f90";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::FortitudeCheckFix),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(fortitude)]
    fn test_custom_tool_fortitude_f_90_3b0b8d0e32ad7855() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let output = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let ft = ".f90";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "fortitude".to_owned(),
                arguments: vec![
                    "check".to_owned(),
                    "--quiet".to_owned(),
                    "--no-respect-gitignore".to_owned(),
                    "--fix".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_fortitude_check_fix_unsafe {
    #[test_with::executable(fortitude || pipx || uv)]
    fn test_fortitude_check_fix_unsafe_f_90_4107850c5b247cb5()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"program example
    implicit none

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let output = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let ft = ".f90";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::FortitudeCheckFixUnsafe),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(fortitude)]
    fn test_custom_tool_fortitude_f_90_4107850c5b247cb5() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"program example
    implicit none

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let output = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example
"#;

        let ft = ".f90";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "fortitude".to_owned(),
                arguments: vec![
                    "check".to_owned(),
                    "--quiet".to_owned(),
                    "--no-respect-gitignore".to_owned(),
                    "--fix".to_owned(),
                    "--unsafe-fixes".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_fortran_linter {
    #[test_with::executable(fortran-linter || pipx || uv)]
    fn test_fortran_linter_f_90_a4a8950ee39644a8() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example"#;

        let output = r#"program example
implicit none (type, external)

contains
integer function addnum(a, b)
    integer, intent(in) :: a, b
    return a + b
end function addnum

end program example
"#;

        let ft = ".f90";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::FortranLinter),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(fortran-linter)]
    fn test_custom_tool_fortran_linter_f_90_a4a8950ee39644a8()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"program example
    implicit none (type, external)

    contains
        integer function addnum(a, b)
            integer, intent(in) :: a, b
            return a + b
        end function addnum

end program example"#;

        let output = r#"program example
implicit none (type, external)

contains
integer function addnum(a, b)
    integer, intent(in) :: a, b
    return a + b
end function addnum

end program example
"#;

        let ft = ".f90";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "fortran-linter".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_fourmolu {
    #[test_with::executable(fourmolu)]
    fn test_fourmolu_haskell_718612a8aa064d19() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers :: Int -> Int -> Int
addNumbers a b = do
    a + b
"#;

        let ft = "haskell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Fourmolu),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(fourmolu)]
    fn test_custom_tool_fourmolu_haskell_718612a8aa064d19()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers :: Int -> Int -> Int
addNumbers a b = do
    a + b
"#;

        let ft = "haskell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "fourmolu".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_fprettify {
    #[test_with::executable(fprettify || pipx || uv)]
    fn test_fprettify_fortran_e500b54621ef1a7a() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"program demo
integer :: endif,if,elseif
integer,DIMENSION(2) :: function
endif=3;if=2
if(endif==2)then
endif=5
elseif=if+4*(endif+&
2**10)
elseif(endif==3)then
function(if)=endif/elseif
print*,endif
endif
end program"#;

        let output = r#"program demo
   integer :: endif, if, elseif
   integer, DIMENSION(2) :: function
   endif = 3; if = 2
   if (endif == 2) then
      endif = 5
      elseif = if + 4*(endif + &
                       2**10)
   elseif (endif == 3) then
      function(if) = endif/elseif
      print *, endif
   end if
end program
"#;

        let ft = "fortran";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Fprettify),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(fprettify)]
    fn test_custom_tool_fprettify_fortran_e500b54621ef1a7a()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"program demo
integer :: endif,if,elseif
integer,DIMENSION(2) :: function
endif=3;if=2
if(endif==2)then
endif=5
elseif=if+4*(endif+&
2**10)
elseif(endif==3)then
function(if)=endif/elseif
print*,endif
endif
end program"#;

        let output = r#"program demo
   integer :: endif, if, elseif
   integer, DIMENSION(2) :: function
   endif = 3; if = 2
   if (endif == 2) then
      endif = 5
      elseif = if + 4*(endif + &
                       2**10)
   elseif (endif == 3) then
      function(if) = endif/elseif
      print *, endif
   end if
end program
"#;

        let ft = "fortran";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "fprettify".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_fvm_dart_format {
    #[test_with::executable(fvm)]
    fn test_fvm_dart_format_dart_1e68d7619b4be391() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"class Adder {   int add(int a, int b) {     return a + b;   } }    "#;

        let output = r#"class Adder {
  int add(int a, int b) {
    return a + b;
  }
}
"#;

        let ft = "dart";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::FvmDartFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(fvm)]
    fn test_custom_tool_fvm_dart_1e68d7619b4be391() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"class Adder {   int add(int a, int b) {     return a + b;   } }    "#;

        let output = r#"class Adder {
  int add(int a, int b) {
    return a + b;
  }
}
"#;

        let ft = "dart";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "fvm".to_owned(),
                arguments: vec!["dart".to_owned(), "format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_gleam_format {
    #[test_with::executable(gleam)]
    fn test_gleam_format_gleam_d23656d11ef3a81d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"pub fn add(a:Int,b:Int)->Int{a+b}"#;

        let output = r#"pub fn add(a: Int, b: Int) -> Int {
  a + b
}
"#;

        let ft = "gleam";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::GleamFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(gleam)]
    fn test_custom_tool_gleam_gleam_d23656d11ef3a81d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"pub fn add(a:Int,b:Int)->Int{a+b}"#;

        let output = r#"pub fn add(a: Int, b: Int) -> Int {
  a + b
}
"#;

        let ft = "gleam";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "gleam".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_gofmt {
    #[test_with::executable(gofmt)]
    fn test_gofmt_go_3b56f602fe22977b() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"package main

   func add(a int , b int  ) int {
                return a + b
       }

    "#;

        let output = r#"package main

func add(a int, b int) int {
	return a + b
}
"#;

        let ft = "go";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Gofmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(gofmt)]
    fn test_custom_tool_gofmt_go_3b56f602fe22977b() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"package main

   func add(a int , b int  ) int {
                return a + b
       }

    "#;

        let output = r#"package main

func add(a int, b int) int {
	return a + b
}
"#;

        let ft = "go";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "gofmt".to_owned(),
                arguments: vec!["-w".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_gofumpt {
    #[test_with::executable(gofumpt)]
    fn test_gofumpt_go_3b56f602fe22977b() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"package main

   func add(a int , b int  ) int {
                return a + b
       }

    "#;

        let output = r#"package main

func add(a int, b int) int {
	return a + b
}
"#;

        let ft = "go";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Gofumpt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(gofumpt)]
    fn test_custom_tool_gofumpt_go_3b56f602fe22977b() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"package main

   func add(a int , b int  ) int {
                return a + b
       }

    "#;

        let output = r#"package main

func add(a int, b int) int {
	return a + b
}
"#;

        let ft = "go";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "gofumpt".to_owned(),
                arguments: vec!["-w".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_goimports {
    #[test_with::executable(goimports)]
    fn test_goimports_go_4af43f410d7fff15() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"package main

import (
	"os"
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#;

        let output = r#"package main

import (
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#;

        let ft = "go";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Goimports),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(goimports)]
    fn test_custom_tool_goimports_go_4af43f410d7fff15() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"package main

import (
	"os"
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#;

        let output = r#"package main

import (
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#;

        let ft = "go";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "goimports".to_owned(),
                arguments: vec!["-w".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_golines {
    #[test_with::executable(golines)]
    fn test_golines_go_4af43f410d7fff15() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"package main

import (
	"os"
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#;

        let output = r#"package main

import (
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#;

        let ft = "go";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Golines),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(golines)]
    fn test_custom_tool_golines_go_4af43f410d7fff15() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"package main

import (
	"os"
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#;

        let output = r#"package main

import (
	"fmt"
)

func add(a int, b int) int {
	fmt.Print(a)
	fmt.Print(b)
	return a + b
}
"#;

        let ft = "go";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "golines".to_owned(),
                arguments: vec!["-w".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_google_java_format {
    #[test_with::executable(google-java-format)]
    fn test_google_java_format_java_9d3ffaedafc37e65() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"class HelloWorld {
    public static void main(String[] args) {
                System.out.println("Hello");
                System.out.println("World!");
                 }
}"#;

        let output = r#"class HelloWorld {
  public static void main(String[] args) {
    System.out.println("Hello");
    System.out.println("World!");
  }
}
"#;

        let ft = "java";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::GoogleJavaFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(google-java-format)]
    fn test_custom_tool_google_java_format_java_9d3ffaedafc37e65()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"class HelloWorld {
    public static void main(String[] args) {
                System.out.println("Hello");
                System.out.println("World!");
                 }
}"#;

        let output = r#"class HelloWorld {
  public static void main(String[] args) {
    System.out.println("Hello");
    System.out.println("World!");
  }
}
"#;

        let ft = "java";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "google-java-format".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_grain_format {
    #[test_with::executable(grain)]
    fn test_grain_format_grain_68b6e8ad56bbb476() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"module Hello

                                print("Hello, world!")
"#;

        let output = r#"module Hello

print("Hello, world!")
"#;

        let ft = "grain";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::GrainFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(grain)]
    fn test_custom_tool_grain_grain_68b6e8ad56bbb476() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"module Hello

                                print("Hello, world!")
"#;

        let output = r#"module Hello

print("Hello, world!")
"#;

        let ft = "grain";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "grain".to_owned(),
                arguments: vec![
                    "format".to_owned(),
                    "$PATH".to_owned(),
                    "-o".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_hindent {
    #[test_with::executable(hindent)]
    fn test_hindent_haskell_c34a44cf19c5fdd7() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers :: Int -> Int -> Int
addNumbers a b = do
  a + b
"#;

        let ft = "haskell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Hindent),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(hindent)]
    fn test_custom_tool_hindent_haskell_c34a44cf19c5fdd7() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers :: Int -> Int -> Int
addNumbers a b = do
  a + b
"#;

        let ft = "haskell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "hindent".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_hlint {
    #[test_with::executable(hlint)]
    fn test_hlint_haskell_ea6b440c7b6ee01d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"add :: Int -> Int -> Int
add a b = a + b"#;

        let output = r#"add :: Int -> Int -> Int
add a b = a + b"#;

        let ft = "haskell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Hlint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(hlint)]
    fn test_custom_tool_hlint_haskell_ea6b440c7b6ee01d() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"add :: Int -> Int -> Int
add a b = a + b"#;

        let output = r#"add :: Int -> Int -> Int
add a b = a + b"#;

        let ft = "haskell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "hlint".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_html_beautify {
    #[test_with::executable(html-beautify || bunx || deno || npx || pnpm || yarn)]
    fn test_html_beautify_html_63850f31f2ef5caf() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;

        let output = r#"<div>
    <p>
        Mads was here
    </p>
</div>"#;

        let ft = "html";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::HtmlBeautify),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(html-beautify)]
    fn test_custom_tool_html_beautify_html_63850f31f2ef5caf()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;

        let output = r#"<div>
    <p>
        Mads was here
    </p>
</div>"#;

        let ft = "html";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "html-beautify".to_owned(),
                arguments: vec![
                    "-r".to_owned(),
                    "--type".to_owned(),
                    "html".to_owned(),
                    "-f".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_htmlbeautifier {
    #[test_with::executable(htmlbeautifier || gem)]
    fn test_htmlbeautifier_html_7e86d833d3fbf4e3() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;

        let output = r#"<div>
  <p>
    Mads was here
  </p>
</div>
"#;

        let ft = "html";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Htmlbeautifier),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(htmlbeautifier)]
    fn test_custom_tool_htmlbeautifier_html_7e86d833d3fbf4e3()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;

        let output = r#"<div>
  <p>
    Mads was here
  </p>
</div>
"#;

        let ft = "html";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "htmlbeautifier".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_hurlfmt {
    #[test_with::executable(hurlfmt)]
    fn test_hurlfmt_hurl_cc8490154955ef91() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"  GET        https://example.ord/cats/123           "#;

        let output = r#"GET https://example.ord/cats/123"#;

        let ft = "hurl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Hurlfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(hurlfmt)]
    fn test_custom_tool_hurlfmt_hurl_cc8490154955ef91() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"  GET        https://example.ord/cats/123           "#;

        let output = r#"GET https://example.ord/cats/123"#;

        let ft = "hurl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "hurlfmt".to_owned(),
                arguments: vec!["--in-place".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_isort {
    #[test_with::executable(isort || pipx || uv)]
    fn test_isort_python_e2ac93e0195d9bc1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"from q import d
import b
import a
import c


def add(a: int, b: int) -> int:
  return a + b
"#;

        let output = r#"import a
import b
import c
from q import d


def add(a: int, b: int) -> int:
  return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Isort),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(isort)]
    fn test_custom_tool_isort_python_e2ac93e0195d9bc1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"from q import d
import b
import a
import c


def add(a: int, b: int) -> int:
  return a + b
"#;

        let output = r#"import a
import b
import c
from q import d


def add(a: int, b: int) -> int:
  return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "isort".to_owned(),
                arguments: vec!["--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_jq {
    #[test_with::executable(jq)]
    fn test_jq_json_fddcd253f4dfd781() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{"key":1}"#;

        let output = r#"{
  "key": 1
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Jq),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(jq)]
    fn test_custom_tool_jq_json_fddcd253f4dfd781() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{"key":1}"#;

        let output = r#"{
  "key": 1
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "jq".to_owned(),
                arguments: vec![],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_jqfmt {
    #[test_with::executable(jqfmt)]
    fn test_jqfmt_jq_634e34d16cece292() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{one: .two, three: [.four, .five, [.fivetwo, .fivethree]], six: map(select((.seven | .eight | .nine)))}"#;

        let output = r#"{ one: .two, three: [.four, .five, [.fivetwo, .fivethree]], six: map(select((.seven | .eight | .nine))) }"#;

        let ft = "jq";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Jqfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(jqfmt)]
    fn test_custom_tool_jqfmt_jq_634e34d16cece292() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{one: .two, three: [.four, .five, [.fivetwo, .fivethree]], six: map(select((.seven | .eight | .nine)))}"#;

        let output = r#"{ one: .two, three: [.four, .five, [.fivetwo, .fivethree]], six: map(select((.seven | .eight | .nine))) }"#;

        let ft = "jq";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "jqfmt".to_owned(),
                arguments: vec![],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_js_beautify {
    #[test_with::executable(js-beautify || bunx || deno || npx || pnpm || yarn)]
    fn test_js_beautify_javascript_151bf21bc63609e8() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"function add (a,b){return a +b }"#;

        let output = r#"function add(a, b) {
    return a + b
}"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::JsBeautify),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(js-beautify)]
    fn test_custom_tool_js_beautify_javascript_151bf21bc63609e8()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"function add (a,b){return a +b }"#;

        let output = r#"function add(a, b) {
    return a + b
}"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "js-beautify".to_owned(),
                arguments: vec![
                    "-r".to_owned(),
                    "--type".to_owned(),
                    "js".to_owned(),
                    "-f".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_json_repair {
    #[test_with::executable(json_repair || pipx || uv)]
    fn test_json_repair_json_590de0f66ae2a041() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"[1 2 3]"#;

        let output = r#"[
  1,
  2,
  3
]
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::JsonRepair),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(json_repair)]
    fn test_custom_tool_json_repair_json_590de0f66ae2a041()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"[1 2 3]"#;

        let output = r#"[
  1,
  2,
  3
]
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "json_repair".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_jsonlint {
    #[test_with::executable(jsonlint || bunx || deno || npx || pnpm || yarn)]
    fn test_jsonlint_json_5d1a6be238b35a5c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{ "k": "v" }"#;

        let output = r#"{
  "k": "v"
}"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Jsonlint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(jsonlint)]
    fn test_custom_tool_jsonlint_json_5d1a6be238b35a5c() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"{ "k": "v" }"#;

        let output = r#"{
  "k": "v"
}"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "jsonlint".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_jsonlint_sort {
    #[test_with::executable(jsonlint || bunx || deno || npx || pnpm || yarn)]
    fn test_jsonlint_sort_json_d3194ce4b6550755() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{ "b": "b", "a": "a" }"#;

        let output = r#"{
  "a": "a",
  "b": "b"
}"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::JsonlintSort),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(jsonlint)]
    fn test_custom_tool_jsonlint_json_d3194ce4b6550755() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"{ "b": "b", "a": "a" }"#;

        let output = r#"{
  "a": "a",
  "b": "b"
}"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "jsonlint".to_owned(),
                arguments: vec!["-s".to_owned(), "-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_jsonpp {
    #[test_with::executable(jsonpp)]
    fn test_jsonpp_json_d19292d79f47b2c7() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{
              "key": "value",
  "key2": ["value2", "value3", 1            , null]
 }"#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Jsonpp),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(jsonpp)]
    fn test_custom_tool_jsonpp_json_d19292d79f47b2c7() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{
              "key": "value",
  "key2": ["value2", "value3", 1            , null]
 }"#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "jsonpp".to_owned(),
                arguments: vec!["-s".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_juliaformatter_jl {
    #[test_with::executable(julia)]
    fn test_juliaformatter_jl_julia_6775294e3dc9244() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"function add( a:: Int32,  b::Int32 )
            c = a+ b
            return c
            end "#;

        let output = r#"function add(a::Int32, b::Int32)
    c = a + b
    return c
end
"#;

        let ft = "julia";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::JuliaformatterJl),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(julia)]
    fn test_custom_tool_julia_julia_6775294e3dc9244() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"function add( a:: Int32,  b::Int32 )
            c = a+ b
            return c
            end "#;

        let output = r#"function add(a::Int32, b::Int32)
    c = a + b
    return c
end
"#;

        let ft = "julia";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "julia".to_owned(),
                arguments: vec![
                    "-E".to_owned(),
                    "using JuliaFormatter;format_file(\"$PATH\")".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_just {
    #[test_with::executable(just || bunx || deno || npx || pnpm || yarn || pipx || uv)]
    fn test_just_just_ef70afaf3ede68b9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"build:
                cargo build
                cargo build --release
            "#;

        let output = r#"build:
    cargo build
    cargo build --release
"#;

        let ft = "just";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Just),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(just)]
    fn test_custom_tool_just_just_ef70afaf3ede68b9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"build:
                cargo build
                cargo build --release
            "#;

        let output = r#"build:
    cargo build
    cargo build --release
"#;

        let ft = "just";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "just".to_owned(),
                arguments: vec![
                    "--fmt".to_owned(),
                    "--unstable".to_owned(),
                    "--justfile".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_kcl_fmt {
    #[test_with::executable(kcl)]
    fn test_kcl_fmt_kcl_709718d5d09b85cd() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"apiVersion = "apps/v1"
kind = "Deployment"
metadata = {
    name =  "nginx"
                   labels.app = "nginx"
}
spec = {
    replicas    = 3
    selector.matchLabels = metadata.labels
    template.metadata.labels =                  metadata.labels
    template.spec.containers = [     {
        name = metadata.name
        image = "${metadata.name}:1.14.2"
        ports = [{                                                  containerPort = 80}]
    }]
}
"#;

        let output = r#"apiVersion = "apps/v1"
kind = "Deployment"
metadata = {
    name = "nginx"
    labels.app = "nginx"
}
spec = {
    replicas = 3
    selector.matchLabels = metadata.labels
    template.metadata.labels = metadata.labels
    template.spec.containers = [{
        name = metadata.name
        image = "${metadata.name}:1.14.2"
        ports = [{containerPort = 80}]
    }]
}
"#;

        let ft = "kcl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::KclFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(kcl)]
    fn test_custom_tool_kcl_kcl_709718d5d09b85cd() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"apiVersion = "apps/v1"
kind = "Deployment"
metadata = {
    name =  "nginx"
                   labels.app = "nginx"
}
spec = {
    replicas    = 3
    selector.matchLabels = metadata.labels
    template.metadata.labels =                  metadata.labels
    template.spec.containers = [     {
        name = metadata.name
        image = "${metadata.name}:1.14.2"
        ports = [{                                                  containerPort = 80}]
    }]
}
"#;

        let output = r#"apiVersion = "apps/v1"
kind = "Deployment"
metadata = {
    name = "nginx"
    labels.app = "nginx"
}
spec = {
    replicas = 3
    selector.matchLabels = metadata.labels
    template.metadata.labels = metadata.labels
    template.spec.containers = [{
        name = metadata.name
        image = "${metadata.name}:1.14.2"
        ports = [{containerPort = 80}]
    }]
}
"#;

        let ft = "kcl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "kcl".to_owned(),
                arguments: vec!["fmt".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_kdlfmt {
    #[test_with::executable(kdlfmt || bunx || deno || npx || pnpm || yarn)]
    fn test_kdlfmt_kdl_3d75351f7ec84869() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"world {    child "1"
child "2"   }
"#;

        let output = r#"world {
    child "1"
    child "2"
}
"#;

        let ft = "kdl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Kdlfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(kdlfmt)]
    fn test_custom_tool_kdlfmt_kdl_3d75351f7ec84869() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"world {    child "1"
child "2"   }
"#;

        let output = r#"world {
    child "1"
    child "2"
}
"#;

        let ft = "kdl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "kdlfmt".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_kdlfmt_v_1 {
    #[test_with::executable(kdlfmt || bunx || deno || npx || pnpm || yarn)]
    fn test_kdlfmt_v_1_kdl_4324893eeee4a998() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"world {
    child "1"
child "2"
     }
"#;

        let output = r#"world {
    child "1"
    child "2"
}
"#;

        let ft = "kdl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::KdlfmtV1),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(kdlfmt)]
    fn test_custom_tool_kdlfmt_kdl_4324893eeee4a998() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"world {
    child "1"
child "2"
     }
"#;

        let output = r#"world {
    child "1"
    child "2"
}
"#;

        let ft = "kdl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "kdlfmt".to_owned(),
                arguments: vec![
                    "format".to_owned(),
                    "--kdl-version".to_owned(),
                    "v1".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_kdlfmt_v_2 {
    #[test_with::executable(kdlfmt || bunx || deno || npx || pnpm || yarn)]
    fn test_kdlfmt_v_2_kdl_3d75351f7ec84869() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"world {    child "1"
child "2"   }
"#;

        let output = r#"world {
    child "1"
    child "2"
}
"#;

        let ft = "kdl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::KdlfmtV2),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(kdlfmt)]
    fn test_custom_tool_kdlfmt_kdl_3d75351f7ec84869() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"world {    child "1"
child "2"   }
"#;

        let output = r#"world {
    child "1"
    child "2"
}
"#;

        let ft = "kdl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "kdlfmt".to_owned(),
                arguments: vec![
                    "format".to_owned(),
                    "--kdl-version".to_owned(),
                    "v2".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_ktfmt {
    #[test_with::executable(ktfmt)]
    fn test_ktfmt_kotlin_434b08006e9b780a() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            "#;

        let output = r#"fun add(a: Int, b: Int): Int {
  return a + b
}
"#;

        let ft = "kotlin";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Ktfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(ktfmt)]
    fn test_custom_tool_ktfmt_kotlin_434b08006e9b780a() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            "#;

        let output = r#"fun add(a: Int, b: Int): Int {
  return a + b
}
"#;

        let ft = "kotlin";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "ktfmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_ktlint {
    #[test_with::executable(ktlint)]
    fn test_ktlint_kotlin_3421435c9e766a31() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            "#;

        let output = r#"

fun add(
    a: Int,
    b: Int,
): Int = a + b
"#;

        let ft = "kotlin";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Ktlint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(ktlint)]
    fn test_custom_tool_ktlint_kotlin_3421435c9e766a31() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"            fun add(a:Int ,b:Int ):Int {
                    return a + b
                }
            "#;

        let output = r#"

fun add(
    a: Int,
    b: Int,
): Int = a + b
"#;

        let ft = "kotlin";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "ktlint".to_owned(),
                arguments: vec![
                    "--format".to_owned(),
                    "--log-level=error".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_luaformatter {
    #[test_with::executable(lua-format)]
    fn test_luaformatter_lua_df0e81b2c9a1a835() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"

        local               function        add (                                       a , b
)
local c=a+b
return    c


end
    "#;

        let output = r#"local function add(a, b)
    local c = a + b
    return c

end
"#;

        let ft = "lua";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Luaformatter),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(lua-format)]
    fn test_custom_tool_lua_format_lua_df0e81b2c9a1a835() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"

        local               function        add (                                       a , b
)
local c=a+b
return    c


end
    "#;

        let output = r#"local function add(a, b)
    local c = a + b
    return c

end
"#;

        let ft = "lua";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "lua-format".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mado_check {
    #[test_with::executable(mado)]
    fn test_mado_check_markdown_2eeb10259aff4747() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"# Hello world

- Hello
- world
"#;

        let output = r#"# Hello world

- Hello
- world
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MadoCheck),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mado)]
    fn test_custom_tool_mado_markdown_2eeb10259aff4747() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"# Hello world

- Hello
- world
"#;

        let output = r#"# Hello world

- Hello
- world
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mado".to_owned(),
                arguments: vec!["check".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mago_format {
    #[test_with::executable(mago)]
    fn test_mago_format_php_17cf4527911d3cc9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<?php
echo "Hello World!";
?>"#;

        let output = r#"<?php

echo 'Hello World!';
"#;

        let ft = "php";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MagoFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mago)]
    fn test_custom_tool_mago_php_17cf4527911d3cc9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<?php
echo "Hello World!";
?>"#;

        let output = r#"<?php

echo 'Hello World!';
"#;

        let ft = "php";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mago".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mago_lint {
    #[test_with::executable(mago)]
    fn test_mago_lint_php_513b2cc3a1e145ed() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<?php
echo 'Hello World!';
"#;

        let output = r#"<?php
echo 'Hello World!';
"#;

        let ft = "php";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MagoLint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mago)]
    fn test_custom_tool_mago_php_513b2cc3a1e145ed() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<?php
echo 'Hello World!';
"#;

        let output = r#"<?php
echo 'Hello World!';
"#;

        let ft = "php";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mago".to_owned(),
                arguments: vec!["lint".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mago_lint_fix {
    #[test_with::executable(mago)]
    fn test_mago_lint_fix_php_513b2cc3a1e145ed() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<?php
echo 'Hello World!';
"#;

        let output = r#"<?php
echo 'Hello World!';
"#;

        let ft = "php";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MagoLintFix),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mago)]
    fn test_custom_tool_mago_php_513b2cc3a1e145ed() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<?php
echo 'Hello World!';
"#;

        let output = r#"<?php
echo 'Hello World!';
"#;

        let ft = "php";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mago".to_owned(),
                arguments: vec!["lint".to_owned(), "--fix".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mago_lint_fix_unsafe {
    #[test_with::executable(mago)]
    fn test_mago_lint_fix_unsafe_php_8b9097d14e83ef96() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<?php
echo 'Hello World!';
"#;

        let output = r#"<?php

declare(strict_types=1);

echo 'Hello World!';
"#;

        let ft = "php";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MagoLintFixUnsafe),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mago)]
    fn test_custom_tool_mago_php_8b9097d14e83ef96() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<?php
echo 'Hello World!';
"#;

        let output = r#"<?php

declare(strict_types=1);

echo 'Hello World!';
"#;

        let ft = "php";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mago".to_owned(),
                arguments: vec![
                    "lint".to_owned(),
                    "--fix".to_owned(),
                    "--potentially-unsafe".to_owned(),
                    "--unsafe".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_markdownfmt {
    #[test_with::executable(markdownfmt)]
    fn test_markdownfmt_markdown_9b495bc15a7833bc() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"# hello w   world

this   text has      weird spacing

- first
* second"#;

        let output = r#"hello w world
=============

this text has weird spacing

-	first
-	second
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Markdownfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(markdownfmt)]
    fn test_custom_tool_markdownfmt_markdown_9b495bc15a7833bc()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"# hello w   world

this   text has      weird spacing

- first
* second"#;

        let output = r#"hello w world
=============

this text has weird spacing

-	first
-	second
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "markdownfmt".to_owned(),
                arguments: vec!["-w".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_markdownlint {
    #[test_with::executable(markdownlint || bunx || deno || npx || pnpm || yarn)]
    fn test_markdownlint_markdown_27f5778fc1db5182() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"# Hello world

- asd
* vasd
"#;

        let output = r#"# Hello world

- asd
- vasd
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Markdownlint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(markdownlint)]
    fn test_custom_tool_markdownlint_markdown_27f5778fc1db5182()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"# Hello world

- asd
* vasd
"#;

        let output = r#"# Hello world

- asd
- vasd
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "markdownlint".to_owned(),
                arguments: vec!["--fix".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_markdownlint_cli_2 {
    #[test_with::executable(markdownlint-cli2 || bunx || deno || npx || pnpm || yarn)]
    fn test_markdownlint_cli_2_markdown_27f5778fc1db5182() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"# Hello world

- asd
* vasd
"#;

        let output = r#"# Hello world

- asd
- vasd
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MarkdownlintCli2),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(markdownlint-cli2)]
    fn test_custom_tool_markdownlint_cli_2_markdown_27f5778fc1db5182()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"# Hello world

- asd
* vasd
"#;

        let output = r#"# Hello world

- asd
- vasd
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "markdownlint-cli2".to_owned(),
                arguments: vec!["--fix".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mbake_format {
    #[test_with::executable(mbake || pipx || uv)]
    fn test_mbake_format_makefile_edbe638c5985a6ce() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"CC:=gcc
CFLAGS= -Wall -g
SOURCES=main.c \n  utils.c \n  helper.c

.PHONY: clean
all: $(TARGET)
	$(CC) $(CFLAGS) -o $@ $^

.PHONY: install
clean:
	rm -f *.o
"#;

        let output = r#"CC := gcc
CFLAGS = -Wall -g
SOURCES = main.c \n  utils.c \n  helper.c

.PHONY: clean
all: $(TARGET)
	$(CC) $(CFLAGS) -o $@ $^

.PHONY: install
clean:
	rm -f *.o
"#;

        let ft = "Makefile";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MbakeFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mbake)]
    fn test_custom_tool_mbake_makefile_edbe638c5985a6ce() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"CC:=gcc
CFLAGS= -Wall -g
SOURCES=main.c \n  utils.c \n  helper.c

.PHONY: clean
all: $(TARGET)
	$(CC) $(CFLAGS) -o $@ $^

.PHONY: install
clean:
	rm -f *.o
"#;

        let output = r#"CC := gcc
CFLAGS = -Wall -g
SOURCES = main.c \n  utils.c \n  helper.c

.PHONY: clean
all: $(TARGET)
	$(CC) $(CFLAGS) -o $@ $^

.PHONY: install
clean:
	rm -f *.o
"#;

        let ft = "Makefile";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mbake".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mbake_validate {
    #[test_with::executable(mbake || pipx || uv)]
    fn test_mbake_validate_makefile_3892d48c3f3530c1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"CC := gcc
CFLAGS = -Wall -g
SOURCES = main.c \n  utils.c \n  helper.c

.PHONY: all clean install

all: $(TARGET)
	$(CC) $(CFLAGS) -o $@ $^

clean:
	rm -f *.o
"#;

        let output = r#"CC := gcc
CFLAGS = -Wall -g
SOURCES = main.c \n  utils.c \n  helper.c

.PHONY: all clean install

all: $(TARGET)
	$(CC) $(CFLAGS) -o $@ $^

clean:
	rm -f *.o
"#;

        let ft = "Makefile";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MbakeValidate),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mbake)]
    fn test_custom_tool_mbake_makefile_3892d48c3f3530c1() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"CC := gcc
CFLAGS = -Wall -g
SOURCES = main.c \n  utils.c \n  helper.c

.PHONY: all clean install

all: $(TARGET)
	$(CC) $(CFLAGS) -o $@ $^

clean:
	rm -f *.o
"#;

        let output = r#"CC := gcc
CFLAGS = -Wall -g
SOURCES = main.c \n  utils.c \n  helper.c

.PHONY: all clean install

all: $(TARGET)
	$(CC) $(CFLAGS) -o $@ $^

clean:
	rm -f *.o
"#;

        let ft = "Makefile";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mbake".to_owned(),
                arguments: vec!["validate".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mdsf_format {
    #[test_with::executable(mdsf || bunx || deno || npx || pnpm || yarn)]
    fn test_mdsf_format_markdown_1e1586f943958589() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#""#;

        let output = r#""#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MdsfFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mdsf)]
    fn test_custom_tool_mdsf_markdown_1e1586f943958589() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#""#;

        let output = r#""#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mdsf".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mdsf_verify {
    #[test_with::executable(mdsf || bunx || deno || npx || pnpm || yarn)]
    fn test_mdsf_verify_markdown_1e1586f943958589() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#""#;

        let output = r#""#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MdsfVerify),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mdsf)]
    fn test_custom_tool_mdsf_markdown_1e1586f943958589() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#""#;

        let output = r#""#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mdsf".to_owned(),
                arguments: vec!["verify".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mise_fmt {
    #[test_with::executable(mise || bunx || deno || npx || pnpm || yarn)]
    fn test_mise_fmt_toml_7a3c9e91cda91a26() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"[env]
NODE_ENV = 'production'


[tools]
erlang                = ['23.3', '24.0']
terraform = '1.0.0'














[tasks.build]
run = 'echo "running build tasks"'
"#;

        let output = r#"[env]
NODE_ENV = 'production'


[tools]
erlang = ['23.3', '24.0']
terraform = '1.0.0'


[tasks.build]
run = 'echo "running build tasks"'
"#;

        let ft = "toml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MiseFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mise)]
    fn test_custom_tool_mise_toml_7a3c9e91cda91a26() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"[env]
NODE_ENV = 'production'


[tools]
erlang                = ['23.3', '24.0']
terraform = '1.0.0'














[tasks.build]
run = 'echo "running build tasks"'
"#;

        let output = r#"[env]
NODE_ENV = 'production'


[tools]
erlang = ['23.3', '24.0']
terraform = '1.0.0'


[tasks.build]
run = 'echo "running build tasks"'
"#;

        let ft = "toml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mise".to_owned(),
                arguments: vec!["fmt".to_owned(), "--stdin".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_misspell {
    #[test_with::executable(misspell)]
    fn test_misspell_markdown_1e37d19484d12443() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"langauge
"#;

        let output = r#"language
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Misspell),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(misspell)]
    fn test_custom_tool_misspell_markdown_1e37d19484d12443()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"langauge
"#;

        let output = r#"language
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "misspell".to_owned(),
                arguments: vec!["-w".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_mix_format {
    #[test_with::executable(mix)]
    fn test_mix_format_elixir_ab535c627dfb140() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
        def              add(a  ,      b   )   do    a   +   b                 end

"#;

        let output = r#"def add(a, b) do
  a + b
end
"#;

        let ft = "elixir";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::MixFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(mix)]
    fn test_custom_tool_mix_elixir_ab535c627dfb140() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
        def              add(a  ,      b   )   do    a   +   b                 end

"#;

        let output = r#"def add(a, b) do
  a + b
end
"#;

        let ft = "elixir";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "mix".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_nginxbeautifier {
    #[test_with::executable(nginxbeautifier || bunx || deno || npx || pnpm || yarn)]
    fn test_nginxbeautifier_conf_5c2a2e0d4f44354f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"server {
    listen 80;
        listen [::]:80;
       server_name example.com;
    }
"#;

        let output = r#"server {
	listen 80;
	listen [::]:80;
	server_name example.com;
}
"#;

        let ft = ".conf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Nginxbeautifier),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(nginxbeautifier)]
    fn test_custom_tool_nginxbeautifier_conf_5c2a2e0d4f44354f()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"server {
    listen 80;
        listen [::]:80;
       server_name example.com;
    }
"#;

        let output = r#"server {
	listen 80;
	listen [::]:80;
	server_name example.com;
}
"#;

        let ft = ".conf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "nginxbeautifier".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_nginxfmt {
    #[test_with::executable(nginxfmt || pipx || uv)]
    fn test_nginxfmt_conf_2e651ac1789b7182() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"server {
    listen 80;
        listen [::]:80;
       server_name example.com;
    }
"#;

        let output = r#"server {
    listen 80;
    listen [::]:80;
    server_name example.com;
}
"#;

        let ft = ".conf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Nginxfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(nginxfmt)]
    fn test_custom_tool_nginxfmt_conf_2e651ac1789b7182() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"server {
    listen 80;
        listen [::]:80;
       server_name example.com;
    }
"#;

        let output = r#"server {
    listen 80;
    listen [::]:80;
    server_name example.com;
}
"#;

        let ft = ".conf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "nginxfmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_nimpretty {
    #[test_with::executable(nimpretty)]
    fn test_nimpretty_nim_2c41c79e1d74972a() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"proc           add( a         :int , b:int )        : int =
  return a +          b  "#;

        let output = r#"proc add(a: int, b: int): int =
  return a + b
"#;

        let ft = "nim";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Nimpretty),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(nimpretty)]
    fn test_custom_tool_nimpretty_nim_2c41c79e1d74972a() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"proc           add( a         :int , b:int )        : int =
  return a +          b  "#;

        let output = r#"proc add(a: int, b: int): int =
  return a + b
"#;

        let ft = "nim";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "nimpretty".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_nixfmt {
    #[test_with::executable(nixfmt)]
    fn test_nixfmt_nix_c01c4e4dcc81ab28() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{ lib, buildPythonPackage, fetchFromGitHub, redis }:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let output = r#"{ lib, buildPythonPackage, fetchFromGitHub, redis }:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let ft = "nix";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Nixfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(nixfmt)]
    fn test_custom_tool_nixfmt_nix_c01c4e4dcc81ab28() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{ lib, buildPythonPackage, fetchFromGitHub, redis }:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let output = r#"{ lib, buildPythonPackage, fetchFromGitHub, redis }:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let ft = "nix";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "nixfmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_nixpkgs_fmt {
    #[test_with::executable(nixpkgs-fmt)]
    fn test_nixpkgs_fmt_nix_c2c7561cdeb3702() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{
            lib, buildPythonPackage, fetchFromGitHub, redis }:

buildPythonPackage rec {
  pname =   "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let output = r#"{ lib
, buildPythonPackage
, fetchFromGitHub
, redis
}:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let ft = "nix";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::NixpkgsFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(nixpkgs-fmt)]
    fn test_custom_tool_nixpkgs_fmt_nix_c2c7561cdeb3702() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"{
            lib, buildPythonPackage, fetchFromGitHub, redis }:

buildPythonPackage rec {
  pname =   "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let output = r#"{ lib
, buildPythonPackage
, fetchFromGitHub
, redis
}:

buildPythonPackage rec {
  pname = "huey";
  version = "2.4.2";

  src = fetchFromGitHub {
    owner = "coleifer";
    repo = pname;
    rev = version;
    sha256 = "00fi04991skq61gjrmig8ry6936pc8zs7p8py8spfipbxf1irkjg";
  };

  propagatedBuildInputs = [ redis ];

  # connects to redis
  doCheck = false;

  meta = with lib; {
    description = "A little task queue for python";
    homepage = "https://github.com/coleifer/huey";
    license = licenses.mit;
    maintainers = [ maintainers.globin ];
  };
}
"#;

        let ft = "nix";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "nixpkgs-fmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_nph {
    #[test_with::executable(nph)]
    fn test_nph_nim_b53c066cb1d15828() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"proc add(a:int,b:int):int =
            return a+b
"#;

        let output = r#"proc add(a: int, b: int): int =
  return a + b
"#;

        let ft = "nim";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Nph),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(nph)]
    fn test_custom_tool_nph_nim_b53c066cb1d15828() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"proc add(a:int,b:int):int =
            return a+b
"#;

        let output = r#"proc add(a: int, b: int): int =
  return a + b
"#;

        let ft = "nim";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "nph".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_ocamlformat {
    #[test_with::executable(ocamlformat)]
    fn test_ocamlformat_ocaml_5f599d285848218() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
let add a b  =  a +  b
            "#;

        let output = r#"let add a b = a + b
"#;

        let ft = "ocaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Ocamlformat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(ocamlformat)]
    fn test_custom_tool_ocamlformat_ocaml_5f599d285848218()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
let add a b  =  a +  b
            "#;

        let output = r#"let add a b = a + b
"#;

        let ft = "ocaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "ocamlformat".to_owned(),
                arguments: vec![
                    "--ignore-invalid-option".to_owned(),
                    "--inplace".to_owned(),
                    "--enable-outside-detected-project".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_ocp_indent {
    #[test_with::executable(ocp-indent)]
    fn test_ocp_indent_ocaml_87a2cd7557f7a90b() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
let add a b
                             = a + b
            "#;

        let output = r#"
let add a b
  = a + b
"#;

        let ft = "ocaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::OcpIndent),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(ocp-indent)]
    fn test_custom_tool_ocp_indent_ocaml_87a2cd7557f7a90b()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
let add a b
                             = a + b
            "#;

        let output = r#"
let add a b
  = a + b
"#;

        let ft = "ocaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "ocp-indent".to_owned(),
                arguments: vec!["--inplace".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_ormolu {
    #[test_with::executable(ormolu)]
    fn test_ormolu_haskell_c34a44cf19c5fdd7() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers :: Int -> Int -> Int
addNumbers a b = do
  a + b
"#;

        let ft = "haskell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Ormolu),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(ormolu)]
    fn test_custom_tool_ormolu_haskell_c34a44cf19c5fdd7() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"
addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers :: Int -> Int -> Int
addNumbers a b = do
  a + b
"#;

        let ft = "haskell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "ormolu".to_owned(),
                arguments: vec![
                    "--mode".to_owned(),
                    "inplace".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_oxfmt {
    #[test_with::executable(oxfmt || bunx || deno || npx || pnpm || yarn)]
    fn test_oxfmt_typescript_6e1a60dfe57a22a9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"const x =     10;"#;

        let output = r#"const x = 10;
"#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Oxfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(oxfmt)]
    fn test_custom_tool_oxfmt_typescript_6e1a60dfe57a22a9()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"const x =     10;"#;

        let output = r#"const x = 10;
"#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "oxfmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_oxlint {
    #[test_with::executable(oxlint || bunx || deno || npx || pnpm || yarn)]
    fn test_oxlint_typescript_a2154a11ef1c153b() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"debugger;"#;

        let output = r#""#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Oxlint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(oxlint)]
    fn test_custom_tool_oxlint_typescript_a2154a11ef1c153b()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"debugger;"#;

        let output = r#""#;

        let ft = "typescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "oxlint".to_owned(),
                arguments: vec!["--fix".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_perflint {
    #[test_with::executable(perflint || pipx || uv)]
    fn test_perflint_python_2a683a1c25614024() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add(a, b): return a + b"#;

        let output = r#"def add(a, b): return a + b"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Perflint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(perflint)]
    fn test_custom_tool_perflint_python_2a683a1c25614024() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"def add(a, b): return a + b"#;

        let output = r#"def add(a, b): return a + b"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "perflint".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_prettier {
    #[test_with::executable(prettier || bunx || deno || npx || pnpm || yarn)]
    fn test_prettier_javascript_f38217e7df306e3e() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
  return a + b;
}
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Prettier),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(prettier)]
    fn test_custom_tool_prettier_javascript_f38217e7df306e3e()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
  return a + b;
}
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "prettier".to_owned(),
                arguments: vec![
                    "--embedded-language-formatting".to_owned(),
                    "off".to_owned(),
                    "--log-level".to_owned(),
                    "error".to_owned(),
                    "--write".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(prettier || bunx || deno || npx || pnpm || yarn)]
    fn test_prettier_json_8e1e8ed2224fd439() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
  "key": "value",
  "key2": ["value2", "value3", 1, null]
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Prettier),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(prettier)]
    fn test_custom_tool_prettier_json_8e1e8ed2224fd439() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
  "key": "value",
  "key2": ["value2", "value3", 1, null]
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "prettier".to_owned(),
                arguments: vec![
                    "--embedded-language-formatting".to_owned(),
                    "off".to_owned(),
                    "--log-level".to_owned(),
                    "error".to_owned(),
                    "--write".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_prettierd {
    #[test_with::executable(prettierd)]
    fn test_prettierd_javascript_f38217e7df306e3e() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
  return a + b;
}
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Prettierd),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(prettierd)]
    fn test_custom_tool_prettierd_javascript_f38217e7df306e3e()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(
            a,b
        ) {
        return a+b
    }

            "#;

        let output = r#"async function asyncAddition(a, b) {
  return a + b;
}
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "prettierd".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(prettierd)]
    fn test_prettierd_json_8e1e8ed2224fd439() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
  "key": "value",
  "key2": ["value2", "value3", 1, null]
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Prettierd),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(prettierd)]
    fn test_custom_tool_prettierd_json_8e1e8ed2224fd439() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
  "key": "value",
  "key2": ["value2", "value3", 1, null]
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "prettierd".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_purs_tidy {
    #[test_with::executable(purs-tidy || bunx || deno || npx || pnpm || yarn)]
    fn test_purs_tidy_purescript_c9e6831b630f7f08() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"module       Test.Main   where

import Prelude

import   Effect   (Effect)
import                  Effect.Class.Console  (log)

main     ::   Effect Unit
main   =    do
  log          "You should add some tests.""#;

        let output = r#"module Test.Main where

import Prelude

import Effect (Effect)
import Effect.Class.Console (log)

main :: Effect Unit
main = do
  log "You should add some tests.""#;

        let ft = "purescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::PursTidy),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(purs-tidy)]
    fn test_custom_tool_purs_tidy_purescript_c9e6831b630f7f08()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"module       Test.Main   where

import Prelude

import   Effect   (Effect)
import                  Effect.Class.Console  (log)

main     ::   Effect Unit
main   =    do
  log          "You should add some tests.""#;

        let output = r#"module Test.Main where

import Prelude

import Effect (Effect)
import Effect.Class.Console (log)

main :: Effect Unit
main = do
  log "You should add some tests.""#;

        let ft = "purescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "purs-tidy".to_owned(),
                arguments: vec!["format-in-place".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_purty {
    #[test_with::executable(purty || bunx || deno || npx || pnpm || yarn)]
    fn test_purty_purescript_37730dad0a7f9fbd() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"module Mdsf where




add   :: Int -> Int  ->    Int
add a   b = a +         b
"#;

        let output = r#"module Mdsf where

add :: Int -> Int -> Int
add a b = a + b
"#;

        let ft = "purescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Purty),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(purty)]
    fn test_custom_tool_purty_purescript_37730dad0a7f9fbd()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"module Mdsf where




add   :: Int -> Int  ->    Int
add a   b = a +         b
"#;

        let output = r#"module Mdsf where

add :: Int -> Int -> Int
add a b = a + b
"#;

        let ft = "purescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "purty".to_owned(),
                arguments: vec!["--write".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_pycln {
    #[test_with::executable(pycln || pipx || uv)]
    fn test_pycln_python_21e4539a9b183542() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"import math"#;

        let output = r#""#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Pycln),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(pycln)]
    fn test_custom_tool_pycln_python_21e4539a9b183542() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"import math"#;

        let output = r#""#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "pycln".to_owned(),
                arguments: vec![
                    "--no-gitignore".to_owned(),
                    "--quiet".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_pyflakes {
    #[test_with::executable(pyflakes || pipx || uv)]
    fn test_pyflakes_python_8c5d8d3b8d3870d1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Pyflakes),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(pyflakes)]
    fn test_custom_tool_pyflakes_python_8c5d8d3b8d3870d1() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "pyflakes".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_pyink {
    #[test_with::executable(pyink || pipx || uv)]
    fn test_pyink_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Pyink),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(pyink)]
    fn test_custom_tool_pyink_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "pyink".to_owned(),
                arguments: vec!["--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_pylint {
    #[test_with::executable(pylint || pipx || uv)]
    fn test_pylint_python_826209940b0fafbc() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#""""
mdsf test module for pylint
"""


def add(a: int, b: int) -> int:
    """
    Add the numbers
    """
    return a + b
"#;

        let output = r#""""
mdsf test module for pylint
"""


def add(a: int, b: int) -> int:
    """
    Add the numbers
    """
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Pylint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(pylint)]
    fn test_custom_tool_pylint_python_826209940b0fafbc() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#""""
mdsf test module for pylint
"""


def add(a: int, b: int) -> int:
    """
    Add the numbers
    """
    return a + b
"#;

        let output = r#""""
mdsf test module for pylint
"""


def add(a: int, b: int) -> int:
    """
    Add the numbers
    """
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "pylint".to_owned(),
                arguments: vec!["--module-naming-style=any".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_pymarkdownlnt_fix {
    #[test_with::executable(pymarkdownlnt || pipx || uv)]
    fn test_pymarkdownlnt_fix_markdown_db65f91c72704aab() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"Line 1




Line 2




Line 3
"#;

        let output = r#"Line 1

Line 2

Line 3
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::PymarkdownlntFix),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(pymarkdownlnt)]
    fn test_custom_tool_pymarkdownlnt_markdown_db65f91c72704aab()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"Line 1




Line 2




Line 3
"#;

        let output = r#"Line 1

Line 2

Line 3
"#;

        let ft = "markdown";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "pymarkdownlnt".to_owned(),
                arguments: vec![
                    "--return-code-scheme".to_owned(),
                    "minimal".to_owned(),
                    "fix".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_pyrefly {
    #[test_with::executable(pyrefly || pipx || uv)]
    fn test_pyrefly_python_13af245604aaa0cd() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b


add(1, 2)
"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b


add(1, 2)
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Pyrefly),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(pyrefly)]
    fn test_custom_tool_pyrefly_python_13af245604aaa0cd() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b


add(1, 2)
"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b


add(1, 2)
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "pyrefly".to_owned(),
                arguments: vec!["check".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_pyupgrade {
    #[test_with::executable(pyupgrade || pipx || uv)]
    fn test_pyupgrade_python_efcc3b576317ef09() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"set([x for x in y])"#;

        let output = r#"{x for x in y}"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Pyupgrade),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(pyupgrade)]
    fn test_custom_tool_pyupgrade_python_efcc3b576317ef09()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"set([x for x in y])"#;

        let output = r#"{x for x in y}"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "pyupgrade".to_owned(),
                arguments: vec!["--exit-zero-even-if-changed".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_reorder_python_imports {
    #[test_with::executable(reorder-python-imports || pipx || uv)]
    fn test_reorder_python_imports_python_8ddc1587af0094c1()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"import sys
import pyramid
import reorder_python_imports"#;

        let output = r#"import sys

import pyramid
import reorder_python_imports
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ReorderPythonImports),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(reorder-python-imports)]
    fn test_custom_tool_reorder_python_imports_python_8ddc1587af0094c1()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"import sys
import pyramid
import reorder_python_imports"#;

        let output = r#"import sys

import pyramid
import reorder_python_imports
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "reorder-python-imports".to_owned(),
                arguments: vec!["--exit-zero-even-if-changed".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_rescript_format {
    #[test_with::executable(rescript || bunx || deno || npx || pnpm || yarn)]
    fn test_rescript_format_rescript_59c7490e2a041de3() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"module Button = {
  @react.component
  let make = (~count) =>   {
    let times = switch    count {
            | 1          =>   "once"
    | 2  =>         "twice"
    |   n =>      n->Int.toString ++ " times"
     }
     let text =                           `Click me ${times}`

    <button> {text->React.string} </button>
  }
}"#;

        let output = r#"module Button = {
  @react.component
  let make = (~count) => {
    let times = switch count {
    | 1 => "once"
    | 2 => "twice"
    | n => n->Int.toString ++ " times"
    }
    let text = `Click me ${times}`

    <button> {text->React.string} </button>
  }
}
"#;

        let ft = "rescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::RescriptFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(rescript)]
    fn test_custom_tool_rescript_rescript_59c7490e2a041de3()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"module Button = {
  @react.component
  let make = (~count) =>   {
    let times = switch    count {
            | 1          =>   "once"
    | 2  =>         "twice"
    |   n =>      n->Int.toString ++ " times"
     }
     let text =                           `Click me ${times}`

    <button> {text->React.string} </button>
  }
}"#;

        let output = r#"module Button = {
  @react.component
  let make = (~count) => {
    let times = switch count {
    | 1 => "once"
    | 2 => "twice"
    | n => n->Int.toString ++ " times"
    }
    let text = `Click me ${times}`

    <button> {text->React.string} </button>
  }
}
"#;

        let ft = "rescript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "rescript".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_roc_format {
    #[test_with::executable(roc)]
    fn test_roc_format_roc_1204aa2d8186919d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"app "helloWorld"
    packages { pf: "https://github.com/roc-lang/" }
    imports [pf.Stdout]
    provides [main] to pf






main =
    Stdout.line "Hello, World!"


    "#;

        let output = r#"app [main] { pf: platform "https://github.com/roc-lang/" }

import pf.Stdout

main =
    Stdout.line "Hello, World!"

"#;

        let ft = "roc";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::RocFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(roc)]
    fn test_custom_tool_roc_roc_1204aa2d8186919d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"app "helloWorld"
    packages { pf: "https://github.com/roc-lang/" }
    imports [pf.Stdout]
    provides [main] to pf






main =
    Stdout.line "Hello, World!"


    "#;

        let output = r#"app [main] { pf: platform "https://github.com/roc-lang/" }

import pf.Stdout

main =
    Stdout.line "Hello, World!"

"#;

        let ft = "roc";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "roc".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_rubocop {
    #[test_with::executable(rubocop)]
    fn test_rubocop_ruby_d2b8a6db3c8eee1c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let ft = "ruby";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Rubocop),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(rubocop)]
    fn test_custom_tool_rubocop_ruby_d2b8a6db3c8eee1c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let ft = "ruby";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "rubocop".to_owned(),
                arguments: vec![
                    "--fix-layout".to_owned(),
                    "--autocorrect".to_owned(),
                    "--format".to_owned(),
                    "quiet".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_rubyfmt {
    #[test_with::executable(rubyfmt)]
    fn test_rubyfmt_ruby_d2b8a6db3c8eee1c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let ft = "ruby";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Rubyfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(rubyfmt)]
    fn test_custom_tool_rubyfmt_ruby_d2b8a6db3c8eee1c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let ft = "ruby";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "rubyfmt".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_ruff_check {
    #[test_with::executable(ruff || pipx || uv)]
    fn test_ruff_check_python_e2f9361cc55100c5() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::RuffCheck),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(ruff)]
    fn test_custom_tool_ruff_python_e2f9361cc55100c5() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "ruff".to_owned(),
                arguments: vec![
                    "check".to_owned(),
                    "--fix".to_owned(),
                    "--quiet".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_ruff_format {
    #[test_with::executable(ruff || pipx || uv)]
    fn test_ruff_format_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::RuffFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(ruff)]
    fn test_custom_tool_ruff_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "ruff".to_owned(),
                arguments: vec![
                    "format".to_owned(),
                    "--quiet".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_rufo {
    #[test_with::executable(rufo || gem)]
    fn test_rufo_ruby_d2b8a6db3c8eee1c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let ft = "ruby";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Rufo),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(rufo)]
    fn test_custom_tool_rufo_ruby_d2b8a6db3c8eee1c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  return a + b
end
"#;

        let ft = "ruby";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "rufo".to_owned(),
                arguments: vec!["--simple-exit".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_rustfmt {
    #[test_with::executable(rustfmt)]
    fn test_rustfmt_rust_70ad564760e773e9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    "#;

        let output = r#"pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let ft = "rust";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Rustfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(rustfmt)]
    fn test_custom_tool_rustfmt_rust_70ad564760e773e9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    "#;

        let output = r#"pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let ft = "rust";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "rustfmt".to_owned(),
                arguments: vec![
                    "--edition".to_owned(),
                    "2021".to_owned(),
                    "--quiet".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_rustywind {
    #[test_with::executable(rustywind || bunx || deno || npx || pnpm || yarn)]
    fn test_rustywind_html_f482eb2ece82bb0d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<div class="flex-col flex"></div>
"#;

        let output = r#"<div class="flex flex-col"></div>
"#;

        let ft = "html";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Rustywind),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(rustywind)]
    fn test_custom_tool_rustywind_html_f482eb2ece82bb0d() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"<div class="flex-col flex"></div>
"#;

        let output = r#"<div class="flex flex-col"></div>
"#;

        let ft = "html";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "rustywind".to_owned(),
                arguments: vec!["--write".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_scalafmt {
    #[test_with::executable(scalafmt)]
    fn test_scalafmt_scala_cbd61c065383c05b() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"object Addition {
             def main() = {
                 println(1 + 3)
             }
    }"#;

        let output = r#"object Addition {
  def main() = {
    println(1 + 3)
  }
}
"#;

        let ft = "scala";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Scalafmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(scalafmt)]
    fn test_custom_tool_scalafmt_scala_cbd61c065383c05b() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"object Addition {
             def main() = {
                 println(1 + 3)
             }
    }"#;

        let output = r#"object Addition {
  def main() = {
    println(1 + 3)
  }
}
"#;

        let ft = "scala";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "scalafmt".to_owned(),
                arguments: vec![
                    "--quiet".to_owned(),
                    "--mode".to_owned(),
                    "any".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_selene {
    #[test_with::executable(selene)]
    fn test_selene_lua_e4a3734aedc452ef() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"function add(a, b)
	return a + b
end

return add
"#;

        let output = r#"function add(a, b)
	return a + b
end

return add
"#;

        let ft = "lua";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Selene),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(selene)]
    fn test_custom_tool_selene_lua_e4a3734aedc452ef() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"function add(a, b)
	return a + b
end

return add
"#;

        let output = r#"function add(a, b)
	return a + b
end

return add
"#;

        let ft = "lua";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "selene".to_owned(),
                arguments: vec![
                    "--no-summary".to_owned(),
                    "--quiet".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_semistandard {
    #[test_with::executable(semistandard || bunx || deno || npx || pnpm || yarn)]
    fn test_semistandard_javascript_dd13bf6b8d6e09a1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"    async function asyncAddition(a,b  )
    {
        return a+b
    }

console.info(asyncAddition(1, 2));
            "#;

        let output = r#"async function asyncAddition (a, b) {
  return a + b;
}

console.info(asyncAddition(1, 2));
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Semistandard),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(semistandard)]
    fn test_custom_tool_semistandard_javascript_dd13bf6b8d6e09a1()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"    async function asyncAddition(a,b  )
    {
        return a+b
    }

console.info(asyncAddition(1, 2));
            "#;

        let output = r#"async function asyncAddition (a, b) {
  return a + b;
}

console.info(asyncAddition(1, 2));
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "semistandard".to_owned(),
                arguments: vec!["--fix".to_owned(), "--stdin".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_shellcheck {
    #[test_with::executable(shellcheck)]
    fn test_shellcheck_shell_7176996a1b8efe54() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"#!/bin/sh

echo "Hello World"
"#;

        let output = r#"#!/bin/sh

echo "Hello World"
"#;

        let ft = "shell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Shellcheck),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(shellcheck)]
    fn test_custom_tool_shellcheck_shell_7176996a1b8efe54()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"#!/bin/sh

echo "Hello World"
"#;

        let output = r#"#!/bin/sh

echo "Hello World"
"#;

        let ft = "shell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "shellcheck".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_shfmt {
    #[test_with::executable(shfmt)]
    fn test_shfmt_bash_9334f16dadf8ef68() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"

#!/bin/bash

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;

        let output = r#"#!/bin/bash

add() {
	echo "$1" + "$2"
}
"#;

        let ft = "bash";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Shfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(shfmt)]
    fn test_custom_tool_shfmt_bash_9334f16dadf8ef68() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"

#!/bin/bash

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;

        let output = r#"#!/bin/bash

add() {
	echo "$1" + "$2"
}
"#;

        let ft = "bash";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "shfmt".to_owned(),
                arguments: vec!["--write".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(shfmt)]
    fn test_shfmt_shell_9c24a79abf093e10() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"

#!/bin/sh

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;

        let output = r#"#!/bin/sh

add() {
	echo "$1" + "$2"
}
"#;

        let ft = "shell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Shfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(shfmt)]
    fn test_custom_tool_shfmt_shell_9c24a79abf093e10() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"

#!/bin/sh

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;

        let output = r#"#!/bin/sh

add() {
	echo "$1" + "$2"
}
"#;

        let ft = "shell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "shfmt".to_owned(),
                arguments: vec!["--write".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(shfmt)]
    fn test_shfmt_zsh_63d80ef78ac08ee0() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"

#!/bin/zsh

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;

        let output = r#"#!/bin/zsh

add() {
	echo "$1" + "$2"
}
"#;

        let ft = "zsh";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Shfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(shfmt)]
    fn test_custom_tool_shfmt_zsh_63d80ef78ac08ee0() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"

#!/bin/zsh

       add      ()   {
    echo "$1"                 +          "$2"
             }








"#;

        let output = r#"#!/bin/zsh

add() {
	echo "$1" + "$2"
}
"#;

        let ft = "zsh";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "shfmt".to_owned(),
                arguments: vec!["--write".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_sleek {
    #[test_with::executable(sleek)]
    fn test_sleek_sql_d16819f4564d8853() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         "#;

        let output = r#"SELECT
    *
FROM
    tbl
WHERE
    foo = 'bar';"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Sleek),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(sleek)]
    fn test_custom_tool_sleek_sql_d16819f4564d8853() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT  *                  FROM  tbl
                        WHERE                      foo   = 'bar';         "#;

        let output = r#"SELECT
    *
FROM
    tbl
WHERE
    foo = 'bar';"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "sleek".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_smlfmt {
    #[test_with::executable(smlfmt)]
    fn test_smlfmt_sml_ca3c4a53d8aa2d76() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"fun add(a:int, b: int )= a+b
"#;

        let output = r#"fun add (a: int, b: int) = a + b
"#;

        let ft = ".sml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Smlfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(smlfmt)]
    fn test_custom_tool_smlfmt_sml_ca3c4a53d8aa2d76() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"fun add(a:int, b: int )= a+b
"#;

        let output = r#"fun add (a: int, b: int) = a + b
"#;

        let ft = ".sml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "smlfmt".to_owned(),
                arguments: vec!["--force".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_snakefmt {
    #[test_with::executable(snakefmt || pipx || uv)]
    fn test_snakefmt_snakemake_cdccd086422a6b0a() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"from snakemake.utils import min_version
min_version("5.14.0")
configfile: "config.yaml"
include: "rules/foo.smk"
"#;

        let output = r#"from snakemake.utils import min_version

min_version("5.14.0")


configfile: "config.yaml"


include: "rules/foo.smk"
"#;

        let ft = "snakemake";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Snakefmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(snakefmt)]
    fn test_custom_tool_snakefmt_snakemake_cdccd086422a6b0a()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"from snakemake.utils import min_version
min_version("5.14.0")
configfile: "config.yaml"
include: "rules/foo.smk"
"#;

        let output = r#"from snakemake.utils import min_version

min_version("5.14.0")


configfile: "config.yaml"


include: "rules/foo.smk"
"#;

        let ft = "snakemake";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "snakefmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_sql_formatter {
    #[test_with::executable(sql-formatter || bunx || deno || npx || pnpm || yarn)]
    fn test_sql_formatter_sql_85ac36a4bf14f957() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT * FROM tbl WHERE foo = 'bar';"#;

        let output = r#"SELECT
  *
FROM
  tbl
WHERE
  foo = 'bar';
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::SqlFormatter),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(sql-formatter)]
    fn test_custom_tool_sql_formatter_sql_85ac36a4bf14f957()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT * FROM tbl WHERE foo = 'bar';"#;

        let output = r#"SELECT
  *
FROM
  tbl
WHERE
  foo = 'bar';
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "sql-formatter".to_owned(),
                arguments: vec!["--fix".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_sqlfluff_fix {
    #[test_with::executable(sqlfluff || pipx || uv)]
    fn test_sqlfluff_fix_sql_1d0ce1bc071aed78() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT  id                  FROM  tbl
                        WHERE                      foo   = 'bar' ;
"#;

        let output = r#"SELECT id FROM tbl
WHERE foo = 'bar';
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::SqlfluffFix),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(sqlfluff)]
    fn test_custom_tool_sqlfluff_sql_1d0ce1bc071aed78() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT  id                  FROM  tbl
                        WHERE                      foo   = 'bar' ;
"#;

        let output = r#"SELECT id FROM tbl
WHERE foo = 'bar';
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "sqlfluff".to_owned(),
                arguments: vec![
                    "fix".to_owned(),
                    "--disable-progress-bar".to_owned(),
                    "--nocolor".to_owned(),
                    "--dialect".to_owned(),
                    "ansi".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_sqlfluff_format {
    #[test_with::executable(sqlfluff || pipx || uv)]
    fn test_sqlfluff_format_sql_498b1dc9b48f9b5d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT  id                  FROM  tbl
                        WHERE                      foo   = 'bar';         "#;

        let output = r#"SELECT id FROM tbl
WHERE foo = 'bar';
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::SqlfluffFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(sqlfluff)]
    fn test_custom_tool_sqlfluff_sql_498b1dc9b48f9b5d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT  id                  FROM  tbl
                        WHERE                      foo   = 'bar';         "#;

        let output = r#"SELECT id FROM tbl
WHERE foo = 'bar';
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "sqlfluff".to_owned(),
                arguments: vec![
                    "format".to_owned(),
                    "--disable-progress-bar".to_owned(),
                    "--nocolor".to_owned(),
                    "--dialect".to_owned(),
                    "ansi".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_sqlfluff_lint {
    #[test_with::executable(sqlfluff || pipx || uv)]
    fn test_sqlfluff_lint_sql_9ec8d0c58d20cc30() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT asd FROM tbl
WHERE foo = 'bar' LIMIT 10;
"#;

        let output = r#"SELECT asd FROM tbl
WHERE foo = 'bar' LIMIT 10;
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::SqlfluffLint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(sqlfluff)]
    fn test_custom_tool_sqlfluff_sql_9ec8d0c58d20cc30() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT asd FROM tbl
WHERE foo = 'bar' LIMIT 10;
"#;

        let output = r#"SELECT asd FROM tbl
WHERE foo = 'bar' LIMIT 10;
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "sqlfluff".to_owned(),
                arguments: vec![
                    "lint".to_owned(),
                    "--disable-progress-bar".to_owned(),
                    "--nocolor".to_owned(),
                    "--dialect".to_owned(),
                    "ansi".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_sqlfmt {
    #[test_with::executable(sqlfmt || pipx || uv)]
    fn test_sqlfmt_sql_7933045821741e3() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT  *                  FROM  tbl                        WHERE                      foo   = 'bar';"#;

        let output = r#"select *
from tbl
where foo = 'bar'
;
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Sqlfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(sqlfmt)]
    fn test_custom_tool_sqlfmt_sql_7933045821741e3() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT  *                  FROM  tbl                        WHERE                      foo   = 'bar';"#;

        let output = r#"select *
from tbl
where foo = 'bar'
;
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "sqlfmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_sqlint {
    #[test_with::executable(sqlint || gem)]
    fn test_sqlint_sql_590c277c204e093c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT username FROM users
WHERE id = 1;
"#;

        let output = r#"SELECT username FROM users
WHERE id = 1;
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Sqlint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(sqlint)]
    fn test_custom_tool_sqlint_sql_590c277c204e093c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT username FROM users
WHERE id = 1;
"#;

        let output = r#"SELECT username FROM users
WHERE id = 1;
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "sqlint".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_sqruff {
    #[test_with::executable(sqruff || pipx || uv)]
    fn test_sqruff_sql_c48780a07bf33db() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT          * from dummy where Name     > 10
"#;

        let output = r#"SELECT * FROM dummy WHERE name > 10
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Sqruff),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(sqruff)]
    fn test_custom_tool_sqruff_sql_c48780a07bf33db() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT          * from dummy where Name     > 10
"#;

        let output = r#"SELECT * FROM dummy WHERE name > 10
"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "sqruff".to_owned(),
                arguments: vec!["fix".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_squawk {
    #[test_with::executable(squawk || bunx || deno || npx || pnpm || yarn || pipx || uv)]
    fn test_squawk_sql_640a48e4cd6b38bb() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT username FROM users;"#;

        let output = r#"SELECT username FROM users;"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Squawk),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(squawk)]
    fn test_custom_tool_squawk_sql_640a48e4cd6b38bb() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"SELECT username FROM users;"#;

        let output = r#"SELECT username FROM users;"#;

        let ft = "sql";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "squawk".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_standardjs {
    #[test_with::executable(standard || bunx || deno || npx || pnpm || yarn)]
    fn test_standardjs_javascript_548a80949cde541f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(a,b  )
    {
        return a+b
    }

console.info(asyncAddition(1, 2));
            "#;

        let output = r#"async function asyncAddition (a, b) {
  return a + b
}

console.info(asyncAddition(1, 2))
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Standardjs),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(standard)]
    fn test_custom_tool_standard_javascript_548a80949cde541f()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    async function asyncAddition(a,b  )
    {
        return a+b
    }

console.info(asyncAddition(1, 2));
            "#;

        let output = r#"async function asyncAddition (a, b) {
  return a + b
}

console.info(asyncAddition(1, 2))
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "standard".to_owned(),
                arguments: vec!["--fix".to_owned(), "--stdin".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_standardrb {
    #[test_with::executable(standardrb || gem)]
    fn test_standardrb_ruby_bec6c50c1664b6ed() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  a + b
end
"#;

        let ft = "ruby";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Standardrb),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(standardrb)]
    fn test_custom_tool_standardrb_ruby_bec6c50c1664b6ed() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"def   add(  a ,                                                          b )
                        return a + b
                end"#;

        let output = r#"def add(a, b)
  a + b
end
"#;

        let ft = "ruby";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "standardrb".to_owned(),
                arguments: vec!["--fix".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_stylefmt {
    #[test_with::executable(stylefmt || bunx || deno || npx || pnpm || yarn)]
    fn test_stylefmt_css_ed4f8407afa6d974() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"/* custom properties */
:root{--fontSize: 1rem;
  --mainColor       :#12345678;
--highlightColor:hwb(190, 35%, 20%);
}

/* custom media queries */
@custom-media

--viewport-medium(width<=50rem);

/* some var() & calc() */
body{color:var(--mainColor);
    font-size:var(--fontSize);
 line-height: calc(var(--fontSize) * 1.5);
padding: calc((var(--fontSize) / 2) + 1px)}

/* custom media query usage */
@media (--viewport-medium) {
body {font-size: calc(var(--fontSize) * 1.2); }
}

/* custom selectors */
@custom-selector :--heading h1,h2,h3,    h4,h5,h6;
:--heading { margin-top:0 }

/* colors stuff */
a{
color:var(--highlightColor);
    transition:color 1s;
}
a:hover{color  :gray(255,50%) }
a:active{color : rebeccapurple }
a:any-link { color:color(var(--highlightColor) blackness(+20%)) }

/* font stuff */
h2 {font-variant-caps:small-caps;
}table{font-variant-numeric: lining-nums;
}

/* filters */
.blur{filter:blur(4px)}.sepia{
filter: sepia(.8);}
"#;

        let output = r#"/* custom properties */
:root {
  --fontSize: 1rem;
  --mainColor: #12345678;
  --highlightColor: hwb(190, 35%, 20%);
}

/* custom media queries */
@custom-media --viewport-medium (width <= 50rem);

/* some var() & calc() */
body {
  color: var(--mainColor);
  font-size: var(--fontSize);
  line-height: calc(var(--fontSize) * 1.5);
  padding: calc((var(--fontSize) / 2) + 1px);
}

/* custom media query usage */
@media (--viewport-medium) {
  body {
    font-size: calc(var(--fontSize) * 1.2);
  }
}

/* custom selectors */
@custom-selector :--heading h1, h2, h3, h4, h5, h6;

:--heading {
  margin-top: 0;
}

/* colors stuff */
a {
  color: var(--highlightColor);
  transition: color 1s;
}

a:hover {
  color: gray(255, 50%);
}

a:active {
  color: rebeccapurple;
}

a:any-link {
  color: color(var(--highlightColor) blackness(+20%));
}

/* font stuff */
h2 {
  font-variant-caps: small-caps;
}

table {
  font-variant-numeric: lining-nums;
}

/* filters */
.blur {
  filter: blur(4px);
}

.sepia {
  filter: sepia(.8);
}
"#;

        let ft = "css";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Stylefmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(stylefmt)]
    fn test_custom_tool_stylefmt_css_ed4f8407afa6d974() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"/* custom properties */
:root{--fontSize: 1rem;
  --mainColor       :#12345678;
--highlightColor:hwb(190, 35%, 20%);
}

/* custom media queries */
@custom-media

--viewport-medium(width<=50rem);

/* some var() & calc() */
body{color:var(--mainColor);
    font-size:var(--fontSize);
 line-height: calc(var(--fontSize) * 1.5);
padding: calc((var(--fontSize) / 2) + 1px)}

/* custom media query usage */
@media (--viewport-medium) {
body {font-size: calc(var(--fontSize) * 1.2); }
}

/* custom selectors */
@custom-selector :--heading h1,h2,h3,    h4,h5,h6;
:--heading { margin-top:0 }

/* colors stuff */
a{
color:var(--highlightColor);
    transition:color 1s;
}
a:hover{color  :gray(255,50%) }
a:active{color : rebeccapurple }
a:any-link { color:color(var(--highlightColor) blackness(+20%)) }

/* font stuff */
h2 {font-variant-caps:small-caps;
}table{font-variant-numeric: lining-nums;
}

/* filters */
.blur{filter:blur(4px)}.sepia{
filter: sepia(.8);}
"#;

        let output = r#"/* custom properties */
:root {
  --fontSize: 1rem;
  --mainColor: #12345678;
  --highlightColor: hwb(190, 35%, 20%);
}

/* custom media queries */
@custom-media --viewport-medium (width <= 50rem);

/* some var() & calc() */
body {
  color: var(--mainColor);
  font-size: var(--fontSize);
  line-height: calc(var(--fontSize) * 1.5);
  padding: calc((var(--fontSize) / 2) + 1px);
}

/* custom media query usage */
@media (--viewport-medium) {
  body {
    font-size: calc(var(--fontSize) * 1.2);
  }
}

/* custom selectors */
@custom-selector :--heading h1, h2, h3, h4, h5, h6;

:--heading {
  margin-top: 0;
}

/* colors stuff */
a {
  color: var(--highlightColor);
  transition: color 1s;
}

a:hover {
  color: gray(255, 50%);
}

a:active {
  color: rebeccapurple;
}

a:any-link {
  color: color(var(--highlightColor) blackness(+20%));
}

/* font stuff */
h2 {
  font-variant-caps: small-caps;
}

table {
  font-variant-numeric: lining-nums;
}

/* filters */
.blur {
  filter: blur(4px);
}

.sepia {
  filter: sepia(.8);
}
"#;

        let ft = "css";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "stylefmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(stylefmt || bunx || deno || npx || pnpm || yarn)]
    fn test_stylefmt_scss_d3c6918bf17af7f3() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"// mixin for clearfix


        @mixin      clearfix    ()      { &:before,
  &:after {
                content:" ";
    display              : table;  }

  &:after        {clear: both;}
   }.class
{
       padding:10px;@include        clearfix();}
     .base {  color: red;  }

// placeholder
%base
{


padding: 12px
}

.foo{
@extend      .base;}

.bar
      {     @extend            %base;

}
"#;

        let output = r#"// mixin for clearfix


@mixin clearfix() {
  &:before,
  &:after {
    content: " ";
    display: table;
  }

  &:after {
    clear: both;
  }
}

.class {
  padding: 10px;
  @include clearfix();
}

.base {
  color: red;
}

// placeholder
%base {
  padding: 12px;
}

.foo {
  @extend .base;
}

.bar {
  @extend %base;
}
"#;

        let ft = "scss";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Stylefmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(stylefmt)]
    fn test_custom_tool_stylefmt_scss_d3c6918bf17af7f3() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"// mixin for clearfix


        @mixin      clearfix    ()      { &:before,
  &:after {
                content:" ";
    display              : table;  }

  &:after        {clear: both;}
   }.class
{
       padding:10px;@include        clearfix();}
     .base {  color: red;  }

// placeholder
%base
{


padding: 12px
}

.foo{
@extend      .base;}

.bar
      {     @extend            %base;

}
"#;

        let output = r#"// mixin for clearfix


@mixin clearfix() {
  &:before,
  &:after {
    content: " ";
    display: table;
  }

  &:after {
    clear: both;
  }
}

.class {
  padding: 10px;
  @include clearfix();
}

.base {
  color: red;
}

// placeholder
%base {
  padding: 12px;
}

.foo {
  @extend .base;
}

.bar {
  @extend %base;
}
"#;

        let ft = "scss";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "stylefmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_stylish_haskell {
    #[test_with::executable(stylish-haskell)]
    fn test_stylish_haskell_haskell_9589647c4239e2dd() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers::Int->Int->Int
addNumbers a b = do
        a + b

"#;

        let ft = "haskell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::StylishHaskell),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(stylish-haskell)]
    fn test_custom_tool_stylish_haskell_haskell_9589647c4239e2dd()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"addNumbers::Int->Int->Int
addNumbers a b = do
        a + b
        "#;

        let output = r#"addNumbers::Int->Int->Int
addNumbers a b = do
        a + b

"#;

        let ft = "haskell";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "stylish-haskell".to_owned(),
                arguments: vec!["--inplace".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_stylua {
    #[test_with::executable(stylua || bunx || deno || npx || pnpm || yarn)]
    fn test_stylua_lua_ab45775f0dc2fcca() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"

        local               function        add (                                       a , b
)

return              a +b


end

    "#;

        let output = r#"local function add(a, b)
	return a + b
end
"#;

        let ft = "lua";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Stylua),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(stylua)]
    fn test_custom_tool_stylua_lua_ab45775f0dc2fcca() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"

        local               function        add (                                       a , b
)

return              a +b


end

    "#;

        let output = r#"local function add(a, b)
	return a + b
end
"#;

        let ft = "lua";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "stylua".to_owned(),
                arguments: vec!["--verify".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_superhtml_fmt {
    #[test_with::executable(superhtml)]
    fn test_superhtml_fmt_html_7a7c8fbd08a556f1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;

        let output = r#"<div>
	<p>
		Mads was here
	</p>
</div>"#;

        let ft = "html";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::SuperhtmlFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(superhtml)]
    fn test_custom_tool_superhtml_html_7a7c8fbd08a556f1() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"<div>
                    <p>
                    Mads was here
                    </p>
        </div>"#;

        let output = r#"<div>
	<p>
		Mads was here
	</p>
</div>"#;

        let ft = "html";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "superhtml".to_owned(),
                arguments: vec!["fmt".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_swift_format {
    #[test_with::executable(swift-format)]
    fn test_swift_format_swift_5717762df3975151() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#" func add(a:Int ,b:Int)->Int {
    return a + b
    }"#;

        let output = r#"func add(a: Int, b: Int) -> Int {
    return a + b
}
"#;

        let ft = "swift";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::SwiftFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(swift-format)]
    fn test_custom_tool_swift_format_swift_5717762df3975151()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#" func add(a:Int ,b:Int)->Int {
    return a + b
    }"#;

        let output = r#"func add(a: Int, b: Int) -> Int {
    return a + b
}
"#;

        let ft = "swift";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "swift-format".to_owned(),
                arguments: vec!["--in-place".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_swiftformat {
    #[test_with::executable(swiftformat)]
    fn test_swiftformat_swift_5717762df3975151() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#" func add(a:Int ,b:Int)->Int {
    return a + b
    }"#;

        let output = r#"func add(a: Int, b: Int) -> Int {
    return a + b
}
"#;

        let ft = "swift";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Swiftformat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(swiftformat)]
    fn test_custom_tool_swiftformat_swift_5717762df3975151()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#" func add(a:Int ,b:Int)->Int {
    return a + b
    }"#;

        let output = r#"func add(a: Int, b: Int) -> Int {
    return a + b
}
"#;

        let ft = "swift";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "swiftformat".to_owned(),
                arguments: vec!["--quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_taplo {
    #[test_with::executable(taplo || bunx || deno || npx || pnpm || yarn)]
    fn test_taplo_toml_f9c7870e88d1963c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"          package         =              "mdsf"
  author   = "Mads Hougesen"
  "#;

        let output = r#"package = "mdsf"
author = "Mads Hougesen"
"#;

        let ft = "toml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Taplo),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(taplo)]
    fn test_custom_tool_taplo_toml_f9c7870e88d1963c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"          package         =              "mdsf"
  author   = "Mads Hougesen"
  "#;

        let output = r#"package = "mdsf"
author = "Mads Hougesen"
"#;

        let ft = "toml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "taplo".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_terraform_fmt {
    #[test_with::executable(terraform)]
    fn test_terraform_fmt_tf_2c1d9f26008080c1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"resource "aws_instance" "example" {
                ami   = "abc123"

           network_interface  {
             }
}
"#;

        let output = r#"resource "aws_instance" "example" {
  ami = "abc123"

  network_interface {
  }
}
"#;

        let ft = "tf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::TerraformFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(terraform)]
    fn test_custom_tool_terraform_tf_2c1d9f26008080c1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"resource "aws_instance" "example" {
                ami   = "abc123"

           network_interface  {
             }
}
"#;

        let output = r#"resource "aws_instance" "example" {
  ami = "abc123"

  network_interface {
  }
}
"#;

        let ft = "tf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "terraform".to_owned(),
                arguments: vec![
                    "fmt".to_owned(),
                    "-write=true".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_tex_fmt {
    #[test_with::executable(tex-fmt)]
    fn test_tex_fmt_latex_1249f3d7d4b15b30() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"\documentclass{article}

\begin{document}

\begin{itemize}
\item Lists with items
over multiple lines
\end{itemize}

\begin{equation}
E = m c^2
\end{equation}

\end{document}"#;

        let output = r#"\documentclass{article}

\begin{document}

\begin{itemize}
  \item Lists with items
    over multiple lines
\end{itemize}

\begin{equation}
  E = m c^2
\end{equation}

\end{document}
"#;

        let ft = "latex";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::TexFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(tex-fmt)]
    fn test_custom_tool_tex_fmt_latex_1249f3d7d4b15b30() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"\documentclass{article}

\begin{document}

\begin{itemize}
\item Lists with items
over multiple lines
\end{itemize}

\begin{equation}
E = m c^2
\end{equation}

\end{document}"#;

        let output = r#"\documentclass{article}

\begin{document}

\begin{itemize}
  \item Lists with items
    over multiple lines
\end{itemize}

\begin{equation}
  E = m c^2
\end{equation}

\end{document}
"#;

        let ft = "latex";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "tex-fmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_tofu_fmt {
    #[test_with::executable(tofu)]
    fn test_tofu_fmt_tf_4ed0c1fa5333c037() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"resource "aws_instance" "example" {
                ami   = "abc123"

           network_interface  {
             }
}
"#;

        let output = r#"resource "aws_instance" "example" {
  ami = "abc123"

  network_interface {
  }
}
"#;

        let ft = ".tf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::TofuFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(tofu)]
    fn test_custom_tool_tofu_tf_4ed0c1fa5333c037() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"resource "aws_instance" "example" {
                ami   = "abc123"

           network_interface  {
             }
}
"#;

        let output = r#"resource "aws_instance" "example" {
  ami = "abc123"

  network_interface {
  }
}
"#;

        let ft = ".tf";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "tofu".to_owned(),
                arguments: vec![
                    "fmt".to_owned(),
                    "-write=true".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_tombi_format {
    #[test_with::executable(tombi || pipx || uv)]
    fn test_tombi_format_toml_fa35f0f5766ac557() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"[project ]
name =     "hello""#;

        let output = r#"[project]
name = "hello"
"#;

        let ft = "toml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::TombiFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(tombi)]
    fn test_custom_tool_tombi_toml_fa35f0f5766ac557() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"[project ]
name =     "hello""#;

        let output = r#"[project]
name = "hello"
"#;

        let ft = "toml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "tombi".to_owned(),
                arguments: vec!["format".to_owned(), "-".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_tombi_lint {
    #[test_with::executable(tombi || pipx || uv)]
    fn test_tombi_lint_toml_249ef29da68d9e6d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"[project]
name = "hello"
"#;

        let output = r#"[project]
name = "hello"
"#;

        let ft = "toml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::TombiLint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(tombi)]
    fn test_custom_tool_tombi_toml_249ef29da68d9e6d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"[project]
name = "hello"
"#;

        let output = r#"[project]
name = "hello"
"#;

        let ft = "toml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "tombi".to_owned(),
                arguments: vec!["lint".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_toml_sort {
    #[test_with::executable(toml-sort || pipx || uv)]
    fn test_toml_sort_toml_8c2b58a6580e9412() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"

[c]
key = "something"


[a]
key = "something"

[b]
key = "something"

"#;

        let output = r#"[a]
key = "something"

[b]
key = "something"

[c]
key = "something"
"#;

        let ft = "toml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::TomlSort),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(toml-sort)]
    fn test_custom_tool_toml_sort_toml_8c2b58a6580e9412() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"

[c]
key = "something"


[a]
key = "something"

[b]
key = "something"

"#;

        let output = r#"[a]
key = "something"

[b]
key = "something"

[c]
key = "something"
"#;

        let ft = "toml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "toml-sort".to_owned(),
                arguments: vec!["-i".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_topiary {
    #[test_with::executable(topiary)]
    fn test_topiary_json_d426a9ade74002d2() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Topiary),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(topiary)]
    fn test_custom_tool_topiary_json_d426a9ade74002d2() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
              {
              "key": "value",
  "key2": [
      "value2",
      "value3",
      1
            , null]
 }
  "#;

        let output = r#"{
  "key": "value",
  "key2": [
    "value2",
    "value3",
    1,
    null
  ]
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "topiary".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_tsp_format {
    #[test_with::executable(tsp || bunx || deno || npx || pnpm || yarn)]
    fn test_tsp_format_typespec_f4c58025c5f05edc() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"model Pet {  name: string;  age: int32;kind: "dog" | "cat" | "fish";}
"#;

        let output = r#"model Pet {
  name: string;
  age: int32;
  kind: "dog" | "cat" | "fish";
}
"#;

        let ft = "typespec";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::TspFormat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(tsp)]
    fn test_custom_tool_tsp_typespec_f4c58025c5f05edc() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"model Pet {  name: string;  age: int32;kind: "dog" | "cat" | "fish";}
"#;

        let output = r#"model Pet {
  name: string;
  age: int32;
  kind: "dog" | "cat" | "fish";
}
"#;

        let ft = "typespec";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "tsp".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_ty {
    #[test_with::executable(ty || pipx || uv)]
    fn test_ty_python_13af245604aaa0cd() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b


add(1, 2)
"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b


add(1, 2)
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Ty),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(ty)]
    fn test_custom_tool_ty_python_13af245604aaa0cd() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add(a: int, b: int) -> int:
    return a + b


add(1, 2)
"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b


add(1, 2)
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "ty".to_owned(),
                arguments: vec!["check".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_typos {
    #[test_with::executable(typos)]
    fn test_typos_python_cba663e4f5e54b7f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"anouncement"#;

        let output = r#"announcement"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Typos),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(typos)]
    fn test_custom_tool_typos_python_cba663e4f5e54b7f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"anouncement"#;

        let output = r#"announcement"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "typos".to_owned(),
                arguments: vec![
                    "-w".to_owned(),
                    "--no-ignore".to_owned(),
                    "--hidden".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_ufmt {
    #[test_with::executable(ufmt || pipx || uv)]
    fn test_ufmt_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Ufmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(ufmt)]
    fn test_custom_tool_ufmt_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "ufmt".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_uiua_fmt {
    #[test_with::executable(uiua)]
    fn test_uiua_fmt_uiua_df0f003704c81512() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"∿[1        5     8    2]
"#;

        let output = r#"∿[1 5 8 2]
"#;

        let ft = "uiua";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::UiuaFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(uiua)]
    fn test_custom_tool_uiua_uiua_df0f003704c81512() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"∿[1        5     8    2]
"#;

        let output = r#"∿[1 5 8 2]
"#;

        let ft = "uiua";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "uiua".to_owned(),
                arguments: vec!["fmt".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_unimport {
    #[test_with::executable(unimport || pipx || uv)]
    fn test_unimport_python_3940fba56a9a47fc() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"from typing import Optional
"#;

        let output = r#"
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Unimport),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(unimport)]
    fn test_custom_tool_unimport_python_3940fba56a9a47fc() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"from typing import Optional
"#;

        let output = r#"
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "unimport".to_owned(),
                arguments: vec!["-r".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_usort {
    #[test_with::executable(usort || pipx || uv)]
    fn test_usort_python_e2ac93e0195d9bc1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"from q import d
import b
import a
import c


def add(a: int, b: int) -> int:
  return a + b
"#;

        let output = r#"import a
import b
import c
from q import d


def add(a: int, b: int) -> int:
  return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Usort),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(usort)]
    fn test_custom_tool_usort_python_e2ac93e0195d9bc1() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"from q import d
import b
import a
import c


def add(a: int, b: int) -> int:
  return a + b
"#;

        let output = r#"import a
import b
import c
from q import d


def add(a: int, b: int) -> int:
  return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "usort".to_owned(),
                arguments: vec!["format".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_verusfmt {
    #[test_with::executable(verusfmt)]
    fn test_verusfmt_rust_70ad564760e773e9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    "#;

        let output = r#"pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let ft = "rust";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Verusfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(verusfmt)]
    fn test_custom_tool_verusfmt_rust_70ad564760e773e9() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    "#;

        let output = r#"pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let ft = "rust";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "verusfmt".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_veryl_fmt {
    #[test_with::executable(veryl)]
    fn test_veryl_fmt_veryl_529de9cf882c5a00() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"/// documentation comment by markdown format
/// * list item1
/// * list item2
pub module Delay #( // visibility control by `pub` keyword
              param WIDTH:                 u32 = 1, // trailing comma is allowed
) (
    i_clk : input clock       ,
              i_rst : input reset       ,
    i_data: input logic<WIDTH>,
     o_data: input logic<WIDTH>,
)            {
    // unused variable which is not started with `_` are warned
              var _unused_variable: logic;

    // clock and reset signals can be omitted
    // because Veryl can infer these signals
                      always_ff              {
        // abstraction syntax of reset polarity and synchronicity
        if_reset {
            o_data = '0;
        } else {
            o_data = i_data;
        }
    }
}
"#;

        let output = r#"/// documentation comment by markdown format
/// * list item1
/// * list item2
pub module Delay #( // visibility control by `pub` keyword
    param WIDTH: u32 = 1, // trailing comma is allowed
) (
    i_clk : input clock       ,
    i_rst : input reset       ,
    i_data: input logic<WIDTH>,
    o_data: input logic<WIDTH>,
) {
    // unused variable which is not started with `_` are warned
    var _unused_variable: logic;

    // clock and reset signals can be omitted
    // because Veryl can infer these signals
    always_ff {
        // abstraction syntax of reset polarity and synchronicity
        if_reset {
            o_data = '0;
        } else {
            o_data = i_data;
        }
    }
}
"#;

        let ft = "veryl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::VerylFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(veryl)]
    fn test_custom_tool_veryl_veryl_529de9cf882c5a00() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"/// documentation comment by markdown format
/// * list item1
/// * list item2
pub module Delay #( // visibility control by `pub` keyword
              param WIDTH:                 u32 = 1, // trailing comma is allowed
) (
    i_clk : input clock       ,
              i_rst : input reset       ,
    i_data: input logic<WIDTH>,
     o_data: input logic<WIDTH>,
)            {
    // unused variable which is not started with `_` are warned
              var _unused_variable: logic;

    // clock and reset signals can be omitted
    // because Veryl can infer these signals
                      always_ff              {
        // abstraction syntax of reset polarity and synchronicity
        if_reset {
            o_data = '0;
        } else {
            o_data = i_data;
        }
    }
}
"#;

        let output = r#"/// documentation comment by markdown format
/// * list item1
/// * list item2
pub module Delay #( // visibility control by `pub` keyword
    param WIDTH: u32 = 1, // trailing comma is allowed
) (
    i_clk : input clock       ,
    i_rst : input reset       ,
    i_data: input logic<WIDTH>,
    o_data: input logic<WIDTH>,
) {
    // unused variable which is not started with `_` are warned
    var _unused_variable: logic;

    // clock and reset signals can be omitted
    // because Veryl can infer these signals
    always_ff {
        // abstraction syntax of reset polarity and synchronicity
        if_reset {
            o_data = '0;
        } else {
            o_data = i_data;
        }
    }
}
"#;

        let ft = "veryl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "veryl".to_owned(),
                arguments: vec!["fmt".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_vhdl_style_guide {
    #[test_with::executable(vsg || pipx || uv)]
    fn test_vhdl_style_guide_vhdl_a9606d208e8f0a57() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
architecture RTL of FIFO is

  constant c_width : integer := 16;
  constant c_depth :   integer := 512;
  constant c_word :integer := 1024;

begin

end architecture RTL;"#;

        let output = r#"
architecture rtl of fifo is

  constant c_width : integer := 16;
  constant c_depth : integer := 512;
  constant c_word  : integer := 1024;

begin

end architecture rtl;
"#;

        let ft = "vhdl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::VhdlStyleGuide),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(vsg)]
    fn test_custom_tool_vsg_vhdl_a9606d208e8f0a57() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
architecture RTL of FIFO is

  constant c_width : integer := 16;
  constant c_depth :   integer := 512;
  constant c_word :integer := 1024;

begin

end architecture RTL;"#;

        let output = r#"
architecture rtl of fifo is

  constant c_width : integer := 16;
  constant c_depth : integer := 512;
  constant c_word  : integer := 1024;

begin

end architecture rtl;
"#;

        let ft = "vhdl";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "vsg".to_owned(),
                arguments: vec!["-f".to_owned(), "$PATH".to_owned(), "--fix".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_wfindent {
    #[test_with::executable(wfindent)]
    fn test_wfindent_fortran_a51b7de807928738() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"program demo
integer :: endif,if,elseif
integer,DIMENSION(2) :: function
endif=3;if=2
if(endif==2)then
endif=5
elseif=if+4*(endif+&
2**10)
elseif(endif==3)then
function(if)=endif/elseif
print*,endif
endif
end program
"#;

        let output = r#"program demo
   integer :: endif,if,elseif
   integer,DIMENSION(2) :: function
   endif=3;if=2
   if(endif==2)then
      endif=5
      elseif=if+4*(endif+&
         2**10)
   elseif(endif==3)then
      function(if)=endif/elseif
      print*,endif
   endif
end program
"#;

        let ft = "fortran";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Wfindent),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(wfindent)]
    fn test_custom_tool_wfindent_fortran_a51b7de807928738()
    -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"program demo
integer :: endif,if,elseif
integer,DIMENSION(2) :: function
endif=3;if=2
if(endif==2)then
endif=5
elseif=if+4*(endif+&
2**10)
elseif(endif==3)then
function(if)=endif/elseif
print*,endif
endif
end program
"#;

        let output = r#"program demo
   integer :: endif,if,elseif
   integer,DIMENSION(2) :: function
   endif=3;if=2
   if(endif==2)then
      endif=5
      elseif=if+4*(endif+&
         2**10)
   elseif(endif==3)then
      function(if)=endif/elseif
      print*,endif
   endif
end program
"#;

        let ft = "fortran";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "wfindent".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_xmlformat {
    #[test_with::executable(xmlformat || pipx || uv)]
    fn test_xmlformat_xml_5e39abb678e63c0b() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>"#;

        let output = r#"<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>"#;

        let ft = "xml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Xmlformat),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(xmlformat)]
    fn test_custom_tool_xmlformat_xml_5e39abb678e63c0b() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>"#;

        let output = r#"<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>"#;

        let ft = "xml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "xmlformat".to_owned(),
                arguments: vec!["--overwrite".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_xmllint {
    #[test_with::executable(xmllint)]
    fn test_xmllint_xml_29dedc18db9d2e97() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>"#;

        let output = r#"<?xml version="1.0"?>
<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>
"#;

        let ft = "xml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Xmllint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(xmllint)]
    fn test_custom_tool_xmllint_xml_29dedc18db9d2e97() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
<note>
  <to>Tove</to>
          <from>Jani</from>
      <heading>Reminder</heading>
        <body>Don't forget me this weekend!</body>
   </note>"#;

        let output = r#"<?xml version="1.0"?>
<note>
  <to>Tove</to>
  <from>Jani</from>
  <heading>Reminder</heading>
  <body>Don't forget me this weekend!</body>
</note>
"#;

        let ft = "xml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "xmllint".to_owned(),
                arguments: vec![
                    "--format".to_owned(),
                    "$PATH".to_owned(),
                    "--output".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_xo {
    #[test_with::executable(xo || bunx || deno || npx || pnpm || yarn)]
    fn test_xo_javascript_77a8cbfa8cbcea9d() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"    function asyncAddition(a,b  )
    {
        return a+b
    }

                    console.info(asyncAddition(1, 2));"#;

        let output = r#"function asyncAddition(a, b) {
	return a + b;
}

console.info(asyncAddition(1, 2));
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Xo),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(xo)]
    fn test_custom_tool_xo_javascript_77a8cbfa8cbcea9d() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"    function asyncAddition(a,b  )
    {
        return a+b
    }

                    console.info(asyncAddition(1, 2));"#;

        let output = r#"function asyncAddition(a, b) {
	return a + b;
}

console.info(asyncAddition(1, 2));
"#;

        let ft = "javascript";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "xo".to_owned(),
                arguments: vec!["--fix".to_owned(), "--stdin".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_xq {
    #[test_with::executable(xq)]
    fn test_xq_xml_1289078d9c0aa8a3() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<?xml version="1.0"?> <catalog>    <book id="bk112">       <author>Galos, Mike</author>       <title>Visual Studio 7: A Comprehensive Guide</title>       <genre>Computer</genre>       <price>49.95</price>       <publish_date>2001-04-16</publish_date>       <description>Microsoft Visual Studio 7 is explored in depth,       looking at how Visual Basic, Visual C++, C#, and ASP+ are        integrated into a comprehensive development        environment.</description>    </book> </catalog>"#;

        let output = r#"<?xml version="1.0"?>
<catalog>
  <book id="bk112">
    <author>Galos, Mike</author>
    <title>Visual Studio 7: A Comprehensive Guide</title>
    <genre>Computer</genre>
    <price>49.95</price>
    <publish_date>2001-04-16</publish_date>
    <description>Microsoft Visual Studio 7 is explored in depth,       looking at how Visual Basic, Visual C++, C#, and ASP+ are        integrated into a comprehensive development        environment.</description>
  </book>
</catalog>
"#;

        let ft = "xml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Xq),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(xq)]
    fn test_custom_tool_xq_xml_1289078d9c0aa8a3() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<?xml version="1.0"?> <catalog>    <book id="bk112">       <author>Galos, Mike</author>       <title>Visual Studio 7: A Comprehensive Guide</title>       <genre>Computer</genre>       <price>49.95</price>       <publish_date>2001-04-16</publish_date>       <description>Microsoft Visual Studio 7 is explored in depth,       looking at how Visual Basic, Visual C++, C#, and ASP+ are        integrated into a comprehensive development        environment.</description>    </book> </catalog>"#;

        let output = r#"<?xml version="1.0"?>
<catalog>
  <book id="bk112">
    <author>Galos, Mike</author>
    <title>Visual Studio 7: A Comprehensive Guide</title>
    <genre>Computer</genre>
    <price>49.95</price>
    <publish_date>2001-04-16</publish_date>
    <description>Microsoft Visual Studio 7 is explored in depth,       looking at how Visual Basic, Visual C++, C#, and ASP+ are        integrated into a comprehensive development        environment.</description>
  </book>
</catalog>
"#;

        let ft = "xml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "xq".to_owned(),
                arguments: vec![],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_xq_html {
    #[test_with::executable(xq)]
    fn test_xq_html_html_a308d301db0ed4af() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<div>          </div>"#;

        let output = r#"<div></div>
"#;

        let ft = "html";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::XqHtml),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(xq)]
    fn test_custom_tool_xq_html_a308d301db0ed4af() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"<div>          </div>"#;

        let output = r#"<div></div>
"#;

        let ft = "html";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "xq".to_owned(),
                arguments: vec!["--html".to_owned()],
                stdin: true,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_yamlfix {
    #[test_with::executable(yamlfix || pipx || uv)]
    fn test_yamlfix_yaml_9fcbc943bcaf9d7f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"


version:                                                                             2
updates:
  - package-ecosystem:                    "cargo"
    directory:  "/"
    schedule:
      interval:     "monthly"
    assignees:
      -     "hougesen"
    open-pull-requests-limit:       25

  - package-ecosystem:                              "github-actions"
    directory:          "/"
    schedule:
        interval:          "monthly"
    assignees:
        - "hougesen"
    open-pull-requests-limit: 25


        "#;

        let output = r#"---
version: 2
updates:
  - package-ecosystem: cargo
    directory: /
    schedule:
      interval: monthly
    assignees: [hougesen]
    open-pull-requests-limit: 25
  - package-ecosystem: github-actions
    directory: /
    schedule:
      interval: monthly
    assignees: [hougesen]
    open-pull-requests-limit: 25
"#;

        let ft = "yaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Yamlfix),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(yamlfix)]
    fn test_custom_tool_yamlfix_yaml_9fcbc943bcaf9d7f() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"


version:                                                                             2
updates:
  - package-ecosystem:                    "cargo"
    directory:  "/"
    schedule:
      interval:     "monthly"
    assignees:
      -     "hougesen"
    open-pull-requests-limit:       25

  - package-ecosystem:                              "github-actions"
    directory:          "/"
    schedule:
        interval:          "monthly"
    assignees:
        - "hougesen"
    open-pull-requests-limit: 25


        "#;

        let output = r#"---
version: 2
updates:
  - package-ecosystem: cargo
    directory: /
    schedule:
      interval: monthly
    assignees: [hougesen]
    open-pull-requests-limit: 25
  - package-ecosystem: github-actions
    directory: /
    schedule:
      interval: monthly
    assignees: [hougesen]
    open-pull-requests-limit: 25
"#;

        let ft = "yaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "yamlfix".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_yamlfmt {
    #[test_with::executable(yamlfmt)]
    fn test_yamlfmt_yaml_5f37046bfdc59220() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"


version:                                                                             2
updates:
  - package-ecosystem:                    "cargo"
    directory:  "/"
    schedule:
      interval:     "monthly"
    assignees:
      -     "hougesen"
    open-pull-requests-limit:       25

  - package-ecosystem:                              "github-actions"
    directory:          "/"
    schedule:
        interval:          "monthly"
    assignees:
        - "hougesen"
    open-pull-requests-limit: 25


        "#;

        let output = r#"version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "monthly"
    assignees:
      - "hougesen"
    open-pull-requests-limit: 25
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "monthly"
    assignees:
      - "hougesen"
    open-pull-requests-limit: 25
"#;

        let ft = "yaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Yamlfmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(yamlfmt)]
    fn test_custom_tool_yamlfmt_yaml_5f37046bfdc59220() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"


version:                                                                             2
updates:
  - package-ecosystem:                    "cargo"
    directory:  "/"
    schedule:
      interval:     "monthly"
    assignees:
      -     "hougesen"
    open-pull-requests-limit:       25

  - package-ecosystem:                              "github-actions"
    directory:          "/"
    schedule:
        interval:          "monthly"
    assignees:
        - "hougesen"
    open-pull-requests-limit: 25


        "#;

        let output = r#"version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "monthly"
    assignees:
      - "hougesen"
    open-pull-requests-limit: 25
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "monthly"
    assignees:
      - "hougesen"
    open-pull-requests-limit: 25
"#;

        let ft = "yaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "yamlfmt".to_owned(),
                arguments: vec!["-quiet".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_yamllint {
    #[test_with::executable(yamllint || pipx || uv)]
    fn test_yamllint_yaml_e7ca97ee9ae56e12() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"---
name: action
on: [push]
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let output = r#"---
name: action
on: [push]
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let ft = "yaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Yamllint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(yamllint)]
    fn test_custom_tool_yamllint_yaml_e7ca97ee9ae56e12() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"---
name: action
on: [push]
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let output = r#"---
name: action
on: [push]
jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - run: mdsf format .
"#;

        let ft = "yaml";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "yamllint".to_owned(),
                arguments: vec!["$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_yapf {
    #[test_with::executable(yapf || pipx || uv)]
    fn test_yapf_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Yapf),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(yapf)]
    fn test_custom_tool_yapf_python_229ec2b01c2bfe3c() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"def add( a: int ,  b:int)->int: return a+b"#;

        let output = r#"def add(a: int, b: int) -> int:
    return a + b
"#;

        let ft = "python";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "yapf".to_owned(),
                arguments: vec!["--in-place".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_yew_fmt {
    #[test_with::executable(yew-fmt)]
    fn test_yew_fmt_rust_70ad564760e773e9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    "#;

        let output = r#"pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let ft = "rust";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::YewFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(yew-fmt)]
    fn test_custom_tool_yew_fmt_rust_70ad564760e773e9() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"pub
                    async
            fn    add( a: i32,
                            b:i32 )->                   i32 {a+b}
    "#;

        let output = r#"pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

        let ft = "rust";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "yew-fmt".to_owned(),
                arguments: vec![
                    "--edition".to_owned(),
                    "2021".to_owned(),
                    "$PATH".to_owned(),
                ],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_yq {
    #[test_with::executable(yq)]
    fn test_yq_json_b20bccf3f90b7945() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{ "yq": "yq"  }"#;

        let output = r#"{
  "yq": "yq"
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Yq),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(yq)]
    fn test_custom_tool_yq_json_b20bccf3f90b7945() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"{ "yq": "yq"  }"#;

        let output = r#"{
  "yq": "yq"
}
"#;

        let ft = "json";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "yq".to_owned(),
                arguments: vec!["--inplace".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_zig_fmt {
    #[test_with::executable(zig)]
    fn test_zig_fmt_zig_8151c333113cef41() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    fn     add   (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }
    "#;

        let output = r#"fn add(a: i32, b: i32) i32 {
    return a + b;
}
"#;

        let ft = "zig";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::ZigFmt),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(zig)]
    fn test_custom_tool_zig_zig_8151c333113cef41() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"
    fn     add   (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }
    "#;

        let output = r#"fn add(a: i32, b: i32) i32 {
    return a + b;
}
"#;

        let ft = "zig";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "zig".to_owned(),
                arguments: vec!["fmt".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

#[cfg(test)]
mod test_zprint {
    #[test_with::executable(zprint)]
    fn test_zprint_clojure_81eb4a785de214e8() -> Result<(), Box<dyn core::error::Error>> {
        let input = r#"(defn change-start-column [new-start-column style-vec [inline-comment-index
  start-column spaces-before :as comment-vec]] (if (zero? inline-comment-index)
  style-vec (let [delta-spaces (- new-start-column start-column) new-spaces
  (+ spaces-before delta-spaces) previous-element-index (dec
  inline-comment-index) [s c e :as previous-element] (nth style-vec
  previous-element-index) new-previous-element (cond (= e :indent) [(str "
"
  (blanks new-spaces)) c e] (= e :whitespace) [(str (blanks new-spaces))
  c e 26] :else nil)] (assoc style-vec previous-element-index
  new-previous-element))))"#;

        let output = r#"(defn change-start-column
  [new-start-column style-vec
   [inline-comment-index start-column spaces-before :as comment-vec]]
  (if (zero? inline-comment-index)
    style-vec
    (let [delta-spaces (- new-start-column start-column)
          new-spaces (+ spaces-before delta-spaces)
          previous-element-index (dec inline-comment-index)
          [s c e :as previous-element] (nth style-vec previous-element-index)
          new-previous-element
            (cond (= e :indent) [(str "
" (blanks new-spaces)) c e]
                  (= e :whitespace) [(str (blanks new-spaces)) c e 26]
                  :else nil)]
      (assoc style-vec previous-element-index new-previous-element))))"#;

        let ft = "clojure";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Preset(mdsf::tools::Tooling::Zprint),
            input,
            output,
            ft,
        )
    }

    #[test_with::executable(zprint)]
    fn test_custom_tool_zprint_clojure_81eb4a785de214e8() -> Result<(), Box<dyn core::error::Error>>
    {
        let input = r#"(defn change-start-column [new-start-column style-vec [inline-comment-index
  start-column spaces-before :as comment-vec]] (if (zero? inline-comment-index)
  style-vec (let [delta-spaces (- new-start-column start-column) new-spaces
  (+ spaces-before delta-spaces) previous-element-index (dec
  inline-comment-index) [s c e :as previous-element] (nth style-vec
  previous-element-index) new-previous-element (cond (= e :indent) [(str "
"
  (blanks new-spaces)) c e] (= e :whitespace) [(str (blanks new-spaces))
  c e 26] :else nil)] (assoc style-vec previous-element-index
  new-previous-element))))"#;

        let output = r#"(defn change-start-column
  [new-start-column style-vec
   [inline-comment-index start-column spaces-before :as comment-vec]]
  (if (zero? inline-comment-index)
    style-vec
    (let [delta-spaces (- new-start-column start-column)
          new-spaces (+ spaces-before delta-spaces)
          previous-element-index (dec inline-comment-index)
          [s c e :as previous-element] (nth style-vec previous-element-index)
          new-previous-element
            (cond (= e :indent) [(str "
" (blanks new-spaces)) c e]
                  (= e :whitespace) [(str (blanks new-spaces)) c e 26]
                  :else nil)]
      (assoc style-vec previous-element-index new-previous-element))))"#;

        let ft = "clojure";

        crate::common::run_tooling_test(
            mdsf::config::MdsfTool::Custom(mdsf::custom::CustomTool {
                binary: "zprint".to_owned(),
                arguments: vec!["-w".to_owned(), "$PATH".to_owned()],
                stdin: false,
            }),
            input,
            output,
            ft,
        )
    }
}

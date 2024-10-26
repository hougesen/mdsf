use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_nixpkgs_fmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("nixpkgs-fmt")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_nixpkgs_fmt_args(cmd.build(), file_path);
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
mod test_nixpkgs_fmt {
    #[test_with::executable(nixpkgs-fmt)]
    fn test_nixpkgs_fmt_nix_80e79e7e2446f823() {
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
        let file_ext = crate::fttype::get_file_extension("nix");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::nixpkgs_fmt::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}

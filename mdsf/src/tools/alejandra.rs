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
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("alejandra")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_alejandra {
    #[test_with::executable(alejandra)]
    fn test_alejandra_nix_f38bff8f20c2aa02() {
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

        let file_ext = crate::fttype::get_file_extension("nix");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::tools::Tooling::Alejandra
            .format_snippet(
                snippet.path(),
                crate::testing::DEFAULT_TEST_FORMATTER_TIMEOUT,
                crate::testing::DEFAULT_TEST_DEBUG_ENABLED,
                &crate::config::MdsfConfigRunners::all(),
            )
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(result, output);
    }
}

///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
fn set_nixfmt_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

const COMMANDS: [CommandType; 1] = [CommandType::Direct("nixfmt")];

#[inline]
pub fn run(
    file_path: &std::path::Path,
    timeout: u64,
) -> Result<(bool, Option<String>), crate::error::MdsfError> {
    crate::execution::run_tools(&COMMANDS, file_path, timeout, set_nixfmt_args)
}

#[cfg(test)]
mod test_nixfmt {
    #[test_with::executable(nixfmt)]
    fn test_nixfmt_nix_dd53cfbdc3725201() {
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
        let output = Some(
            r#"{ lib, buildPythonPackage, fetchFromGitHub, redis }:

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
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("nix");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::nixfmt::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

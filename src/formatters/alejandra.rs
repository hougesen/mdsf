use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_alejandra(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("alejandra");

    cmd.arg("--quiet").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_alejandra {
    use super::format_using_alejandra;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(alejandra)]
    #[test]
    fn it_should_format_nix() {
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

        let expected_output = r#"{
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

        let snippet =
            setup_snippet(input, Language::Nix.to_file_ext()).expect("it to create a snippet file");

        let output = format_using_alejandra(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(expected_output, output);
    }
}

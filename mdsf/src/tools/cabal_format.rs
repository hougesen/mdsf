use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_cabal_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("cabal")];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_cabal_format_args(cmd.build(), file_path);
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
mod test_cabal_format {
    #[test_with::executable(cabal)]
    fn test_cabal_format_cabal_38e9e2aad5619a6a() {
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
        let file_ext = crate::fttype::get_file_extension("cabal");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::cabal_format::run(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");
        assert_eq!(result, output);
    }
}

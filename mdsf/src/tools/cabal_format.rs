///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, runners::CommandType};

#[inline]
fn set_cabal_format_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg("format");
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path, timeout: u64) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [CommandType::Direct("cabal")];

    crate::execution::run_tools(&commands, file_path, timeout, set_cabal_format_args)
}

#[cfg(test)]
mod test_cabal_format {
    #[test_with::executable(cabal)]
    fn test_cabal_format_cabal_8f11b5df65ea57bd() {
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
        let output = Some(
            r#"cabal-version: 2.4
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
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("cabal");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::cabal_format::run(snippet.path(), 0)
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

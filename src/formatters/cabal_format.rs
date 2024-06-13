use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_cabal_format(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("cabal");

    cmd.arg("format").arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_cabal_format {
    use super::format_using_cabal_format;
    use crate::{formatters::setup_snippet, generated::language_to_ext};

    #[test_with::executable(cabal)]
    fn it_should_format_cabal() {
        let input = "cabal-version: 2.4
name: mdsf
version: 0

executable msdf
    default-language: Haskell2010
    hs-source-dirs: src
    main-is: Mdsf.hs
    build-depends: base >=4.11 && <4.13, pretty >=1.1.3.6 && <1.2, bytestring, Cabal ^>=2.5, containers ^>=0.5.11.0 || ^>=0.6.0.1
    other-extensions:
      DeriveFunctor FlexibleContexts ExistentialQuantification OverloadedStrings
      RankNTypes";

        let expected_output = "cabal-version: 2.4
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
";

        let snippet =
            setup_snippet(input, &language_to_ext("cabal")).expect("it to create a snippet file");

        let output = format_using_cabal_format(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}

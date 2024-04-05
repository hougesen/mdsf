use super::execute_command;
use crate::error::MdsfError;

#[inline]
pub fn format_using_fprettify(
    snippet_path: &std::path::Path,
) -> Result<(bool, Option<String>), MdsfError> {
    let mut cmd = std::process::Command::new("fprettify");

    cmd.arg(snippet_path);

    execute_command(&mut cmd, snippet_path)
}

#[cfg(test)]
mod test_fprettify {
    use super::format_using_fprettify;
    use crate::{formatters::setup_snippet, languages::Language};

    #[test_with::executable(fprettify)]
    #[test]
    fn it_should_format_fortran() {
        let input = "program demo
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
end program";

        let expected_output = "program demo
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
";

        let snippet = setup_snippet(input, Language::Fortran.to_file_ext())
            .expect("it to create a snippet file");

        let output = format_using_fprettify(snippet.path())
            .expect("it to be successful")
            .1
            .expect("it to be some");

        assert_eq!(output, expected_output);
    }
}

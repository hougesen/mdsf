///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use crate::runners::CommandType;

#[inline]
pub fn set_args(
    mut cmd: std::process::Command,
    file_path: &std::path::Path,
) -> std::process::Command {
    cmd.arg(file_path);
    cmd
}

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("fprettify")];

#[cfg(test)]
mod test_fprettify {
    const TIMEOUT: u64 = 0;
    const DEBUG_ENABLED: bool = true;

    #[test_with::executable(fprettify)]
    fn test_fprettify_fortran_e500b54621ef1a7a() {
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

        let file_ext = crate::fttype::get_file_extension("fortran");

        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");

        let result = crate::execution::run_tools(
            &super::COMMANDS,
            snippet.path(),
            super::set_args,
            TIMEOUT,
            false,
            DEBUG_ENABLED,
        )
        .expect("it to be successful")
        .1
        .expect("it to be some");

        assert_eq!(result, output);
    }
}

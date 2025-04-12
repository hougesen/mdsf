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

pub const COMMANDS: [CommandType; 3] = [
    CommandType::Direct("fprettify"),
    CommandType::Uv("fprettify", "fprettify"),
    CommandType::Pipx("fprettify"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_fprettify {
    #[test_with::executable(fprettify || pipx || uv)]
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

        crate::tools::Tooling::Fprettify.test_format_snippet(input, output, &file_ext);
    }
}

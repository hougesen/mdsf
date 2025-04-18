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

pub const COMMANDS: [CommandType; 1] = [CommandType::Direct("wfindent")];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_wfindent {
    #[test_with::executable(wfindent)]
    fn test_wfindent_fortran_a51b7de807928738() {
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
end program
"#;

        let output = r#"program demo
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
end program
"#;

        let file_ext = crate::fttype::get_file_extension("fortran");

        crate::tools::Tooling::Wfindent.test_format_snippet(input, output, &file_ext);
    }
}

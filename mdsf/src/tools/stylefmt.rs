///
/// THIS FILE IS GENERATED USING CODE - DO NOT EDIT MANUALLY
///
use std::process::Command;

use crate::{error::MdsfError, execution::execute_command, runners::CommandType};

#[inline]
fn set_stylefmt_args(mut cmd: Command, file_path: &std::path::Path) -> Command {
    cmd.arg(file_path);
    cmd
}

#[inline]
pub fn run(file_path: &std::path::Path) -> Result<(bool, Option<String>), MdsfError> {
    let commands = [
        CommandType::NodeModules("stylefmt"),
        CommandType::Direct("stylefmt"),
        CommandType::Npm("stylefmt"),
    ];

    for (index, cmd) in commands.iter().enumerate() {
        let cmd = set_stylefmt_args(cmd.build(), file_path);
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
mod test_stylefmt {
    #[test_with::executable(npx)]
    fn test_stylefmt_scss_d2c143e65006b912() {
        let input = r#"// mixin for clearfix


        @mixin      clearfix    ()      { &:before,
  &:after {
                content:" ";
    display              : table;  }

  &:after        {clear: both;}
   }.class
{
       padding:10px;@include        clearfix();}
     .base {  color: red;  }

// placeholder
%base
{


padding: 12px
}

.foo{
@extend      .base;}

.bar
      {     @extend            %base;

}
"#;
        let output = Some(
            r#"// mixin for clearfix


@mixin clearfix() {
  &:before,
  &:after {
    content: " ";
    display: table;
  }

  &:after {
    clear: both;
  }
}

.class {
  padding: 10px;
  @include clearfix();
}

.base {
  color: red;
}

// placeholder
%base {
  padding: 12px;
}

.foo {
  @extend .base;
}

.bar {
  @extend %base;
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("scss");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::stylefmt::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }

    #[test_with::executable(npx)]
    fn test_stylefmt_css_d98446828f359199() {
        let input = r#"/* custom properties */
:root{--fontSize: 1rem;
  --mainColor       :#12345678;
--highlightColor:hwb(190, 35%, 20%);
}

/* custom media queries */
@custom-media

--viewport-medium(width<=50rem);

/* some var() & calc() */
body{color:var(--mainColor);
    font-size:var(--fontSize);
 line-height: calc(var(--fontSize) * 1.5);
padding: calc((var(--fontSize) / 2) + 1px)}

/* custom media query usage */
@media (--viewport-medium) {
body {font-size: calc(var(--fontSize) * 1.2); }
}

/* custom selectors */
@custom-selector :--heading h1,h2,h3,    h4,h5,h6;
:--heading { margin-top:0 }

/* colors stuff */
a{
color:var(--highlightColor);
    transition:color 1s;
}
a:hover{color  :gray(255,50%) }
a:active{color : rebeccapurple }
a:any-link { color:color(var(--highlightColor) blackness(+20%)) }

/* font stuff */
h2 {font-variant-caps:small-caps;
}table{font-variant-numeric: lining-nums;
}

/* filters */
.blur{filter:blur(4px)}.sepia{
filter: sepia(.8);}
"#;
        let output = Some(
            r#"/* custom properties */
:root {
  --fontSize: 1rem;
  --mainColor: #12345678;
  --highlightColor: hwb(190, 35%, 20%);
}

/* custom media queries */
@custom-media --viewport-medium (width <= 50rem);

/* some var() & calc() */
body {
  color: var(--mainColor);
  font-size: var(--fontSize);
  line-height: calc(var(--fontSize) * 1.5);
  padding: calc((var(--fontSize) / 2) + 1px);
}

/* custom media query usage */
@media (--viewport-medium) {
  body {
    font-size: calc(var(--fontSize) * 1.2);
  }
}

/* custom selectors */
@custom-selector :--heading h1, h2, h3, h4, h5, h6;

:--heading {
  margin-top: 0;
}

/* colors stuff */
a {
  color: var(--highlightColor);
  transition: color 1s;
}

a:hover {
  color: gray(255, 50%);
}

a:active {
  color: rebeccapurple;
}

a:any-link {
  color: color(var(--highlightColor) blackness(+20%));
}

/* font stuff */
h2 {
  font-variant-caps: small-caps;
}

table {
  font-variant-numeric: lining-nums;
}

/* filters */
.blur {
  filter: blur(4px);
}

.sepia {
  filter: sepia(.8);
}
"#
            .to_owned(),
        );
        let file_ext = crate::fttype::get_file_extension("css");
        let snippet =
            crate::execution::setup_snippet(input, &file_ext).expect("it to create a snippet file");
        let result = crate::tools::stylefmt::run(snippet.path())
            .expect("it to be successful")
            .1;
        assert_eq!(result, output);
    }
}

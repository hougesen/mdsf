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

pub const COMMANDS: [CommandType; 7] = [
    CommandType::NodeModules("stylefmt"),
    CommandType::Direct("stylefmt"),
    CommandType::Npm("stylefmt"),
    CommandType::Pnpm("stylefmt"),
    CommandType::Bun("stylefmt"),
    CommandType::Deno("stylefmt"),
    CommandType::Yarn("stylefmt"),
];

pub const IS_STDIN: bool = false;

#[cfg(test)]
mod test_stylefmt {
    #[test_with::executable(stylefmt || npx || pnpm || deno || bunx)]
    fn test_stylefmt_scss_d3c6918bf17af7f3() {
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

        let output = r#"// mixin for clearfix


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
"#;

        let file_ext = crate::fttype::get_file_extension("scss");

        crate::tools::Tooling::Stylefmt.test_format_snippet(input, output, &file_ext);
    }

    #[test_with::executable(stylefmt || npx || pnpm || deno || bunx)]
    fn test_stylefmt_css_ed4f8407afa6d974() {
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

        let output = r#"/* custom properties */
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
"#;

        let file_ext = crate::fttype::get_file_extension("css");

        crate::tools::Tooling::Stylefmt.test_format_snippet(input, output, &file_ext);
    }
}

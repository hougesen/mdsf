use core::sync::atomic::AtomicBool;

use config::MdsfConfig;
use error::MdsfError;
use formatters::format_snippet;
use languages::Language;
use parser::{parse_generic_codeblock, parse_go_codeblock};
use terminal::{print_changed_line, print_unchanged_file};

pub mod cli;
pub mod config;
pub mod error;
pub mod formatters;
pub mod languages;
mod parser;
pub mod runners;
pub mod terminal;

#[cfg(test)]
pub static DEBUG: AtomicBool = AtomicBool::new(true);
#[cfg(not(test))]
pub static DEBUG: AtomicBool = AtomicBool::new(false);

const GO_TEMPORARY_PACKAGE_NAME: &str = "package mdsfformattertemporarynamespace\n";

#[inline]
fn format_file(config: &MdsfConfig, filename: &std::path::Path, input: &str) -> (bool, String) {
    let mut output = String::with_capacity(input.len() + 128);

    let mut modified = false;

    let mut lines = input.lines().enumerate();

    while let Some((line_index, line)) = lines.next() {
        // TODO: implement support for code blocks with 4 `
        if line.starts_with("```") {
            if let Some(language) = line.strip_prefix("```").and_then(Language::maybe_from_str) {
                let (is_snippet, code_snippet, snippet_lines) = if language == Language::Go {
                    parse_go_codeblock(&mut lines)
                } else {
                    parse_generic_codeblock(&mut lines)
                };

                if is_snippet {
                    let formatted = format_snippet(
                        config,
                        &LineInfo {
                            filename,
                            language,
                            start: line_index + 1,
                            end: line_index + snippet_lines + 1,
                        },
                        &code_snippet,
                    );

                    output.push_str(line);
                    output.push('\n');

                    if language == Language::Go && formatted.contains(GO_TEMPORARY_PACKAGE_NAME) {
                        output.push_str(formatted.replace(GO_TEMPORARY_PACKAGE_NAME, "").trim());
                    } else {
                        output.push_str(formatted.trim());
                    }

                    output.push_str("\n```");

                    if formatted != code_snippet {
                        modified = true;
                    }
                } else {
                    output.push_str(line);
                    output.push_str(&code_snippet);
                }
            } else {
                output.push_str(line);
            }
        } else {
            output.push_str(line);
        }

        output.push('\n');
    }

    if config.format_finished_document && !output.is_empty() {
        output = format_snippet(
            config,
            &LineInfo {
                filename,
                language: Language::Markdown,
                start: 0,
                end: 0,
            },
            &output,
        );
        modified = true;
    }

    (modified, output)
}

#[inline]
pub fn handle_file(config: &MdsfConfig, path: &std::path::Path) -> Result<(), MdsfError> {
    let time = std::time::Instant::now();

    let input = std::fs::read_to_string(path)?;

    if input.is_empty() {
        print_unchanged_file(path, time.elapsed());
        return Ok(());
    }

    let (modified, output) = format_file(config, path, &input);

    if modified && output != input {
        std::fs::write(path, output)?;

        print_changed_line(path, time.elapsed());

        return Ok(());
    }

    print_unchanged_file(path, time.elapsed());

    Ok(())
}

pub struct LineInfo<'a> {
    pub filename: &'a std::path::Path,
    pub language: Language,
    pub start: usize,
    pub end: usize,
}

#[cfg(test)]
impl<'a> LineInfo<'a> {
    pub fn fake() -> Self {
        Self {
            filename: std::path::Path::new("."),
            language: Language::Rust,
            start: 0,
            end: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::MdsfConfig,
        format_file,
        formatters::{setup_snippet, MdsfFormatter},
        handle_file,
        languages::{go::Go, Lang},
    };

    #[test]
    fn it_should_format_the_code() {
        let input = "```rust
fn           add(
     a:
      i32, b: i32) -> i32 {
    a + b
}
```";

        let expected_output = "```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
";

        let config = MdsfConfig::default();

        {
            let (modified, output) = format_file(&config, std::path::Path::new("."), input);

            assert!(modified);

            assert_eq!(output, expected_output);
        };

        {
            let file = setup_snippet(input, ".md").expect("it to create a file");

            handle_file(&config, file.path()).expect("it to be a success");

            let output = std::fs::read_to_string(file.path()).expect("it to return the string");

            assert_eq!(output, expected_output);
        };
    }

    #[test]
    fn it_should_not_modify_outside_blocks() {
        let input = "# title

Let's play!

Have some fun...

I like \"mdsf\"

| Field            | Description            |
|------------------|------------------------|
| foo              |   foo field            |
| bar              |   bar field            |

```rust
fn           add(
     a:
      i32, b: i32) -> i32 {
    a + b
}
```


";

        let expected_output = "# title

Let's play!

Have some fun...

I like \"mdsf\"

| Field            | Description            |
|------------------|------------------------|
| foo              |   foo field            |
| bar              |   bar field            |

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```


";

        let config = MdsfConfig::default();

        let (modified, output) = format_file(&config, std::path::Path::new("."), input);

        assert!(modified);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn it_should_support_multiple_languages() {
        let input = r#"---
tile1: asd asd
tile2: asd asd
tile3: asd asd

tile4: asd asd

tile5: asd asd
---

this is the content

```go
package main

func add(a int, b int) int {
	return a + b
}
```

This snippets is from 'bash.md':

```sh

#!/bin/bash

       add      ()   {
    echo "$1"                 +          "$2"
             }










```

This snippets is from 'crystal.md':

```crystal




def add(a : Int32, b : Int32)
return a + b
end





```

This snippets is from 'css.md':

```css

       body {         background-color: powderblue;       }       h1 {         color: blue;       }       p {         color: red;       }
```

This snippets is from 'dart.md':

```dart

            class Adder {int add(int a, int b) {return a + b;}}


```

This snippets is from 'elixir.md':

```elixir

        def              add(a  ,      b   )   do    a   +   b                 end


```

This snippets is from 'gleam.md':

```gleam


pub fn add(a:Int,b:Int)->Int{a+b}


```

This snippets is from 'go.md':

```go



  package main

   func add(a int , b int  ) int {
                return a + b
       }


```

This snippets is from 'html.md':

```html

 <!doctype html> <html> <head> <style> body {background-color: powderblue;} h1   {color: blue;} p    {color: red;} </style> </head> <body>  <h1>This is a heading</h1> <p>This is a paragraph.</p>  </body> </html>
```

This snippets is from 'javascript.md':

```javascript



function                                add(
                a,
                         b)
                        {
  return a
  +
         b;
                }




```

This snippets is from 'json.md':

```json

                        // This is a comment
{
        "add": { "a":1,
                                "b": ["1",23,null]}
                }



```

This snippets is from 'lua.md':

```lua

        local               function        add (                                       a , b
)

return              a +b


end


```

This snippets is from 'markdown.md':







# this is a header

this             is a paragraph



```lua

        local               function        add (                                       a , b
)

return              a +b


end


```







This snippets is from 'nim.md':

```nim


proc add( a         :int , b:int )        : int =
        return a +          b





```

This snippets is from 'python.md':

```python



def add (
        a  : int ,              b:int )->int :
                    return a                +b





```

This snippets is from 'roc.md':

```roc

app "helloWorld"
    packages { pf: "https://github.com/roc-lang/" }
    imports [pf.Stdout]
    provides [main] to pf






main =
    Stdout.line "Hello, World!"


```

This snippets is from 'ruby.md':

```ruby


def   add(  a , b )
                    return a + b
                        end





```

This snippets is from 'rust.md':

```rust


pub async
    fn          add(
    a:
    i32,

    b: i32)

    -> i32 {
    a +

    b
                }



```

This snippets is from 'sh.md':

```sh

#!/bin/sh

       add      ()   {
    echo "$1"                 +          "$2"
             }










```

This snippets is from 'sql.md':

```sql



            SELECT * FROM
            tbl WHERE foo =

                        'bar';


```

This snippets is from 'toml.md':

```toml


name =          "mdsf"
        author = "Mads Hougesen"


```

This snippets is from 'typescript.md':

```typescript



function                                add(
                a:number,
                         b              :number)
                        :number{
  return a
  +
         b;
        }




```

This snippets is from 'vue.md':

```vue
<script lang="ts"   setup >
import {

    ref
} from "vue"


    const count   = ref(1)
    function add (a:number,b:number):number {
                return a +b
        }   </script>


<template>
    <button  @click="()=> count = add(count,count )">Increment </button>
        </template>


```

This snippets is from 'zig.md':

```zig



fn add (a : i32    , b :   i32 )             i32 {
        return a + b ;

    }


```
"#;

        let expected_output = r#"---
tile1: asd asd
tile2: asd asd
tile3: asd asd

tile4: asd asd

tile5: asd asd
---

this is the content

```go
package main

func add(a int, b int) int {
	return a + b
}
```

This snippets is from 'bash.md':

```sh
#!/bin/bash

add() {
	echo "$1" + "$2"
}
```

This snippets is from 'crystal.md':

```crystal
def add(a : Int32, b : Int32)
  return a + b
end
```

This snippets is from 'css.md':

```css
body {
  background-color: powderblue;
}
h1 {
  color: blue;
}
p {
  color: red;
}
```

This snippets is from 'dart.md':

```dart
class Adder {
  int add(int a, int b) {
    return a + b;
  }
}
```

This snippets is from 'elixir.md':

```elixir
def add(a, b) do
  a + b
end
```

This snippets is from 'gleam.md':

```gleam
pub fn add(a: Int, b: Int) -> Int {
  a + b
}
```

This snippets is from 'go.md':

```go
package main

func add(a int, b int) int {
	return a + b
}
```

This snippets is from 'html.md':

```html
<!doctype html>
<html>
  <head>
    <style>
      body {
        background-color: powderblue;
      }
      h1 {
        color: blue;
      }
      p {
        color: red;
      }
    </style>
  </head>
  <body>
    <h1>This is a heading</h1>
    <p>This is a paragraph.</p>
  </body>
</html>
```

This snippets is from 'javascript.md':

```javascript
function add(a, b) {
  return a + b;
}
```

This snippets is from 'json.md':

```json
// This is a comment
{
  "add": { "a": 1, "b": ["1", 23, null] }
}
```

This snippets is from 'lua.md':

```lua
local function add(a, b)
	return a + b
end
```

This snippets is from 'markdown.md':







# this is a header

this             is a paragraph



```lua
local function add(a, b)
	return a + b
end
```







This snippets is from 'nim.md':

```nim
proc add(a: int, b: int): int =
        return a + b
```

This snippets is from 'python.md':

```python
def add(a: int, b: int) -> int:
    return a + b
```

This snippets is from 'roc.md':

```roc
app [main] { pf: platform "https://github.com/roc-lang/" }

import pf.Stdout

main =
    Stdout.line "Hello, World!"
```

This snippets is from 'ruby.md':

```ruby
def add(a, b)
  return a + b
end
```

This snippets is from 'rust.md':

```rust
pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

This snippets is from 'sh.md':

```sh
#!/bin/sh

add() {
	echo "$1" + "$2"
}
```

This snippets is from 'sql.md':

```sql
SELECT
  *
FROM
  tbl
WHERE
  foo = 'bar';
```

This snippets is from 'toml.md':

```toml
name = "mdsf"
author = "Mads Hougesen"
```

This snippets is from 'typescript.md':

```typescript
function add(a: number, b: number): number {
  return a + b;
}
```

This snippets is from 'vue.md':

```vue
<script lang="ts" setup>
import { ref } from "vue";

const count = ref(1);
function add(a: number, b: number): number {
  return a + b;
}
</script>

<template>
  <button @click="() => (count = add(count, count))">Increment</button>
</template>
```

This snippets is from 'zig.md':

```zig
fn add(a: i32, b: i32) i32 {
    return a + b;
}
```
"#;

        let config = MdsfConfig::default();

        {
            let (modified, output) = format_file(&config, std::path::Path::new("."), input);

            assert!(modified);

            assert_eq!(output, expected_output);
        };

        {
            let file = setup_snippet(input, ".md").expect("it to create a file");

            handle_file(&config, file.path()).expect("it to be a success");

            let output = std::fs::read_to_string(file.path()).expect("it to return the string");

            assert_eq!(output, expected_output);
        };
    }

    #[test]
    fn it_should_support_go_with_package() {
        let input = "```go
package main

import (
\t\"errors\"
\t\"time\"
)

var err = errors.New(\"dddd\")

type Whatever struct {
     StartAt                time.Time  `json:\"start_at\"`
                End                time.Time  `json:\"end_at\"`
    Delete bool `json:-\"`
}
```";

        let expected_output = "```go
package main

import (
\t\"errors\"
\t\"time\"
)

var err = errors.New(\"dddd\")

type Whatever struct {
\tStartAt time.Time `json:\"start_at\"`
\tEnd     time.Time `json:\"end_at\"`
\tDelete  bool      `json:-\"`
}
```
";

        {
            let config = MdsfConfig::default();

            {
                let (modified, output) = format_file(&config, std::path::Path::new("."), input);

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, ".md").expect("it to create a file");

                handle_file(&config, file.path()).expect("it to be a success");

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        };

        {
            let config = MdsfConfig {
                go: Lang::<Go> {
                    enabled: true,
                    formatter: MdsfFormatter::Single(Go::GoFmt),
                },
                ..MdsfConfig::default()
            };

            {
                let (modified, output) = format_file(&config, std::path::Path::new("."), input);

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, ".md").expect("it to create a file");

                handle_file(&config, file.path()).expect("it to be a success");

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        }
    }

    #[test]
    fn it_should_add_go_package_if_missing() {
        let input = "```go
import (
\t\"errors\"
\t\"time\"
)

var err = errors.New(\"dddd\")

type Whatever struct {
     StartAt                time.Time  `json:\"start_at\"`
                End                time.Time  `json:\"end_at\"`
    Delete bool `json:-\"`
}
```";

        let expected_output = "```go
import (
\t\"errors\"
\t\"time\"
)

var err = errors.New(\"dddd\")

type Whatever struct {
\tStartAt time.Time `json:\"start_at\"`
\tEnd     time.Time `json:\"end_at\"`
\tDelete  bool      `json:-\"`
}
```
";

        {
            let config = MdsfConfig::default();

            {
                let (modified, output) = format_file(&config, std::path::Path::new("."), input);

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, ".md").expect("it to create a file");

                handle_file(&config, file.path()).expect("it to be a success");

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        };

        {
            let config = MdsfConfig {
                go: Lang::<Go> {
                    enabled: true,
                    formatter: MdsfFormatter::Single(Go::GoFmt),
                },
                ..MdsfConfig::default()
            };

            {
                let (modified, output) = format_file(&config, std::path::Path::new("."), input);

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, ".md").expect("it to create a file");

                handle_file(&config, file.path()).expect("it to be a success");

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        }
    }

    #[test]
    fn it_should_not_care_if_go_package_is_set() {
        let input = "With package name

```go



  package main

   func add(a int , b int  ) int {
                return a + b
       }


```


Without package name


```go




   func add(a int , b int  ) int {
                return a + b
       }


```

";

        let expected_output = "With package name

```go
package main

func add(a int, b int) int {
	return a + b
}
```


Without package name


```go
func add(a int, b int) int {
	return a + b
}
```

";

        {
            let config = MdsfConfig::default();

            {
                let (modified, output) = format_file(&config, std::path::Path::new("."), input);

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, ".md").expect("it to create a file");

                handle_file(&config, file.path()).expect("it to be a success");

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        };

        {
            let config = MdsfConfig {
                go: Lang::<Go> {
                    enabled: true,
                    formatter: MdsfFormatter::Single(Go::GoFmt),
                },
                ..MdsfConfig::default()
            };

            {
                let (modified, output) = format_file(&config, std::path::Path::new("."), input);

                assert!(modified);

                assert_eq!(output, expected_output);
            };

            {
                let file = setup_snippet(input, ".md").expect("it to create a file");

                handle_file(&config, file.path()).expect("it to be a success");

                let output = std::fs::read_to_string(file.path()).expect("it to return the string");

                assert_eq!(output, expected_output);
            };
        }
    }
}

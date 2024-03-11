use schemars::JsonSchema;

use crate::{config::default_enabled, formatters::prettier::format_using_prettier};

use super::LanguageFormatter;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub enum VueFormatter {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct Vue {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub formatter: VueFormatter,
}

impl Default for Vue {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: VueFormatter::default(),
        }
    }
}

impl LanguageFormatter for Vue {
    #[inline]
    fn format(&self, snippet_path: &std::path::Path) -> std::io::Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        match self.formatter {
            VueFormatter::Prettier => format_using_prettier(snippet_path, true).map(|res| res.1),
        }
    }
}

#[cfg(test)]
mod test_vue {
    use crate::{
        formatters::setup_snippet,
        languages::{Language, LanguageFormatter},
    };

    use super::{Vue, VueFormatter};

    const INPUT: &str = "<script lang=\"ts\"   setup >
import {

    ref
} from \"vue\"


    const count   = ref(1)
    function add (a:number,b:number):number {
                return a +b
        }   </script>


<template>
    <button  @click=\"()=> count = add(count,count )\">Increment </button>
        </template>

";

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Vue {
            enabled: false,
            formatter: VueFormatter::Prettier,
        };

        let snippet =
            setup_snippet(INPUT, Language::Vue.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l.format(snippet_path).expect("it to not fail").is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Vue {
            enabled: true,
            formatter: VueFormatter::Prettier,
        };

        let snippet =
            setup_snippet(INPUT, Language::Vue.to_file_ext()).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path)
            .expect("it to not fail")
            .expect("it to be a snippet");

        let expected_output = "<script lang=\"ts\" setup>
import { ref } from \"vue\";

const count = ref(1);
function add(a: number, b: number): number {
  return a + b;
}
</script>

<template>
  <button @click=\"() => (count = add(count, count))\">Increment</button>
</template>
";

        assert_eq!(output, expected_output);
    }
}

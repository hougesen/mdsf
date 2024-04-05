use schemars::JsonSchema;

use super::{Lang, LanguageFormatter};
use crate::{
    error::MdsfError,
    formatters::{prettier::format_using_prettier, MdsfFormatter},
};

#[derive(Default, serde::Serialize, serde::Deserialize, JsonSchema)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Vue {
    #[default]
    #[serde(rename = "prettier")]
    Prettier,
}

impl Default for Lang<Vue> {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: MdsfFormatter::<Vue>::default(),
        }
    }
}

impl Default for MdsfFormatter<Vue> {
    #[inline]
    fn default() -> Self {
        Self::Single(Vue::Prettier)
    }
}

impl LanguageFormatter for Vue {
    #[inline]
    fn format_snippet(
        &self,
        snippet_path: &std::path::Path,
    ) -> Result<(bool, Option<String>), MdsfError> {
        match self {
            Self::Prettier => format_using_prettier(snippet_path, true),
        }
    }
}

impl core::fmt::Display for Vue {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Prettier => write!(f, "prettier"),
        }
    }
}

#[cfg(test)]
mod test_vue {
    use super::Vue;
    use crate::{
        formatters::{setup_snippet, MdsfFormatter},
        languages::Lang,
        LineInfo,
    };

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

    const EXTENSION: &str = crate::languages::Language::Vue.to_file_ext();

    #[test]
    fn it_should_be_enabled_by_default() {
        assert!(Lang::<Vue>::default().enabled);
    }

    #[test]
    fn it_should_not_format_when_enabled_is_false() {
        let l = Lang::<Vue> {
            enabled: false,
            formatter: MdsfFormatter::Single(Vue::Prettier),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        assert!(l
            .format(snippet_path, &LineInfo::fake())
            .expect("it to not fail")
            .is_none());
    }

    #[test]
    fn test_prettier() {
        let l = Lang::<Vue> {
            enabled: true,
            formatter: MdsfFormatter::Single(Vue::Prettier),
        };

        let snippet = setup_snippet(INPUT, EXTENSION).expect("it to save the file");
        let snippet_path = snippet.path();

        let output = l
            .format(snippet_path, &LineInfo::fake())
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

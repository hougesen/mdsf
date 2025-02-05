// THIS CODE WAS GENERATED AND SHOULD NOT BE EDITED MANUALLY

#[allow(clippy::too_many_lines)]
#[inline]
pub fn language_to_ext(language: &str) -> Option<&'static str> {
    #[allow(clippy::match_same_arms)]
    match language.to_lowercase().as_str() {
        "1c enterprise" => Some(".bsl"),
        "2-dimensional array" => Some(".2da"),
        "4d" => Some(".4dm"),
        "abap cds" => Some(".asddls"),
        "abap" => Some(".abap"),
        "abl" => Some(".p"),
        "abnf" => Some(".abnf"),
        "acfm" => Some(".afm"),
        "aconf" => Some(".apacheconf"),
        "actionscript 3" => Some(".as"),
        "actionscript" => Some(".as"),
        "actionscript3" => Some(".as"),
        "ad block filters" => Some(".txt"),
        "ad block" => Some(".txt"),
        "ada" => Some(".adb"),
        "ada2005" => Some(".adb"),
        "ada95" => Some(".adb"),
        "adb" => Some(".txt"),
        "adblock filter list" => Some(".txt"),
        "adblock" => Some(".txt"),
        "adobe composite font metrics" => Some(".afm"),
        "adobe font metrics" => Some(".afm"),
        "adobe multiple font metrics" => Some(".afm"),
        "advpl" => Some(".prg"),
        "afdko" => Some(".fea"),
        "agda" => Some(".agda"),
        "ags script" => Some(".asc"),
        "ags" => Some(".asc"),
        "ahk" => Some(".ahk"),
        "aidl" => Some(".aidl"),
        "al" => Some(".al"),
        "alloy" => Some(".als"),
        "altium designer" => Some(".OutJob"),
        "altium" => Some(".OutJob"),
        "amfm" => Some(".afm"),
        "ampl" => Some(".ampl"),
        "amusewiki" => Some(".muse"),
        "angelscript" => Some(".as"),
        "answer set programming" => Some(".lp"),
        "antlers" => Some(".antlers.html"),
        "antlr" => Some(".g4"),
        "apache" => Some(".apacheconf"),
        "apacheconf" => Some(".apacheconf"),
        "apex" => Some(".cls"),
        "api blueprint" => Some(".apib"),
        "apl" => Some(".apl"),
        "apollo guidance computer" => Some(".agc"),
        "applescript" => Some(".applescript"),
        "arc" => Some(".arc"),
        "arexx" => Some(".rexx"),
        "as3" => Some(".as"),
        "ascii stl" => Some(".stl"),
        "asciidoc" => Some(".asciidoc"),
        "asl" => Some(".asl"),
        "asm" => Some(".asm"),
        "asn.1" => Some(".asn"),
        "asp" => Some(".asp"),
        "asp.net" => Some(".asax"),
        "aspectj" => Some(".aj"),
        "aspx" => Some(".asax"),
        "aspx-vb" => Some(".asax"),
        "assembly" => Some(".asm"),
        "astro" => Some(".astro"),
        "asymptote" => Some(".asy"),
        "ats" => Some(".dats"),
        "ats2" => Some(".dats"),
        "au3" => Some(".au3"),
        "augeas" => Some(".aug"),
        "autoconf" => Some(".m4"),
        "autohotkey" => Some(".ahk"),
        "autoit" => Some(".au3"),
        "autoit3" => Some(".au3"),
        "autoitscript" => Some(".au3"),
        "avro idl" => Some(".avdl"),
        "awk" => Some(".awk"),
        "b3d" => Some(".bb"),
        "b4x" => Some(".bas"),
        "ballerina" => Some(".bal"),
        "bash session" => Some(".sh-session"),
        "bash" => Some(".sh"),
        "basic for android" => Some(".bas"),
        "basic" => Some(".bas"),
        "bat" => Some(".bat"),
        "batch" => Some(".bat"),
        "batchfile" => Some(".bat"),
        "bazel" => Some(".bzl"),
        "be" => Some(".be"),
        "beef" => Some(".bf"),
        "befunge" => Some(".befunge"),
        "berry" => Some(".be"),
        "bh" => Some(".bs"),
        "bibtex" => Some(".bib"),
        "bicep" => Some(".bicep"),
        "bikeshed" => Some(".bs"),
        "bison" => Some(".bison"),
        "bitbake" => Some(".bb"),
        "blade" => Some(".blade"),
        "blitz3d" => Some(".bb"),
        "blitzbasic" => Some(".bb"),
        "blitzmax" => Some(".bmx"),
        "blitzplus" => Some(".bb"),
        "bluespec bh" => Some(".bs"),
        "bluespec bsv" => Some(".bsv"),
        "bluespec classic" => Some(".bs"),
        "bluespec" => Some(".bsv"),
        "bmax" => Some(".bmx"),
        "boo" => Some(".boo"),
        "boogie" => Some(".bpl"),
        "bplus" => Some(".bb"),
        "bqn" => Some(".bqn"),
        "brainfuck" => Some(".b"),
        "brighterscript" => Some(".bs"),
        "brightscript" => Some(".brs"),
        "bro" => Some(".zeek"),
        "bsdmake" => Some(".mak"),
        "bsv" => Some(".bsv"),
        "byond" => Some(".dm"),
        "bzl" => Some(".bzl"),
        "c" => Some(".c"),
        "c#" => Some(".cs"),
        "c++" => Some(".cpp"),
        "c++-objdump" => Some(".cppobjdump"),
        "c-objdump" => Some(".c-objdump"),
        "c2hs haskell" => Some(".chs"),
        "c2hs" => Some(".chs"),
        "cabal config" => Some(".cabal"),
        "cabal" => Some(".cabal"),
        "caddy" => Some(".caddyfile"),
        "caddyfile" => Some(".caddyfile"),
        "cadence" => Some(".cdc"),
        "cairo zero" => Some(".cairo"),
        "cairo" => Some(".cairo"),
        "cake" => Some(".cs"),
        "cakescript" => Some(".cs"),
        "cameligo" => Some(".mligo"),
        "cap cds" => Some(".cds"),
        "cap'n proto" => Some(".capnp"),
        "carbon" => Some(".carbon"),
        "carto" => Some(".mss"),
        "cartocss" => Some(".mss"),
        "cds" => Some(".cds"),
        "ceylon" => Some(".ceylon"),
        "cfc" => Some(".cfc"),
        "cfm" => Some(".cfm"),
        "cfml" => Some(".cfm"),
        "chapel" => Some(".chpl"),
        "charity" => Some(".ch"),
        "checksum" => Some(".crc32"),
        "checksums" => Some(".crc32"),
        "chpl" => Some(".chpl"),
        "chuck" => Some(".ck"),
        "cil" => Some(".cil"),
        "circom" => Some(".circom"),
        "cirru" => Some(".cirru"),
        "clarion" => Some(".clw"),
        "clarity" => Some(".clar"),
        "classic asp" => Some(".asp"),
        "classic qbasic" => Some(".bas"),
        "classic quickbasic" => Some(".bas"),
        "classic visual basic" => Some(".bas"),
        "clean" => Some(".icl"),
        "click" => Some(".click"),
        "clipper" => Some(".prg"),
        "clips" => Some(".clp"),
        "clojure" => Some(".clj"),
        "closure templates" => Some(".soy"),
        "cmake" => Some(".cmake"),
        "cobol" => Some(".cob"),
        "coccinelle" => Some(".cocci"),
        "codeql" => Some(".ql"),
        "coffee" => Some(".coffee"),
        "coffee-script" => Some(".coffee"),
        "coffeescript" => Some(".coffee"),
        "coldfusion cfc" => Some(".cfc"),
        "coldfusion html" => Some(".cfm"),
        "coldfusion" => Some(".cfm"),
        "collada" => Some(".dae"),
        "common lisp" => Some(".lisp"),
        "common workflow language" => Some(".cwl"),
        "component pascal" => Some(".cp"),
        "conll" => Some(".conllu"),
        "conll-u" => Some(".conllu"),
        "conll-x" => Some(".conllu"),
        "console" => Some(".sh-session"),
        "containerfile" => Some(".dockerfile"),
        "cool" => Some(".cl"),
        "coq" => Some(".coq"),
        "cperl" => Some(".pl"),
        "cpp" => Some(".cpp"),
        "cpp-objdump" => Some(".cppobjdump"),
        "creole" => Some(".creole"),
        "crystal" => Some(".cr"),
        "csharp" => Some(".cs"),
        "cson" => Some(".cson"),
        "csound document" => Some(".csd"),
        "csound score" => Some(".sco"),
        "csound" => Some(".orc"),
        "csound-csd" => Some(".csd"),
        "csound-orc" => Some(".orc"),
        "csound-sco" => Some(".sco"),
        "css" => Some(".css"),
        "csv" => Some(".csv"),
        "cucumber" => Some(".feature"),
        "cuda" => Some(".cu"),
        "cue sheet" => Some(".cue"),
        "cue" => Some(".cue"),
        "curry" => Some(".curry"),
        "cweb" => Some(".w"),
        "cwl" => Some(".cwl"),
        "cycript" => Some(".cy"),
        "cylc" => Some(".cylc"),
        "cypher" => Some(".cyp"),
        "cython" => Some(".pyx"),
        "d" => Some(".d"),
        "d-objdump" => Some(".d-objdump"),
        "d2" => Some(".d2"),
        "d2lang" => Some(".d2"),
        "dafny" => Some(".dfy"),
        "darcs patch" => Some(".darcspatch"),
        "dart" => Some(".dart"),
        "dataweave" => Some(".dwl"),
        "dcl" => Some(".com"),
        "debian package control file" => Some(".dsc"),
        "delphi" => Some(".pas"),
        "denizenscript" => Some(".dsc"),
        "desktop" => Some(".desktop"),
        "dhall" => Some(".dhall"),
        "diff" => Some(".diff"),
        "digital command language" => Some(".com"),
        "dircolors" => Some(".dircolors"),
        "directx 3d file" => Some(".x"),
        "django" => Some(".jinja"),
        "dlang" => Some(".d"),
        "dm" => Some(".dm"),
        "dns zone" => Some(".zone"),
        "dockerfile" => Some(".dockerfile"),
        "dogescript" => Some(".djs"),
        "dosbatch" => Some(".bat"),
        "dosini" => Some(".ini"),
        "dotenv" => Some(".env"),
        "dpatch" => Some(".darcspatch"),
        "dtrace" => Some(".d"),
        "dtrace-script" => Some(".d"),
        "dylan" => Some(".dylan"),
        "e" => Some(".e"),
        "e-mail" => Some(".eml"),
        "eagle" => Some(".sch"),
        "easybuild" => Some(".eb"),
        "ebnf" => Some(".ebnf"),
        "ec" => Some(".ec"),
        "ecere projects" => Some(".epj"),
        "ecl" => Some(".ecl"),
        "eclipse" => Some(".ecl"),
        "ecmarkdown" => Some(".html"),
        "ecmarkup" => Some(".html"),
        "ecr" => Some(".ecr"),
        "edge" => Some(".edge"),
        "edgeql" => Some(".edgeql"),
        "editor-config" => Some(".editorconfig"),
        "editorconfig" => Some(".editorconfig"),
        "edje data collection" => Some(".edc"),
        "edn" => Some(".edn"),
        "eeschema schematic" => Some(".kicad_sch"),
        "eex" => Some(".html.eex"),
        "eiffel" => Some(".e"),
        "ejs" => Some(".ejs"),
        "electronic business card" => Some(".vcf"),
        "elisp" => Some(".el"),
        "elixir" => Some(".ex"),
        "elm" => Some(".elm"),
        "elvish" => Some(".elv"),
        "emacs lisp" => Some(".el"),
        "emacs muse" => Some(".muse"),
        "emacs" => Some(".el"),
        "email" => Some(".eml"),
        "emberscript" => Some(".em"),
        "eml" => Some(".eml"),
        "envrc" => Some(".sh"),
        "eq" => Some(".eq"),
        "erb" => Some(".erb"),
        "erlang" => Some(".erl"),
        "esdl" => Some(".edgeql"),
        "euphoria" => Some(".e"),
        "f#" => Some(".fs"),
        "f*" => Some(".fst"),
        "factor" => Some(".factor"),
        "fancy" => Some(".fy"),
        "fantom" => Some(".fan"),
        "faust" => Some(".dsp"),
        "fb" => Some(".bi"),
        "fennel" => Some(".fnl"),
        "figfont" => Some(".flf"),
        "figlet font" => Some(".flf"),
        "filebench wml" => Some(".f"),
        "filterscript" => Some(".fs"),
        "firrtl" => Some(".fir"),
        "fish" => Some(".fish"),
        "flex" => Some(".l"),
        "fluent" => Some(".ftl"),
        "flux" => Some(".fx"),
        "formatted" => Some(".for"),
        "forth" => Some(".fth"),
        "fortran free form" => Some(".f90"),
        "fortran" => Some(".f"),
        "foxpro" => Some(".prg"),
        "freebasic" => Some(".bi"),
        "freemarker" => Some(".ftl"),
        "frege" => Some(".fr"),
        "fsharp" => Some(".fs"),
        "fstar" => Some(".fst"),
        "ftl" => Some(".ftl"),
        "fundamental" => Some(".txt"),
        "futhark" => Some(".fut"),
        "g-code" => Some(".g"),
        "game maker language" => Some(".gml"),
        "gaml" => Some(".gaml"),
        "gams" => Some(".gms"),
        "gap" => Some(".g"),
        "gas" => Some(".s"),
        "gcc machine description" => Some(".md"),
        "gdb" => Some(".gdb"),
        "gdscript" => Some(".gd"),
        "gedcom" => Some(".ged"),
        "gemini" => Some(".gmi"),
        "gemtext" => Some(".gmi"),
        "genero 4gl" => Some(".4gl"),
        "genero per" => Some(".per"),
        "genie" => Some(".gs"),
        "genshi" => Some(".kid"),
        "gentoo ebuild" => Some(".ebuild"),
        "gentoo eclass" => Some(".eclass"),
        "geojson" => Some(".json"),
        "gerber image" => Some(".gbr"),
        "gettext catalog" => Some(".po"),
        "gf" => Some(".gf"),
        "gherkin" => Some(".feature"),
        "git config" => Some(".gitconfig"),
        "git-ignore" => Some(".gitignore"),
        "gitconfig" => Some(".gitconfig"),
        "gitignore" => Some(".gitignore"),
        "gitmodules" => Some(".gitconfig"),
        "gleam" => Some(".gleam"),
        "glimmer js" => Some(".gjs"),
        "glimmer ts" => Some(".gts"),
        "glsl" => Some(".glsl"),
        "glyph bitmap distribution format" => Some(".bdf"),
        "glyph" => Some(".glf"),
        "gn" => Some(".gn"),
        "gnu asm" => Some(".s"),
        "gnuplot" => Some(".gp"),
        "go" => Some(".go"),
        "godot resource" => Some(".gdnlib"),
        "golang" => Some(".go"),
        "golo" => Some(".golo"),
        "gosu" => Some(".gs"),
        "grace" => Some(".grace"),
        "gradle kotlin dsl" => Some(".gradle.kts"),
        "gradle" => Some(".gradle"),
        "grammatical framework" => Some(".gf"),
        "graph modeling language" => Some(".gml"),
        "graphql" => Some(".graphql"),
        "graphviz (dot)" => Some(".dot"),
        "groff" => Some(".roff"),
        "groovy server pages" => Some(".gsp"),
        "groovy" => Some(".groovy"),
        "gsc" => Some(".gsc"),
        "gsp" => Some(".gsp"),
        "hack" => Some(".hack"),
        "haml" => Some(".haml"),
        "handlebars" => Some(".handlebars"),
        "haproxy" => Some(".cfg"),
        "harbour" => Some(".hb"),
        "hare" => Some(".ha"),
        "hash" => Some(".crc32"),
        "hashes" => Some(".crc32"),
        "hashicorp configuration language" => Some(".hcl"),
        "haskell" => Some(".hs"),
        "haxe" => Some(".hx"),
        "hbs" => Some(".handlebars"),
        "hcl" => Some(".hcl"),
        "heex" => Some(".html.eex"),
        "help" => Some(".txt"),
        "hiveql" => Some(".q"),
        "hlsl" => Some(".hlsl"),
        "hocon" => Some(".hocon"),
        "holyc" => Some(".hc"),
        "hoon" => Some(".hoon"),
        "html" => Some(".html"),
        "html+django" => Some(".jinja"),
        "html+ecr" => Some(".ecr"),
        "html+eex" => Some(".html.eex"),
        "html+erb" => Some(".erb"),
        "html+jinja" => Some(".jinja"),
        "html+php" => Some(".phtml"),
        "html+razor" => Some(".cshtml"),
        "html+ruby" => Some(".erb"),
        "htmlbars" => Some(".handlebars"),
        "htmldjango" => Some(".jinja"),
        "http" => Some(".http"),
        "hxml" => Some(".hxml"),
        "hy" => Some(".hy"),
        "hylang" => Some(".hy"),
        "hyphy" => Some(".bf"),
        "i7" => Some(".ni"),
        "ical" => Some(".ics"),
        "icalendar" => Some(".ics"),
        "idl" => Some(".pro"),
        "idris" => Some(".idr"),
        "ignore list" => Some(".gitignore"),
        "ignore" => Some(".gitignore"),
        "igor pro" => Some(".ipf"),
        "igor" => Some(".ipf"),
        "igorpro" => Some(".ipf"),
        "ijm" => Some(".ijm"),
        "ile rpg" => Some(".rpgle"),
        "imagej macro" => Some(".ijm"),
        "imba" => Some(".imba"),
        "inc" => Some(".php"),
        "inform 7" => Some(".ni"),
        "inform7" => Some(".ni"),
        "ini" => Some(".ini"),
        "ink" => Some(".ink"),
        "inno setup" => Some(".iss"),
        "io" => Some(".io"),
        "ioke" => Some(".ik"),
        "ipython notebook" => Some(".ipynb"),
        "irc log" => Some(".irclog"),
        "irc logs" => Some(".irclog"),
        "irc" => Some(".irclog"),
        "isabelle" => Some(".thy"),
        "j" => Some(".ijs"),
        "jai" => Some(".jai"),
        "janet" => Some(".janet"),
        "jasmin" => Some(".j"),
        "java properties" => Some(".properties"),
        "java server page" => Some(".gsp"),
        "java server pages" => Some(".jsp"),
        "java template engine" => Some(".jte"),
        "java" => Some(".java"),
        "javascript" => Some(".js"),
        "javascript+erb" => Some(".js.erb"),
        "jcl" => Some(".jcl"),
        "jest snapshot" => Some(".snap"),
        "jetbrains mps" => Some(".mps"),
        "jflex" => Some(".flex"),
        "jinja" => Some(".jinja"),
        "jison lex" => Some(".jisonlex"),
        "jison" => Some(".jison"),
        "jolie" => Some(".ol"),
        "jq" => Some(".jq"),
        "jruby" => Some(".rb"),
        "js" => Some(".js"),
        "json with comments" => Some(".jsonc"),
        "json" => Some(".json"),
        "json5" => Some(".json5"),
        "jsonc" => Some(".jsonc"),
        "jsoniq" => Some(".jq"),
        "jsonl" => Some(".json"),
        "jsonld" => Some(".jsonld"),
        "jsonnet" => Some(".jsonnet"),
        "jsp" => Some(".jsp"),
        "jte" => Some(".jte"),
        "julia" => Some(".jl"),
        "jupyter notebook" => Some(".ipynb"),
        "just" => Some(".just"),
        "justfile" => Some(".just"),
        "kaitai struct" => Some(".ksy"),
        "kak" => Some(".kak"),
        "kakounescript" => Some(".kak"),
        "kakscript" => Some(".kak"),
        "kdl" => Some(".kdl"),
        "kerboscript" => Some(".ks"),
        "keyvalues" => Some(".vdf"),
        "kicad layout" => Some(".kicad_pcb"),
        "kicad legacy layout" => Some(".brd"),
        "kicad schematic" => Some(".kicad_sch"),
        "kickstart" => Some(".ks"),
        "kit" => Some(".kit"),
        "kotlin" => Some(".kt"),
        "krl" => Some(".krl"),
        "ksy" => Some(".ksy"),
        "kusto" => Some(".csl"),
        "kvlang" => Some(".kv"),
        "labview" => Some(".lvproj"),
        "lark" => Some(".lark"),
        "lasso" => Some(".lasso"),
        "lassoscript" => Some(".lasso"),
        "latex" => Some(".tex"),
        "latte" => Some(".latte"),
        "lean 4" => Some(".lean"),
        "lean" => Some(".lean"),
        "leex" => Some(".html.eex"),
        "less" => Some(".less"),
        "less-css" => Some(".less"),
        "lex" => Some(".l"),
        "lfe" => Some(".lfe"),
        "lhaskell" => Some(".lhs"),
        "lhs" => Some(".lhs"),
        "ligolang" => Some(".ligo"),
        "lilypond" => Some(".ly"),
        "limbo" => Some(".b"),
        "linear programming" => Some(".lp"),
        "linker script" => Some(".ld"),
        "linux kernel module" => Some(".mod"),
        "liquid" => Some(".liquid"),
        "lisp" => Some(".lisp"),
        "litcoffee" => Some(".litcoffee"),
        "literate agda" => Some(".lagda"),
        "literate coffeescript" => Some(".litcoffee"),
        "literate haskell" => Some(".lhs"),
        "live-script" => Some(".ls"),
        "livecode script" => Some(".livecodescript"),
        "livescript" => Some(".ls"),
        "llvm" => Some(".ll"),
        "logos" => Some(".xm"),
        "logtalk" => Some(".lgt"),
        "lolcode" => Some(".lol"),
        "lookml" => Some(".lkml"),
        "loomscript" => Some(".ls"),
        "ls" => Some(".ls"),
        "lsl" => Some(".lsl"),
        "ltspice symbol" => Some(".asy"),
        "lua" => Some(".lua"),
        "luau" => Some(".luau"),
        "m" => Some(".mumps"),
        "m2" => Some(".m2"),
        "m4" => Some(".m4"),
        "m4sugar" => Some(".m4"),
        "m68k" => Some(".asm"),
        "macaulay2" => Some(".m2"),
        "macruby" => Some(".rb"),
        "mail" => Some(".eml"),
        "make" => Some(".mak"),
        "makefile" => Some(".mak"),
        "mako" => Some(".mako"),
        "man page" => Some(".roff"),
        "man" => Some(".roff"),
        "man-page" => Some(".roff"),
        "manpage" => Some(".roff"),
        "markdown" => Some(".md"),
        "marko" => Some(".marko"),
        "markojs" => Some(".marko"),
        "mask" => Some(".mask"),
        "mathematica" => Some(".mathematica"),
        "matlab" => Some(".matlab"),
        "max" => Some(".maxpat"),
        "max/msp" => Some(".maxpat"),
        "maxmsp" => Some(".maxpat"),
        "maxscript" => Some(".ms"),
        "mbox" => Some(".eml"),
        "mcfunction" => Some(".mcfunction"),
        "md" => Some(".md"),
        "mdoc" => Some(".roff"),
        "mdsvex" => Some(".svx"),
        "mdx" => Some(".mdx"),
        "mediawiki" => Some(".mediawiki"),
        "mercury" => Some(".m"),
        "mermaid example" => Some(".mmd"),
        "mermaid" => Some(".mmd"),
        "metal" => Some(".metal"),
        "mf" => Some(".mak"),
        "microsoft developer studio project" => Some(".dsp"),
        "microsoft visual studio solution" => Some(".sln"),
        "minid" => Some(".minid"),
        "miniyaml" => Some(".yaml"),
        "minizinc data" => Some(".dzn"),
        "minizinc" => Some(".mzn"),
        "mint" => Some(".mint"),
        "mirah" => Some(".druby"),
        "mirc script" => Some(".mrc"),
        "mlir" => Some(".mlir"),
        "mma" => Some(".mathematica"),
        "modelica" => Some(".mo"),
        "modula-2" => Some(".mod"),
        "modula-3" => Some(".i3"),
        "module management system" => Some(".mms"),
        "mojo" => Some(".mojo"),
        "monkey c" => Some(".mc"),
        "monkey" => Some(".monkey"),
        "moocode" => Some(".moo"),
        "moonbit" => Some(".mbt"),
        "moonscript" => Some(".moon"),
        "motoko" => Some(".mo"),
        "motorola 68k assembly" => Some(".asm"),
        "move" => Some(".move"),
        "mps" => Some(".mps"),
        "mql4" => Some(".mq4"),
        "mql5" => Some(".mq5"),
        "mtml" => Some(".mtml"),
        "muf" => Some(".muf"),
        "mumps" => Some(".mumps"),
        "mupad" => Some(".mu"),
        "muse" => Some(".muse"),
        "mustache" => Some(".mustache"),
        "myghty" => Some(".myt"),
        "nanorc" => Some(".nanorc"),
        "nargo" => Some(".nr"),
        "nasal" => Some(".nas"),
        "nasl" => Some(".nasl"),
        "nasm" => Some(".asm"),
        "ncl" => Some(".ncl"),
        "ne-on" => Some(".neon"),
        "nearley" => Some(".ne"),
        "nemerle" => Some(".n"),
        "neon" => Some(".neon"),
        "neosnippet" => Some(".snip"),
        "nesc" => Some(".nc"),
        "netlinx" => Some(".axs"),
        "netlinx+erb" => Some(".axs.erb"),
        "netlogo" => Some(".nlogo"),
        "nette object notation" => Some(".neon"),
        "newlisp" => Some(".nl"),
        "nextflow" => Some(".nf"),
        "nginx configuration file" => Some(".nginx"),
        "nginx" => Some(".nginx"),
        "nim" => Some(".nim"),
        "ninja" => Some(".ninja"),
        "nit" => Some(".nit"),
        "nix" => Some(".nix"),
        "nixos" => Some(".nix"),
        "njk" => Some(".njk"),
        "nl" => Some(".nl"),
        "nmodl" => Some(".mod"),
        "node" => Some(".js"),
        "noir" => Some(".nr"),
        "nroff" => Some(".roff"),
        "nsis" => Some(".nsi"),
        "nu" => Some(".nu"),
        "nu-script" => Some(".nu"),
        "numpy" => Some(".numpy"),
        "nunjucks" => Some(".njk"),
        "nush" => Some(".nu"),
        "nushell" => Some(".nu"),
        "nushell-script" => Some(".nu"),
        "nvim" => Some(".vim"),
        "nwscript" => Some(".nss"),
        "oasv2-json" => Some(".json"),
        "oasv2-yaml" => Some(".yaml"),
        "oasv3-json" => Some(".json"),
        "oasv3-yaml" => Some(".yaml"),
        "oberon" => Some(".ob2"),
        "obj-c" => Some(".m"),
        "obj-c++" => Some(".mm"),
        "obj-j" => Some(".j"),
        "objc" => Some(".m"),
        "objc++" => Some(".mm"),
        "objdump" => Some(".objdump"),
        "object data instance notation" => Some(".odin"),
        "objective-c" => Some(".m"),
        "objective-c++" => Some(".mm"),
        "objective-j" => Some(".j"),
        "objectivec" => Some(".m"),
        "objectivec++" => Some(".mm"),
        "objectivej" => Some(".j"),
        "objectpascal" => Some(".pas"),
        "objectscript" => Some(".cls"),
        "objj" => Some(".j"),
        "ocaml" => Some(".ml"),
        "octave" => Some(".matlab"),
        "odin" => Some(".odin"),
        "odin-lang" => Some(".odin"),
        "odinlang" => Some(".odin"),
        "omgrofl" => Some(".omgrofl"),
        "omnet++ msg" => Some(".msg"),
        "omnet++ ned" => Some(".ned"),
        "omnetpp-msg" => Some(".msg"),
        "omnetpp-ned" => Some(".ned"),
        "oncrpc" => Some(".x"),
        "ooc" => Some(".ooc"),
        "opa" => Some(".opa"),
        "opal" => Some(".opal"),
        "open policy agent" => Some(".rego"),
        "opencl" => Some(".cl"),
        "openedge abl" => Some(".p"),
        "openedge" => Some(".p"),
        "openqasm" => Some(".qasm"),
        "openscad" => Some(".scad"),
        "openstep property list" => Some(".plist"),
        "opentype feature file" => Some(".fea"),
        "org" => Some(".org"),
        "osascript" => Some(".applescript"),
        "overpassql" => Some(".overpassql"),
        "ox" => Some(".ox"),
        "oxygene" => Some(".oxygene"),
        "oz" => Some(".oz"),
        "p4" => Some(".p4"),
        "pact" => Some(".pact"),
        "pan" => Some(".pan"),
        "pandoc" => Some(".md"),
        "papyrus" => Some(".psc"),
        "parrot assembly" => Some(".pasm"),
        "parrot internal representation" => Some(".pir"),
        "parrot" => Some(".parrot"),
        "pascal" => Some(".pas"),
        "pasm" => Some(".pasm"),
        "pawn" => Some(".pwn"),
        "pcbnew" => Some(".kicad_pcb"),
        "pddl" => Some(".pddl"),
        "peg.js" => Some(".pegjs"),
        "pep8" => Some(".pep"),
        "perl" => Some(".pl"),
        "perl-6" => Some(".6pl"),
        "perl6" => Some(".6pl"),
        "php" => Some(".php"),
        "pic" => Some(".pic"),
        "pickle" => Some(".pkl"),
        "picolisp" => Some(".l"),
        "piglatin" => Some(".pig"),
        "pikchr" => Some(".pic"),
        "pike" => Some(".pike"),
        "pir" => Some(".pir"),
        "pkl" => Some(".pkl"),
        "plain text" => Some(".txt"),
        "plantuml" => Some(".puml"),
        "plpgsql" => Some(".pgsql"),
        "plsql" => Some(".pls"),
        "pod 6" => Some(".pod"),
        "pod" => Some(".pod"),
        "pogoscript" => Some(".pogo"),
        "polar" => Some(".polar"),
        "pony" => Some(".pony"),
        "portugol" => Some(".por"),
        "posh" => Some(".ps1"),
        "postcss" => Some(".pcss"),
        "postscr" => Some(".ps"),
        "postscript" => Some(".ps"),
        "pot" => Some(".po"),
        "pov-ray sdl" => Some(".pov"),
        "pov-ray" => Some(".pov"),
        "povray" => Some(".pov"),
        "powerbuilder" => Some(".pbt"),
        "powershell" => Some(".ps1"),
        "praat" => Some(".praat"),
        "prisma" => Some(".prisma"),
        "processing" => Some(".pde"),
        "progress" => Some(".p"),
        "proguard" => Some(".pro"),
        "prolog" => Some(".pl"),
        "promela" => Some(".pml"),
        "propeller spin" => Some(".spin"),
        "proto" => Some(".proto"),
        "protobuf text format" => Some(".textproto"),
        "protobuf" => Some(".proto"),
        "protocol buffer text format" => Some(".textproto"),
        "protocol buffer" => Some(".proto"),
        "protocol buffers" => Some(".proto"),
        "public key" => Some(".asc"),
        "pug" => Some(".jade"),
        "puppet" => Some(".pp"),
        "pure data" => Some(".pd"),
        "purebasic" => Some(".pb"),
        "purescript" => Some(".purs"),
        "pwsh" => Some(".ps1"),
        "pyret" => Some(".arr"),
        "pyrex" => Some(".pyx"),
        "python traceback" => Some(".pytb"),
        "python" => Some(".py"),
        "python3" => Some(".py"),
        "q" => Some(".q"),
        "q#" => Some(".qs"),
        "qb" => Some(".bas"),
        "qb64" => Some(".bas"),
        "qbasic" => Some(".bas"),
        "ql" => Some(".ql"),
        "qmake" => Some(".pro"),
        "qml" => Some(".qml"),
        "qsharp" => Some(".qs"),
        "qt script" => Some(".qs"),
        "quickbasic" => Some(".bas"),
        "r" => Some(".r"),
        "racket" => Some(".rkt"),
        "ragel" => Some(".rl"),
        "ragel-rb" => Some(".rl"),
        "ragel-ruby" => Some(".rl"),
        "rake" => Some(".rb"),
        "raku" => Some(".6pl"),
        "raml" => Some(".raml"),
        "rascal" => Some(".rsc"),
        "raw token data" => Some(".raw"),
        "raw" => Some(".raw"),
        "razor" => Some(".cshtml"),
        "rb" => Some(".rb"),
        "rbs" => Some(".rbs"),
        "rbx" => Some(".rb"),
        "rdoc" => Some(".rdoc"),
        "realbasic" => Some(".rbbas"),
        "reason" => Some(".re"),
        "reasonligo" => Some(".religo"),
        "rebol" => Some(".reb"),
        "red" => Some(".red"),
        "red/system" => Some(".red"),
        "redcode" => Some(".cw"),
        "regex" => Some(".regexp"),
        "regexp" => Some(".regexp"),
        "regular expression" => Some(".regexp"),
        "ren'py" => Some(".rpy"),
        "renderscript" => Some(".rs"),
        "renpy" => Some(".rpy"),
        "rescript" => Some(".res"),
        "restructuredtext" => Some(".rst"),
        "rexx" => Some(".rexx"),
        "rez" => Some(".r"),
        "rhtml" => Some(".erb"),
        "rich text format" => Some(".rtf"),
        "ring" => Some(".ring"),
        "riot" => Some(".riot"),
        "rmarkdown" => Some(".qmd"),
        "robotframework" => Some(".robot"),
        "roc" => Some(".roc"),
        "roff manpage" => Some(".1"),
        "roff" => Some(".roff"),
        "ron" => Some(".ron"),
        "rouge" => Some(".rg"),
        "routeros script" => Some(".rsc"),
        "rpc" => Some(".x"),
        "rpcgen" => Some(".x"),
        "rpgle" => Some(".rpgle"),
        "rpm spec" => Some(".spec"),
        "rs" => Some(".rs"),
        "rs-274x" => Some(".gbr"),
        "rscript" => Some(".r"),
        "rss" => Some(".xml"),
        "rst" => Some(".rst"),
        "ruby" => Some(".rb"),
        "runoff" => Some(".rnh"),
        "rust" => Some(".rs"),
        "rusthon" => Some(".py"),
        "sage" => Some(".sage"),
        "salt" => Some(".sls"),
        "saltstack" => Some(".sls"),
        "saltstate" => Some(".sls"),
        "sarif" => Some(".json"),
        "sas" => Some(".sas"),
        "sass" => Some(".sass"),
        "scala" => Some(".scala"),
        "scaml" => Some(".scaml"),
        "scenic" => Some(".scenic"),
        "scheme" => Some(".scm"),
        "scilab" => Some(".sci"),
        "scss" => Some(".scss"),
        "sdc" => Some(".tcl"),
        "sed" => Some(".sed"),
        "self" => Some(".self"),
        "selinux kernel policy language" => Some(".te"),
        "selinux policy" => Some(".te"),
        "sepolicy" => Some(".te"),
        "sfv" => Some(".sfv"),
        "sh" => Some(".sh"),
        "shaderlab" => Some(".shader"),
        "shell" => Some(".sh"),
        "shell-script" => Some(".sh"),
        "shellsession" => Some(".sh-session"),
        "shen" => Some(".shen"),
        "sieve" => Some(".sieve"),
        "simple file verification" => Some(".sfv"),
        "slash" => Some(".sl"),
        "slice" => Some(".ice"),
        "slim" => Some(".slim"),
        "slint" => Some(".slint"),
        "smali" => Some(".smali"),
        "smalltalk" => Some(".st"),
        "smarty" => Some(".tpl"),
        "smithy" => Some(".smithy"),
        "sml" => Some(".ml"),
        "smpl" => Some(".cocci"),
        "smt" => Some(".smt2"),
        "snakefile" => Some(".smk"),
        "snakemake" => Some(".smk"),
        "snipmate" => Some(".snip"),
        "snippet" => Some(".yasnippet"),
        "solidity" => Some(".sol"),
        "sourcemod" => Some(".sp"),
        "sourcepawn" => Some(".sp"),
        "soy" => Some(".soy"),
        "sparql" => Some(".sparql"),
        "specfile" => Some(".spec"),
        "spline font database" => Some(".sfd"),
        "splus" => Some(".r"),
        "sqf" => Some(".sqf"),
        "sql" => Some(".sql"),
        "sqlpl" => Some(".sql"),
        "sqlrpgle" => Some(".rpgle"),
        "squeak" => Some(".st"),
        "squirrel" => Some(".nut"),
        "srecode template" => Some(".srt"),
        "stan" => Some(".stan"),
        "standard ml" => Some(".ml"),
        "star" => Some(".star"),
        "starlark" => Some(".bzl"),
        "stata" => Some(".do"),
        "stl" => Some(".stl"),
        "stla" => Some(".stl"),
        "ston" => Some(".ston"),
        "stringtemplate" => Some(".st"),
        "stylus" => Some(".styl"),
        "subrip text" => Some(".srt"),
        "sugarss" => Some(".sss"),
        "sum" => Some(".crc32"),
        "sums" => Some(".crc32"),
        "supercollider" => Some(".sc"),
        "survex data" => Some(".svx"),
        "svelte" => Some(".svelte"),
        "svg" => Some(".svg"),
        "sway" => Some(".sw"),
        "sweave" => Some(".rnw"),
        "swift" => Some(".swift"),
        "swig" => Some(".i"),
        "systemverilog" => Some(".sv"),
        "tab-seperated values" => Some(".tsv"),
        "tact" => Some(".tact"),
        "talon" => Some(".talon"),
        "tcl" => Some(".tcl"),
        "tcsh" => Some(".tcsh"),
        "tea" => Some(".tea"),
        "templ" => Some(".templ"),
        "terra" => Some(".t"),
        "terraform template" => Some(".tftpl"),
        "terraform" => Some(".hcl"),
        "tex" => Some(".tex"),
        "texinfo" => Some(".texinfo"),
        "text proto" => Some(".textproto"),
        "text" => Some(".txt"),
        "textgrid" => Some(".TextGrid"),
        "textile" => Some(".textile"),
        "thrift" => Some(".thrift"),
        "ti program" => Some(".8xp"),
        "tl" => Some(".tl"),
        "tl-verilog" => Some(".tlv"),
        "tla" => Some(".tla"),
        "toit" => Some(".toit"),
        "toml" => Some(".toml"),
        "topojson" => Some(".json"),
        "traveling salesman problem" => Some(".tsp"),
        "travelling salesman problem" => Some(".tsp"),
        "troff" => Some(".roff"),
        "ts" => Some(".ts"),
        "tsp" => Some(".tsp"),
        "tsplib data" => Some(".tsp"),
        "tsql" => Some(".sql"),
        "tsv" => Some(".tsv"),
        "tsx" => Some(".tsx"),
        "turing" => Some(".t"),
        "turtle" => Some(".ttl"),
        "twig" => Some(".twig"),
        "txl" => Some(".txl"),
        "typ" => Some(".typ"),
        "type language" => Some(".tl"),
        "typescript" => Some(".ts"),
        "typespec" => Some(".tsp"),
        "typst" => Some(".typ"),
        "udiff" => Some(".diff"),
        "ultisnip" => Some(".snip"),
        "ultisnips" => Some(".snip"),
        "unified parallel c" => Some(".upc"),
        "unity3d asset" => Some(".anim"),
        "unix asm" => Some(".s"),
        "unix assembly" => Some(".s"),
        "uno" => Some(".uno"),
        "unrealscript" => Some(".uc"),
        "ur" => Some(".ur"),
        "ur/web" => Some(".ur"),
        "urweb" => Some(".ur"),
        "v" => Some(".v"),
        "vala" => Some(".vala"),
        "valve data format" => Some(".vdf"),
        "vb .net" => Some(".vb"),
        "vb 6" => Some(".bas"),
        "vb.net" => Some(".vb"),
        "vb6" => Some(".bas"),
        "vba" => Some(".bas"),
        "vbnet" => Some(".vb"),
        "vbscript" => Some(".vbs"),
        "vcard" => Some(".vcf"),
        "vcl" => Some(".vcl"),
        "vdf" => Some(".vdf"),
        "velocity template language" => Some(".vtl"),
        "velocity" => Some(".vtl"),
        "verilog" => Some(".v"),
        "vhdl" => Some(".vhdl"),
        "vim help file" => Some(".txt"),
        "vim script" => Some(".vim"),
        "vim snippet" => Some(".snip"),
        "vim" => Some(".vim"),
        "vimhelp" => Some(".txt"),
        "viml" => Some(".vim"),
        "vimscript" => Some(".vim"),
        "virtual contact file" => Some(".vcf"),
        "visual basic .net" => Some(".vb"),
        "visual basic 6" => Some(".bas"),
        "visual basic 6.0" => Some(".bas"),
        "visual basic classic" => Some(".bas"),
        "visual basic for applications" => Some(".bas"),
        "visual basic" => Some(".vb"),
        "vlang" => Some(".v"),
        "volt" => Some(".volt"),
        "vtl" => Some(".vtl"),
        "vtt" => Some(".vtt"),
        "vue" => Some(".vue"),
        "vyper" => Some(".vy"),
        "wasm" => Some(".wast"),
        "wast" => Some(".wast"),
        "wavefront material" => Some(".mtl"),
        "wavefront object" => Some(".obj"),
        "wdl" => Some(".wdl"),
        "web ontology language" => Some(".owl"),
        "webassembly interface type" => Some(".wit"),
        "webassembly" => Some(".wast"),
        "webidl" => Some(".webidl"),
        "webvtt" => Some(".vtt"),
        "wgsl" => Some(".wgsl"),
        "whiley" => Some(".whiley"),
        "wiki" => Some(".mediawiki"),
        "wikitext" => Some(".mediawiki"),
        "win32 message file" => Some(".mc"),
        "winbatch" => Some(".bat"),
        "windows registry entries" => Some(".reg"),
        "wisp" => Some(".wisp"),
        "wit" => Some(".wit"),
        "witcher script" => Some(".ws"),
        "wl" => Some(".mathematica"),
        "wolfram lang" => Some(".mathematica"),
        "wolfram language" => Some(".mathematica"),
        "wolfram" => Some(".mathematica"),
        "wollok" => Some(".wlk"),
        "workflow description language" => Some(".wdl"),
        "world of warcraft addon data" => Some(".toc"),
        "wren" => Some(".wren"),
        "wrenlang" => Some(".wren"),
        "wsdl" => Some(".xml"),
        "x bitmap" => Some(".xbm"),
        "x pixmap" => Some(".xpm"),
        "x10" => Some(".x10"),
        "xbase" => Some(".prg"),
        "xbm" => Some(".xbm"),
        "xc" => Some(".xc"),
        "xdc" => Some(".tcl"),
        "xdr" => Some(".x"),
        "xhtml" => Some(".html"),
        "xml property list" => Some(".plist"),
        "xml" => Some(".xml"),
        "xml+genshi" => Some(".kid"),
        "xml+kid" => Some(".kid"),
        "xojo" => Some(".xojo_code"),
        "xonsh" => Some(".xsh"),
        "xpages" => Some(".xsp-config"),
        "xpm" => Some(".xpm"),
        "xproc" => Some(".xpl"),
        "xquery" => Some(".xquery"),
        "xs" => Some(".xs"),
        "xsd" => Some(".xml"),
        "xsl" => Some(".xslt"),
        "xslt" => Some(".xslt"),
        "xten" => Some(".x10"),
        "xtend" => Some(".xtend"),
        "yacc" => Some(".y"),
        "yaml" => Some(".yml"),
        "yang" => Some(".yang"),
        "yara" => Some(".yar"),
        "yas" => Some(".yasnippet"),
        "yasnippet" => Some(".yasnippet"),
        "yml" => Some(".yml"),
        "yul" => Some(".yul"),
        "zap" => Some(".zap"),
        "zeek" => Some(".zeek"),
        "zenscript" => Some(".zs"),
        "zephir" => Some(".zep"),
        "zig" => Some(".zig"),
        "zil" => Some(".zil"),
        "zimpl" => Some(".zimpl"),
        "zsh" => Some(".sh"),
        _ => None,
    }
}

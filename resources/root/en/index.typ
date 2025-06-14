#import "../lib.typ": *

#show: schema.with("page")

#import "../index.typ": process


#title[Typsite Documentation]
#date[2025-04-12 02:08]
#author[Glomzzz]

#inline(alignment: center,scale: 37.5%,image("../icon.png"))

= Introduction
*Typsite* is a tool for building static websites. It uses pure *Typst* to write content and processes it through *Typsite* to generate a fully functional static site.

= Features

- Standard *Typst* expressions
- Framework support: hierarchical headings, section templates, sidebar, footer
- Rich text: paragraphs, sections, quotes, code blocks, math formulas, footnotes, page embedding
- Supports modern web standards such as HTML5, CSS3, and JavaScript (ES6+)
- Incremental compilation and real-time preview

= #embed("./typst.typ", sidebar: "only-title")

= Installation

- Download the binary from the #link("[https://github.com/Glomzzz/typsite/releases/latest")[release]
  - Make sure you have *Typst 0.13+* installed
- Build via Nix & Flakes
  - Ensure you’ve enabled `experimental-features = nix-command flakes`

```shell
git clone https://github.com/Glomzzz/typsite.git

cd typsite
nix build .
```

= Initialization

Run `typsite init` to initialize a Typsite project in the current directory.

```
.
├── root         ---  Typst root directory
│   ├── index.typ    --- Main article
│   └── lib.typ      --- Typsite library file
├── .typsite     ---  Typsite configuration directory
│   ├── assets       ---  Resources (synced to output directory)
│   ├── components   ---  Component templates
│   ├── themes       ---  Code highlighting themes
│   ├── rewrite      ---  Rewrite templates
│   ├── schemas      ---  Page templates
│   └── options.toml ---  Project configuration
├── .cache       ---  Cache directory
└── publish      ---  Output directory
```

= Writing

#import "@preview/frame-it:1.2.0": *

#let (example, feature, variant, syntax) = frames(
  feature: ("Feature",),
  variant: ("Variant",),
  example: ("Example", gray),
  syntax: ("Syntax",),
)

#show: frame-style(styles.boxy)

Typsite is compatible with most native *Typst* syntax:
#inline(scale: 150%, alignment: center)[
  #table(
    columns: (auto, auto, auto, auto),
    stroke: 0.4pt,
    fill: white,
    align: center,
    [*Element*], [*Support Status*], [*Element*], [*Support Status*],
    [`heading`], [#text(fill: blue.lighten(50%), [*t*])], [`par`], [#text(fill: blue.lighten(50%), [*t*])],
    [`text`], [#text(fill: blue.lighten(50%), [*t*])], [`strong`], [#text(fill: blue.lighten(50%), [*t*])],
    [`list`], [#text(fill: blue.lighten(50%), [*t*])], [`emph`], [✅],
    [`overline`], [✅], [`underline`], [✅],
    [`super`], [✅], [`sub`], [✅],
    [`raw`], [✅], [`link`], [✅],
    [`label`], [✅], [`ref`], [✅],
    [`footnote`], [✅], [`math.equation`], [✅],
    [`highlight`], [✅], [`text with color`], [✅],
    [`align`#super[1]], [inline#super[2]], [`strike`], [inline],
    [`table`], [inline], [`terms`], [inline],
    [`figure`], [inline],
  )
]

#footnote[For `align`, please use the `text-align` function from the library] <align>
#footnote[For complex styled elements, use the `inline` function for *SVG* embedding]

= CLI (Command Line Interface)

```
Usage: typsite <COMMAND>

Commands:
  init     Initialize a new Typsite project in the specified directory
  compile  Compile or watch a project; specify input/output [alias: c]
  clean    Clear cache and output directories
  syntect  Check the list of supported syntax and code highlighting
  help     Print this message or help info for a subcommand

Options:
  -h, --help     Print help
  -V, --version  Print version info
```

== init

```
Initialize a new Typsite project in the specified directory

Usage: typsite init [OPTIONS]

Options:
  -d, --dir <DIR>  Project root directory [default: ./]
  -h, --help       Print help
```

== compile

```
Compile or watch the project, specifying input/output directories

Usage: typsite compile [OPTIONS]

Options:
      --config <CONFIG>  
      --host <HOST>      Serve host [default: localhost]
      --port <PORT>      Serve port, must be specified to watch mode [default: 0]
      --cache <CACHE>    Cache directory [default: ./.cache]
  -i, --input <INPUT>    Typst root directory [default: ./root] [alias: --i]
  -o, --output <OUTPUT>  Output directory [default: ./publish] [alias: --o]
      --no-pretty-url
      --no-short-slug
  -h, --help             Print help
```

== clean

```
Clear cache and output directories

Usage: typsite clean [OPTIONS]

Options:
  -o, --output <OUTPUT>  Output directory [default: ./publish]
  -c, --cache <CACHE>    Cache directory for typst_html_export content [default: ./.cache]
  -h, --help             Print help
```

== syntect

```
View the list of syntax highlighting and supported languages  
Usage: typsite syntect [OPTIONS]

Options:
      --config <CONFIG>  Config path [default: ./.typsite]
  -h, --help             Print help
```


= Architecture & Flow

#inline(scale: 200%, fill: color.white, alignment: center)[#process]

= Configuration

You can view all default configurations here:

Based on these, you can fully customize your entire site.

- *schema*: Page templates, handling page structure
- *components / rewrites*: Components/Rewriters, build up the page content
- *assets*: Resource files, automatically synced during compilation
- *themes*: Code highlighting files
- *syntaxes*: Code syntaxes files

= #embed("./article.typ", sidebar: "only_title", open: false)
#text-align(center)[Why not take a look at #cite-title("./example.typ") first?]


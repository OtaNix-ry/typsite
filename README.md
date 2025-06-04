
# Typsite
[ **English** | [中文](./README-cn.md) ]

## 1. Introduction

Typsite is a static site generator (SSG) that uses pure `Typst` for content creation. It processes these `Typst` files to generate a complete static website.

## 2. Features

-   Core `Typst` expressions and syntax
-   Framework: Incremental headings, section templates, sidebar, footer
-   Rich Text: Paragraphs, sections, quotes, code blocks, math formulas, footnotes, page embeds
-   Support for modern web technologies: HTML5, CSS3, and JavaScript (ES6+)

## 3. About Typst

Typst is a modern typesetting system, similar to LaTeX but designed to be simpler and easier to learn. It's primarily used for creating beautifully typeset documents like academic papers, books, and reports.

You can find the official English documentation here: [Typst Document](https://typst.app/docs/)
For a Chinese tutorial on Typst, I highly recommend the [Typst Blue Book](https://typst-doc-cn.github.io/tutorial/introduction.html) (The author has a remarkably clear understanding of the subject!).

The rest of this section explains the connection between Typst, HTML, and Typsite.

### 3.1 Typst's HTML Export Feature

Typst introduced HTML export functionality in version `0.13`. This includes an `html-export` mode and two core functions: `html.elem` and `html.frame`. These allow us to write content in Typst that targets HTML+CSS output.

### 3.2 Typsite: A Typst-based Static Site Generator

Inspired by this, I developed a static site generator named `Typsite` using `Rust`.

Currently, Typst's HTML export:
-   Has good support for simple rich text.
-   Complex styling requires users to manually write it using the `html.elem` function.
-   Cannot automatically convert all Typst ecosystem content to HTML.
-   For content with complex Typst styling, `html.frame` can be used to convert it to SVG and embed it in HTML.
-   Each compilation only supports single-file HTML output.

For details on supported features and plans, you can track this [issue: HTML export #5512](https://github.com/typst/typst/issues/5512).

While there are official plans for Typst to automatically convert styles to HTML+CSS, this doesn't conflict with Typsite. In fact, Typst's advancement will make Typsite even more useful, as Typsite's primary role is to manage inter-article interactions and build a fully-featured static website.

## 4. Installation

-   Download the binary from the [Release page](https://github.com/Glomzzz/typsite/releases/latest).
    -   Ensure you have Typst **0.13+** installed.
-   Build using Nix & Flakes.
    -   Ensure you have enabled `experimental-features = nix-command flakes` in your Nix configuration.

```shell
git clone https://github.com/Glomzzz/typsite.git
cd typsite
nix build .
```

## 5. Initialization

You can initialize a Typsite project in the current directory using typsite init.
```
.
├── root           --- Typst root directory
│   ├── index.typ  --- Article file
│   └── lib.typ    --- Typsite library file
├── .typsite       --- Typsite configuration directory
│   ├── assets     --- Asset directory (synced to the output directory)
│   ├── components --- Component templates
│   ├── themes     --- Code highlighting themes
│   ├── rewrite    --- Rewriter templates
│   ├── schemas    --- Page templates
│   └── options.toml --- Project configuration
├── .cache         --- Cache directory
└── publish        --- Output directory
```

## 6. Command Line
```
Usage: typsite <COMMAND>

Commands:
  init     Initialize a new typsite project in the specified directory
  compile  Compile or watch the project, specifying input and output directories [alias: c]
  clean    Clear cache and output directories
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version information

```
### 6.1 init
Initialize a new typsite project in the specified directory

```
Usage: typsite init [OPTIONS]

Options:
  -d, --dir <DIR>  Project root directory [default: ./]
  -h, --help       Print help

```
### 6.2 compile
Compile or watch the project, specifying input and output directories

```
Usage: typsite compile [OPTIONS]

Options:
      --port <PORT>      Server port [default: 0]
      --config <CONFIG>  Project HTML configuration path [default: ./.typsite]
      --cache <CACHE>    Cache directory [default: ./.cache]
  -i, --input <INPUT>    Typst root directory, where typst files are located [default: ./root] [alias: --i]
  -o, --output <OUTPUT>  Output directory [default: ./publish] [alias: --o]
      --no-pretty-url
      --no-short-slug
  -h, --help             Print help

```
### 6.3 clean
Clear cache and output directories

```
Usage: typsite clean [OPTIONS]

Options:
  -o, --output <OUTPUT>  Output directory [default: ./publish]
  -c, --cache <CACHE>    Cache directory, used to store raw typst_html_export content [default: ./.cache]
  -h, --help             Print help

```
## 7. Architecture & Flow

![alt text](./process.png)

## 8. Configuration

You can view and modify the configuration in the **options.toml** file and the **.typsite** directory within your project.
Based on these configurations, you can fully customize your entire site.

- `schemas`: Page templates, responsible for the page structure/framework.

- `components` / `rewrites`: Components/Rewriters, which make up the page content.

- `assets`: Directory for static assets, automatically synced to the output directory during compilation.

- `themes`: Code highlighting theme files.

## Contribution

We welcome your contributions to Typsite!

## Acknowledgements

- [kokic](https://github.com/kokic) : [Kodama](https://github.com/kokic/kodama) for **Markdown + Typst + LaTeX SSG**

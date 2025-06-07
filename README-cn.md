
# Typsite

## 1. 介绍

Typsite 是一个用于构建静态网站的工具，其文章内容由纯 `Typst` 编写，经由 `Typsite` 进行处理后，最终生成一个健全的静态站点。


## 2. 功能

- `Typst` 的常规表达
- 框架: 标题递增、小节模板、侧边栏、页脚
- 富文本：段落、小节、引用、代码块、数学公式、注脚、页面嵌入
- 支持现代 Web 技术规范，如 HTML5、CSS3 和 JavaScript（ES6+）

## 3. Typst 简介

Typst 是一种现代化的排版系统，类似于 LaTeX，但设计更为简洁、易学，它主要用于创建学术论文、书籍、报告等需要精美排版的文档。

你可以在这里查看其官方英文文档：[Typst Document](https://typst.app/docs/)
对于Typst的中文教程，我强烈推荐：[Typst 蓝书](https://typst-doc-cn.github.io/tutorial/introduction.html) (天呐，这位编者非常清楚地知道自己是在阐述一套本体论!).

此节的剩余部分将会介绍 Typst & HTML -> Typsite

### 3.1 Typst 的 HTML 导出功能

Typst 于 `0.13` 增加了 HTML 导出功能，包括 `html-export` 模式以及两个核心函数：`html.elem` 和 `html.frame` —— 我们可以利用这些函数来基于 Typst 编写以HTML+CSS为目标内容。

### 3.2 Typsite：基于 Typst 的静态站点生成器

受此启发，我用 `Rust` 开发了名为 `Typsite` 的静态站点生成器.

目前，Typst 的 HTML 导出:
- 对简单富文本已有良好支持
- 复杂样式需要用户通过 `html.elem` 函数手动编写
- 无法自动将所有 Typst 生态内容转换为 HTML
- 对于包含复杂 Typst 样式的内容，可以使用 `html.frame` 将其转为 SVG 并嵌入 HTML
- 对于每一次 compile, 只支持单文件 HTML 输出

对于详细的已支持内容与计划可以追踪这个 [issue: HTML export #5512](https://github.com/typst/typst/issues/5512)

虽然官方有计划支持自动 typst style -> HTML+CSS，但这并不会与 Typsite 产生任何冲突。 恰恰相反，Typst 的发展将使 Typsite 更加实用，因为 `Typsite` 主要职能是协调文章间的交互，并最终构建一套功能完善的静态网站。

## 4. 安装

- 通过模板: [Typsite Template](https://github.com/Glomzzz/typsite-template) (推荐, for linux / macos)
- 通过 [Release 页面](https://github.com/Glomzzz/typsite/releases/latest)下载二进制文件
    - 请确保你已安装了**0.13+**的typst
- 通过 Nix & Flakes 构建
    - 请确保你已经开启了 `experimental-features = nix-command flakes`

```shell
git clone https://github.com/Glomzzz/typsite.git

cd typsite
nix build .
```

## 5. 初始化

通过 `typsite init`, 可以在当前文件夹初始化 Typsite.

```
.
├── root           ---  typst 根目录
│   ├── index.typ  --- 文章
│   └── lib.typ    --- typsite 库文件
├── .typsite       --- typsite 配置目录
│   ├── assets     ---  资源目录 (会同步到输出目录)
│   ├── components ---  组件模板
│   ├── themes     ---  代码高亮
│   ├── rewrite    ---  重写器模板
│   ├── schemas    ---  页面模板
│   └── options.toml ---  项目配置
├── .cache         ---  缓存目录
└── publish        ---  输出目录
```

## 6. 命令行

```shell
用法: typsite <COMMAND>

命令:
  init     在指定目录中初始化一个新的 typsite 项目
  compile  编译或监听项目，指定输入和输出目录 [别名: c]
  clean    清除缓存和输出目录
  help     打印此消息或指定子命令的帮助信息

选项:
  -h, --help     打印帮助
  -V, --version  打印版本信息
```

### 6.1 init

```shell
在指定目录中初始化一个新的 typsite 项目

用法: typsite init [OPTIONS]

选项:
  -d, --dir <DIR>  项目根目录 [默认: ./]
  -h, --help       打印帮助
```

### 6.2 compile

```shell
编译或监听项目，指定输入和输出目录

用法: typsite compile [OPTIONS]

选项:
      --port <PORT>      服务端口 [默认: 0]
      --config <CONFIG>  项目 html 配置路径 [默认: ./.typsite]
      --cache <CACHE>    缓存目录 [默认: ./.cache]
  -i, --input <INPUT>    Typst 根目录，存放 typst 文件的位置 [默认: ./root] [别名: --i]
  -o, --output <OUTPUT>  输出目录 [默认: ./publish] [别名: --o]
      --no-pretty-url
      --no-short-slug
  -h, --help             打印帮助
```

### 6.3 clean

```shell
清除缓存和输出目录

用法: typsite clean [OPTIONS]

选项:
  -o, --output <OUTPUT>  输出目录 [默认: ./publish]
  -c, --cache <CACHE>    缓存目录，用于存储原始 typst_html_export 内容 [默认: ./.cache]
  -h, --help             打印帮助
```

## 7. 架构 & 流程

![](./process.png)

## 8. 配置

你可以在项目中的 `options.toml` 和 `.typsite` 目录查看和修改配置。
基于这些配置, 你可以完全地自定义你的整个站点.

- `schema`: 页面模板, 负责处理页面框架
- `components` / `rewrites`: 组件/重写器, 组成页面内容
- `assets`: 资源文件目录, 会在compile时自动同步到输出目录
- `themes`: 代码高亮文件

## 贡献 

来和我一起壮大typsite!

## 鸣谢
- [kokic](https://github.com/kokic) : [Kodama](https://github.com/kokic/kodama) for **Markdown + Typst + LaTeX SSG**

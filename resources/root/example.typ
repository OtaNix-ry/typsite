#import "lib.typ": *

#show: schema.with("page")


#title[内容示例]
#date[2025-06-05 07:12]
#author[Glomzzz]

== 好看的

这是一段普通的文本.

#text-align(center)[

  #html-text(size: 52pt, weight: "bold", fill: rgb("#22D3EE"))[Typst]
  \
  \
  #html-text(size: 38pt, fill: rgb("#22D3EE"))[🔥*已经崛起了!*🔥] <rise-up>
  \
  \

  #html-text(size: 22pt, style: "italic", fill: red)[🚀_这TeX人没收到通知吗？_🚀]
  \
  \
]



#text-align(center)[
  #html-text(size: 40pt)[#underline[我的]#highlight(fill: green.lighten(50%))[大树人]，#overline[没了]！#footnote(<np>)]
]

$
  ker tau & = {[x]_U in V slash U | [x]_W = [0]_W} \
  & = {[x]_U in V slash U | x in W} \
$

#footnote[自然先知的铁树树人, 在7.39b 游戏性版本更新中, 也许永远地离开了我们....] <np>

== 好玩的

点@amazing[我]能跳转到神奇的地方.

点@rise-up[我]能跳转到 #html-text(fill: red)[**崛起**]!

== 好听的

Another One Bites the Dust#footnote(<dust>)


#footnote[ #link("https://music.apple.com/us/song/another-one-bites-the-dust/1440650719")[来听!] ] <dust>

== 神奇的地方 <amazing>

引用: #cite("./typst.typ")[我能自定义引用段的内容]   or  我也能直接用引用文章的标题:  #cite-title("./typst.typ")

我还能嵌入页面!

#html-text(size: 30pt)[⬇️] 我还能直接把嵌入的内容当作某一个特定heading-level的section来用! 
=== #embed("./typst.typ", open: false, sidebar: "only_title")


=== RUUUST
```rust
fn main() {
    let f: fn(&'static str) -> usize = |s| unsafe {
        *s.as_ptr().offset(1) as usize & 0xFF
    };
    println!("{}", (0..5).map(|i| f("hello") ^ i).fold(0, |a, b| a ^ b));
}
```

=== Typsite 流程图

#import "./index.typ": process

#inline(scale: 200%,fill: color.white,alignment: center)[#process]


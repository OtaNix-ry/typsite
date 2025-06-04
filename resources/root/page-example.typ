
#import "lib.typ": *

#show : schema.with("page") // Schema

// Metadata
// MetaOptions
#sidebar("full")
#heading_numbering("roman")
// MetaGraph
#parent("index.typ")
// MetaContent
#title[页面示例]
#taxon[Figure]
#date[2025-04-02 2:08]

// Content
== 我是示例页面！

你可以这样调用元内容(MetaContent)：#metacontent("title")

=== 当然，我支持在heading中调用元内容：#metacontent("taxon")

我是在 #metacontent("date") 创建的。

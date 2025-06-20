// Site lib, feel free to customize your own site
#import "@local/typsite:0.1.0": (
  inline,
  embed,
  metacontent,
  get-metacontent,
  title,
  page-title,
  taxon,
  author,
  date,
  heading-numbering,
  sidebar,
  parent,
)

#import "html.typ" as html

#import "site.typ" : (
  details, 
  inline-math,
  mathml-or-inline,
  auto-filter
)

#import "rewrite.typ" :  (
  cite, cite-title
)


/// Schema for a article page
/// Usage:
/// ```typ
/// #show : schema.with("page")
/// 
/// #Your article content 
/// ```
/// - name(str): one of file names in `.typsite/schemas/`
///     The name of the schema
/// - head(content):
///     Custom head of a article
/// - body(content):
///     The body of the article
/// -> HTML document with a schema ~> HTML Page
#let schema(name,head:[], body) = {
  import "@local/typsite:0.1.0": schema
  schema(
    name,
    body,
    [
      #import "@local/typsite:0.1.0": mathyml
      #mathyml.include-mathfont()
      #head
    ],
    body => {
      import "rule.typ": *
      show: rule-decorate
      show: rule-equation-mathyml-or-inline
      show: rule-footnote
      show: rule-link-common
      show: rule-link-anchor
      show: rule-ref-footnote
      show: rule-ref-label
      show: rule-raw
      show: rule-label
      body
    },
  )
}


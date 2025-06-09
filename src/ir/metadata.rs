pub mod content;
pub mod graph;
pub mod options;

use crate::compile::error::TypResult;
use crate::compile::registry::{Key, KeyRegistry};
use crate::config::TypsiteConfig;
use crate::ir::metadata::content::{MetaContents, PureMetaContents};
use crate::ir::metadata::graph::{MetaNode, PureMetaNode};
use crate::ir::metadata::options::MetaOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata<'a> {
    pub contents: MetaContents<'a>,
    pub options: MetaOptions,
    pub node: MetaNode,
}

impl<'a> Metadata<'a> {
    pub fn from(
        slug: Key,
        pure: PureMetadata,
        config: &'a TypsiteConfig,
        registry: &KeyRegistry,
    ) -> TypResult<Metadata<'a>> {
        let contents = MetaContents::from(slug.clone(), pure.contents, config)?;
        let options = pure.options;
        let node = MetaNode::from(slug, pure.node, registry)?;
        Ok(Metadata {
            contents,
            options,
            node,
        })
    }

    pub fn inline(&self, html: &str) -> String {
        self.inline_with(html, &[])
    }

    /*
    {content_key} -> replace with self meta content
    {content_key@parent} -> replace with parent's meta content
     */
    pub fn inline_with(&self, html: &str, replacements: &[(&str, &str)]) -> String {
        self.contents.inline_with(html, replacements)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PureMetadata {
    contents: PureMetaContents,

    options: MetaOptions,

    node: PureMetaNode,
}
impl From<Metadata<'_>> for PureMetadata {
    fn from(metadata: Metadata) -> Self {
        let contents = PureMetaContents::from(metadata.contents);
        let options = metadata.options;
        let node = PureMetaNode::from(metadata.node);
        Self {
            contents,
            options,
            node,
        }
    }
}

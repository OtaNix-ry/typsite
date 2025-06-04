use crate::ir::article::sidebar::SidebarIndex;
use crate::config::TypsiteConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RewriterType {
    Start,
    End,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BodyRewriter<'a> {
    pub id: &'a str,
    pub rewriter_type: RewriterType,
    pub attributes: HashMap<String, String>,
    pub sidebar_index: SidebarIndex,
    pub body_index: usize,
}

impl<'a> BodyRewriter<'a> {
    pub fn new(
        id: &'a str,
        rewriter_type: RewriterType,
        attributes: HashMap<String, String>,
        sidebar_index: SidebarIndex,
        body_index: usize,
    ) -> Self {
        Self {
            id,
            rewriter_type,
            attributes,
            sidebar_index,
            body_index,
        }
    }
    pub fn from(
        self_slug: &str,
        pure: PureRewriter,
        config: &'a TypsiteConfig,
    ) -> Option<BodyRewriter<'a>> {
        let id = pure.id.as_str();
        let rewriter = config.rules.rule_name(id);
        if let Some(id) = rewriter {
            Some(BodyRewriter {
                id,
                rewriter_type: pure.rewriter_type,
                attributes: pure.attributes,
                sidebar_index: pure.sidebar_pos,
                body_index: pure.body_index,
            })
        } else {
            eprintln!("[WARN] Rewriter not found: {id} in {self_slug}, skip this rewriter.");
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaRewriter<'a> {
    pub id: &'a str,
    pub rewriter_type: RewriterType,
    pub attributes: HashMap<String, String>,
    pub body_index: usize,
}

impl<'a> MetaRewriter<'a> {
    pub fn new(
        rule_id: &'a str,
        rewriter_type: RewriterType,
        attributes: HashMap<String, String>,
        body_index: usize,
    ) -> Self {
        Self {
            id: rule_id,
            rewriter_type,
            attributes,
            body_index,
        }
    }
    pub fn from(
        self_slug: &str,
        pure: PureRewriter,
        config: &'a TypsiteConfig,
    ) -> Option<MetaRewriter<'a>> {
        let id = pure.id.as_str();
        let rewriter = config.rules.rule_name(id);
        if let Some(id) = rewriter {
            Some(MetaRewriter {
                id,
                rewriter_type: pure.rewriter_type,
                attributes: pure.attributes,
                body_index: pure.body_index,
            })
        } else {
            eprintln!("[WARN] Rewriter not found: {id} in {self_slug}, skip this meta rewriter.");
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PureRewriter {
    id: String,
    rewriter_type: RewriterType,
    attributes: HashMap<String, String>,
    sidebar_pos: SidebarIndex,
    body_index: usize,
}

impl PureRewriter {
    #[allow(dead_code)]
    pub fn new(
        id: String,
        rewriter_type: RewriterType,
        attributes: HashMap<String, String>,
        sidebar_pos: SidebarIndex,
        body_index: usize,
    ) -> Self {
        Self {
            id,
            rewriter_type,
            attributes,
            sidebar_pos,
            body_index,
        }
    }
}

impl From<BodyRewriter<'_>> for PureRewriter {
    fn from(rewriter: BodyRewriter) -> Self {
        Self {
            id: rewriter.id.to_string(),
            rewriter_type: rewriter.rewriter_type,
            attributes: rewriter.attributes,
            sidebar_pos: rewriter.sidebar_index,
            body_index: rewriter.body_index,
        }
    }
}
impl From<MetaRewriter<'_>> for PureRewriter {
    fn from(rewriter: MetaRewriter) -> Self {
        Self {
            id: rewriter.id.to_string(),
            rewriter_type: rewriter.rewriter_type,
            attributes: rewriter.attributes,
            sidebar_pos: SidebarIndex::default(),
            body_index: rewriter.body_index,
        }
    }
}

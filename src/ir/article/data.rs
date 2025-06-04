use crate::ir::metadata::Metadata;
use crate::ir::pending::Pending;
use crate::compile::compiler::{ compile_options};
use crate::compile::registry::Key;
use crate::compile::watch::WATCH_AUTO_RELOAD_SCRIPT;
use crate::config::TypsiteConfig;
use crate::config::schema::{BACKLINK_KEY, REFERENCE_KEY};
use crate::pass::{pass_embed, pass_rewriter_body, pass_schema};
use crate::util::html::{OutputHead, OutputHtml};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use super::dep::Indexes;
use super::Article;


pub struct GlobalData<'a, 'b, 'c> {
    pub config: &'a TypsiteConfig<'a>,
    pub articles: &'b HashMap<Key, Article<'a>>,
    pendings: HashMap<Key, OnceLock<Pending<'c>>>,
    global_body_rewrite_indexes: HashMap<Key, Indexes>,
    global_body_embed_indexes: HashMap<Key, Indexes>,
}
impl<'c, 'b: 'c, 'a: 'b> GlobalData<'a, 'b, 'c> {
    pub fn new(
        config: &'a TypsiteConfig<'a>,
        articles: &'b HashMap<Key, Article<'a>>,
        pendings: HashMap<Key, OnceLock<Pending<'c>>>,
        global_body_rewrite_indexes: HashMap<Key, Indexes>,
        global_body_embed_indexes: HashMap<Key, Indexes>,
    ) -> Self {
        Self {
            config,
            articles,
            pendings,
            global_body_rewrite_indexes,
            global_body_embed_indexes,
        }
    }
    pub fn article(&'c self, id: &str) -> Option<&'b Article<'a>> {
        self.articles.get(id)
    }

    pub fn metadata(&'c self, id: &str) -> Option<&'b Metadata<'a>> {
        let article = self.article(id)?;
        Some(article.get_metadata())
    }

    pub(super) fn init_cache(&'c self, article: &'b Article<'a>) -> (Vec<String>, Vec<String>, Vec<String>) {
        let rewriter_indexes = self
            .global_body_rewrite_indexes
            .get(article.slug.as_str())
            .unwrap();
        let metadata = &article.metadata;
        let mut body = article.body.clone();
        let mut full_sidebar = article.full_sidebar.cache(metadata);
        let embed_sidebar = article.embed_sidebar.cache(metadata);
        pass_rewriter_body(
            article.slug.clone(),
            &mut body.content,
            &mut full_sidebar,
            &body.rewriters,
            rewriter_indexes,
            self,
        );
        (body.content, full_sidebar, embed_sidebar)
    }

    pub(super) fn get_pending_or_init(&'c self, article: &'b Article<'a>) -> &'c Pending<'c> {
        self.pendings
            .get(article.slug.as_str())
            .map(|pending| {
                pending.get_or_init(|| {
                    let embed_indexes = self
                        .global_body_embed_indexes
                        .get(article.slug.as_str())
                        .unwrap();
                    let content = article.get_content_or_init(self);
                    pass_embed(
                        article.slug.clone(),
                        content,
                        &article.embeds,
                        embed_indexes,
                        self,
                    )
                })
            })
            .unwrap()
    }

    pub fn init_backlink(
        &'c self,
        article: &'b Article<'a>,
        content: &str,
        sidebar: &str,
    ) -> anyhow::Result<()> {
        let schema = self.config.schemas.get(BACKLINK_KEY);
        let backlink = match schema {
            None => {
                eprintln!(
                    "[WARN] Schema {} not found in article {}",
                    BACKLINK_KEY, article.slug
                );
                OutputHtml::empty()
            }
            Some(schema) => {
                match pass_schema(
                    self.config,
                    schema,
                    article,
                    content.as_str(),
                    sidebar.as_str(),
                    self,
                ) {
                    Ok(html) => html,
                    Err(err) => {
                        eprintln!(
                            "[WARN] Error occurred while passing article {} with schema {BACKLINK_KEY}: {:?}",
                            article.slug, err
                        );
                        OutputHtml::empty()
                    }
                }
            }
        };
        article
            .cache
            .backlink
            .set(backlink)
            .map_err(|_| anyhow::anyhow!("Failed to set backlink for article {}", article.slug))
    }
    pub fn init_reference(
        &'c self,
        article: &'b Article<'a>,
        content: &str,
        sidebar: &str,
    ) -> anyhow::Result<()> {
        let schema = self.config.schemas.get(REFERENCE_KEY);
        let reference = match schema {
            None => {
                eprintln!(
                    "[WARN] Schema {} not found in article {}",
                    REFERENCE_KEY, article.slug
                );
                OutputHtml::empty()
            }
            Some(schema) => {
                match pass_schema(
                    self.config,
                    schema,
                    article,
                    content.as_str(),
                    sidebar.as_str(),
                    self,
                ) {
                    Ok(html) => html,
                    Err(err) => {
                        eprintln!(
                            "[WARN] Error occurred while passing article {} with schema {REFERENCE_KEY}: {:?}",
                            article.slug, err
                        );
                        OutputHtml::empty()
                    }
                }
            }
        };
        article
            .cache
            .reference
            .set(reference)
            .map_err(|_| anyhow::anyhow!("Failed to set reference for article {}", article.slug))
    }

    pub fn init_html_head(&'c self, article: &'b Article<'a>) -> &'b OutputHead<'a> {
        article.cache.html_head.get_or_init(|| {
            let metadata = article.get_metadata();
            let schema = article.schema;

            let mut head = OutputHead::empty();
            // Head
            head.start(metadata.inline(schema.head.as_str()));

            head.push(self.config.section.head.as_str());
            head.push(self.config.heading_numbering.head.as_str());

            if schema.sidebar {
                head.push(self.config.sidebar.each.head.as_str());
                head.push(self.config.sidebar.block.head.as_str());
            }

            if !metadata.node.children.is_empty() {
                head.push(self.config.embed.embed.head.as_str());
                head.push(self.config.embed.embed_title.head.as_str());
            }

            if !article.get_anchors().is_empty() {
                head.push(self.config.anchor.define.head.as_str());
                head.push(self.config.anchor.goto.head.as_str());
            }

            if compile_options().watch {
                head.push(WATCH_AUTO_RELOAD_SCRIPT.as_str());
            }

            let rules = article.all_used_rules(self);
            {
                let mut heads = HashSet::new();
                for rule_id in rules.iter() {
                    let rule = self.config.rules.get(rule_id);
                    match rule {
                        None => {
                            eprintln!(
                                "[WARN] Rule {} not found in article {}",
                                rule_id, article.slug
                            );
                            continue;
                        }
                        Some(rule) => {
                            heads.insert(&rule.head);
                        }
                    }
                }
                for rule_head in heads {
                    head.push(rule_head.as_str());
                }
            }
            head
        })
    }
}


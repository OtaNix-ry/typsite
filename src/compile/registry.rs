use crate::config::TypsiteConfig;
use anyhow::*;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

pub type Key = Arc<str>;

pub type SlugPath = Arc<Path>;

pub type KeyMap<V> = HashMap<String, V>;

pub struct KeyRegistry {
    known_articles: KeyMap<Key>,
    article_paths: KeyMap<SlugPath>,
}

impl KeyRegistry {
    pub fn new() -> Self {
        Self {
            known_articles: KeyMap::new(),
            article_paths: KeyMap::new(),
        }
    }

    pub fn register_slug(&mut self, slug: String) -> Key {
        match self.known_articles.get(&slug) {
            Some(key) => key.clone(),
            None => {
                let key: Arc<str> = Arc::from(slug.clone());
                self.known_articles.insert(slug, key.clone());
                key
            }
        }
    }
    pub fn remove_slug(&mut self, slug: &str) {
        self.known_articles.remove(slug);
        self.article_paths.remove(slug);
    }

    pub fn register_paths<I, P>(&mut self, config: &TypsiteConfig, paths: I)
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        paths.into_iter().for_each(|path| {
            self.register_article_path(config, path.as_ref());
        });
    }
    pub fn register_article_path(
        &mut self,
        config: &TypsiteConfig,
        path: &Path,
    ) -> (Key, SlugPath) {
        let slug = config.path_to_slug(path);
        let slug: Key = self.register_slug(slug);
        let arc: Arc<Path> = Arc::from(path);
        (
            slug.clone(),
            self.article_paths
                .entry(slug.to_string())
                .or_insert(arc.clone())
                .clone(),
        )
    }

    pub fn slug(&self, slug: &str) -> Option<Key> {
        self.known_articles.get(slug).cloned()
    }

    pub fn path(&self, slug: &str) -> Option<SlugPath> {
        self.article_paths.get(slug).cloned()
    }

    pub fn know(&self, slug: String, tag: &str, from: &str) -> Result<Key> {
        self.known_articles
            .get(&slug)
            .cloned()
            .context(format!("{tag} not found: {slug} in {from}"))
    }
}

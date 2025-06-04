use crate::compile::registry::{Key, KeyRegistry};
use crate::config::TypsiteConfig;
use crate::ir::article::{Article, PureArticle};
use crate::util::error::{log_err, log_err_or_ok};
use crate::util::fs::write_into_file;
use crate::walk_glob;
use anyhow::Context;
use glob::glob;
use rayon::prelude::*;
use std::collections::hash_map::Drain;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct ArticleCache<'a> {
    cache_article_path: PathBuf,
    pub cache: HashMap<Key, Article<'a>>,
}

impl<'a> ArticleCache<'a> {
    pub fn new(cache_path: &Path) -> ArticleCache<'a> {
        let cache_article_path = cache_path.join("article");
        Self {
            cache_article_path,
            cache: HashMap::new(),
        }
    }
    pub fn load(
        &mut self,
        config: &'a TypsiteConfig,
        deleted: &HashSet<PathBuf>,
        registry: &mut KeyRegistry,
    ) {
        let deleted_json = deleted
            .iter()
            .map(|path| path.with_extension("json"))
            .collect::<HashSet<PathBuf>>();

        let pures = walk_glob!("{}/**/*.json", self.cache_article_path.display())
            .filter(|path| !deleted_json.contains(path))
            .par_bridge()
            .map(|path| {
                std::fs::read_to_string(&path)
                    .context("Failed to read pure article file")
                    .and_then(|json| {
                        serde_json::from_str::<PureArticle>(&json)
                            .context("Failed to parse pure article")
                    })
            })
            .filter_map(log_err_or_ok)
            .collect::<Vec<PureArticle>>();

        self.insert_batch(config, pures, registry);

        deleted_json.into_par_iter().for_each(|path| {
            let _ = std::fs::remove_file(&path);
        });
    }

    fn insert_batch(
        &mut self,
        config: &'a TypsiteConfig,
        pures: Vec<PureArticle>,
        registry: &mut KeyRegistry,
    ) {
        registry.register_paths(config, pures.iter().map(|pure| pure.path.as_path()));
        let registry: &KeyRegistry = registry;
        let articles: Vec<(Key, Article<'a>)> = pures
            .into_iter()
            .par_bridge()
            .map(|pure| {
                let article = Article::from(pure, config, registry);
                (article.slug.clone(), article)
            })
            .collect();
        self.cache.extend(articles);
    }

    #[allow(clippy::type_complexity)]
    pub fn write_cache(
        &mut self,
        slugs: HashMap<Key, (Vec<String>, Vec<String>, Vec<String>)>,
    ) -> anyhow::Result<()> {
        slugs
            .into_iter()
            .map(|(slug, cache)| {
                let article = self.cache.remove(slug.as_str()).expect("Article not found");
                let path = self
                    .cache_article_path
                    .join(format!("{}.json", article.path.display()));
                let pure = PureArticle::from(article, cache);
                serde_json::to_string::<PureArticle>(&pure)
                    .context("Failed to serialize pure article")
                    .map(|json| (path, json))
            })
            .filter_map(log_err_or_ok)
            .collect::<Vec<(PathBuf, String)>>()
            .into_iter()
            .par_bridge()
            .map(|(path, json)| write_into_file(path, &json))
            .for_each(log_err);
        Ok(())
    }

    pub fn drain(&mut self) -> Drain<'_, Key, Article<'a>> {
        self.cache.drain()
    }

    pub fn refresh<T: IntoIterator<Item = (Key, Article<'a>)>>(
        &mut self,
        registry: &mut KeyRegistry,
        new: T,
    ) {
        new.into_iter().for_each(|(slug, article)| {
            registry.register_slug(slug.to_string());
            self.cache.insert(slug, article);
        });
    }
}

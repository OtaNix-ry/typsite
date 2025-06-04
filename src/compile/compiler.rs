use crate::compile::cache::article::ArticleCache;
use crate::compile::cache::dep::RevDeps;
use crate::compile::options::CompileOptions;
use crate::compile::registry::KeyRegistry;
use crate::util::path::{file_ext, format_path};
use analysis::*;
use anyhow::*;
use html_pass::pass_html;
use initializer::{Input,  initialize};
use lazy_static::lazy_static;
use output_sync::sync_files_to_output;
use page_composer::{PageData, compose_pages};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, MutexGuard};
use std::{
    path::PathBuf,
    result::Result::Ok,
};
use typst_pass::compile_typsts;

use super::watch::watch;

mod analysis;
mod html_pass;
mod initializer;
mod output_sync;
mod page_composer;
mod typst_pass;

lazy_static! {
    pub static ref COMPILE_OPTIONS: Mutex<CompileOptions> = Mutex::new(CompileOptions::empty());
}
fn init_compile_options(config: CompileOptions) {
    let mut guard = COMPILE_OPTIONS.lock().unwrap();
    *guard = config;
}

pub fn compile_options() -> MutexGuard<'static, CompileOptions> {
    COMPILE_OPTIONS.lock().unwrap()
}
pub fn with_options<T>(f: impl FnOnce(&CompileOptions) -> T) -> T {
    let guard = COMPILE_OPTIONS.lock().unwrap();
    f(&guard)
}

pub struct Compiler {
    typst_path: PathBuf,             // Typst root
    html_cache_path: PathBuf, // Typst-export-html path (in which are raw typst-html-export files)
    config_path: PathBuf,     // Config root
    pub(crate) cache_path: PathBuf, // Cache root
    pub(crate) output_path: PathBuf, // Output
}

impl Compiler {
    pub fn new(
        options: CompileOptions,
        cache_path: PathBuf,
        config_path: PathBuf,
        typst_path: PathBuf,
        output_path: PathBuf,
    ) -> Result<Self> {
        init_compile_options(options);
        let cache_path = format_path(cache_path);
        let html_cache_path = cache_path.join("html");
        let config_path = format_path(config_path);
        let typst_path = format_path(typst_path);
        let output_path = format_path(output_path);
        Ok(Self {
            html_cache_path,
            cache_path,
            typst_path,
            config_path,
            output_path,
        })
    }
    pub async fn watch(self, port: u16) -> Result<()> {
        watch(self, port).await
    }
    pub fn compile(&self) -> Result<bool> {
        //1. Initialize input & config
        let input = initialize(
            &self.cache_path,
            &self.typst_path,
            &self.html_cache_path,
            &self.config_path,
        )?;
        // If all files are not changed, return
        if input.changed() {
            return Ok(false);
        }
        let Input {
            mut monitor,
            config,
            changed_typst_paths,
            deleted_typst_paths,
            changed_config_paths,
            deleted_config_paths,
            changed_non_typst,
            deleted_non_typst,
            overall_compile_needed,
        } = input;

        let mut registry = KeyRegistry::new();

        // Article Manager, which manages all articles' slugs and paths
        let mut article_cache = ArticleCache::new(&self.cache_path);

        // If it's init, register all typst paths
        if overall_compile_needed {
            registry.register_paths(&config, changed_typst_paths.iter());
        } else {
            // Load Article Cache If needed
            article_cache.load(&config, &deleted_typst_paths, &mut registry);
        }

        //2. Export typst as HTML
        // Only compile updated typst files into html
        compile_typsts(
            &self.typst_path,
            &self.html_cache_path,
            &changed_typst_paths,
        );

        // Get updated html files ( which are compiled from typst files )
        let mut changed_html_paths: Vec<PathBuf> = monitor
            .refresh_html(&self.html_cache_path, overall_compile_needed)?
            .into_iter()
            .collect();

        //3. Pass HTML
        // Pass updated html files
        let (changed_articles, error_articles) = pass_html(
            &self.html_cache_path,
            &config,
            &mut registry,
            &mut changed_html_paths,
        );

        let changed_article_slugs = changed_articles
            .iter()
            .map(|article| article.slug.clone())
            .collect::<HashSet<_>>();

        //4. Analyse articles
        // Record parents and backlinks
        let (parents, backlinks) = analyse_parents_and_backlinks(&changed_articles);

        // Collect all updated articles
        let mut updated_articles = article_cache
            .drain() // Drain all articles from Article Manager ( for a simpler lifetime)
            .chain(changed_articles.into_iter().map(|a| (a.slug.clone(), a)))
            .collect::<HashMap<_, _>>();

        // Update parents and backlinks into all articles apply_parents_and_backlinks(&mut updated_articles, parents, backlinks);
        apply_parents_and_backlinks(&mut updated_articles, parents, backlinks);

        // Load Reverse Dependency Cache
        let mut rev_dependency = RevDeps::load(
            &config,
            &self.cache_path,
            &deleted_typst_paths,
            &mut registry,
        );

        // Refresh Dependency Cache
        // in which we record all the dependencies(with its exactly indexes) of each article,
        // and the Reverse Dependencies of each file path are collected. ( Reverse Dependencies = Map<Path -> The files that depend on this file>)
        rev_dependency.refresh(&config, &registry, &updated_articles);

        // 5. Compose pages
        let PageData { output, cache } = compose_pages(
            &config,
            changed_article_slugs,
            changed_typst_paths,
            &changed_config_paths,
            &updated_articles,
            rev_dependency,
            overall_compile_needed,
        )?;

        let updated = !output.is_empty();
        // 6. Update cache
        article_cache.refresh(&mut registry, updated_articles);
        article_cache.write_cache(cache)?;

        // 7. Sync files to output
        let assets_path = self.config_path.join("assets");
        let updated_assets = changed_config_paths
            .into_iter()
            .filter(|path| {
                path.starts_with(&assets_path) && file_ext(path) != Some("html".to_string())
            })
            .collect();
        let deleted_assets = deleted_config_paths
            .into_iter()
            .filter(|path| {
                path.starts_with(&assets_path) && file_ext(path) != Some("html".to_string())
            })
            .collect();
        sync_files_to_output(
            monitor,
            &assets_path,
            &self.typst_path,
            &self.html_cache_path,
            &self.output_path,
            output,
            error_articles,
            changed_non_typst,
            deleted_non_typst,
            updated_assets,
            deleted_assets,
        );

        Ok(updated)
    }
}

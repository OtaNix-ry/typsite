use crate::util::error::{log_err, log_err_or_ok};
use crate::util::fs::write_into_file;
use crate::util::path::relative_path;
use crate::walk_glob;
use anyhow::{Context, Result};
use blake3::{Hash, Hasher};
use glob::glob;
use memmap2::Mmap;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs::File, path::Path};

pub struct Monitor<'a> {
    config_path: &'a Path,
    typst_path:&'a Path,
    hash_path: PathBuf,
    typst_hash_cache: HashMap<PathBuf, Hash>,
    non_typst_hash_cache: HashMap<PathBuf, Hash>,
    html_hash_cache: HashMap<PathBuf, Hash>,
    config_hash_cache: HashMap<PathBuf, Hash>,
}

impl<'a> Monitor<'a> {
    fn load_hashes(hash_path: &Path, hash_cache_path: &Path, ext: &str) -> HashMap<PathBuf, Hash> {
        walk_glob!("{}/**/*{ext}.hash", hash_cache_path.display())
            .par_bridge()
            .map(|path| {
                std::fs::read_to_string(&path)
                    .context("Failed to read hash file")
                    .and_then(|hash| {
                        let hash = Hash::from_str(&hash).context("Failed to parse hash")?;
                        let mut path =
                            relative_path(hash_path, &path).context("Failed to convert path")?;
                        path.set_extension("");
                        Ok((path, hash))
                    })
            })
            .filter_map(log_err_or_ok)
            .collect()
    }

    pub fn load(
        cache_path: &Path,
        typst_path: &'a Path,
        html_cache_path: &Path,
        config_path: &'a Path,
    ) -> Monitor<'a> {
        let hash_path = cache_path.join("hash");
        let typst_hash = Self::load_hashes(&hash_path, &hash_path.join(typst_path), ".typ");
        let non_typst_hash = Self::load_hashes(&hash_path, &hash_path.join(typst_path), "[!.typ]");
        let html_hash = Self::load_hashes(&hash_path, &hash_path.join(html_cache_path), "");
        let config_hash = Self::load_hashes(&hash_path, &hash_path.join(config_path), "");
        Self {
            config_path,
            typst_path,
            hash_path,
            typst_hash_cache: typst_hash,
            non_typst_hash_cache: non_typst_hash,
            html_hash_cache: html_hash,
            config_hash_cache: config_hash,
        }
    }

    pub fn refresh_html(
        &mut self,
        html_cache_path: &Path,
        overall_compile_needed: bool,
    ) -> Result<HashSet<PathBuf>> {
        let pattern = format!("{}/**/*.html", html_cache_path.display());
        let new_results: HashMap<PathBuf, Hash> = hash_pattern(&pattern).into_iter().collect();
        let all_htmls: HashSet<PathBuf> = new_results.keys().cloned().collect();
        let (updated, _) = refresh(&self.hash_path, &mut self.html_hash_cache, new_results)?;
        Ok(if overall_compile_needed {
            all_htmls
        } else {
            updated
        })
    }

    pub fn refresh_config(
        &mut self,
    ) -> Result<(HashSet<PathBuf>, HashSet<PathBuf>)> {
        let pattern = format!("{}/**/*", self.config_path.display());
        let hash_new = hash_pattern(&pattern).into_iter().collect();
        refresh(&self.hash_path, &mut self.config_hash_cache, hash_new)
    }

    pub fn refresh_typst(
        &mut self,
    ) -> Result<(HashSet<PathBuf>, HashSet<PathBuf>, HashSet<PathBuf>)> {
        let pattern = format!("{}/**/*.typ", self.typst_path.display());
        let hash_new: HashMap<PathBuf, Hash> = hash_pattern(&pattern).into_iter().collect();
        let all_typsts: HashSet<PathBuf> = hash_new.keys().cloned().collect();
        refresh(&self.hash_path, &mut self.typst_hash_cache, hash_new)
            .map(|(updated, deleted)| (all_typsts, updated, deleted))
    }

    pub fn refresh_non_typst(
        &mut self,
    ) -> Result<(HashSet<PathBuf>, HashSet<PathBuf>)> {
        let pattern = format!("{}/**/*[!.typ]", self.typst_path.display());
        let hash_new = hash_pattern(&pattern).into_iter().collect();
        refresh(&self.hash_path, &mut self.non_typst_hash_cache, hash_new)
    }
    pub fn delete_html_cache(&self, path: &Path) {
        std::fs::remove_file(path).unwrap_or_else(|e| eprintln!("{e:?}"));
    }
}

fn hash_pattern(pattern: &str) -> Vec<(PathBuf, Hash)> {
    walk_glob!("{pattern}")
        .par_bridge()
        .filter_map(|path| {
            let hash = compute_hash(&path)?;
            Some((path, hash))
        })
        .collect()
}

fn compute_hash(path: &Path) -> Option<Hash> {
    let file = File::open(path).ok()?;
    let mmap = unsafe { Mmap::map(&file).ok()? };

    let mut hasher = Hasher::new();
    hasher.update(&mmap);
    Some(hasher.finalize())
}
fn refresh(
    hash_path: &Path,
    hash_cache: &mut HashMap<PathBuf, Hash>,
    hash_new: HashMap<PathBuf, Hash>,
) -> Result<(HashSet<PathBuf>, HashSet<PathBuf>)> {
    // Deleted Paths
    let mut deleted_paths: HashSet<PathBuf> = HashSet::new();
    {
        let mut temp_cache = HashMap::new();
        hash_cache.drain().for_each(|(path, hash)| {
            if !hash_new.contains_key(&path) {
                deleted_paths.insert(path);
            } else {
                temp_cache.insert(path, hash);
            }
        });
        hash_cache.extend(temp_cache);
    }

    let updated: Vec<(PathBuf, Hash)> = hash_new
        .into_iter()
        .filter_map(|(path, hash)| match hash_cache.get(&path) {
            Some(old) if old == &hash => None, // no change
            _ => Some((path, hash)),           // changed or new
        })
        .collect();

    write_cache(hash_path, &updated, &deleted_paths)?;

    let updated_paths: HashSet<PathBuf> = updated.into_iter().map(|(path, _)| path).collect();

    Ok((updated_paths, deleted_paths))
}

fn write_cache(
    hash_path: &Path,
    updated: &Vec<(PathBuf, Hash)>,
    deleted: &HashSet<PathBuf>,
) -> Result<()> {
    updated
        .par_iter()
        .map(|(path, hash)| {
            let content = hash.to_hex().to_string();
            let mut path = path.clone();
            path.add_extension("hash");
            let hash_path = hash_path.join(&path);
            write_into_file(hash_path, &content).context("Failed to write hash file")
        })
        .for_each(log_err);
    deleted
        .par_iter()
        .map(|path| {
            let mut path = path.clone();
            path.add_extension("hash");
            let hash_path = hash_path.join(&path);
            std::fs::remove_file(hash_path).context("Failed to remove hash file")
        })
        .for_each(log_err);
    Ok(())
}

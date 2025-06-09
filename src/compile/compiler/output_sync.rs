use super::ErrorArticles;
use super::cache::monitor::Monitor;
use crate::util::error::log_err;
use crate::util::fs::{remove_file, write_into_file};
use crate::util::path::relative_path;
use anyhow::{Ok, *};
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use super::page_composer::Output;

pub fn sync_files_to_output(
    monitor: Monitor,
    assets_path: &Path,
    typst_path: &Path,
    html_cache_path: &Path,
    output_path: &Path,
    output: Output,
    proj_options_errors: Vec<String>,
    error_typst_articles: ErrorArticles,
    error_cache_articles: ErrorArticles,
    error_pending_articles: ErrorArticles,
    error_page_articles: ErrorArticles,
    changed_non_typst: HashSet<PathBuf>,
    deleted_non_typst: HashSet<PathBuf>,
    changed_assets: HashSet<PathBuf>,
    deleted_assets: HashSet<PathBuf>,
) {
    if !proj_options_errors.is_empty() {
        println!(
            "Project options.toml errors:\n    {}",
            proj_options_errors.join("\n    ")
        );
    }
    println!("Output:");
    sync_files(
        typst_path,
        output_path,
        changed_non_typst,
        deleted_non_typst,
    );
    sync_files(assets_path, output_path, changed_assets, deleted_assets);
    write_pages(typst_path, output_path, output);
    let mut errors = Vec::new();
    errors.extend(error_typst_articles);
    errors.extend(error_cache_articles);
    errors.extend(error_pending_articles);
    errors.extend(error_page_articles);
    delete_errors(monitor, errors, html_cache_path, typst_path, output_path);
}

fn write_pages(typst_path: &Path, output_path: &Path, output: Output) {
    output
        .into_iter()
        .map(|(typ_path, html)| {
            let output_path = relative_path(typst_path, &typ_path)
                .map(|p| output_path.join(p.with_extension("html")))
                .unwrap();
            if output_path.exists() {
                println!("  ∓ {output_path:#?}");
            } else {
                println!("  + {output_path:#?}");
            }
            write_into_file(output_path, &html.to_html(), "")
        })
        .for_each(log_err);
}
fn sync_files(from: &Path, to: &Path, updated: HashSet<PathBuf>, deleted: HashSet<PathBuf>) {
    updated
        .into_par_iter()
        .map(|path| copy_to_output(from, &path, to))
        .for_each(log_err);

    deleted
        .into_par_iter()
        .map(|path| delete_output(from, &path, to))
        .for_each(log_err);
}

fn copy_to_output(parent: &Path, file: &Path, output_path: &Path) -> Result<()> {
    let file_path = relative_path(parent, file)?;
    let output_path = output_path.join(&file_path);
    if let Some(parent) = output_path.parent() {
        create_dir_all(parent).context(format!(
            "Create directory failed while creating file: {output_path:#?}"
        ))?;
    }
    let exists = output_path.exists();
    fs::copy(file, &output_path).context(format!("Copy {file:#?} to {output_path:#?}  failed."))?;
    if exists {
        println!("  ∓ {output_path:#?}");
    } else {
        println!("  + {output_path:#?}");
    }
    Ok(())
}

fn delete_output(parent: &Path, file: &Path, output_path: &Path) -> Result<()> {
    let file_path =
        relative_path(parent, file).context(format!("Remove file {file:#?} failed."))?;
    let output = output_path.join(&file_path);
    if !output.exists() {
        println!("  ? {output:#?}");
        return Ok(());
    }
    remove_file(&output, "output")?;
    println!("  - {output:#?}");
    // check if the dir is empty, if it is, delete the dir
    let mut parent = output.parent().unwrap();
    while parent != output {
        if fs::read_dir(parent)?.next().is_none() {
            fs::remove_dir(parent)?;
            parent = parent.parent().unwrap();
        } else {
            break;
        }
    }
    Ok(())
}

fn delete_errors(
    monitor: Monitor,
    errors: ErrorArticles,
    html_cache_path: &Path,
    typst_path: &Path,
    output_path: &Path,
) {
    errors
        .into_iter()
        .map(|(mut path, error)| {
            monitor.delete_cache(&path,html_cache_path);
            path.set_extension("html");
            let html_path = relative_path(html_cache_path, &path).unwrap_or(path);
            let result = delete_output(typst_path, &html_path, output_path);
            println!("{error}");
            result
        })
        .for_each(log_err);
}

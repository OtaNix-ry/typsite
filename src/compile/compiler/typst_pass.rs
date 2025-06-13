use crate::compile::error::TypError;
use crate::config::TypsiteConfig;
use rayon::prelude::*;
use std::sync::Arc;
use std::{
    fs::create_dir_all,
    path::Path,
    result::Result::Ok,
};

use crate::util::error::TypsiteError;
use crate::util::fs::create_all_parent_dir;
use anyhow::{Context, Error};
use std::process::Command;

use super::{ErrorArticles, PathBufs};

pub fn compile_typst(root: &Path, input: &Path, output: &Path) -> anyhow::Result<()> {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .arg(format!(
                "typst c {} --root {} -f=html --features html  {}",
                input.display(),
                root.display(),
                output.display()
            ))
            .output()
            .context(format!("Typst compile to HTML failed: {}", input.display()))?
    } else {
        create_all_parent_dir(output)?;
        Command::new("typst")
            .arg("c")
            .arg(input)
            .arg("--root")
            .arg("root")
            .arg("-f=html")
            .arg("--features")
            .arg("html")
            .arg("--input")
            .arg("html-frames=true")
            .arg(output)
            .output()
            .context(format!("Typst compile to HTML failed: {}", input.display()))?
    };
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(Error::new(TypsiteError::Typst(stderr.to_string())))
    }
}

pub fn compile_typsts(
    config: &TypsiteConfig<'_>,
    typst_path: &Path,
    html_cache_path: &Path,
    changed_typst_paths: &PathBufs,
) -> ErrorArticles {
    changed_typst_paths
        .par_iter()
        .map(|typ_path| {
            let slug = config.path_to_slug(typ_path);
            let mut html_path = typ_path.clone();
            html_path.set_extension("html");
            let cache_output = html_cache_path.join(&html_path);
            create_dir_all(cache_output.parent().unwrap()).unwrap();
            (
                slug,
                cache_output.clone(),
                compile_typst(typst_path, typ_path, &cache_output),
            )
        })
        .filter_map(|(slug, path, res)| {
            if let Ok(slug) = slug {
                res.err().map(|err| {
                    let err = TypError::new_with(Arc::from(slug), vec![err]);
                    (path, format!("{err}"))
                })
            } else if let Err(err) = slug {
                Some((path, format!("{err}")))
            } else {
                None
            }
        })
        .collect()
}

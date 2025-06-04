use crate::util::error::log_err;
use rayon::prelude::*;
use std::collections::HashSet;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    result::Result::Ok,
};

use crate::util::error::TypsiteError;
use crate::util::fs::create_all_parent_dir;
use anyhow::{Context, Error};
use std::process::Command;

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
    typst_path: &Path,
    html_cache_path: &Path,
    changed_typst_paths: &HashSet<PathBuf>,
) {
    changed_typst_paths
        .par_iter()
        .map(|typ_path| {
            let mut output = html_cache_path.join(typ_path);
            output.set_extension("html");
            create_dir_all(output.parent().unwrap())?;
            compile_typst(typst_path, typ_path, &output)
        })
        .for_each(log_err);
}

use crate::util::error::TypsiteError;
use anyhow::{anyhow, Context};
use std::fs;
use std::path::{Path, PathBuf};

pub fn create_all_parent_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context(format!(
            "Create directory failed while creating file: {path:?}"
        ))?;
    }
    Ok(())
}

pub fn write_into_file(path: PathBuf, content: &str, source: &str) -> anyhow::Result<()> {
    create_all_parent_dir(&path)?;
    fs::write(&path, content)
        .map_err(TypsiteError::Io)
        .context(format!("Failed to write {source} {path:?}"))
}

pub fn remove_file<P: AsRef<Path>>(path: P, source: &str) -> anyhow::Result<()> {
    std::fs::remove_file(path.as_ref()).context(format!("Failed to remove {source}: {:?}",path.as_ref()))
}
pub fn remove_file_log_err<P: AsRef<Path>>(path: P, source: &str) {
    remove_file(path, source).unwrap_or_else(|err| eprintln!("[WARN] {err}"));
}
pub fn remove_file_ignore<P: AsRef<Path>>(path: P) {
    std::fs::remove_file(path.as_ref()).unwrap_or(());
}

pub fn copy_file<P: AsRef<Path>,Q: AsRef<Path>>(from: P, to: Q) -> anyhow::Result<()> {
    create_all_parent_dir(to.as_ref())?;
    std::fs::copy(from.as_ref(),to.as_ref()).map_err(|err| {
        anyhow!("Failed to copy {:?} to {:?}: {err}",from.as_ref(),to.as_ref())
    }).map(|_| ())
}
#[macro_export]
macro_rules! walk_glob {
    ($($arg:tt)*) => {
        glob(&format!($($arg)*))
            .expect("Invalid pattern")
            .filter_map(Result::ok)
    }
}

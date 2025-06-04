use crate::util::error::TypsiteError;
use anyhow::Context;
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

pub fn write_into_file(path: PathBuf, content: &str) -> anyhow::Result<()> {
    create_all_parent_dir(&path)?;
    fs::write(&path, content)
        .map_err(TypsiteError::Io)
        .context(format!("Failed to write file {path:?}"))
}

#[macro_export]
macro_rules! walk_glob {
    ($($arg:tt)*) => {
        glob(&format!($($arg)*))
            .expect("Invalid pattern")
            .filter_map(Result::ok)
    }
}

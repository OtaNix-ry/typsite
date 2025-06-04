use include_dir::{Dir, include_dir};
use std::path::Path;

static DEFAULT_TYPSITE: Dir = include_dir!("./resources");

pub fn copy_default_typsite(output: &Path) -> std::io::Result<()> {
    DEFAULT_TYPSITE.extract(output)
}

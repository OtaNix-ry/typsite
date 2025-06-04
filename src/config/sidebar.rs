use crate::config::{SIDEBAR_BLOCK_PATH, SIDEBAR_EACH_PATH};
use crate::util::html::{HtmlWithElem, HtmlWithTail};
use crate::util::str::SidebarElem;
use std::path::Path;
use std::sync::Arc;

pub struct SidebarConfig {
    pub each: HtmlWithElem<SidebarElem>,
    pub block: HtmlWithTail,
    pub block_path: Arc<Path>,
    pub each_path: Arc<Path>,
}

impl SidebarConfig {
    pub fn load(config: &Path) -> anyhow::Result<Self> {
        let block_path = Arc::from(config.join(SIDEBAR_BLOCK_PATH));
        let each_path = Arc::from(config.join(SIDEBAR_EACH_PATH));
        let block = HtmlWithTail::load(&block_path, "{sidebar-each}")?;
        let each = HtmlWithElem::load(&each_path)?;
        Ok(Self {
            each,
            block,
            block_path,
            each_path,
        })
    }
}

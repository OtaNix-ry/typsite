use crate::config::{EMBED_PATH, EMBED_TITLE_PATH};
use crate::util::html::{Html, HtmlWithElem};
use crate::util::str::SectionElem;
use std::path::Path;
use std::sync::Arc;

pub struct EmbedConfig {
    pub embed_path: Arc<Path>,
    pub embed_title_path: Arc<Path>,
    pub embed: HtmlWithElem<SectionElem>,
    pub embed_title: Html,
}

impl EmbedConfig {
    pub fn load(config: &Path) -> anyhow::Result<Self> {
        let embed_path = Arc::from(config.join(EMBED_PATH));
        let embed_title_path = Arc::from(config.join(EMBED_TITLE_PATH));
        let embed = HtmlWithElem::load(&embed_path)?;
        let embed_title = Html::load(&embed_title_path)?;
        Ok(Self {
            embed_path,
            embed_title_path,
            embed,
            embed_title,
        })
    }
}

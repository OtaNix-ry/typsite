use crate::config::SECTION_PATH;
use crate::util::html::HtmlWithElem;
use crate::util::str::SectionElem;
use std::path::Path;
use std::sync::Arc;

pub struct SectionConfig {
    pub path: Arc<Path>,
    pub head: String,
    pub body: Vec<SectionElem>,
    title_index: usize,
    content_index: usize,
}

impl SectionConfig {
    pub fn load(config: &Path) -> anyhow::Result<Self> {
        let path = Arc::from(config.join(SECTION_PATH));
        let HtmlWithElem { head, body } = HtmlWithElem::load(&path)?;
        if body
            .iter()
            .filter(|&it| it == &SectionElem::Content || it == &SectionElem::Title)
            .count()
            != 2
        {
            return Err(anyhow::anyhow!(
                "Invalid heading config, must contain exactly one title and one content tag"
            ));
        }
        let title_index = body
            .iter()
            .position(|it| it == &SectionElem::Title)
            .unwrap();
        let content_index = body
            .iter()
            .position(|it| it == &SectionElem::Content)
            .unwrap();
        Ok(SectionConfig {
            path,
            head,
            body,
            title_index,
            content_index,
        })
    }

    pub fn before_title(&self) -> &[SectionElem] {
        &self.body[..self.title_index]
    }
    pub fn before_content(&self) -> &[SectionElem] {
        &self.body[self.title_index + 1..self.content_index]
    }
    pub fn after_content(&self) -> &[SectionElem] {
        &self.body[self.content_index + 1..]
    }
}

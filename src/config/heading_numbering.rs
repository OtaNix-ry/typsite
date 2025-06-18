use crate::config::HEADING_NUMBERING_PATH;
use crate::ir::article::sidebar::{HeadingNumberingStyle, Pos};
use crate::util::html::Html;
use crate::util::str::ac_replace;
use crate::util::{pos_base_on, pos_slug};
use std::path::Path;
use std::sync::Arc;

pub struct HeadingNumberingConfig {
    pub path: Arc<Path>,
    pub head: String,
    pub body: String,
}

impl HeadingNumberingConfig {
    pub fn load(config: &Path) -> anyhow::Result<Self> {
        let path = Arc::from(config.join(HEADING_NUMBERING_PATH));
        let Html { head, body } = Html::load(&path)?;
        Ok(Self { path, head, body })
    }
    pub fn get_with_pos_anchor(
        &self,
        style: HeadingNumberingStyle,
        base_anchor: Option<&Pos>,
        base_numbering: Option<&Pos>,
        pos: &Pos,
        anchor: &str,
    ) -> String {
        let pos_anchor = pos_base_on(base_anchor, Some(pos));
        let pos_numbering = pos_base_on(base_numbering, Some(pos));
        let anchor = pos_slug(&pos_anchor, anchor);
        let numbering = style.display(&pos_numbering);

        ac_replace(
            self.body.as_str(),
            &[("{numbering}", &numbering), ("{anchor}", &anchor)],
        )
    }
}

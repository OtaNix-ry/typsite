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

    pub fn get(
        &self,
        style: HeadingNumberingStyle,
        base: Option<&Pos>,
        pos: &Pos,
        anchor: &str,
    ) -> String {
        let result = pos_base_on(base, pos);
        let numbering = style.display(&result);
        ac_replace(
            self.body.as_str(),
            &[("{numbering}", &numbering), ("{anchor}", anchor)],
        )
    }
    pub fn get_with_pos_anchor(
        &self,
        style: HeadingNumberingStyle,
        base: Option<&Pos>,
        pos: &Pos,
        anchor: &str,
    ) -> String {
        let pos = pos_base_on(base, pos);
        let anchor = pos_slug(&pos, anchor);
        let numbering = style.display(&pos);

        ac_replace(
            self.body.as_str(),
            &[("{numbering}", &numbering), ("{anchor}", &anchor)],
        )
    }
}

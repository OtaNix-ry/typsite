use crate::compile::registry::Key;
use crate::config::anchor::AnchorConfig;
use crate::config::heading_numbering::HeadingNumberingConfig;
use crate::config::TypsiteConfig;
use crate::ir::article::data::GlobalData;
use crate::ir::article::sidebar::{HeadingNumberingStyle, Pos, SidebarIndex, SidebarType};
use crate::ir::embed::SectionType;
use crate::util::str::{SectionElem, ac_replace};
use crate::util::{pos_base_on, pos_slug};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub struct BodyNumberingData {
    pos: Pos,
    anchor: String,
    body_index: usize,
}

impl BodyNumberingData {
    pub fn new(pos: Pos, anchor: String, body_index: usize) -> Self {
        Self {
            pos,
            anchor,
            body_index,
        }
    }
    fn based_on(
        &self,
        config: &HeadingNumberingConfig,
        base: Option<&Pos>,
        style: HeadingNumberingStyle,
        body: &mut [String],
    ) {
        let numbering = config.get(style, base, &self.pos, self.anchor.as_str());
        body[self.body_index] = numbering.clone();
    }
}

pub struct SidebarNumberingData {
    pos: Pos,
    anchor: String,
    sidebar_index: SidebarIndex,
}
impl SidebarNumberingData {
    pub fn new(pos: Pos, anchor: String, sidebar_index: SidebarIndex) -> Self {
        Self {
            pos,
            anchor,
            sidebar_index,
        }
    }
    fn based_on(
        &self,
        config: &HeadingNumberingConfig,
        base: Option<&Pos>,
        style: HeadingNumberingStyle,
        sidebar: &mut [String],
    ) {
        let numbering = config.get(style, base, &self.pos, self.anchor.as_str());
        for &index in &self.sidebar_index {
            sidebar[index] = numbering.clone();
        }
    }
}
pub struct SidebarAnchorData {
    pos: Pos,
    anchor: String,
    sidebar_index: SidebarIndex,
}
impl SidebarAnchorData {
    pub fn new(pos: Pos, anchor: String, sidebar_index: SidebarIndex) -> Self {
        Self {
            pos,
            anchor,
            sidebar_index,
        }
    }
    fn based_on(&self, base: Option<&Pos>, sidebar: &mut [String]) {
        let pos = pos_base_on(base, &self.pos);
        let anchor = pos_slug(&pos, &self.anchor);
        for &index in &self.sidebar_index {
            sidebar[index] = anchor.clone();
        }
    }
}
pub struct SidebarShowChildrenData {
    sidebar_index: SidebarIndex,
}
impl SidebarShowChildrenData {
    pub fn new(sidebar_index: SidebarIndex) -> Self {
        Self { sidebar_index }
    }
    fn based_on(&self, section_type: SectionType, sidebar: &mut [String]) {
        let if_show = match section_type {
            SectionType::OnlyTitle => "none",
            _ => "block",
        };
        for &index in &self.sidebar_index {
            sidebar[index] = if_show.to_string();
        }
    }
}

pub struct EmbedData<'c> {
    pos: Pos,
    slug: Key,
    body_index: usize,
    full_sidebar_index: SidebarIndex,
    embed_sidebar_index: SidebarIndex,
    open: bool,
    title: String,
    full_sidebar_title_index: SidebarIndex,
    embed_sidebar_title_index: SidebarIndex,
    pending: &'c Pending<'c>,
    pub section_type: SectionType,
}

fn combine(base: &Pos, pos: &Pos) -> Pos {
    let mut combined = base.clone();
    combined.extend(pos.iter());
    combined
}

impl<'c> EmbedData<'c> {
    pub fn new(
        pos: Pos,
        slug: Key,
        section_type: SectionType,
        body_index: usize,
        full_sidebar_index: SidebarIndex,
        embed_sidebar_index: SidebarIndex,
        open: bool,
        title: String,
        full_sidebar_title_index: SidebarIndex,
        embed_sidebar_title_index: SidebarIndex,
        child: &'c Pending,
    ) -> Self {
        Self {
            pos,
            slug,
            section_type,
            body_index,
            full_sidebar_index,
            embed_sidebar_index,
            open,
            title,
            full_sidebar_title_index,
            embed_sidebar_title_index,
            pending: child,
        }
    }

    fn based_on(
        &self,
        config: &TypsiteConfig,
        global_data: &'c GlobalData<'_, '_, 'c>,
        base: Option<&Pos>,
        style: HeadingNumberingStyle,
        body: &mut [String],
        sidebar: &mut [String],
        sidebar_type: SidebarType,
    ) {
        // let empty_pos = vec![];
        // let (base, pos) = match &self.section_type {
        //     SectionType::None => (None, &empty_pos),
        //     _ => (base, &self.pos),
        // };
        let pos = &self.pos;
        let metadata = global_data.metadata(self.slug.as_str()).unwrap();

        let mut embed_article_body = Vec::new();
        let mut embed_article_sidebar = String::new();

        let numbering = config
            .heading_numbering
            .get_with_pos_anchor(style, base, pos, &self.slug); // also as anchor
        let embed_config = &config.embed.embed;

        let base = base.map(|base| combine(base, pos));
        let (body_vec, mut full_sidebar_vec, mut embed_sidebar_vec) = self.pending.based_on(
            config,
            global_data,
            base.as_ref(),
            Some(style),
            sidebar_type,
            self.section_type,
        );

        if sidebar_type == SidebarType::All {
            match self.section_type {
                SectionType::None => {}
                _ => {
                    for &title_index in &self.full_sidebar_title_index {
                        full_sidebar_vec[title_index] = self.title.clone();
                    }
                    embed_article_sidebar = full_sidebar_vec.join("");
                }
            }
        } else {
            for &title_index in &self.embed_sidebar_title_index {
                embed_sidebar_vec[title_index] = self.title.clone();
            }
            embed_article_sidebar = embed_sidebar_vec.join("")
        };

        for elem in &embed_config.body {
            let str = match elem {
                SectionElem::Plain(plain) => plain.clone(),
                SectionElem::Content => body_vec.join(""),
                SectionElem::Level => self.slug.to_string(),
                SectionElem::Title => self.title.clone(),
                SectionElem::HeadingNumbering => numbering.clone(),
            };
            embed_article_body.push(str);
        }

        let embed_body = metadata.inline(embed_article_body.join("").as_str());
        body[self.body_index] = ac_replace(
            embed_body.as_str(),
            &[("{open}=\"\"", if self.open { "open=\"\"" } else { "" })],
        );
        let indexes = if sidebar_type == SidebarType::All {
            &self.full_sidebar_index
        } else {
            &self.embed_sidebar_index
        };
        for &index in indexes {
            sidebar[index] = embed_article_sidebar.clone();
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum AnchorKind {
    Define,
    GotoHead,
    GotoTail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnchorData {
    anchor: String,
    kind: AnchorKind,
    body_indexes: HashSet<usize>,
}

impl AnchorData {
    pub fn new(anchor: String, kind: AnchorKind, body_indexes: HashSet<usize>) -> Self {
        Self {
            anchor,
            kind,
            body_indexes,
        }
    }
    fn based_on(&self, config: &AnchorConfig, base: Option<&Pos>, body: &mut [String]) {
        let text = match &self.kind {
            AnchorKind::Define => config.get_define(base, self.anchor.as_str()),
            AnchorKind::GotoHead => config.get_goto_head(base, self.anchor.as_str()),
            AnchorKind::GotoTail => config.get_goto_tail(base, self.anchor.as_str()),
        };
        for &index in &self.body_indexes {
            body[index] = text.clone();
        }
    }
}

pub struct Pending<'c> {
    // body, sidebar
    pub raw: &'c (Vec<String>, Vec<String>, Vec<String>),
    pub body_numberings: Vec<BodyNumberingData>,
    pub sidebar_show_children: SidebarShowChildrenData,
    pub sidebar_numberings: Vec<SidebarNumberingData>,
    pub sidebar_anchors: Vec<SidebarAnchorData>,
    pub embeds: Vec<EmbedData<'c>>,
    pub anchors: &'c Vec<AnchorData>,
}

impl<'c> Pending<'c> {
    pub fn new(
        raw: &'c (Vec<String>, Vec<String>, Vec<String>),
        body_numberings: Vec<BodyNumberingData>,
        sidebar_show_children: SidebarShowChildrenData,
        sidebar_numberings: Vec<SidebarNumberingData>,
        sidebar_anchors: Vec<SidebarAnchorData>,
        embeds: Vec<EmbedData<'c>>,
        anchors: &'c Vec<AnchorData>,
    ) -> Self {
        Self {
            raw,
            body_numberings,
            sidebar_show_children,
            sidebar_numberings,
            sidebar_anchors,
            embeds,
            anchors,
        }
    }
    pub fn based_on(
        &self,
        config: &TypsiteConfig,
        global_data: &'c GlobalData<'_, '_, 'c>,
        base: Option<&Pos>,
        style: Option<HeadingNumberingStyle>,
        sidebar_type: SidebarType,
        section_type: SectionType,
    ) -> (Vec<String>, Vec<String>, Vec<String>) {
        let (mut body, mut full_sidebar, mut embed_sidebar) = self.raw.clone();
        let style = style.unwrap_or_default();

        for numbering_in_body in &self.body_numberings {
            numbering_in_body.based_on(&config.heading_numbering, base, style, &mut body);
        }

        let sidebar = if sidebar_type == SidebarType::All {
            for numbering_in_sidebar in &self.sidebar_numberings {
                numbering_in_sidebar.based_on(
                    &config.heading_numbering,
                    base,
                    style,
                    &mut full_sidebar,
                );
            }
            for anchor_in_sidebar in &self.sidebar_anchors {
                anchor_in_sidebar.based_on(base, &mut full_sidebar);
            }
            &mut full_sidebar
        } else {
            &mut embed_sidebar
        };
        self.sidebar_show_children.based_on(section_type, sidebar);
        for embed in &self.embeds {
            embed.based_on(
                config,
                global_data,
                base,
                style,
                &mut body,
                sidebar,
                sidebar_type,
            );
        }

        for anchor in self.anchors {
            anchor.based_on(&config.anchor, base, &mut body);
        }

        (body, full_sidebar, embed_sidebar)
    }
}

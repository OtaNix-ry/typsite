use crate::compile::registry::Key;
use crate::ir::article::sidebar::{SidebarIndexes, SidebarPos};
use crate::ir::embed::{Embed, EmbedVariables, SectionType};

pub struct EmbedBuilder {
    slug: Key,
    open: bool,
    variables: EmbedVariables,
    full_sidebar_pos: SidebarPos,
    embed_sidebar_pos: SidebarPos,
    section_type: SectionType,
    body_index: usize,
}

impl EmbedBuilder {
    pub fn new(
        slug: Key,
        open: bool,
        variables: EmbedVariables,
        full_sidebar_pos: SidebarPos,
        embed_sidebar_pos: SidebarPos,
        section_type: SectionType,
        body_index: usize,
    ) -> Self {
        Self {
            slug,
            open,
            variables,
            full_sidebar_pos,
            embed_sidebar_pos,
            section_type,
            body_index,
        }
    }

    pub fn build<F, E>(self, full_sidebar_mapping: &mut F, embed_sidebar_mapping: &mut E) -> Embed
    where
        F: FnMut(SidebarPos) -> SidebarIndexes,
        E: FnMut(SidebarPos) -> SidebarIndexes,
    {
        let full_index = full_sidebar_mapping(self.full_sidebar_pos.clone());
        let embed_index = embed_sidebar_mapping(self.embed_sidebar_pos.clone());
        Embed::new(
            self.slug,
            self.open,
            self.variables,
            self.section_type,
            self.full_sidebar_pos,
            full_index,
            embed_index,
            self.body_index,
        )
    }
}

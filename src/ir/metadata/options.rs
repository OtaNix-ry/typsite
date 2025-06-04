use crate::ir::article::sidebar::{HeadingNumberingStyle, SidebarType};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetaOptions {
    pub heading_numbering_style: HeadingNumberingStyle,
    pub sidebar_type: SidebarType,
}

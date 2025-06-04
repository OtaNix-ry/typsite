use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::mem::transmute;
use std::path::Path;

pub struct CompileOptions {
    pub watch: bool,
    pub short_slug: bool,
    pub pretty_url: bool,
    pub options: Option<Options>,
}

impl CompileOptions {
    pub(crate) fn empty() -> CompileOptions {
        CompileOptions {
            watch: false,
            short_slug: true,
            pretty_url: true,
            options: None,
        }
    }
    pub fn options<'a>(&self) -> &'a Options {
        unsafe { transmute(self.options.as_ref().unwrap()) } // TODO: Fix this unsafe code
    }
}

pub const OPTIONS_PATH: &str = "options.toml";
#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub default_metadata: DefaultMetadata,
    pub typst_lib: TypstLib,
    pub code_fallback_style: CodeFallbackStyle,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TypstLib {
    pub paths: HashSet<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultMetadata {
    pub content: metadata::Content,
    pub options: metadata::Options,
    pub graph: metadata::Graph,
}

impl Options {
    pub fn load(config_path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(config_path.join(OPTIONS_PATH))?;
        let options: Self = toml::from_str(&content)?;
        Ok(options)
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CodeFallbackStyle {
    pub dark: String,
    pub light: String,
}
pub mod metadata {
    use crate::ir::article::sidebar::{HeadingNumberingStyle, SidebarType};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Content {
        #[serde(flatten, default = "HashMap::new")]
        pub default: HashMap<String, String>,
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Options {
        #[serde(default = "default_heading_numbering")]
        pub heading_numbering: HeadingNumberingStyle,
        #[serde(default = "default_sidebar_type")]
        pub sidebar_type: SidebarType,
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Graph {
        #[serde(
            default = "default_parent",
            deserialize_with = "deserialize_optional_parent"
        )]
        pub parent: Option<String>,
    }

    pub fn default_sidebar_type() -> SidebarType {
        SidebarType::All
    }
    pub fn default_heading_numbering() -> HeadingNumberingStyle {
        HeadingNumberingStyle::Bullet
    }
    pub fn default_parent() -> Option<String> {
        None
    }

    pub fn deserialize_optional_parent<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(if s.trim().is_empty() { None } else { Some(s) })
    }
}

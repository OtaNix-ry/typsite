use crate::config::anchor::AnchorConfig;
use crate::config::footer::FooterConfig;
use crate::config::heading_numbering::HeadingNumberingConfig;
use crate::config::rewrite::RulesConfig;
use crate::config::schema::SchemaConfig;
use crate::config::section::SectionConfig;
use crate::config::theme::ThemesConfig;
use crate::util::html::Html;
use crate::util::path::format_path_ref;
use crate::util::path::{dir_name, file_stem};
use anyhow::{Context, Result};
use embed::EmbedConfig;
use sidebar::SidebarConfig;
use std::path::Path;
use std::sync::Arc;

pub mod anchor;
pub mod embed;
pub mod footer;
pub mod heading_numbering;
pub mod resources;
pub mod rewrite;
pub mod schema;
pub mod section;
pub mod sidebar;
pub mod theme;

const RULES_DIR: &str = "rewrite/";
const SCHEMAS_DIR: &str = "schemas/";

const THEMES_DIR: &str = "themes";

const SECTION_PATH: &str = "components/section.html";
const HEADING_NUMBERING_PATH: &str = "components/heading-numbering.html";
const FOOTER_BACKLINKS_PATH: &str = "components/footer/backlinks.html";
const FOOTER_REFERENCES_PATH: &str = "components/footer/references.html";
const ANCHOR_DEF_PATH: &str = "components/anchor_def.html";
const ANCHOR_GOTO_PATH: &str = "components/anchor_goto.html";
const FOOTER_PATH: &str = "components/footer.html";
const EMBED_PATH: &str = "components/embed.html";
const EMBED_TITLE_PATH: &str = "components/embed_title.html";

const SIDEBAR_BLOCK_PATH: &str = "components/sidebar.html";
const SIDEBAR_EACH_PATH: &str = "components/sidebar_each.html";
pub struct TypsiteConfig<'a> {
    pub section: SectionConfig,
    pub heading_numbering: HeadingNumberingConfig,
    pub footer: FooterConfig,
    pub anchor: AnchorConfig,
    pub sidebar: SidebarConfig,
    pub embed: EmbedConfig,
    pub rules: RulesConfig,
    pub schemas: SchemaConfig,
    pub themes: ThemesConfig,
    config_path: &'a Path,
    pub typst_path: &'a Path,
    pub typst_root_name: String,
}

impl<'a> TypsiteConfig<'a> {
    pub fn load(config_path: &'a Path, typst_path: &'a Path) -> Result<Self> {
        // Components
        let section = SectionConfig::load(config_path)?;
        let heading_numbering = HeadingNumberingConfig::load(config_path)?;
        let footer = FooterConfig::load(config_path)?;
        let anchor = AnchorConfig::load(config_path)?;
        let sidebar = SidebarConfig::load(config_path)?;
        let embed = EmbedConfig::load(config_path)?;
        let rules = RulesConfig::load(config_path)?;
        let schemas = SchemaConfig::load(config_path)?;
        let themes = ThemesConfig::load(config_path)?;

        let typst_path = format_path_ref(typst_path);
        let typst_root_name = dir_name(typst_path)
            .context("Failed to load input path")?
            .to_string();

        let config = Self {
            rules,
            section,
            heading_numbering,
            footer,
            anchor,
            sidebar,
            embed,
            schemas,
            config_path,
            typst_path,
            typst_root_name,
            themes,
        };
        Ok(config)
    }

    pub fn path_ref(&'a self, path: &Path) -> Option<Arc<Path>> {
        if path.starts_with(self.typst_path) {
            None
        } else if path.starts_with(self.config_path) {
            let path = path.strip_prefix(self.config_path).ok()?;
            let path_str = path.as_os_str().to_str()?;
            match path_str {
                SECTION_PATH => Some(self.section.path.clone()),
                HEADING_NUMBERING_PATH => Some(self.heading_numbering.path.clone()),
                FOOTER_PATH => Some(self.footer.footer.path.clone()),
                FOOTER_BACKLINKS_PATH => Some(self.footer.backlinks.path.clone()),
                FOOTER_REFERENCES_PATH => Some(self.footer.references.path.clone()),
                ANCHOR_DEF_PATH => Some(self.anchor.define.path.clone()),
                ANCHOR_GOTO_PATH => Some(self.anchor.goto.path.clone()),
                SIDEBAR_BLOCK_PATH => Some(self.sidebar.block_path.clone()),
                SIDEBAR_EACH_PATH => Some(self.sidebar.each_path.clone()),
                EMBED_PATH => Some(self.embed.embed_path.clone()),
                EMBED_TITLE_PATH => Some(self.embed.embed_title_path.clone()),
                path_str if path_str.starts_with(RULES_DIR) => file_stem(path)
                    .and_then(|rule| self.rules.get(rule))
                    .and_then(|rule| rule.path.clone()),
                path_str if path_str.starts_with(SCHEMAS_DIR) => file_stem(path)
                    .and_then(|schema| self.schemas.get(schema).map(|schema| schema.path.clone())),
                path_str if path_str.starts_with(THEMES_DIR) => file_stem(path)
                    .and_then(|theme| self.themes.path_exactly(theme))
                    .cloned(),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn path_to_slug(&self, path: &Path) -> String {
        self.format_slug(path.to_string_lossy().as_ref())
    }

    pub fn format_slug(&self, slug: &str) -> String {
        let slug = slug.replace("\\", "/");
        let slug = slug.trim_start_matches(self.typst_root_name.as_str());
        let slug = slug.strip_suffix(".typ").unwrap_or(slug);
        if !slug.starts_with('/') {
            format!("/{slug}")
        } else {
            slug.to_string()
        }
    }
}

pub struct HtmlConfig {
    pub path: Arc<Path>,
    pub head: String,
    pub body: String,
}

impl HtmlConfig {
    pub fn load(config: &Path, path: &str) -> Result<Self> {
        let path = Arc::from(config.join(path));
        let Html { head, body } = Html::load(&path)?;
        Ok(Self { path, head, body })
    }
}

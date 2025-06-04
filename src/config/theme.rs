use crate::config::THEMES_DIR;
use crate::util::error::log_err_or_ok;
use crate::util::path::file_stem;
use crate::walk_glob;
use anyhow::*;
use glob::glob;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use syntect::highlighting::{Theme, ThemeSet};

#[derive(Debug, Default)]
pub struct ThemesConfig {
    themes: BTreeMap<String, (Arc<Path>, Theme)>,
}

impl ThemesConfig {
    pub fn load(config_path: &Path) -> Result<Self> {
        let mut thiz = Self::default();
        let theme_path = config_path.join(THEMES_DIR);
        let themes = walk_glob!("{}/**/*.tmTheme", theme_path.display())
            .filter(|entry| entry.is_file())
            .map(|path| {
                let name = file_stem(&path)
                    .map(|s| s.to_string())
                    .context(format!("Failed to load theme {}", path.display()));
                let theme = ThemeSet::get_theme(&path)
                    .context(format!("Failed to load theme {}", path.display()));
                name.and_then(|name| theme.map(|theme| (name, (Arc::from(path), theme))))
            })
            .filter_map(log_err_or_ok);
        thiz.themes.extend(themes);
        Ok(thiz)
    }

    fn get_pair(&self, name: &str, scheme: ColorScheme) -> Option<&(Arc<Path>,Theme)> {
        let name_with_suffix = if name.ends_with(scheme.suffix()) {
            name.to_string()
        } else {
            format!("{name}{}", scheme.suffix())
        };
        self.themes
            .get(&name_with_suffix)
            .or_else(|| if scheme != ColorScheme::None { self.get_pair(name, ColorScheme::None) } else { None })
    }

    // light & dark theme
    pub fn get(&self, name: &str) -> Option<(&Theme, &Theme)> {
        use ColorScheme::*;
        let dark = &self.get_pair(name, Dark)?.1;
        let light = &self.get_pair(name, Light)?.1;
        Some((light,dark))
    }
    pub fn path_exactly(&self,name:&str) -> Option<&Arc<Path>> {
        self.themes.get(name).map(|(path,_)| path)
    }

    // light & dark theme
    pub fn path(&self, name: &str) -> Option<(&Arc<Path>, &Arc<Path>)> {
        use ColorScheme::*;
        let dark = &self.get_pair(name, Dark)?.0;
        let light = &self.get_pair(name, Light)?.0;
        Some((light,dark))
    }
}

#[derive(Debug,PartialEq)]
enum ColorScheme {
    Dark,
    Light,
    None,
}
impl ColorScheme {
    pub fn suffix(&self) -> &str {
        match self {
            ColorScheme::Dark => "_dark",
            ColorScheme::Light => "_light",
            ColorScheme::None => "",
        }
    }
}

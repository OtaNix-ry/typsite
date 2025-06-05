use crate::{compile::proj_options, config::TypsiteConfig};
use crate::pass::pure::PurePass;
use crate::pass::rewrite::code::highlighter::highlight;
use crate::pass::rewrite::*;
use crate::util::html::Attributes;
use crate::util::str::ac_replace;
use std::collections::{HashMap, HashSet};
use typsite_macros::rewrite_pass;

pub mod highlighter;

rewrite_pass![CodeBlockPass, id = "code", atom = true];

impl TagRewritePass for CodeBlockPass {
    fn init(
        &self,
        mut attrs: Attributes,
        _: &mut PurePass,
    ) -> Result<HashMap<String, String>> {
        let lang = attrs.take("lang")?;
        let theme = attrs.take("theme").unwrap_or("onedark".into());
        let content = attrs.take("content")?;
        Ok([
            ("lang".into(), lang.to_string()),
            ("theme".into(), theme.to_string()),
            ("content".into(), content.to_string()),
        ]
        .into_iter()
        .collect())
    }

    fn dependents<'a>(
        &self,
        attrs: &HashMap<String, String>,
        pass: &PurePass<'a, '_>,
    ) -> Option<HashSet<Source>> {
        let mut path = HashSet::new();
        let theme = attrs.get("theme")?;
        let (light,dark) = pass.config.themes.path(theme)?;
        path.insert(Source::Path(light.clone()));
        path.insert(Source::Path(dark.clone()));
        Some(path)
    }

    fn pure_start(
        &self,
        attrs: &HashMap<String, String>,
        config: &TypsiteConfig,
        body: &str,
    ) -> Option<String> {
        let lang = attrs.get("lang")?;
        let theme = attrs.get("theme")?;
        let content = attrs.get("content")?;
        let (light,dark) = config.themes.get(theme)?;
        let fallback = &proj_options().unwrap().code_fallback_style;
        let light = highlight(lang, content, light, &fallback.light);
        let dark = highlight(lang, content, dark, &fallback.dark);
        Some(ac_replace(body, &[("{content-light}", &light),("{content-dark}",&dark)]))
    }
}

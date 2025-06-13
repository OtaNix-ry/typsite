use crate::pass::pure::PurePass;
use crate::pass::rewrite::code::highlighter::highlight;
use crate::pass::rewrite::*;
use crate::util::html::Attributes;
use crate::util::str::ac_replace;
use crate::{compile::proj_options, config::TypsiteConfig};
use std::collections::{HashMap, HashSet};
use typsite_macros::rewrite_pass;

pub mod highlighter;

rewrite_pass![CodeBlockPass, id = "code", atom = true];

impl TagRewritePass for CodeBlockPass {
    fn init(&self, mut attrs: Attributes, _: &mut PurePass) -> Result<HashMap<String, String>> {
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
    ) -> Result<HashSet<Source>> {
        let mut path = HashSet::new();
        let lang = attrs.get("lang").unwrap();
        let config = &pass.config.highlight;
        if !config.is_syntax_by_default(lang) {
            let syntax = config
                .find_syntax_path(lang)
                .context(format!("Can't find syntax path for lang {lang}"))?;
            path.insert(Source::Path(syntax));
            for metadata_path in config.metadata_paths() {
                path.insert(Source::Path(metadata_path));
            }
        }
        let theme = attrs.get("theme").unwrap();
        let (light, dark) = config.find_theme_path_pair(theme).context(format!(
            "Can't find theme path pair(light & dark) for theme {theme}"
        ))?;
        path.insert(Source::Path(light.clone()));
        path.insert(Source::Path(dark.clone()));
        Ok(path)
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
        let syntax = config.highlight.find_syntax(lang);
        let syntax_set = config.highlight.syntax_set(lang);
        let (light, dark) = config.highlight.find_theme(theme)?;
        let fallback = &proj_options().unwrap().code_fallback_style;
        let light = highlight(syntax_set, syntax, content, light, &fallback.light);
        let dark = highlight(syntax_set, syntax, content, dark, &fallback.dark);
        Some(ac_replace(
            body,
            &[("{content-light}", &light), ("{content-dark}", &dark)],
        ))
    }
}

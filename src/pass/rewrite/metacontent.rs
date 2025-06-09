use crate::pass::pure::PurePass;
use crate::pass::rewrite::Atom;
use crate::pass::rewrite::*;
use crate::util::html::Attributes;
use anyhow::anyhow;
use std::collections::{HashMap, HashSet};
use typsite_macros::rewrite_pass;

rewrite_pass![
    MetaContentPass,
    id = "metacontent",
    atom = false,
    pure = false
];
impl TagRewritePass for MetaContentPass {
    fn init(&self, attrs: Attributes, pass: &mut PurePass) -> Result<HashMap<String, String>> {
        let slug = attrs.get("from");
        if slug.is_none() {
            return Err(anyhow!("Metadata-Get: expect `from` attribute"));
        }
        let meta_key = attrs.get("get");
        if meta_key.is_none() {
            return Err(anyhow!("Metadata-Get: expect `meta_key` attribute"));
        }
        let slug = slug.unwrap();
        let meta_key = meta_key.unwrap();
        let slug = if slug == "$self" {
            pass.slug.clone()
        } else {
            pass.resolve_slug(slug.as_str(), "Metadata-Get")?
        };
        Ok([
            ("key".to_string(), meta_key.to_string()),
            ("from".to_string(), slug.to_string()),
        ]
        .into_iter()
        .collect())
    }

    fn dependents<'a>(
        &self,
        attrs: &HashMap<String, String>,
        pass: &PurePass<'a, '_>,
    ) -> Option<HashSet<Source>> {
        let slug = attrs.get("from")?;
        if slug == pass.slug.as_str() {
            return None;
        }
        let slug = pass.registry.slug(slug.as_str())?;
        Some([Source::Article(slug)].into_iter().collect())
    }

    fn impure_start<'c, 'b: 'c, 'a: 'b>(
        &self,
        attrs: &HashMap<String, String>,
        global_data: &'c GlobalData<'a, 'b, 'c>,
        _: &str,
    ) -> Option<String> {
        let slug = &attrs["from"];
        let key = &attrs["key"];
        global_data
            .metadata(slug)
            .and_then(|metadata| Some(metadata.contents.get(key)?.to_string()))
    }
}

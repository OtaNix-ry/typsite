use crate::ir::article::sidebar::Pos;

pub mod error;
pub mod fs;
pub mod html;
pub mod path;
pub mod str;

pub fn pos_slug(pos: &[usize], slug: &str) -> String {
    if pos.is_empty() {
        return slug[1..].to_string();
    }
    let pos = pos
        .iter()
        .map(|u| (u + 1).to_string())
        .collect::<Vec<_>>()
        .join(".");
    // no "/"
    format!("{}-{}", &slug[1..], pos)
}

pub fn pos_base_on(base: Option<&Pos>, pos: Option<&Pos>) -> Pos {
    match base {
        Some(base) => {
            let mut result = base.clone();
            if let Some(pos) = pos {
                result.extend(pos);
            }
            result
        }
        None => pos.cloned().unwrap_or_default(),
    }
}

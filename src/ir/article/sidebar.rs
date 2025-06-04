use crate::ir::metadata::Metadata;
use crate::ir::metadata::content::TITLE_KEY;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type Pos = Vec<usize>;
pub type SidebarPos = (Pos, usize);
pub type SidebarIndex = HashSet<usize>;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sidebar {
    contents: Vec<String>,
    title_index: SidebarIndex,
    show_children: SidebarIndex,
    #[serde(with = "pos_to_sidebar_index_serde")]
    numberings: HashMap<Pos, SidebarIndex>,
    #[serde(with = "pos_to_sidebar_index_serde")]
    anchors: HashMap<Pos, SidebarIndex>
}
impl Sidebar {
    pub fn new(
        contents: Vec<String>,
        title_index: SidebarIndex,
        show_children: SidebarIndex,
        numbering: HashMap<Pos, SidebarIndex>,
        anchor: HashMap<Pos, SidebarIndex>
    ) -> Self {
        Self {
            contents,
            title_index,
            show_children,
            numberings: numbering,
            anchors: anchor
        }
    }

    pub fn with_contents(self, contents: Vec<String>) -> Self {
        Self { contents, ..self }
    }

    pub fn cache(&self, metadata: &Metadata) -> Vec<String> {
        let title = metadata.contents.get(TITLE_KEY).unwrap();
        let mut contents = self.contents.clone();
        for &title_index in &self.title_index {
            contents[title_index] = title.to_string();
        }
        contents
    }
    pub fn title_index(&self) -> &SidebarIndex {
        &self.title_index
    }

    pub fn numberings(&self) -> &HashMap<Pos, SidebarIndex> {
        &self.numberings
    }

    pub fn anchors(&self) -> &HashMap<Pos, SidebarIndex> {
        &self.anchors
    }
    pub fn show_children(&self) -> &SidebarIndex {
        &self.show_children
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum SidebarType {
    All,
    OnlyEmbed,
}
impl From<&str> for SidebarType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "all" => SidebarType::All,
            "only_embed" => SidebarType::OnlyEmbed,
            _ => SidebarType::All,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum HeadingNumberingStyle {
    Bullet,
    Roman,
    Alphabet,
    None,
}
impl From<&str> for HeadingNumberingStyle {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "none" => Self::None,
            "roman" => Self::Roman,
            "alphabet" => Self::Alphabet,
            _ => Self::Bullet,
        }
    }
}
impl Default for HeadingNumberingStyle {
    fn default() -> Self {
        Self::Bullet
    }
}

const ROMANS: [&str; 20] = [
    "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII", "XIII", "XIV", "XV",
    "XVI", "XVII", "XVIII", "XIX", "XX",
];

impl HeadingNumberingStyle {
    pub fn display(&self, pos: &Pos) -> String {
        match self {
            Self::Bullet => pos
                .iter()
                .map(|it| (it + 1).to_string())
                .collect::<Vec<String>>()
                .join("."),
            Self::Roman => pos
                .iter()
                .map(|it| ROMANS[*it])
                .collect::<Vec<&str>>()
                .join("."),
            Self::Alphabet => pos
                .iter()
                .map(|it| ((b'A' + *it as u8) as char).to_string())
                .collect::<Vec<String>>()
                .join("."),
            Self::None => String::new(),
        }
    }
}

mod pos_to_sidebar_index_serde {
    use super::*;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(
        map: &HashMap<Pos, SidebarIndex>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut entries = Vec::with_capacity(map.len());
        for (pos, set) in map {
            entries.push((
                pos.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join("-"),
                set,
            ));
        }
        entries.serialize(serializer)
    }
    pub fn deserialize<'ce, D: Deserializer<'ce>>(
        deserializer: D,
    ) -> Result<HashMap<Pos, SidebarIndex>, D::Error> {
        let entries: Vec<(String, SidebarIndex)> = Vec::deserialize(deserializer)?;
        let mut map = HashMap::with_capacity(entries.len());
        for (pos_str, set) in entries {
            let pos: Pos = if pos_str.is_empty() {
                vec![]
            } else {
                pos_str
                    .split('-')
                    .map(|s| s.parse().map_err(Error::custom))
                    .collect::<Result<_, _>>()?
            };
            map.insert(pos, set);
        }
        Ok(map)
    }
}

#[cfg(test)]
mod test_serde_pos_to_sidebar_index {
    use std::collections::HashMap;

    use anyhow::{Context, Result};

    use crate::ir::article::sidebar::{Pos, SidebarIndex};
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct WrapperPosToSidebarIndex {
        #[serde(with = "super::pos_to_sidebar_index_serde")]
        map: HashMap<Pos, SidebarIndex>,
    }
    #[test]
    fn pos_to_sidebar_index() -> Result<()> {
        let mut test: HashMap<Pos, SidebarIndex> = HashMap::new();
        test.insert(vec![1], vec![10].into_iter().collect());
        test.insert(vec![1, 1], vec![12, 15].into_iter().collect());
        test.insert(vec![2, 3], vec![17, 20].into_iter().collect());
        test.insert(vec![], vec![3].into_iter().collect());
        let test = WrapperPosToSidebarIndex { map: test };

        let str =
            serde_json::to_string(&test).context("Failed to deserialize HashMap<Pos,SidebarIndex>!")?;
        let test_read = serde_json::from_str::<WrapperPosToSidebarIndex>(&str)
            .context("Failed to serialize HashMap<Pos,SidebarIndex>!")?;
        assert_eq!(test, test_read);
        Ok(())
    }
}

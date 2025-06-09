use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    compile::{
        error::{TypError, TypResult},
        registry::Key,
    },
    config::TypsiteConfig,
    ir::{
        article::sidebar::Pos,
        rewriter::{BodyRewriter, PureRewriter},
    },
};

#[derive(Debug, Clone)]
pub struct Body<'a> {
    pub content: Vec<String>,
    pub rewriters: Vec<BodyRewriter<'a>>,
    pub numberings: HashMap<Pos, usize>,
}

impl<'a> Body<'a> {
    pub fn new(
        content: Vec<String>,
        rewriters: Vec<BodyRewriter<'a>>,
        numberings: HashMap<Pos, usize>,
    ) -> Body<'a> {
        Body {
            content,
            rewriters,
            numberings,
        }
    }
    pub fn from(self_slug: Key, pure: PureBody, config: &'a TypsiteConfig) -> TypResult<Body<'a>> {
        let content = pure.content;
        let mut err = TypError::new(self_slug.clone());
        let rewriters = pure
            .rewriters
            .into_iter()
            .map(|rewriter| err.ok(BodyRewriter::from(self_slug.clone(), rewriter, config)))
            .collect::<Vec<Option<_>>>();
        let numberings = pure.numberings;
        err.err_or(move || {
            let rewriters = rewriters.into_iter().flatten().collect();
            Self::new(content, rewriters, numberings)
        })
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PureBody {
    pub content: Vec<String>,
    rewriters: Vec<PureRewriter>,
    #[serde(with = "pos_to_index_serde")]
    numberings: HashMap<Pos, usize>,
}

impl From<Body<'_>> for PureBody {
    fn from(body: Body<'_>) -> Self {
        let content = body.content;
        let rewriters = body.rewriters.into_iter().map(PureRewriter::from).collect();
        let numberings = body.numberings;
        PureBody {
            content,
            rewriters,
            numberings,
        }
    }
}

mod pos_to_index_serde {
    use super::*;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(
        map: &HashMap<Pos, usize>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut entries = Vec::with_capacity(map.len());
        for (pos, index) in map {
            entries.push((
                pos.iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join("-"),
                index,
            ));
        }
        entries.serialize(serializer)
    }
    pub fn deserialize<'ce, D: Deserializer<'ce>>(
        deserializer: D,
    ) -> Result<HashMap<Pos, usize>, D::Error> {
        let entries: Vec<(String, usize)> = Vec::deserialize(deserializer)?;
        let mut map = HashMap::with_capacity(entries.len());
        for (pos_str, index) in entries {
            let pos: Pos = if pos_str.is_empty() {
                vec![]
            } else {
                pos_str
                    .split('-')
                    .map(|s| s.parse().map_err(Error::custom))
                    .collect::<Result<_, _>>()?
            };
            map.insert(pos, index);
        }
        Ok(map)
    }
}

#[cfg(test)]
mod test_serde_pos_to_index {
    use super::*;
    use anyhow::*;
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct WrapperPosToIndex {
        #[serde(with = "super::pos_to_index_serde")]
        map: HashMap<Pos, usize>,
    }
    #[test]
    fn body_numberings() -> Result<()> {
        let mut test: HashMap<Pos, usize> = HashMap::new();
        test.insert(vec![1], 10);
        test.insert(vec![1, 1], 12);
        test.insert(vec![2, 3], 15);
        test.insert(vec![], 19);
        let test = WrapperPosToIndex { map: test };
        let str =
            serde_json::to_string(&test).context("Failed to deserialize HashMap<Pos,usize> !")?;
        let test_read = serde_json::from_str::<WrapperPosToIndex>(&str)
            .context("Failed to deserialize HashMap<Pos,usize> !")?;
        assert_eq!(test, test_read);
        Ok(())
    }
}

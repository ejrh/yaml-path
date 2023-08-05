use yaml_rust::Yaml;

use crate::PathError;
use crate::PathError::{NotAHash, NotAnIndex};
use crate::segment::Segment::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Segment {
    Key(Yaml),
    Wildcard
}

impl Segment {
    pub(crate) fn evaluate<'a>(&self, root: &'a Yaml) -> Result<Vec<&'a Yaml>, PathError> {
        match self {
            Key(key) => self.evaluate_key(root, key),
            Wildcard => self.evaluate_wildcard(root)
        }
    }

    fn evaluate_key<'a>(&self, root: &'a Yaml, key: &Yaml) -> Result<Vec<&'a Yaml>, PathError> {
        let value = if root.is_array() {
            let Yaml::Integer(index) = key
            else {
                //println!("invalid index {:?}", key);
                return Err(NotAnIndex);
            };
            let vec = root.as_vec().expect("root should be an array");
            &vec[*index as usize]
        } else {
            let Some(hash) = root.as_hash() else { return Err(NotAHash) };
            //println!("hash lookup key {:?}", key);
            hash.get(key).unwrap()
        };
        let results = Vec::from([value]);
        Ok(results)
    }

    fn evaluate_wildcard<'a>(&self, root: &'a Yaml) -> Result<Vec<&'a Yaml>, PathError> {
        let results = match root {
            Yaml::Array(arr) => arr.iter().map(|y| y).collect(),
            Yaml::Hash(hash) => hash.values().map(|y| y).collect(),
            _ => { return Err(PathError::NotAHash); }
        };
        Ok(results)
    }
}

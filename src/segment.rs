use yaml_rust::Yaml;

use crate::PathError;
use crate::PathError::{NotAHash, NotAnIndex};
use crate::segment::Segment::Key;

pub(crate) enum Segment {
    Key(Yaml)
}

impl Segment {
    pub(crate) fn evaluate<'a>(&self, root: &'a Yaml) -> Result<&'a Yaml, PathError> {
        match self {
            Key(key) => self.evaluate_key(root, key)
        }
    }

    fn evaluate_key<'a>(&self, root: &'a Yaml, key: &Yaml) -> Result<&'a Yaml, PathError> {
        let value = if root.is_array() {
            let Yaml::Integer(index) = key
                else { return Err(NotAnIndex); };
            let vec = root.as_vec().expect("root should be an array");
            &vec[*index as usize]
        } else {
            let Some(hash) = root.as_hash() else { return Err(NotAHash) };
            hash.get(key).unwrap()
        };
        Ok(value)
    }
}

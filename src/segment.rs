use yaml_rust::Yaml;

use crate::PathError;
use crate::PathError::{NodeNotFound, NotAHash, NotAnIndex};
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
            let Some(val) = hash.get(key) else { return Err(NodeNotFound) };
            val
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

#[cfg(test)]
mod test {
    use super::*;

    use yaml_rust::YamlLoader;

    #[test]
    fn test_hash_key() {
        let doc = YamlLoader::load_from_str("a: 1\ntest: 42\n19: nineteen").unwrap();
        let seg = Segment::Key(Yaml::String(String::from("absent")));
        let res = seg.evaluate(&doc[0]);
        assert_eq!(Err(NodeNotFound), res);

        let seg = Segment::Key(Yaml::String(String::from("test")));
        let res = seg.evaluate(&doc[0]);
        assert_eq!(Ok(Vec::from([&Yaml::Integer(42)])), res);

        let seg = Segment::Key(Yaml::Integer(19));
        let res = seg.evaluate(&doc[0]);
        assert_eq!(Ok(Vec::from([&Yaml::String(String::from("nineteen"))])), res);
    }

    #[test]
    fn test_wildcard() {
        let doc = YamlLoader::load_from_str("a: 1\ntest: 42\n19: nineteen").unwrap();
        let seg = Segment::Wildcard;
        let res = seg.evaluate(&doc[0]).unwrap();
        let mut iter = res.into_iter();
        assert_eq!(Some(&Yaml::Integer(1)), iter.next());
        assert_eq!(Some(&Yaml::Integer(42)), iter.next());
        assert_eq!(Some(&Yaml::String(String::from("nineteen"))), iter.next());
        assert_eq!(None, iter.next());
    }
}

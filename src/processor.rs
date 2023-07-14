use std::str::FromStr;
use yaml_rust::Yaml;

use crate::{Path, PathError};
use crate::segment::Segment::Key;

pub struct Processor<'a> {
    document: &'a Yaml
}

impl<'a> Processor<'a> {
    pub fn new(document: &'a Yaml) -> Processor {
        Processor { document }
    }

    pub fn get_all(&self, path: &Path) -> Result<Vec<&'a Yaml>, PathError> {
        let mut root = self.document;
        let mut results = Vec::new();
        for seg in &path.segments {
            let name = match seg {
                Key(name) => name,
                _ => { return Err(PathError::NotAHash)}
            };
            let key = if let Ok(name_as_int) = i64::from_str(name) {
                Yaml::Integer(name_as_int)
            } else {
                Yaml::String(name.clone())
            };
            root = root.as_hash().unwrap().get(&key).unwrap();
        }
        results.push(root);
        Ok(results)
    }
}

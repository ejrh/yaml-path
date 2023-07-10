use yaml_rust::Yaml;

use crate::{Path, PathError};
use crate::segment::Segment;
use crate::segment::Segment::Key;

pub struct Processor<'a> {
    document: &'a Yaml
}

impl<'a> Processor<'a> {
    pub fn new(document: &'a Yaml) -> Processor {
        Processor { document }
    }

    pub fn get_all(&self, path: &Path) -> Result<Vec<&'a Yaml>, PathError> {
        let mut results = Vec::new();
        let name = match &path.segments[0] {
            Key(name) => name,
            _ => { return Err(PathError::NotAHash)}
        };
        let name = Yaml::String(name.clone());
        results.push(self.document.as_hash().unwrap().get(&name).unwrap());
        Ok(results)
    }
}

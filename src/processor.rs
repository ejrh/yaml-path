use yaml_rust::Yaml;

use crate::{Path, PathError};

pub struct Processor<'a> {
    document: &'a Yaml
}

impl<'a> Processor<'a> {
    pub fn new(document: &'a Yaml) -> Processor {
        Processor { document }
    }

    pub fn get_all(&'a self, path: &Path) -> Result<Vec<&'a Yaml>, PathError> {
        let mut root = self.document;
        let mut results = Vec::new();
        for seg in &path.segments {
            root = seg.evaluate(root).unwrap();
        }
        results.push(root);
        Ok(results)
    }
}

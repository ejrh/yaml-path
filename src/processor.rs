use yaml_rust::Yaml;

use crate::{Path, PathError};

pub struct Processor<'a> {
    document: &'a Yaml
}

impl<'a> Processor<'a> {
    pub fn new(document: &'a Yaml) -> Processor {
        Processor { document }
    }

    pub fn get_all(&self, path: &Path) -> Result<Vec<&'a Yaml>, PathError> {
        let mut roots = Vec::from([self.document]);
        for seg in &path.segments {
            let mut new_roots = Vec::new();
            for r in roots {
                let mut new_r = seg.evaluate(r)?;
                new_roots.append(&mut new_r);
            }
            roots = new_roots;
        }
        Ok(roots)
    }

    pub fn get_one(&self, path: &Path) -> Result<&'a Yaml, PathError> {
        let results = self.get_all(path)?;
        if results.is_empty() {
            return Err(PathError::NodeNotFound);
        }
        if results.len() > 1 {
            return Err(PathError::TooManyNodes);
        }
        Ok(results[0])
    }
}

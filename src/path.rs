use crate::PathError;
use crate::segment::Segment;

enum Separator {
    AutoDetect,
    Slash,
    Dot
}

pub struct Path {
    separator: Separator,
    original: String,
    pub(crate) segments: Vec<Segment>
}

impl Path {
    pub fn new(path_str: &str) -> Result<Path, PathError> {
        let separator = Separator::Slash;
        let mut segments = Vec::new();
        let path_str = path_str.strip_prefix('/').unwrap_or(path_str);
        for part in path_str.split('/') {
            segments.push(Segment::Key(String::from(part)));
        }
        Ok(Path {
            separator,
            original: String::from(path_str),
            segments
        })
    }
}

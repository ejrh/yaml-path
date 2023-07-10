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
        segments.push(Segment::Key(String::from(path_str)));
        Ok(Path {
            separator,
            original: String::from(path_str),
            segments
        })
    }
}

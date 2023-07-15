use std::str::FromStr;
use yaml_rust::Yaml;
use crate::PathError;
use crate::segment::Segment;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Separator {
    AutoDetect,
    Slash,
    Dot
}

pub struct Path {
    separator: Separator,
    original: String,
    pub(crate) segments: Vec<Segment>
}

pub(crate) fn detect_separator(path_str: &str) -> Separator {
    if path_str.starts_with('/') { Separator::Slash } else { Separator::Dot }
}

impl Separator {
    fn symbol(&self) -> char {
        match *self {
            Self::Dot => '.',
            Self::Slash => '/',
            Self::AutoDetect => '.'
        }
    }
}

impl Path {
    pub fn new(path_str: &str) -> Result<Path, PathError> {
        let separator = detect_separator(path_str);
        let mut segments = Vec::new();
        let path_str = path_str.strip_prefix('/').unwrap_or(path_str);
        for part in path_str.split(separator.symbol()) {
            let key = i64::from_str(part)
                .map(Yaml::Integer)
                .unwrap_or_else(|_| Yaml::String(String::from(part)));
            segments.push(Segment::Key(key));
        }
        Ok(Path {
            separator,
            original: String::from(path_str),
            segments
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_separator() {
        assert_eq!(Separator::Dot, detect_separator(""));
        assert_eq!(Separator::Dot, detect_separator("test"));
        assert_eq!(Separator::Slash, detect_separator("/"));
        assert_eq!(Separator::Slash, detect_separator("/key1"));
        assert_eq!(Separator::Dot, detect_separator("key1/key2"));
    }
}

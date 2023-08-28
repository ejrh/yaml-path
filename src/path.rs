use std::str::FromStr;
use yaml_rust::Yaml;
use crate::PathError;
use crate::PathError::ParseError;
use crate::segment::Segment;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Separator {
    AutoDetect,
    Slash,
    Dot
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path {
    separator: Separator,
    original: String,
    pub(crate) segments: Vec<Segment>
}

impl FromStr for Path {
    type Err = PathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Path::new(s)
    }
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
        let original = String::from(path_str);
        let mut segments = Vec::new();

        let (segment_parts, separator) = segment_parts(path_str)?;

        for part in segment_parts {
            if part.is_empty() {
                return Err(ParseError);
            } else if part == "*" {
                segments.push(Segment::Wildcard);
            } else {
                let key = i64::from_str(part)
                    .map(Yaml::Integer)
                    .unwrap_or_else(|_| Yaml::String(String::from(part)));
                segments.push(Segment::Key(key));
            }
        }
        Ok(Path {
            separator,
            original,
            segments
        })
    }

    pub fn get_all<'a>(&self, doc: &'a Yaml) -> Result<Vec<&'a Yaml>, PathError> {
        let mut roots = Vec::from([doc]);
        for seg in &self.segments {
            let mut new_roots = Vec::new();
            for r in roots {
                let mut new_r = seg.evaluate(r)?;
                new_roots.append(&mut new_r);
            }
            roots = new_roots;
        }
        Ok(roots)
    }

    pub fn get_one<'a>(&self, doc: &'a Yaml) -> Result<&'a Yaml, PathError> {
        let results = self.get_all(doc)?;
        if results.is_empty() {
            return Err(PathError::NodeNotFound);
        }
        if results.len() > 1 {
            return Err(PathError::TooManyNodes);
        }
        Ok(results[0])
    }
}

fn segment_parts(path_str: &str) -> Result<(Vec<&str>, Separator), PathError> {
    let separator = detect_separator(path_str);
    let path_str = path_str.strip_prefix('/').unwrap_or(path_str);

    let mut parts = Vec::new();
    let mut start_pos = 0;

    let mut new_part = |start_pos, end_pos| {
        if end_pos == start_pos {
            return;
        }
        parts.push(&path_str[start_pos..end_pos]);
    };

    for (pos, ch) in path_str.chars().enumerate() {
        if ch == separator.symbol() {
            new_part(start_pos, pos);
            start_pos = pos + 1;
        } else if ch == '/' || ch == '.' {
            return Err(ParseError);
        }
    }
    new_part(start_pos, path_str.len());
    if parts.is_empty() {
        return Err(ParseError);
    }
    return Ok((parts, separator));
}

#[cfg(test)]
mod test {
    use crate::PathError::ParseError;
    use super::*;

    #[test]
    fn test_detect_separator() {
        assert_eq!(Separator::Dot, detect_separator(""));
        assert_eq!(Separator::Dot, detect_separator("test"));
        assert_eq!(Separator::Slash, detect_separator("/"));
        assert_eq!(Separator::Slash, detect_separator("/key1"));
        assert_eq!(Separator::Dot, detect_separator("key1/key2"));
    }

    #[test]
    fn test_bad_path_separators() {
        assert_eq!(Err(ParseError), Path::new(""));
        assert_eq!(Err(ParseError), Path::new("/"));
        assert_eq!(Err(ParseError), Path::new("."));
        assert_eq!(Err(ParseError), Path::new("foo/bar"));
        assert_eq!(Err(ParseError), Path::new("/foo.bar"));
        assert_eq!(Err(ParseError), Path::new("foo.bar/baz"));
    }

    #[test]
    fn test_segment_parts() {
        assert_eq!(Err(ParseError), segment_parts(""));
        assert_eq!(Err(ParseError), segment_parts("/"));
        assert_eq!(Ok((vec!["a"], Separator::Dot)), segment_parts("a"));
        assert_eq!(Ok((vec!["a", "b"], Separator::Slash)), segment_parts("/a/b"));
        assert_eq!(Ok((vec!["a", "b"], Separator::Dot)), segment_parts("a.b"));
    }

    #[test]
    fn test_parse() {
        let path = Path::new("/1/*/b").unwrap();
        let mut iter = path.segments.iter();
        assert_eq!(Some(&Segment::Key(Yaml::Integer(1))), iter.next());
        assert_eq!(Some(&Segment::Wildcard), iter.next());
        assert_eq!(Some(&Segment::Key(Yaml::String(String::from("b")))), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_fromstr_parse() {
        let path: Path = "/hello".parse().unwrap();
        assert_eq!("/hello", path.original);
    }
}

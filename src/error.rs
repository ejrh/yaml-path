use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PathError {
    ParseError,
    NotAHash
}

impl Display for PathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Error for PathError {

}

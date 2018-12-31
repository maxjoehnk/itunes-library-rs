use std::io;
use xml;

pub enum Error {
    IO(io::Error),
    Parse(xml::reader::Error)
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<xml::reader::Error> for Error {
    fn from(error: xml::reader::Error) -> Self {
        Error::Parse(error)
    }
}

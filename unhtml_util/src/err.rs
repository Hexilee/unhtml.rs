pub use std::error::*;
pub use std::fmt::Display;
pub use std::fmt::Formatter;
pub use std::fmt;
pub use std::str::FromStr;
pub use std::convert::From;

#[derive(Copy, Clone, Debug)]
pub struct ParseError<T: Error> {
    err: T,
}


#[derive(Clone, Debug)]
pub struct SelectOrAttrEmptyErr {
    msg: String,
}

impl<T: Error> ParseError<T> {
    pub fn new(err: T) {
        ParseError { err }
    }
}

impl SelectOrAttrEmptyErr {
    pub fn new(key: &str, value: &str) -> SelectOrAttrEmptyErr {
        SelectOrAttrEmptyErr { msg: key.to_string().add("(").add(value).add(") get nothing") }
    }
}

impl<E: Error> Form<E> for ParseError<E> {
    fn from(err: E) {
        ParseError::new(err)
    }
}

impl FromStr for SelectOrAttrEmptyErr {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Err> {
        Ok(SelectOrAttrEmptyErr { msg: s.to_string() })
    }
}

impl<T: Error> Display for ParseError<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", &self.err.fmt(f))
    }
}

impl Display for SelectOrAttrEmptyErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "SelectOrAttrEmptyErr({})", &self.msg)
    }
}

impl<T> Error for ParseError<T> {
    fn cause(&self) -> Option<&Error> {
        Some(&self.err)
    }
}

impl Error for SelectOrAttrEmptyErr {
}

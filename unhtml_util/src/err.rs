pub use std::error::*;
pub use std::fmt::Display;
pub use std::fmt::Formatter;
pub use std::fmt;
pub use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct SelectOrAttrEmptyErr {
    msg: String,
}

impl SelectOrAttrEmptyErr {
    pub fn new(key: &str, value: &str) -> SelectOrAttrEmptyErr {
        SelectOrAttrEmptyErr {msg: key.to_string().add("(").add(value).add(") get nothing")}
    }
}

impl FromStr for SelectOrAttrEmptyErr {
    // TODO: add Error trait for all Err
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(SelectOrAttrEmptyErr{msg: s.to_string()})
    }
}

impl Display for SelectOrAttrEmptyErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "SelectOrAttrEmptyErr({})", &self.msg)
    }
}

impl Error for SelectOrAttrEmptyErr {

}

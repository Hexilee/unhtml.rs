pub use std::error::*;
pub use std::fmt::Display;
pub use std::fmt::Formatter;
pub use std::fmt;
pub use std::str::FromStr;
pub use std::convert::From;
pub use failure::Error;

#[derive(Fail, Debug)]
pub enum ParseError {
    #[fail(display = "{}({}) get nothing", attr, value)]
    SelectOrAttrEmptyErr {
        attr: String,
        value: String
    }
}
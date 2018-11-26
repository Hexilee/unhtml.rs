pub use cssparser::ParseError;
pub use selectors::parser::SelectorParseErrorKind;

pub trait UnHtml {
    fn from_str(data: &str) -> Result<Self, ParseError<SelectorParseErrorKind>>
        where Self: Sized
    ;
}
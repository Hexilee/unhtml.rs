use cssparser::ParseError;
use selectors::parser::SelectorParseErrorKind;

pub trait UnHtml {
    fn from_str(data: &str) -> Result<Box<Self>, ParseError<SelectorParseErrorKind>>;
}
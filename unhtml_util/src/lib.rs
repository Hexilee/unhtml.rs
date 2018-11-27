pub const HTML_IDENT: &str = "html";
pub const SELECTOR_IDENT: &str = "selector";
pub const ATTR_IDENT: &str = "attr";
pub const DEFAULT_IDENT: &str = "default";
pub const EQUAL_PUNCT: char = '=';
pub const ROOT_SELECTOR: &str = "root";
pub const ATTR_INNER_HTML: &str = "inner_html";

pub use scraper::Selector;
pub use scraper::Html;
pub use std::error::*;
pub use std::str::FromStr;

type ParseError = Box<Error>;

pub fn get_str_by_selector_and_attr(selector: &'static str, attr: &'static str) -> Box<Fn(&str) -> Option<String>> {
    let selector = Selector::parse(selector).unwrap();
    Box::new(move |html: &str| {
        let doc = Html::parse_fragment(html);
        let first_elem = doc.select(&selector).next()?;
        Some(first_elem.value().attr(attr)?.to_string())
    })
}

pub trait VecFromStr<T: std::str::FromStr> {
    fn vec_from_str(str_vec: Vec<&str>) -> Result<Vec<T>, T::Err> {
        let mut result = Vec::with_capacity(str_vec.len());
        for string in str_vec.iter() {
            result.push(T::from_str(*string)?)
        }
        Ok(result)
    }
}

impl<T: std::str::FromStr> VecFromStr<T> for T {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
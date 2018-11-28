#![feature(custom_attribute)]
extern crate failure;
#[macro_use]
extern crate failure_derive;

pub mod err;

pub const HTML_IDENT: &str = "html";
pub const SELECTOR_IDENT: &str = "selector";
pub const ATTR_IDENT: &str = "attr";
pub const DEFAULT_IDENT: &str = "default";
pub const EQUAL_PUNCT: char = '=';
pub const ROOT_SELECTOR: &str = "root";
pub const ATTR_INNER_TEXT: &str = "innerText";

pub use scraper::Selector;
pub use scraper::Html;
pub use scraper::ElementRef;
pub use std::str::FromStr;
pub use failure::Error;
use self::err::ParseError;

pub fn get_elem_by_selector_and_attr<E, T>(selector_str: &'static str, attr: &'static str) -> Box<Fn(ElementRef) -> Result<T, Error>>
    where E: std::error::Error + Send + Sync + 'static,
          T: FromStr<Err=E> {
    let selector = Selector::parse(selector_str).unwrap();
    Box::new(move |elem: ElementRef| {
        let first_elem = elem.select(&selector).next().ok_or(
            ParseError::SelectOrAttrEmptyErr { attr: "selector".to_string(), value: selector_str.to_string() }
        )?;
        Ok(T::from_str(first_elem.value().attr(attr).ok_or(
            ParseError::SelectOrAttrEmptyErr { attr: "attr".to_string(), value: attr.to_string() }
        )?)?)
    })
}

pub fn get_elem_by_selector_and_inner_text(selector: &str) -> Box<Fn(&str) -> Option<String>> {
    let selector = Selector::parse(selector).unwrap();
    Box::new(move |html: &str| {
        let doc = Html::parse_fragment(html);
        let first_elem = doc.select(&selector).next()?;
        Some(first_elem.inner_html().to_string())
    })
}

pub fn get_elem_by_selector(selector: &str) -> Box<Fn(&str) -> Option<String>> {
    let selector = Selector::parse(selector).unwrap();
    Box::new(move |html: &str| {
        let doc = Html::parse_fragment(html);
        let first_elem = doc.select(&selector).next()?;
        Some(first_elem.html().to_string())
    })
}

pub fn get_elem_by_attr(attr: &'static str) -> Box<Fn(&str) -> Option<String>> {
    Box::new(move |html: &str| {
        let doc = Html::parse_fragment(html);
        Some(ElementRef::wrap(doc.tree.nodes().by_ref().next()?)?.value().attr(attr)?.to_string())
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
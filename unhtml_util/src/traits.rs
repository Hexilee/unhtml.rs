use scraper::Selector;
use scraper::ElementRef;
use scraper::html::Select;
use std::str::FromStr;
use failure::Error;
use super::err::ParseError;

pub trait FromHtml {
    type Err: std::error::Error + Send + Sync + 'static;
    type Elem: FromStr<Err=Self::Err>;
    fn get_elem_by_selector_and_attr(selector_str: &'static str, attr: &'static str) -> Box<Fn(ElementRef) -> Result<Self::Elem, Error>> {
        let selector = Selector::parse(selector_str).unwrap();
        Box::new(move |elem: ElementRef| {
            let first_elem = elem.select(&selector).next().ok_or(
                ParseError::SelectOrAttrEmptyErr { attr: "selector".to_string(), value: selector_str.to_string() }
            )?;
            Ok(Self::Elem::from_str(first_elem.value().attr(attr).ok_or(
                ParseError::SelectOrAttrEmptyErr { attr: "attr".to_string(), value: attr.to_string() }
            )?)?)
        })
    }

    fn get_elem_by_selector_and_inner_text(selector_str: &'static str) -> Box<Fn(ElementRef) -> Result<Self::Elem, Error>> {
        let selector = Selector::parse(selector_str).unwrap();
        Box::new(move |elem: ElementRef| {
            let first_elem = elem.select(&selector).next().ok_or(
                ParseError::SelectOrAttrEmptyErr { attr: "selector".to_string(), value: selector_str.to_string() }
            )?;
            Ok(Self::Elem::from_str(&first_elem.inner_html())?)
        })
    }

    fn get_elem_by_selector_and_html(selector_str: &'static str) -> Box<Fn(ElementRef) -> Result<Self::Elem, Error>> {
        let selector = Selector::parse(selector_str).unwrap();
        Box::new(move |elem: ElementRef| {
            let first_elem = elem.select(&selector).next().ok_or(
                ParseError::SelectOrAttrEmptyErr { attr: "selector".to_string(), value: selector_str.to_string() }
            )?;
            Ok(Self::Elem::from_str(&first_elem.html())?)
        })
    }

    fn get_elem_by_attr(attr: &'static str) -> Box<Fn(ElementRef) -> Result<Self::Elem, Error>> {
        Box::new(move |elem: ElementRef| {
            Ok(Self::Elem::from_str(elem.value().attr(attr).ok_or(
                ParseError::SelectOrAttrEmptyErr { attr: "attr".to_string(), value: attr.to_string() }
            )?)?)
        })
    }

    fn get_elem_by_inner_text(elem_ref: ElementRef) -> Result<Self::Elem, Error> {
        Ok(Self::Elem::from_str(&elem_ref.inner_html())?)
    }

    fn get_elem_by_html(elem_ref: ElementRef) -> Result<Self::Elem, Error> {
        Ok(Self::Elem::from_str(&elem_ref.html())?)
    }
}

pub trait VecFromHtml {
    type Err: std::error::Error + Send + Sync + 'static;
    type Elem: FromStr<Err=Self::Err> + 'static;
    fn from_attr(selector_str: &str, attr: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        Self::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_str(
                element_ref.value().attr(attr).ok_or(
                    ParseError::SelectOrAttrEmptyErr { attr: "attr".to_string(), value: attr.to_string() }
                )?
            )?)
        })
    }

    fn from_inner_text(selector_str: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        Self::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_str(&element_ref.inner_html())?)
        })
    }

    fn from_html(selector_str: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        Self::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_str(&element_ref.html())?)
        })
    }

    fn vec_from<GetElemFun>(selector_str: &str, root_element_ref: ElementRef, get_elem_fun: GetElemFun) -> Result<Vec<Self::Elem>, Error>
        where GetElemFun: Fn(ElementRef) -> Result<Self::Elem, Error> {
        let selector = Selector::parse(selector_str).unwrap();
        let selects = root_element_ref.select(&selector);
        let mut list = Vec::new();
        for elem_ref in selects {
            list.push(get_elem_fun(elem_ref)?);
        }
        Ok(list)
    }
}


impl<E, T> FromHtml for T
    where E: std::error::Error + Send + Sync + 'static,
          T: FromStr<Err=E> {
    type Err = E;
    type Elem = T;
}

impl<E, T> VecFromHtml for Vec<T>
    where E: std::error::Error + Send + Sync + 'static,
          T: FromStr<Err=E> + 'static {
    type Err = E;
    type Elem = T;
}
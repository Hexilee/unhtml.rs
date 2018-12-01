use scraper::Selector;
use scraper::ElementRef;
use std::str::FromStr;
use failure::Error;
use super::err::ParseError;

pub trait FromHtml: Sized {
    fn from_selector_and_attr(selector_str: &str, attr: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(selector_str).unwrap();
        let first_elem = elem_ref.select(&selector).next().ok_or(
            ParseError::SelectOrAttrEmptyErr { attr: "selector".to_string(), value: selector_str.to_string() }
        )?;
        Ok(Self::from_html(first_elem.value().attr(attr).ok_or(
            ParseError::SelectOrAttrEmptyErr { attr: "attr".to_string(), value: attr.to_string() }
        )?)?)
    }

    fn from_selector_and_inner_text(selector_str: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(selector_str).unwrap();
        let first_elem = elem_ref.select(&selector).next().ok_or(
            ParseError::SelectOrAttrEmptyErr { attr: "selector".to_string(), value: selector_str.to_string() }
        )?;
        Ok(Self::from_html(&first_elem.inner_html())?)
    }

    fn from_selector_and_html(selector_str: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(selector_str).unwrap();
        let first_elem = elem_ref.select(&selector).next().ok_or(
            ParseError::SelectOrAttrEmptyErr { attr: "selector".to_string(), value: selector_str.to_string() }
        )?;
        Ok(Self::from_html(&first_elem.html())?)
    }

    fn from_attr(attr: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        Ok(Self::from_html(elem_ref.value().attr(attr).ok_or(
            ParseError::SelectOrAttrEmptyErr { attr: "attr".to_string(), value: attr.to_string() }
        )?)?)
    }

    fn from_inner_text(elem_ref: ElementRef) -> Result<Self, Error> {
        Ok(Self::from_html(&elem_ref.inner_html())?)
    }

    fn from_html_ref(elem_ref: ElementRef) -> Result<Self, Error> {
        Ok(Self::from_html(&elem_ref.html())?)
    }

    fn from_html(html: &str) -> Result<Self, Error>;
}

pub trait VecFromHtml {
    type Elem: FromHtml;
    fn from_attr(selector_str: &str, attr: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        Self::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_html(
                element_ref.value().attr(attr).ok_or(
                    ParseError::SelectOrAttrEmptyErr { attr: "attr".to_string(), value: attr.to_string() }
                )?
            )?)
        })
    }

    fn from_inner_text(selector_str: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        Self::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_html(&element_ref.inner_html())?)
        })
    }

    fn from_html(selector_str: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        Self::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_html(&element_ref.html())?)
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
    fn from_html(html: &str) -> Result<Self, Error> {
        Ok(T::from_str(html)?)
    }
}

impl<T> VecFromHtml for Vec<T>
    where T: FromHtml {
    type Elem = T;
}
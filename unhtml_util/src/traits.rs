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
    fn vec_from<Fun>(getter_fn: Fun) -> Box<Fn(Select) -> Result<Vec<Self::Elem>, Error>>
        where Fun: Fn(ElementRef) -> Result<Self::Elem, Error> + 'static + Copy {
        Box::new(move |selects| {
            let mut list = Vec::new();
            for elem_ref in selects {
                list.push(getter_fn(elem_ref)?);
            }
            Ok(list)
        })
    }

    fn vec_from_single_attr<Fun>(string: &'static str, getter_fn: Fun) -> Box<Fn(Select) -> Result<Vec<Self::Elem>, Error>>
        where Fun: Fn(&'static str) -> Box<Fn(ElementRef) -> Result<Self::Elem, Error>> + 'static + Copy {
        Box::new(move |selects| {
            let mut list = Vec::new();
            for elem_ref in selects {
                list.push(getter_fn(string)(elem_ref)?);
            }
            Ok(list)
        })
    }

    fn vec_from_double_attr<Fun>(str_former: &'static str, str_latter: &'static str, getter_fn: Fun) -> Box<Fn(Select) -> Result<Vec<Self::Elem>, Error>>
        where Fun: Fn(&'static str, &'static str) -> Box<Fn(ElementRef) -> Result<Self::Elem, Error>> + 'static + Copy {
        Box::new(move |selects| {
            let mut list = Vec::new();
            for elem_ref in selects {
                list.push(getter_fn(str_former, str_latter)(elem_ref)?);
                println!("push 1");
            }
            Ok(list)
        })
    }

    fn vec_by_selector_and_attr(selector_str: &'static str, attr: &'static str) -> Box<Fn(Select) -> Result<Vec<Self::Elem>, Error>> {
        Self::vec_from_double_attr(selector_str, attr, Self::Elem::get_elem_by_selector_and_attr)
    }

    fn vec_by_selector_and_html(selector_str: &'static str) -> Box<Fn(Select) -> Result<Vec<Self::Elem>, Error>> {
        Self::vec_from_single_attr(selector_str, Self::Elem::get_elem_by_selector_and_html)
    }

    fn vec_by_selector_and_inner_text(selector_str: &'static str) -> Box<Fn(Select) -> Result<Vec<Self::Elem>, Error>> {
        Self::vec_from_single_attr(selector_str, Self::Elem::get_elem_by_selector_and_inner_text)
    }

    fn vec_by_attr(selector_str: &'static str) -> Box<Fn(Select) -> Result<Vec<Self::Elem>, Error>> {
        Self::vec_from_single_attr(selector_str, Self::Elem::get_elem_by_attr)
    }

    fn vec_by_html(selects: Select) -> Result<Vec<Self::Elem>, Error> {
        Self::vec_from(Self::Elem::get_elem_by_html)(selects)
    }

    fn vec_by_inner_text(selects: Select) -> Result<Vec<Self::Elem>, Error> {
        Self::vec_from(Self::Elem::get_elem_by_inner_text)(selects)
    }
}


impl<E, T> FromHtml for T
    where E: std::error::Error + Send + Sync + 'static,
          T: FromStr<Err=E> {
    type Err = E;
    type Elem = T;
}

impl<E, T> VecFromHtml for T
    where E: std::error::Error + Send + Sync + 'static,
          T: FromStr<Err=E> + 'static {
    type Err = E;
    type Elem = T;
}
use scraper::Selector;
use scraper::Html;
use scraper::ElementRef;
use std::str::FromStr;
use failure::Error;
use super::err::DeserializeError;
use super::polyfill::*;

/// Deserialize from html
pub trait FromHtml: Sized {
    /// # Examples
    ///
    /// ```
    /// use unhtml::*;
    /// let html = Html::parse_fragment(r#"
    ///     <!DOCTYPE html>
    /// <html lang="en">
    /// <head>
    ///     <meta charset="UTF-8">
    ///     <title>Title</title>
    /// </head>
    /// <body>
    ///     <div id="test">
    ///         <a href="1"></a>
    ///     </div>
    /// </body>
    /// </html>
    ///     "#);
    /// let selector = Selector::parse("#test").unwrap();
    /// let result = u8::from_selector_and_attr("a", "href", html.select(&selector).next().unwrap()).unwrap();
    /// assert_eq!(1u8, result);
    /// ```
    fn from_selector_and_attr(selector_str: &str, attr: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(selector_str).unwrap();
        let first_elem = elem_ref.select(&selector).next().ok_or(
            DeserializeError::SourceNotFound { attr: "selector".to_string(), value: selector_str.to_string() }
        )?;
        Ok(Self::from_html(first_elem.value().attr(attr).ok_or(
            DeserializeError::SourceNotFound { attr: "attr".to_string(), value: attr.to_string() }
        )?)?)
    }

    fn from_selector_and_inner_text(selector_str: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(selector_str).unwrap();
        let first_elem = elem_ref.select(&selector).next().ok_or(
            DeserializeError::SourceNotFound { attr: "selector".to_string(), value: selector_str.to_string() }
        )?;
        Ok(Self::from_html(&first_elem.inner_html())?)
    }

    fn from_selector_and_html(selector_str: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(selector_str).unwrap();
        let first_elem = elem_ref.select(&selector).next().ok_or(
            DeserializeError::SourceNotFound { attr: "selector".to_string(), value: selector_str.to_string() }
        )?;
        Ok(Self::from_html(&first_elem.html())?)
    }

    fn from_attr(attr: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        Ok(Self::from_html(elem_ref.value().attr(attr).ok_or(
            DeserializeError::SourceNotFound { attr: "attr".to_string(), value: attr.to_string() }
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
                    DeserializeError::SourceNotFound { attr: "attr".to_string(), value: attr.to_string() }
                )?
            )?)
        })
    }

    fn from_inner_text(selector_str: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        Self::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_html(&element_ref.inner_html())?)
        })
    }

    fn from_html_ref(selector_str: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        Self::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_html(&element_ref.html())?)
        })
    }

    fn from_html(selector_str: &str, html: &str) -> Result<Vec<Self::Elem>, Error> {
        Self::from_html_ref(selector_str, Html::parse_fragment(html).root_element())
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
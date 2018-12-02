use scraper::{ElementRef, Selector, Html};
use std::str::FromStr;
use failure::Error;
use super::err::DeserializeError;
use super::util;

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

    /// # Examples
    ///
    /// ```
    /// use unhtml::*;
    /// let html = Html::parse_fragment(r#"
    /// <body>
    ///     <div id="test">
    ///         <a>1</a>
    ///     </div>
    /// </body>
    /// "#);
    /// let selector = Selector::parse("#test").unwrap();
    /// let result = u8::from_selector_and_inner_text("a", html.select(&selector).next().unwrap()).unwrap();
    /// assert_eq!(1u8, result);
    /// ```
    fn from_selector_and_inner_text(selector_str: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(selector_str).unwrap();
        let first_elem = elem_ref.select(&selector).next().ok_or(
            DeserializeError::SourceNotFound { attr: "selector".to_string(), value: selector_str.to_string() }
        )?;
        Ok(Self::from_html(&first_elem.inner_html())?)
    }

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
    ///         <a>1</a>
    ///     </div>
    /// </body>
    /// </html>
    /// "#);
    /// let selector = Selector::parse("#test").unwrap();
    /// let result = String::from_selector_and_html("a", html.select(&selector).next().unwrap()).unwrap();
    /// assert_eq!("<a>1</a>".to_string(), result);
    /// ```
    fn from_selector_and_html(selector_str: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(selector_str).unwrap();
        let first_elem = elem_ref.select(&selector).next().ok_or(
            DeserializeError::SourceNotFound { attr: "selector".to_string(), value: selector_str.to_string() }
        )?;
        Ok(Self::from_html(&first_elem.html())?)
    }

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
    /// "#);
    /// let selector = Selector::parse("#test > a").unwrap();
    /// let result = u8::from_attr("href", html.select(&selector).next().unwrap()).unwrap();
    /// assert_eq!(1u8, result);
    /// ```
    fn from_attr(attr: &str, elem_ref: ElementRef) -> Result<Self, Error> {
        Ok(Self::from_html(elem_ref.value().attr(attr).ok_or(
            DeserializeError::SourceNotFound { attr: "attr".to_string(), value: attr.to_string() }
        )?)?)
    }

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
    ///         <a>1</a>
    ///     </div>
    /// </body>
    /// </html>
    /// "#);
    /// let selector = Selector::parse("#test > a").unwrap();
    /// let result = u8::from_inner_text(html.select(&selector).next().unwrap()).unwrap();
    /// assert_eq!(1u8, result);
    /// ```
    fn from_inner_text(elem_ref: ElementRef) -> Result<Self, Error> {
        Ok(Self::from_html(&elem_ref.inner_html())?)
    }

    /// # Examples
    ///
    /// ```
    /// use unhtml::*;
    /// let html = Html::parse_fragment(r#"
    /// <!DOCTYPE html>
    /// <html lang="en">
    /// <head>
    ///     <meta charset="UTF-8">
    ///     <title>Title</title>
    /// </head>
    /// <body>
    ///     <div id="test">
    ///         <a>1</a>
    ///     </div>
    /// </body>
    /// </html>
    /// "#);
    /// let selector = Selector::parse("#test > a").unwrap();
    /// let result = String::from_html_ref(html.select(&selector).next().unwrap()).unwrap();
    /// assert_eq!("<a>1</a>".to_string(), result);
    /// ```
    fn from_html_ref(elem_ref: ElementRef) -> Result<Self, Error> {
        Ok(Self::from_html(&elem_ref.html())?)
    }

    /// implemented by default for all types that implemented `FromStr<Err=E> where E: std::error::Error`
    /// # Examples
    /// ```
    /// use unhtml::*;
    /// let result = u8::from_html("1").unwrap();
    /// assert_eq!(1u8, result);
    /// ```
    fn from_html(html: &str) -> Result<Self, Error>;
}

/// Deserialize `Vec<T>` from html where `T`implemented `FromHtml`
pub trait VecFromHtml {
    type Elem: FromHtml;

    /// # Examples
    /// ```
    /// use unhtml::*;
    /// let html = Html::parse_fragment(r#"
    /// <!DOCTYPE html>
    /// <html lang="en">
    /// <head>
    ///     <meta charset="UTF-8">
    ///     <title>Title</title>
    /// </head>
    /// <body>
    ///     <div id="test">
    ///         <a href="1"></a>
    ///         <a href="2"></a>
    ///         <a href="3"></a>
    ///     </div>
    /// </body>
    /// </html>
    /// "#);
    /// let results = Vec::<u8>::from_attr("#test > a", "href", html.root_element()).unwrap();
    /// assert_eq!(1u8, results[0]);
    /// assert_eq!(2u8, results[1]);
    /// assert_eq!(3u8, results[2]);
    /// ```
    fn from_attr(selector_str: &str, attr: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        util::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_html(
                element_ref.value().attr(attr).ok_or(
                    DeserializeError::SourceNotFound { attr: "attr".to_string(), value: attr.to_string() }
                )?
            )?)
        })
    }

    /// # Examples
    /// ```
    /// use unhtml::*;
    /// let html = Html::parse_fragment(r#"
    /// <!DOCTYPE html>
    /// <html lang="en">
    /// <head>
    ///     <meta charset="UTF-8">
    ///     <title>Title</title>
    /// </head>
    /// <body>
    ///     <div id="test">
    ///         <a>1</a>
    ///         <a>2</a>
    ///         <a>3</a>
    ///     </div>
    /// </body>
    /// </html>
    /// "#);
    /// let results = Vec::<u8>::from_inner_text("#test > a", html.root_element()).unwrap();
    /// assert_eq!(1u8, results[0]);
    /// assert_eq!(2u8, results[1]);
    /// assert_eq!(3u8, results[2]);
    /// ```
    fn from_inner_text(selector_str: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        util::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_html(&element_ref.inner_html())?)
        })
    }

    /// # Examples
    /// ```
    /// use unhtml::*;
    /// let html = Html::parse_fragment(r#"
    /// <!DOCTYPE html>
    /// <html lang="en">
    /// <head>
    ///     <meta charset="UTF-8">
    ///     <title>Title</title>
    /// </head>
    /// <body>
    ///     <div id="test">
    ///         <a href="1"></a>
    ///         <a href="2"></a>
    ///         <a href="3"></a>
    ///     </div>
    /// </body>
    /// </html>
    /// "#);
    /// let results = Vec::<String>::from_html_ref("#test > a", html.root_element()).unwrap();
    /// assert_eq!(r#"<a href="1"></a>"#, results[0]);
    /// assert_eq!(r#"<a href="2"></a>"#, results[1]);
    /// assert_eq!(r#"<a href="3"></a>"#, results[2]);
    /// ```
    fn from_html_ref(selector_str: &str, root_element_ref: ElementRef) -> Result<Vec<Self::Elem>, Error> {
        util::vec_from(selector_str, root_element_ref, |element_ref| {
            Ok(Self::Elem::from_html(&element_ref.html())?)
        })
    }

    /// # Examples
    /// ```
    /// use unhtml::*;
    /// let results = Vec::<String>::from_html("#test > a", r#"
    /// <!DOCTYPE html>
    /// <html lang="en">
    /// <head>
    ///     <meta charset="UTF-8">
    ///     <title>Title</title>
    /// </head>
    /// <body>
    ///     <div id="test">
    ///         <a href="1"></a>
    ///         <a href="2"></a>
    ///         <a href="3"></a>
    ///     </div>
    /// </body>
    /// </html>
    /// "#).unwrap();
    /// assert_eq!(r#"<a href="1"></a>"#, results[0]);
    /// assert_eq!(r#"<a href="2"></a>"#, results[1]);
    /// assert_eq!(r#"<a href="3"></a>"#, results[2]);
    /// ```
    fn from_html(selector_str: &str, html: &str) -> Result<Vec<Self::Elem>, Error> {
        Self::from_html_ref(selector_str, Html::parse_fragment(html).root_element())
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
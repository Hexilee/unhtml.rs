use super::err::HtmlError;
use ego_tree::NodeRef;
use failure::Error;
use scraper::{html, ElementRef, Html, Node, Selector};
use std::str::FromStr;

pub trait Select<'a> {
    fn select_elements(
        self,
        selector: &'a Selector,
    ) -> Box<dyn Iterator<Item = ElementRef<'a>> + 'a>;
}

impl<'a, T> Select<'a> for T
where
    T: Iterator<Item = ElementRef<'a>> + 'a,
{
    fn select_elements(
        self,
        selector: &'a Selector,
    ) -> Box<dyn Iterator<Item = ElementRef<'a>> + 'a> {
        Box::new(self.flat_map(move |elem_ref| elem_ref.select(selector)))
    }
}

type ElemIter<'a> = &'a mut (dyn Iterator<Item = ElementRef<'a>> + 'a);

/// Deserialize from html
pub trait FromHtml: Sized {
    /// # Examples
    ///
    /// ```
    /// use unhtml::scraper::{Html, Selector};
    /// use unhtml::FromHtml;
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
    /// let result = String::from_elements(html.select(&selector).next().unwrap()).unwrap();
    /// assert_eq!("<a>1</a>".to_string(), result);
    /// ```
    fn from_elements(select: ElemIter) -> Result<Self, Error>;

    /// implemented by default for all types that implemented `FromStr<Err=E> where E: std::error::Error`
    /// # Examples
    /// ```
    /// use unhtml::FromHtml;
    /// let result = u8::from_html("1").unwrap();
    /// assert_eq!(1u8, result);
    /// ```
    fn from_html(html: &str) -> Result<Self, Error> {
        let root_selector = Selector::parse(":root").unwrap();
        Self::from_elements(&mut Html::parse_fragment(html).select(&root_selector))
    }
}

pub trait FromText: Sized {
    fn from_inner_text(select: ElemIter) -> Result<Self, Error>;
    fn from_attr(select: ElemIter, attr: &str) -> Result<Self, Error>;
}

impl<T> FromText for T
where
    T: FromStr,
    T::Err: failure::Fail,
{
    fn from_inner_text(select: ElemIter) -> Result<Self, Error> {
        let first = select.next().ok_or(HtmlError::SourceEmpty)?;
        Ok(first.inner_html().trim().parse()?)
    }
    fn from_attr(select: ElemIter, attr: &str) -> Result<Self, Error> {
        let first = select.next().ok_or(HtmlError::SourceEmpty)?;
        let attr = first.value().attr(attr).ok_or(HtmlError::SourceNotFound {
            source_type: "attr".into(),
            source_name: attr.into(),
        })?;
        Ok(attr.trim().parse()?)
    }
}

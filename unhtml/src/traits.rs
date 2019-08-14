use super::err::HtmlError;
use failure::Error;
use scraper::{ElementRef, Html, Selector};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use std::path::PathBuf;

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

impl<T> FromText for Option<T>
where
    T: FromText,
{
    fn from_inner_text(select: ElemIter) -> Result<Self, Error> {
        Ok(match T::from_inner_text(select) {
            Ok(ret) => Some(ret),
            Err(_) => None,
        })
    }

    fn from_attr(select: ElemIter, attr: &str) -> Result<Self, Error> {
        Ok(match T::from_attr(select, attr) {
            Ok(ret) => Some(ret),
            Err(_) => None,
        })
    }
}

impl<T> FromHtml for Option<T>
where
    T: FromHtml,
{
    fn from_elements(select: ElemIter) -> Result<Self, Error> {
        Ok(match T::from_elements(select) {
            Ok(ret) => Some(ret),
            Err(_) => None,
        })
    }
}

impl<T> FromText for Vec<T>
where
    T: FromText,
{
    fn from_inner_text(select: ElemIter) -> Result<Self, Error> {
        let mut ret = vec![];
        for elem in select {
            ret.push(T::from_inner_text(&mut vec![elem].into_iter())?)
        }
        Ok(ret)
    }

    fn from_attr(select: ElemIter, attr: &str) -> Result<Self, Error> {
        let mut ret = vec![];
        for elem in select {
            ret.push(T::from_attr(&mut vec![elem].into_iter(), attr)?)
        }
        Ok(ret)
    }
}

impl<T> FromHtml for Vec<T>
where
    T: FromHtml,
{
    fn from_elements(select: ElemIter) -> Result<Self, Error> {
        let mut ret = vec![];
        for elem in select {
            ret.push(T::from_elements(&mut vec![elem].into_iter())?)
        }
        Ok(ret)
    }
}

impl FromText for () {
    fn from_inner_text(_select: ElemIter) -> Result<Self, Error> {
        Ok(())
    }

    fn from_attr(_select: ElemIter, _attr: &str) -> Result<Self, Error> {
        Ok(())
    }
}

macro_rules! from_text {
    ($($typ:ty),*) => {
        $(
            impl FromText for $typ {
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
        )*
    };
}

from_text!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    String,
    Ipv4Addr,
    Ipv6Addr,
    SocketAddrV4,
    SocketAddrV6,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU128,
    NonZeroUsize,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroI128,
    NonZeroIsize,
    PathBuf
);

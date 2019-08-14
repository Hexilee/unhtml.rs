use super::err::HtmlError;
use crate::Result;
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

pub type ElemIter<'a> = &'a mut (dyn Iterator<Item = ElementRef<'a>> + 'a);

/// parse html
pub trait FromHtml: Sized {
    fn from_elements(select: ElemIter) -> Result<Self>;

    fn from_html(html: &str) -> Result<Self> {
        let root_selector = Selector::parse(":root").unwrap();
        Self::from_elements(&mut Html::parse_fragment(html).select(&root_selector))
    }
}

pub trait Element<'a, T> {
    fn element(&'a mut self) -> Result<T>;
}

pub trait FromText: Sized {
    fn from_inner_text(select: ElemIter) -> Result<Self>;
    fn from_attr(select: ElemIter, attr: &str) -> Result<Self>;
}

pub trait Text<'a, T> {
    fn inner_text(&'a mut self) -> Result<T>;
    fn attr(&'a mut self, attr: &str) -> Result<T>;
}

impl<'a, T, I> Element<'a, T> for I
where
    T: FromHtml,
    I: Iterator<Item = ElementRef<'a>> + 'a,
{
    fn element(&'a mut self) -> Result<T> {
        T::from_elements(self)
    }
}

impl<'a, T, I> Text<'a, T> for I
where
    T: FromText,
    I: Iterator<Item = ElementRef<'a>> + 'a,
{
    fn inner_text(&'a mut self) -> Result<T> {
        T::from_inner_text(self)
    }

    fn attr(&'a mut self, attr: &str) -> Result<T> {
        T::from_attr(self, attr)
    }
}

impl<T> FromText for Option<T>
where
    T: FromText,
{
    fn from_inner_text(select: ElemIter) -> Result<Self> {
        Ok(match T::from_inner_text(select) {
            Ok(ret) => Some(ret),
            Err(_) => None,
        })
    }

    fn from_attr(select: ElemIter, attr: &str) -> Result<Self> {
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
    fn from_elements(select: ElemIter) -> Result<Self> {
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
    fn from_inner_text(select: ElemIter) -> Result<Self> {
        let mut ret = vec![];
        for elem in select {
            ret.push(vec![elem].into_iter().inner_text()?)
        }
        Ok(ret)
    }

    fn from_attr(select: ElemIter, attr: &str) -> Result<Self> {
        let mut ret = vec![];
        for elem in select {
            ret.push(vec![elem].into_iter().attr(attr)?)
        }
        Ok(ret)
    }
}

impl<T> FromHtml for Vec<T>
where
    T: FromHtml,
{
    fn from_elements(select: ElemIter) -> Result<Self> {
        let mut ret = vec![];
        for elem in select {
            ret.push(vec![elem].into_iter().element()?)
        }
        Ok(ret)
    }
}

impl FromText for () {
    fn from_inner_text(_select: ElemIter) -> Result<Self> {
        Ok(())
    }

    fn from_attr(_select: ElemIter, _attr: &str) -> Result<Self> {
        Ok(())
    }
}

macro_rules! from_text {
    ($($typ:ty),*) => {
        $(
            impl FromText for $typ {
                fn from_inner_text(select: ElemIter) -> Result<Self> {
                    let first = select.next().ok_or(HtmlError::SourceEmpty)?;
                    let mut ret = String::new();
                    for next_segment in first.text() {
                        ret += next_segment.trim();
                    }
                    Ok(ret.parse()?)
                }
                fn from_attr(select: ElemIter, attr: &str) -> Result<Self> {
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

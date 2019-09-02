use crate::Result;
use scraper::{ElementRef, Html, Selector};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use std::path::PathBuf;
use std::str::FromStr;

pub trait Select<'b, 'a: 'b> {
    fn select_elements(
        self,
        selector: &'b Selector,
    ) -> Box<dyn Iterator<Item = ElementRef<'a>> + 'b>;
}

impl<'b, 'a: 'b, T> Select<'b, 'a> for T
where
    T: Iterator<Item = ElementRef<'a>> + 'b,
{
    fn select_elements(
        self,
        selector: &'b Selector,
    ) -> Box<dyn Iterator<Item = ElementRef<'a>> + 'b> {
        Box::new(self.flat_map(move |elem_ref| elem_ref.select(selector)))
    }
}

pub type ElemIter<'b, 'a> = &'b mut (dyn Iterator<Item = ElementRef<'a>> + 'b);

/// parse html
pub trait FromHtml: Sized {
    fn from_elements(select: ElemIter) -> Result<Self>;
    fn from_html(html: &str) -> Result<Self> {
        Self::from_elements(
            &mut Html::parse_document(html).select(&Selector::parse(":root").unwrap()),
        )
    }
}

pub trait Element<'b, 'a: 'b, T: 'a> {
    fn element(&'b mut self) -> Result<T>;
}

pub trait FromText: Sized {
    fn from_inner_text(select: ElemIter) -> Result<Self>;
    fn from_attr(select: ElemIter, attr: &str) -> Result<Self>;
}

pub trait Text<'b, 'a: 'b, T: 'a> {
    fn inner_text(&'b mut self) -> Result<T>;
    fn attr(&'b mut self, attr: &'b str) -> Result<T>;
}

impl<'b, 'a: 'b, T, I> Element<'b, 'a, T> for I
where
    T: FromHtml + 'a,
    I: Iterator<Item = ElementRef<'a>> + 'b,
{
    fn element(&'b mut self) -> Result<T> {
        T::from_elements(self)
    }
}

impl<'b, 'a: 'b, T, I> Text<'b, 'a, T> for I
where
    T: FromText + 'a,
    I: Iterator<Item = ElementRef<'a>> + 'b,
{
    fn inner_text(&'b mut self) -> Result<T> {
        T::from_inner_text(self)
    }

    fn attr(&'b mut self, attr: &'b str) -> Result<T> {
        T::from_attr(self, attr)
    }
}

default impl<T> FromText for T
where
    T: FromStr,
    T::Err: ToString,
{
    fn from_inner_text(select: ElemIter) -> Result<Self> {
        unimplemented!()
    }

    fn from_attr(select: ElemIter, attr: &str) -> Result<Self> {
        unimplemented!()
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
                    let first = select.next().ok_or(())?;
                    let mut ret = String::new();
                    for next_segment in first.text() {
                        ret += next_segment.trim();
                    }
                    Self::from_str(&ret).map_err(|err| (ret.to_owned(), stringify!($typ).to_owned(), err.to_string()).into())
                }
                fn from_attr(select: ElemIter, attr: &str) -> Result<Self> {
                    let first = select.next().ok_or(())?;
                    let attr = first.value().attr(attr).ok_or((attr.to_owned(), first.html()))?;
                    Self::from_str(attr.trim()).map_err(|err| (attr.trim().to_owned(), stringify!($typ).to_owned(), err.to_string()).into())
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

#![feature(custom_attribute)]
#[macro_use]
extern crate failure_derive;

#[cfg(test)]
mod test;
mod err;
mod traits;
mod polyfill;
mod util;
pub use self::err::*;
pub use scraper::{Selector, Html};
pub use self::traits::*;
pub use self::polyfill::*;
pub use failure;
pub const HTML_IDENT: &str = "html";
pub const SELECTOR_IDENT: &str = "selector";
pub const ATTR_IDENT: &str = "attr";
pub const DEFAULT_IDENT: &str = "default";
pub const EQUAL_PUNCT: char = '=';
pub const ATTR_INNER_TEXT: &str = "value";

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


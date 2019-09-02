//! [![Build status](https://img.shields.io/travis/Hexilee/unhtml.rs/master.svg)](https://travis-ci.org/Hexilee/unhtml.rs)
//! [![Crate version](https://img.shields.io/crates/v/unhtml.svg)](https://crates.io/crates/unhtml)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Hexilee/unhtml.rs/blob/master/LICENSE)
//! [![Rust Docs](https://docs.rs/unhtml/badge.svg)](https://docs.rs/unhtml)
//!
//!
#![feature(specialization)]

pub extern crate scraper;
#[doc(inline)]
pub use self::err::{Error, Result};
#[doc(inline)]
pub use self::traits::{ElemIter, Element, FromHtml, FromText, Select, Text};

#[cfg(feature = "derive")]
pub use unhtml_derive::{FromHtml, FromText};

mod err;
#[cfg(test)]
mod test;
mod traits;

//! [![Build status](https://img.shields.io/travis/Hexilee/unhtml.rs/master.svg)](https://travis-ci.org/Hexilee/unhtml.rs)
//! [![Crate version](https://img.shields.io/crates/v/unhtml.svg)](https://crates.io/crates/unhtml)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Hexilee/unhtml.rs/blob/master/LICENSE)
//! [![Rust Docs](https://docs.rs/unhtml/badge.svg)](https://docs.rs/unhtml)
//!
//!

pub extern crate failure;
pub extern crate scraper;
mod err;
#[cfg(test)]
mod test;
mod traits;
pub use self::err::*;
pub use self::traits::*;

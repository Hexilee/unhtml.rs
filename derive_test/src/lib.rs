#![feature(custom_attribute)]
extern crate unhtml;

#[macro_use]
extern crate unhtml_derive;

use unhtml::traits::*;

#[cfg(test)]
mod test;

#[derive(UnHtml)]
#[html(selector="#test")]
struct User {
    #[html(selector="#test", default="Hexilee")]
    name: &'static str,

    #[html(selector="#test", default=20)]
    age: u8,

    #[html(selector="#test", default=true)]
    like_lemon: bool,
}

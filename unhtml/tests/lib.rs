#![feature(custom_attribute)]

extern crate unhtml;
extern crate unhtml_util;
use unhtml::unhtml;
use unhtml_util::*;

#[cfg(test)]
mod test;

#[unhtml]
//#[html(selector = "#test")]
struct User {
    #[html(selector = "#test", default = "Hexilee")]
    name: String,

    #[html(selector = "#test", default = 20)]
    age: u8,

    #[html(selector = "#test", default = true)]
    like_lemon: bool,
}

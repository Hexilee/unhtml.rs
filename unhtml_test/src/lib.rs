#![feature(custom_attribute)]

extern crate unhtml;

use unhtml::unhtml;

#[cfg(test)]
mod test;

#[unhtml]
#[selector = "#test"]
struct User {
    #[html(selector = "#test", default = "Hexilee")]
    name: String,

    #[html(selector = "#test", default = 20)]
    age: u8,

    #[html(selector = "#test", default = true)]
    like_lemon: bool,
}

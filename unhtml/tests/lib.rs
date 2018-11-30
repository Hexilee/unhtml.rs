#![feature(custom_attribute)]

extern crate unhtml;
extern crate unhtml_util;
use unhtml::unhtml;
use unhtml_util::*;

#[cfg(test)]
mod test;

// default test case
#[unhtml]
struct DefaultUser {
    // invoke String::from
    #[html(selector = "#non-exist", default = "Hexilee")]
    name: String,

    // invoke u8::from<u8>
    #[html(default = 20)]
    age: u8,

    // invoke i64::from_str
    #[html(default = "-1000")]
    assets: i64,

    #[html(default = true)]
    like_lemon: bool,
}

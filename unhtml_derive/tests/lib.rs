#[macro_use]
extern crate unhtml_derive;

use unhtml::{self, VecFromHtml};

#[cfg(test)]
mod test;

// default test case
#[derive(FromHtml)]
struct DefaultUser {
    // invoke String::from_html
    #[html(selector = "#non-exist", default = "Hexilee")]
    name: String,

    // invoke u8::from<u8>
    #[html(default = 20)]
    age: u8,

    #[html(default = "-1000")]
    assets: i64,

    #[html(default = true)]
    like_lemon: bool,
}

#[derive(FromHtml)]
#[html(selector = "#test")]
struct SingleUser {
    #[html(selector = "p:nth-child(1)", attr = "inner")]
    name: String,

    #[html(selector = "p:nth-child(2)", attr = "inner")]
    age: u8,

    #[html(selector = "p:nth-child(3)", attr = "inner")]
    like_lemon: bool,
}

#[derive(FromHtml)]
#[html(selector = "a")]
struct Link {
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    value: String,
}

#[derive(FromHtml)]
struct TestUser {
    #[html(selector = "p:nth-child(1)", attr = "inner")]
    name: String,

    #[html(selector = "p:nth-child(2)", attr = "inner")]
    age: u8,

    #[html(selector = "p:nth-child(3)", attr = "inner")]
    like_lemon: bool,
}

#[derive(FromHtml)]
#[html(selector = "#test")]
struct TestUsers {
    #[html(selector = "div")]
    users: Vec<TestUser>,
}
extern crate unhtml;

#[macro_use]
extern crate unhtml_derive;

use unhtml::traits::*;

#[cfg(test)]
mod test;

#[derive(UnHtml)]
struct User {
    name: &'static str
}

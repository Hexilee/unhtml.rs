#![feature(proc_macro_diagnostic, decl_macro)]

//! ## unhtml_derive
//!
//! [![Build status](https://img.shields.io/travis/Hexilee/unhtml.rs/master.svg)](https://travis-ci.org/Hexilee/unhtml.rs)
//! [![Crate version](https://img.shields.io/crates/v/unhtml_derive.svg)](https://crates.io/crates/unhtml_derive)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Hexilee/unhtml.rs/blob/master/LICENSE)
//! [![Rust Docs](https://docs.rs/unhtml_derive/badge.svg)](https://docs.rs/unhtml_derive)
//!
//! derive for crate unhtml [![Crate version](https://img.shields.io/crates/v/unhtml.svg)](https://crates.io/crates/unhtml)
//!
//! ----------------
//!
//! Table of Contents
//! =================
//!
//! * [Derive Target](#derive-target)
//! * [Basic Usage](#basic-usage)
//! * [Attributes](#attributes)
//!     * [html](#html)
//!         * [target](#target)
//!         * [specification](#specification)
//!     * [selector](#selector)
//!         * [target](#target-1)
//!         * [literal type](#literal-type)
//!         * [specification](#specification-1)
//!         * [default behavior](#default-behavior)
//!     * [attr](#attr)
//!         * [target](#target-2)
//!         * [literal type](#literal-type-1)
//!         * [specification](#specification-2)
//!         * [default behavior](#default-behavior-1)
//!     * [default](#default)
//!         * [target](#target-3)
//!         * [literal type](#literal-type-2)
//!         * [specification](#specification-3)
//!         * [default behavior](#default-behavior-2)
//! * [Field Type](#field-type)
//!     * [any type implemented FromHtml, without generics](#any-type-implemented-fromhtml-without-generics)
//!     * [Vec](#vec)
//! * [Source HTML](#source-html)
//!     * [with top selector](#with-top-selector)
//!     * [without top selector](#without-top-selector)
//!
//! ### Derive Target
//!
//! `struct`
//!
//! ### Basic Usage
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! #[html(selector = "#test")]
//! struct SingleUser {
//!     #[html(selector = "p:nth-child(1)", attr = "inner")]
//!     name: String,
//!
//!     #[html(selector = "p:nth-child(2)", attr = "inner")]
//!     age: u8,
//!
//!     #[html(selector = "p:nth-child(3)", attr = "inner")]
//!     like_lemon: bool,
//! }
//!
//! let user = SingleUser::from_html(r#"<!DOCTYPE html>
//! <html lang="en">
//! <head>
//!     <meta charset="UTF-8">
//!     <title>Title</title>
//! </head>
//! <body>
//!     <div id="test">
//!         <div>
//!             <p>Hexilee</p>
//!             <p>20</p>
//!             <p>true</p>
//!         </div>
//!     </div>
//! </body>
//! </html>"#).unwrap();
//! assert_eq!("Hexilee", &user.name);
//! assert_eq!(20, user.age);
//! assert!(user.like_lemon);
//! ```
//!
//! ### Attributes
//! #### html
//!
//! ##### target
//!
//! `derive target` or `field`
//!
//! ##### specification
//!
//!
//! `#[html(selector = "...", attr = "...", default = ...)]`
//!
//! `selector`, `attr`, `default` or `html` itself can be unnecessary.
//!
//! This is valid
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//!
//! #[derive(FromHtml)]
//! struct SingleString {
//!     value: String,
//! }
//! ```
//!
//!
//! #### selector
//!
//! ##### target
//!
//! `derive target` or `field`
//!
//!
//! ##### literal type
//!
//! `string`
//!
//! ##### specification
//!
//! selector must be a valid css-selector, invalid selector will cause a compile-time panic
//!
//! ```rust,ignore
//! // panic
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! #[html(selector = "<>")]
//! struct SingleUser {}
//! ```
//!
//! ```rust,ignore
//! // panic
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::*;
//!
//! #[derive(FromHtml)]
//! struct SingleUser {
//!     #[html(selector = "<>", attr = "inner")]
//!     name: String,
//! }
//! ```
//!
//! if multi element is selected and field type is not `Vec`, the first will be chosen
//!
//! ```rust,ignore
//!
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! #[html(selector = "a")]
//! struct Link {
//!     #[html(attr = "href")]
//!     href: String,
//!
//!     #[html(attr = "inner")]
//!     value: String,
//! }
//!
//! let link = Link::from_html(r#"
//! <a href="https://github.com">Github</a>
//! <a href="https://google.com">Google</a>
//! "#).unwrap();
//! assert_eq!("https://github.com", &link.href);
//! assert_eq!("Github", &link.value);
//! ```
//!
//!
//! ##### default behavior
//!
//! html of its root element
//!
//! ```rust,ignore
//!
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! #[html(selector = "a")]
//! struct Link {
//!     #[html(attr = "href")]
//!     href: String,
//!
//!     #[html(attr = "inner")]
//!     value: String,
//! }
//!
//! let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
//! assert_eq!("https://github.com", &link.href);
//! assert_eq!("Github", &link.value);
//! ```
//!
//!
//! #### attr
//!
//! ##### target
//!
//! `field`
//!
//!
//! ##### literal type
//!
//! `string`
//!
//! ##### specification
//!
//! - `value` refer to `innerHtml`
//! - any other `attr` refer to `html element attribute`
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! #[html(selector = "a")]
//! struct Link {
//!     #[html(attr = "href")]
//!     href: String,
//!
//!     #[html(attr = "inner")]
//!     value: String,
//! }
//!
//! let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
//! assert_eq!("https://github.com", &link.href);
//! assert_eq!("Github", &link.value);
//! ```
//!
//! ##### default behavior
//!
//! html of the whole element (not `innerHtml`!)
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! #[html(selector = "a")]
//! struct Link {
//!     #[html(attr = "href")]
//!     href: String,
//!
//!     #[html(attr = "inner")]
//!     value: String,
//!
//!     source: String,
//! }
//!
//! let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
//! assert_eq!("https://github.com", &link.href);
//! assert_eq!("Github", &link.value);
//! assert_eq!(r#"<a href="https://github.com">Github</a>"#, &link.source);
//! ```
//!
//! #### default
//!
//! ##### target
//!
//! `field`
//!
//! ##### literal type
//!
//! any `literal type`
//!
//! ##### specification
//!
//! - the same type with `field`
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! struct DefaultUser {
//!     // invoke String::from_html
//!     #[html(selector = "#non-exist", default = "Hexilee")]
//!     name: String,
//!
//!     // invoke u8::from<u8>
//!     #[html(default = 20)]
//!     age: u8,
//!
//!     #[html(default = true)]
//!     like_lemon: bool,
//! }
//!
//! let user = DefaultUser::from_html("<p></p>").unwrap();
//! assert_eq!("Hexilee", &user.name);
//! assert_eq!(20, user.age);
//! assert_eq!(-1000, user.assets);
//! assert!(user.like_lemon);
//! ```
//!
//! - `string`
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! #[html(selector = "a")]
//! struct Link {
//!     #[html(attr = "href")]
//!     href: String,
//!
//!     #[html(attr = "inner")]
//!     value: String,
//! }
//!
//! #[derive(FromHtml)]
//! struct Website {
//!     #[html(default = "10")]
//!     age: u8,
//!
//!     #[html(default = "<a href='https://github.com'>Github</a>")]
//!     link: Link,
//! }
//!
//! let website = Website::from_html("<p></p>").unwrap();
//! let link = website.link;
//! assert_eq!(10u8, website.age);
//! assert_eq!("https://github.com", &link.href);
//! assert_eq!("Github", &link.value);
//! ```
//!
//! ##### default behavior
//!
//! return a Err(unhtml::failure::Error) when selected nothing
//!
//! ```rust,ignore
//! // panic
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! #[html(selector = "a")]
//! struct Link {
//!     // no default
//!     #[html(attr = "href")]
//!     href: String,
//!
//!     #[html(attr = "inner")]
//!     value: String,
//! }
//!
//! let link = Link::from_html(r#"<a>Github</a>"#).unwrap();
//! ```
//!
//! ### Field Type
//!
//! ##### any type implemented FromHtml, without generics
//!
//! ```rust,ignore
//! // panic
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! #[html(selector = "a")]
//! struct Link {
//!     // no default
//!     #[html(attr = "href")]
//!     href: &str,
//! }
//! ```
//!
//! ```rust,ignore
//! // panic
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! struct Website {
//!     // no default
//!     #[html(attr = "href")]
//!     hrefs: std::collections::LinkedList<String>,
//! }
//! ```
//!
//! ```rust,ignore
//! // panic
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! struct Website {
//!     // no default
//!     #[html(attr = "href")]
//!     hrefs: [String],
//! }
//! ```
//!
//! ##### Vec
//!
//! > Should `use unhtml::VecFromHtml`
//!
//! ```rust,ignore
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml, VecFromHtml};
//!
//! #[derive(FromHtml)]
//! struct TestUser {
//!     #[html(selector = "p:nth-child(1)", attr = "inner")]
//!     name: String,
//!
//!     #[html(selector = "p:nth-child(2)", attr = "inner")]
//!     age: u8,
//!
//!     #[html(selector = "p:nth-child(3)", attr = "inner")]
//!     like_lemon: bool,
//! }
//!
//! #[derive(FromHtml)]
//! #[html(selector = "#test")]
//! struct TestUsers {
//!     #[html(selector = "div")]
//!     users: Vec<TestUser>,
//! }
//!
//! let users = TestUsers::from_html(r#"<!DOCTYPE html>
//! <html lang="en">
//! <head>
//!     <meta charset="UTF-8">
//!     <title>Title</title>
//! </head>
//! <body>
//!     <div id="test">
//!         <div>
//!             <p>Hexilee</p>
//!             <p>20</p>
//!             <p>true</p>
//!         </div>
//!         <div>
//!             <p>BigBrother</p>
//!             <p>21</p>
//!             <p>false</p>
//!         </div>
//!     </div>
//! </body>
//! </html>"#).unwrap();
//! let hexilee = &users.users[0];
//! let big_brother = &users.users[1];
//! assert_eq!("Hexilee", &hexilee.name);
//! assert_eq!(20, hexilee.age);
//! assert!(hexilee.like_lemon);
//! assert_eq!("BigBrother", &big_brother.name);
//! assert_eq!(21, big_brother.age);
//! assert!(!big_brother.like_lemon);
//! ```
//!
//! as the documentation of crate `unhtml`, if you want `Vec<TestUser>` straightly, you can just:
//!
//!
//! ```rust,ignore
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, VecFromHtml};
//!
//! #[derive(FromHtml)]
//! struct TestUser {
//!     #[html(selector = "p:nth-child(1)", attr = "inner")]
//!     name: String,
//!
//!     #[html(selector = "p:nth-child(2)", attr = "inner")]
//!     age: u8,
//!
//!     #[html(selector = "p:nth-child(3)", attr = "inner")]
//!     like_lemon: bool,
//! }
//!
//! let users = Vec::<TestUser>::from_html("#test > div", r#"<!DOCTYPE html>
//! <html lang="en">
//! <head>
//!     <meta charset="UTF-8">
//!     <title>Title</title>
//! </head>
//! <body>
//!     <div id="test">
//!         <div>
//!             <p>Hexilee</p>
//!             <p>20</p>
//!             <p>true</p>
//!         </div>
//!         <div>
//!             <p>BigBrother</p>
//!             <p>21</p>
//!             <p>false</p>
//!         </div>
//!     </div>
//! </body>
//! </html>"#).unwrap();
//! let hexilee = &users[0];
//! let big_brother = &users[1];
//! assert_eq!("Hexilee", &hexilee.name);
//! assert_eq!(20, hexilee.age);
//! assert!(hexilee.like_lemon);
//! assert_eq!("BigBrother", &big_brother.name);
//! assert_eq!(21, big_brother.age);
//! assert!(!big_brother.like_lemon);
//! ```
//!
//! ### Source HTML
//!
//! ##### with top selector
//! all source html will be parsed as `fragment`. The top element is `html` and there is no `DOCTYPE`, `head` or `body`.
//!
//! ```html
//! <!DOCTYPE html>
//! <html lang="en">
//! <head>
//!     <meta charset="UTF-8">
//!     <title>Title</title>
//! </head>
//! <body>
//!     <div id="test">
//!         <div>
//!             <p>Hexilee</p>
//!             <p>20</p>
//!             <p>true</p>
//!         </div>
//!     </div>
//! </body>
//! </html>
//! ```
//!
//! will be parsed as:
//!
//! ```html
//! <html lang="en">
//!     <meta charset="UTF-8">
//!     <title>Title</title>
//!     <div id="test">
//!         <div>
//!             <p>Hexilee</p>
//!             <p>20</p>
//!             <p>true</p>
//!         </div>
//!     </div>
//! </html>
//! ```
//!
//! and
//!
//! ```html
//! <p>Hexilee</p>
//! ```
//!
//! will be parsed as:
//!
//! ```html
//! <html>
//!     <p>Hexilee</p>
//! </html>
//! ```
//!
//! ```rust,ignore
//!
//! // panic
//!
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! struct Document {
//!     // no default
//!     #[html(selector = "head")]
//!     head: String,
//!
//!     #[html(selector = "body")]
//!     body: String,
//! }
//!
//! let dicument = Document::from_html(r#"<!DOCTYPE html>
//! <html lang="en">
//! <head>
//!     <meta charset="UTF-8">
//!     <title>Title</title>
//! </head>
//! <body>
//!     <div id="test">
//!         <div>
//!             <p>Hexilee</p>
//!             <p>20</p>
//!             <p>true</p>
//!         </div>
//!     </div>
//! </body>
//! </html>"#).unwrap();
//! ```
//!
//! ##### without top selector
//!
//! when derived struct doesn't have `top selector`, all source html will be parsed as `pure fragment`. There is no `DOCTYPE`, `html`, `head` or `body`.
//!
//! ```html
//! <!DOCTYPE html>
//! <html lang="en">
//! <head>
//!     <meta charset="UTF-8">
//!     <title>Title</title>
//! </head>
//! <body>
//!     <div id="test">
//!         <div>
//!             <p>Hexilee</p>
//!             <p>20</p>
//!             <p>true</p>
//!         </div>
//!     </div>
//! </body>
//! </html>
//! ```
//!
//! will be parsed as:
//!
//! ```html
//! <meta charset="UTF-8">
//! <title>Title</title>
//! <div id="test">
//!    <div>
//!        <p>Hexilee</p>
//!        <p>20</p>
//!        <p>true</p>
//!    </div>
//! </div>
//! ```
//!
//! and
//!
//! ```html
//! <p>Hexilee</p>
//! ```
//!
//! will be parsed as:
//!
//! ```html
//! <p>Hexilee</p>
//! ```
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate unhtml_derive;
//! extern crate unhtml;
//! use unhtml::{self, FromHtml};
//!
//! #[derive(FromHtml)]
//! #[html(selector = "a")]
//! struct Link {
//!     #[html(attr = "href")]
//!     href: String,
//!
//!     #[html(attr = "inner")]
//!     value: String,
//! }
//!
//! let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
//! assert_eq!("https://github.com", &link.href);
//! assert_eq!("Github", &link.value);
//! ```

extern crate proc_macro;

mod attr_meta;
mod html;
mod parse;
mod text;

use proc_macro::{Diagnostic, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemTrait};

#[proc_macro_derive(FromHtml, attributes(html))]
pub fn html_derive(input: TokenStream) -> TokenStream {
    html::derive(input)
        .unwrap_or_else(|err| {
            err.emit();
            quote!()
        })
        .into()
}

#[proc_macro_derive(FromText)]
pub fn text_derive(input: TokenStream) -> TokenStream {
    text::derive(input)
        .unwrap_or_else(|err| {
            err.emit();
            quote!()
        })
        .into()
}

type Result<T> = std::result::Result<T, Diagnostic>;

### unhtml_derive

[![Build status](https://img.shields.io/travis/Hexilee/unhtml.rs/master.svg)](https://travis-ci.org/Hexilee/unhtml.rs)
[![Crate version](https://img.shields.io/crates/v/unhtml_derive.svg)](https://crates.io/crates/unhtml_derive)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Hexilee/unhtml.rs/blob/master/LICENSE)
[![Rust Docs](https://docs.rs/unhtml_derive/badge.svg)](https://docs.rs/unhtml_derive)

derive for crate unhtml [![Crate version](https://img.shields.io/crates/v/unhtml.svg)](https://crates.io/crates/unhtml)

----------------

#### Derive Target

`struct`

#### Basic Usage

```rust
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::*;

#[derive(FromHtml)]
#[html(selector = "#test")]
struct SingleUser {
    #[html(selector = "p:nth-child(1)", attr = "value")]
    name: String,

    #[html(selector = "p:nth-child(2)", attr = "value")]
    age: u8,

    #[html(selector = "p:nth-child(3)", attr = "value")]
    like_lemon: bool,
}

let user = SingleUser::from_html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <div>
            <p>Hexilee</p>
            <p>20</p>
            <p>true</p>
        </div>
    </div>
</body>
</html>"#).unwrap();
assert_eq!("Hexilee".to_string(), user.name);
assert_eq!(20, user.age);
assert!(user.like_lemon);
```

#### Attributes
##### html
###### specification


`#[html(selector = "...", attr = "...", default = ...)]`

`selector`, `attr`, `default` or `html` itself can be unnecessary.

This is valid

```rust
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::*;


#[derive(FromHtml)]
struct SingleString {
    value: String,
}
```


###### target

derive target itself or field


##### selector

###### literal type

`string`

###### specification

selector must be a invalid css-selector, invalid selector will cause a compile-time panic

```rust,should_panic
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::*;

#[derive(FromHtml)]
#[html(selector = "<>")]
struct SingleUser {}
```

```rust,should_panic
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::*;

#[derive(FromHtml)]
struct SingleUser {
    #[html(selector = "<>", attr = "value")]
    name: String,
}
```

###### target

attribute target can be derive target itself or field

###### default behavior

assign value of its root element

```rust

#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::*;

#[derive(FromHtml)]
#[html(selector = "#test")]
struct Link {
    #[html(attr = "href")]
    href: String,
    
    #[html(attr = "value")]
    value: String,
}

let link = Link::from_html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <a id="test" href="https://github.com">Github</p>
</body>
</html>"#).unwrap();
assert_eq!("https://github.com".to_string(), link.href);
assert_eq!("Github".to_string(), link.value);
```


##### attr

##### default

#### Field Type



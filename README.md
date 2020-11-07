HTML deserializer for rust

[![Stable Test](https://github.com/Hexilee/unhtml.rs/workflows/Stable%20Test/badge.svg)](https://github.com/Hexilee/unhtml.rs/actions)
[![Rust Docs](https://docs.rs/unhtml/badge.svg)](https://docs.rs/unhtml)
[![Crate version](https://img.shields.io/crates/v/unhtml.svg)](https://crates.io/crates/unhtml)
[![Download](https://img.shields.io/crates/d/unhtml.svg)](https://crates.io/crates/unhtml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Hexilee/unhtml.rs/blob/master/LICENSE)
----------------

Table of Contents
=================

* [Derive Target](#derive-target)
* [Basic Usage](#basic-usage)
* [Attributes](#attributes)
    * [html](#html)
        * [target](#target)
        * [specification](#specification)
    * [selector](#selector)
        * [target](#target-1)
        * [literal type](#literal-type)
        * [specification](#specification-1)
        * [default behavior](#default-behavior)
    * [attr](#attr)
        * [target](#target-2)
        * [literal type](#literal-type-1)
        * [specification](#specification-2)
        * [default behavior](#default-behavior-1)
    * [default](#default)
        * [target](#target-3)
        * [literal type](#literal-type-2)
        * [specification](#specification-3)
        * [default behavior](#default-behavior-2)
* [Field Type](#field-type)
    * [any type implemented FromHtml, without generics](#any-type-implemented-fromhtml-without-generics)
    * [Vec](#vec)
* [Source HTML](#source-html)
    * [with top selector](#with-top-selector)
    * [without top selector](#without-top-selector)


### Derive Target

`struct`

### Basic Usage

```rust
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

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
assert_eq!("Hexilee", &user.name);
assert_eq!(20, user.age);
assert!(user.like_lemon);
```

### Attributes
#### html

##### target

`derive target` or `field`

##### specification


`#[html(selector = "...", attr = "...", default = ...)]`

`selector`, `attr`, `default` or `html` itself can be unnecessary.

This is valid

```rust
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};


#[derive(FromHtml)]
struct SingleString {
    value: String,
}
```


#### selector

##### target

`derive target` or `field`


##### literal type

`string`

##### specification

selector must be a valid css-selector, invalid selector will cause a compile-time panic

```rust,should_panic
// panic
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
#[html(selector = "<>")]
struct SingleUser {}
```

```rust,should_panic
// panic
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::*;

#[derive(FromHtml)]
struct SingleUser {
    #[html(selector = "<>", attr = "inner")]
    name: String,
}
```

if multi element is selected and field type is not `Vec`, the first will be chosen

```rust

#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
#[html(selector = "a")]
struct Link {
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    value: String,
}

let link = Link::from_html(r#"
<a href="https://github.com">Github</a> 
<a href="https://google.com">Google</a> 
"#).unwrap();
assert_eq!("https://github.com", &link.href);
assert_eq!("Github", &link.value);
```


##### default behavior

html of its root element

```rust

#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct Link {
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    value: String,
}

let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
assert_eq!("https://github.com", &link.href);
assert_eq!("Github", &link.value);
```


#### attr

##### target

`field`


##### literal type

`string`

##### specification

- `inner` refer to `innerHtml`
- any other `attr` refer to `html element attribute`

```rust
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct Link {
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    value: String,
}

let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
assert_eq!("https://github.com", &link.href);
assert_eq!("Github", &link.value);
```

##### default behavior

html of the whole element (not `innerHtml`!)

```rust
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct Link {
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    value: String,
    
    source: String,
}

let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
assert_eq!("https://github.com", &link.href);
assert_eq!("Github", &link.value);
assert_eq!(r#"<a href="https://github.com">Github</a>"#, &link.source);
```

#### default

##### target

`field`

##### literal type

any `literal type`

##### specification

- the same type with `field`

```rust
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct DefaultUser {
    // invoke String::from_html
    #[html(selector = "#non-exist", default = "Hexilee")]
    name: String,

    // invoke u8::from<u8>
    #[html(default = 20)]
    age: u8,

    #[html(default = true)]
    like_lemon: bool,
}

let user = DefaultUser::from_html("<p></p>").unwrap();
assert_eq!("Hexilee", &user.name);
assert_eq!(20, user.age);
assert_eq!(-1000, user.assets);
assert!(user.like_lemon);
```

- `string`

```rust
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct Link {
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    value: String,
}

#[derive(FromHtml)]
struct Website {
    #[html(default = "10")]
    age: u8,

    #[html(default = "<a href='https://github.com'>Github</a>")]
    link: Link,
}

let website = Website::from_html("<p></p>").unwrap();
let link = website.link;
assert_eq!(10u8, website.age);
assert_eq!("https://github.com", &link.href);
assert_eq!("Github", &link.value);
```

##### default behavior

return a Err(unhtml::failure::Error) when selected nothing

```rust,should_panic
// panic
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct Link {
    // no default
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    value: String,
}

let link = Link::from_html(r#"<a>Github</a>"#).unwrap();
```

### Field Type

##### any type implemented FromHtml, without generics

```rust,should_panic
// panic
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct Link {
    // no default
    #[html(attr = "href")]
    href: &str,
}
```

```rust,should_panic
// panic
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct Website {
    // no default
    #[html(attr = "href")]
    hrefs: std::collections::LinkedList<String>,
}
```

```rust,should_panic
// panic
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct Website {
    // no default
    #[html(attr = "href")]
    hrefs: [String],
}
```

##### Vec

> Should `use unhtml::VecFromHtml`

```rust
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml, VecFromHtml};

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

let users = TestUsers::from_html(r#"<!DOCTYPE html>
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
        <div>
            <p>BigBrother</p>
            <p>21</p>
            <p>false</p>
        </div>
    </div>
</body>
</html>"#).unwrap();
let hexilee = &users.users[0];
let big_brother = &users.users[1];
assert_eq!("Hexilee", &hexilee.name);
assert_eq!(20, hexilee.age);
assert!(hexilee.like_lemon);
assert_eq!("BigBrother", &big_brother.name);
assert_eq!(21, big_brother.age);
assert!(!big_brother.like_lemon);
```

as the documentation of crate `unhtml`, if you want `Vec<TestUser>` straightly, you can just:


```rust
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, VecFromHtml};

#[derive(FromHtml)]
struct TestUser {
    #[html(selector = "p:nth-child(1)", attr = "inner")]
    name: String,

    #[html(selector = "p:nth-child(2)", attr = "inner")]
    age: u8,

    #[html(selector = "p:nth-child(3)", attr = "inner")]
    like_lemon: bool,
}

let users = Vec::<TestUser>::from_html("#test > div", r#"<!DOCTYPE html>
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
        <div>
            <p>BigBrother</p>
            <p>21</p>
            <p>false</p>
        </div>
    </div>
</body>
</html>"#).unwrap();
let hexilee = &users[0];
let big_brother = &users[1];
assert_eq!("Hexilee", &hexilee.name);
assert_eq!(20, hexilee.age);
assert!(hexilee.like_lemon);
assert_eq!("BigBrother", &big_brother.name);
assert_eq!(21, big_brother.age);
assert!(!big_brother.like_lemon);
```

### Source HTML

##### with top selector
all source html will be parsed as `fragment`. The top element is `html` and there is no `DOCTYPE`, `head` or `body`.

```html,ignore
<!DOCTYPE html>
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
</html>
```

will be parsed as:

```html,ignore
<html lang="en">
    <meta charset="UTF-8">
    <title>Title</title>
    <div id="test">
        <div>
            <p>Hexilee</p>
            <p>20</p>
            <p>true</p>
        </div>
    </div>
</html>
```

and 

```html,ignore
<p>Hexilee</p>
```

will be parsed as:

```html,ignore
<html>
    <p>Hexilee</p>
</html>    
```

```rust,should_panic

// panic

extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct Document {
    // no default
    #[html(selector = "head")]
    head: String,

    #[html(selector = "body")]
    body: String,
}

let dicument = Document::from_html(r#"<!DOCTYPE html>
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
```

##### without top selector

when derived struct doesn't have `top selector`, all source html will be parsed as `pure fragment`. There is no `DOCTYPE`, `html`, `head` or `body`.

```html,ignore
<!DOCTYPE html>
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
</html>
```

will be parsed as:

```html,ignore
<meta charset="UTF-8">
<title>Title</title>
<div id="test">
   <div>
       <p>Hexilee</p>
       <p>20</p>
       <p>true</p>
   </div>
</div>
```

and 

```html,ignore
<p>Hexilee</p>
```

will be parsed as:

```html,ignore
<p>Hexilee</p>
```

```rust
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::{self, FromHtml};

#[derive(FromHtml)]
struct Link {
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    value: String,
}

let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
assert_eq!("https://github.com", &link.href);
assert_eq!("Github", &link.value);
```



### unhtml

[![Stable Test](https://github.com/Hexilee/unhtml.rs/workflows/Stable%20Test/badge.svg)](https://github.com/Hexilee/unhtml.rs/actions)
[![Rust Docs](https://docs.rs/unhtml/badge.svg)](https://docs.rs/unhtml)
[![Crate version](https://img.shields.io/crates/v/unhtml.svg)](https://crates.io/crates/unhtml)
[![Download](https://img.shields.io/crates/d/unhtml.svg)](https://crates.io/crates/unhtml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Hexilee/unhtml.rs/blob/master/LICENSE)

There are two `trait` in crate `unhtml` 

- `FromHtml`

The only method of `FromHtml` you should care about is `fn from_html(html: &str) -> Result<Self, Error>` and this method is implemented for all types implemented `FromStr<E, T>`

```rust
impl<E, T> FromHtml for T
    where E: failure::Fail,
          T: FromStr<Err=E> {
    fn from_html(html: &str) -> Result<Self, Error> {
        Ok(T::from_str(html.trim())?)
    }
}
```

You can implement `FromHtml` automatically for `struct` by crate `unhtml_derive`
[![Crate version](https://img.shields.io/crates/v/unhtml_derive.svg)](https://crates.io/crates/unhtml_derive) 

- `VecFromHtml`

`VecFromHtml` is implemented for `Vec<T> where T: FromHtml` by default

```rust
impl<T> VecFromHtml for Vec<T>
    where T: FromHtml {
    type Elem = T;
}
```

As `FromHtml` is implemented for `u8` by default

```rust
use unhtml::scraper::Html;
use unhtml::VecFromHtml;
let html = Html::parse_fragment(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <a href="1"></a>
        <a href="2"></a>
        <a href="3"></a>
    </div>
</body>
</html>
"#);
let results = Vec::<u8>::from_attr("#test > a", "href", html.root_element()).unwrap();
assert_eq!(1u8, results[0]);
assert_eq!(2u8, results[1]);
assert_eq!(3u8, results[2]);
```
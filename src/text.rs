use std::str::FromStr;
use unhtml::derive::FromText;
use unhtml::scraper::{Html, Selector};
use unhtml::{failure::Error, Result, Text};

#[derive(Debug, FromText, Eq, PartialEq)]
struct U8(u8);

impl FromStr for U8 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s.parse()?))
    }
}

#[test]
fn test_inner_text() {
    let selector = Selector::parse("p").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <p> 1 </p>
            </div>
            <div>
                <p> 2 </p>
            </div>
            <p> 3 </p>
        </div>
    "##,
    );
    assert_eq!(U8(1), html.select(&selector).inner_text().unwrap());
}

#[test]
fn test_vec_inner_text() {
    let selector = Selector::parse("p").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <p> 1 </p>
            </div>
            <div>
                <p> 2 </p>
            </div>
            <p> 3 </p>
        </div>
    "##,
    );
    let result: Vec<U8> = html.select(&selector).inner_text().unwrap();
    assert_eq!(result, vec![U8(1), U8(2), U8(3)]);
}

#[test]
fn test_option_inner_text() {
    let selector = Selector::parse("p").unwrap();
    let foo_selector = Selector::parse("a").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <p> 1 </p>
            </div>
            <div>
                <p> 2 </p>
            </div>
            <p> 3 </p>
        </div>
    "##,
    );
    assert_eq!(Some(U8(1)), html.select(&selector).inner_text().unwrap());
    let foo_result: Option<U8> = html.select(&foo_selector).inner_text().unwrap();
    assert_eq!(None, foo_result);
}

#[test]
fn test_fail_inner_text() {
    let foo_selector = Selector::parse("a").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <p> 1 </p>
            </div>
            <div>
                <p> 2 </p>
            </div>
            <p> 3 </p>
        </div>
    "##,
    );
    let foo_result: Result<U8> = html.select(&foo_selector).inner_text();
    assert!(!foo_result.is_ok());
}

#[test]
fn test_attr() {
    let selector = Selector::parse("p").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <p value=" 1"></p>
            </div>
            <div>
                <p value=" 2"></p>
            </div>
            <p value="3 "></p>
        </div>
    "##,
    );
    assert_eq!(U8(1), html.select(&selector).attr("value").unwrap());
}

#[test]
fn test_vec_attr() {
    let selector = Selector::parse("p").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <p value=" 1"></p>
            </div>
            <div>
                <p value=" 2"></p>
            </div>
            <p value="3 "></p>
        </div>
    "##,
    );
    let result: Vec<U8> = html.select(&selector).attr("value").unwrap();
    assert_eq!(result, vec![U8(1), U8(2), U8(3)]);
}

#[test]
fn test_option_attr() {
    let selector = Selector::parse("p").unwrap();
    let foo_selector = Selector::parse("a").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <p value=" 1"></p>
            </div>
            <div>
                <p value=" 2"></p>
            </div>
            <p value="3 "></p>
        </div>
    "##,
    );
    assert_eq!(Some(U8(1)), html.select(&selector).attr("value").unwrap());
    let foo_result: Option<U8> = html.select(&foo_selector).attr("value").unwrap();
    assert_eq!(None, foo_result);
}

#[test]
fn test_fail_attr() {
    let foo_selector = Selector::parse("a").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <p value=" 1"></p>
            </div>
            <div>
                <p value=" 2"></p>
            </div>
            <p value="3 "></p>
        </div>
    "##,
    );
    let foo_result: Result<U8> = html.select(&foo_selector).attr("value");
    assert!(!foo_result.is_ok());
}

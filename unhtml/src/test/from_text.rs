use crate::Text;
use scraper::{Html, Selector};

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
    let result: u8 = html.select(&selector).inner_text().unwrap();
    assert_eq!(1, result);
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
    let result: Vec<u8> = html.select(&selector).inner_text().unwrap();
    assert_eq!(3, result.len());
    assert_eq!(1, result[0]);
    assert_eq!(2, result[1]);
    assert_eq!(3, result[2]);
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
    assert_eq!(Some(1u8), html.select(&selector).inner_text().unwrap());
    let foo_result: Option<u8> = html.select(&foo_selector).inner_text().unwrap();
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
    let foo_result: Result<u8, _> = html.select(&foo_selector).inner_text();
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
    let result: u8 = html.select(&selector).attr("value").unwrap();
    assert_eq!(1, result);
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
    let result: Vec<u8> = html.select(&selector).attr("value").unwrap();
    assert_eq!(3, result.len());
    assert_eq!(1, result[0]);
    assert_eq!(2, result[1]);
    assert_eq!(3, result[2]);
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
    assert_eq!(Some(1u8), html.select(&selector).attr("value").unwrap());
    let foo_result: Option<u8> = html.select(&foo_selector).attr("value").unwrap();
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
    let foo_result: Result<u8, _> = html.select(&foo_selector).attr("value");
    assert!(!foo_result.is_ok());
}

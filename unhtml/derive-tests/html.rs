use unhtml::{
    scraper::{Html, Selector},
    Element, Result,
};

use unhtml::derive::FromHtml;

#[derive(FromHtml, Debug, Eq, PartialEq)]
struct Link {
    #[html(attr = "href")]
    href: String,

    #[html(attr = "inner")]
    text: String,
}

#[derive(FromHtml, Debug, Eq, PartialEq)]
struct Website {
    #[html(selector = "title", attr = "inner")]
    title: Option<String>,

    #[html(selector = "a")]
    links: Vec<Link>,
}

#[test]
fn test_element() {
    let selector = Selector::parse("a").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <a href="https://github.com"> Github </a>
            </div>
            <div>
                <a href="https://www.zjuqsc.com"> ZJU QSC </a>
            </div>
            <a href="https://google.com"> Google </a>
        </div>
    "##,
    );
    assert_eq!(
        Link {
            href: "https://github.com".into(),
            text: "Github".into(),
        },
        html.select(&selector).element().unwrap()
    );
}

#[test]
fn test_vec_element() {
    let selector = Selector::parse("a").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <a href="https://github.com"> Github </a>
            </div>
            <div>
                <a href="https://www.zjuqsc.com"> ZJU QSC </a>
            </div>
            <a href="https://google.com"> Google </a>
        </div>
    "##,
    );

    let actual: Vec<Link> = html.select(&selector).element().unwrap();
    assert_eq!(
        actual,
        vec![
            Link {
                href: "https://github.com".into(),
                text: "Github".into(),
            },
            Link {
                href: "https://www.zjuqsc.com".into(),
                text: "ZJU QSC".into(),
            },
            Link {
                href: "https://google.com".into(),
                text: "Google".into(),
            },
        ]
    );
}

#[test]
fn test_fail_element() {
    let foo_selector = Selector::parse("p").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <a href="https://github.com"> Github </a>
            </div>
            <div>
                <a href="https://www.zjuqsc.com"> ZJU QSC </a>
            </div>
            <a href="https://google.com"> Google </a>
        </div>
    "##,
    );
    let result: Result<Link> = html.select(&foo_selector).element();
    assert!(!result.is_ok());
}

#[test]
fn test_option_element() {
    let selector = Selector::parse("a").unwrap();
    let foo_selector = Selector::parse("p").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <a href="https://github.com"> Github </a>
            </div>
            <div>
                <a href="https://www.zjuqsc.com"> ZJU QSC </a>
            </div>
            <a href="https://google.com"> Google </a>
        </div>
    "##,
    );
    assert_eq!(
        Some(Link {
            href: "https://github.com".into(),
            text: "Github".into(),
        }),
        html.select(&selector).element().unwrap()
    );
    let result: Option<Link> = html.select(&foo_selector).element().unwrap();
    assert_eq!(result, None);
}

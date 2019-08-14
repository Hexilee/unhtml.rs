use crate::{
    scraper::{ElementRef, Html, Selector},
    ElemIter, Element, FromHtml, Result, Text,
};

#[derive(Debug, Eq, PartialEq)]
struct Link {
    href: String,
    text: String,
}

impl FromHtml for Link {
    fn from_elements(select: ElemIter) -> Result<Self> {
        let elements: Vec<ElementRef> = select.collect();
        Ok(Self {
            href: elements.clone().into_iter().attr("href")?,
            text: elements.clone().into_iter().inner_text()?,
        })
    }
}

#[test]
fn test_element() {
    let selector = Selector::parse("a").unwrap();
    let html = Html::parse_fragment(
        r##"
        <div>
            <div>
                <a href="https://github.com"> Github </p>
            </div>
            <div>
                <a href="https://www.zjuqsc.com"> ZJU QSC </p>
            </div>
            <a href="https://google.com"> Google </p>
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
                <a href="https://github.com"> Github </p>
            </div>
            <div>
                <a href="https://www.zjuqsc.com"> ZJU QSC </p>
            </div>
            <a href="https://google.com"> Google </p>
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

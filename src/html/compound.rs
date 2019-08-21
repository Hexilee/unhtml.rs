use super::foo::Link;
use unhtml::FromHtml;

#[derive(FromHtml, Debug, Eq, PartialEq)]
struct Websites {
    #[html(selector = "#current_site")]
    current: Option<Link>,

    #[html(selector = "a")]
    links: Vec<Link>,
}

#[test]
fn test_compound() {
    assert_eq!(
        Websites::from_html("").unwrap(),
        Websites {
            current: None,
            links: vec![],
        }
    );
    assert_eq!(
        Websites::from_html(
            r##"
            <div>
                <div>
                    <a href="https://github.com"> Github </a>
                </div>
                <div>
                    <a id="current_site" href="https://www.zjuqsc.com"> ZJU QSC </a>
                </div>
                <a href="https://google.com"> Google </a>
            </div>
        "##,
        )
        .unwrap(),
        Websites {
            current: Some(Link {
                href: "https://www.zjuqsc.com".into(),
                text: "ZJU QSC".into(),
            }),
            links: vec![
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
        }
    );
}

use unhtml::FromHtml;

#[derive(FromHtml, Debug, Eq, PartialEq)]
#[html(selector = "a")]
pub struct Link {
    #[html(attr = "href")]
    pub href: String,

    #[html(attr = "inner")]
    pub text: String,
}

#[test]
fn root_selector() {
    assert_eq!(
        Link::from_html(
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
        "##
        )
        .unwrap(),
        Link {
            href: "https://github.com".into(),
            text: "Github".into()
        }
    )
}

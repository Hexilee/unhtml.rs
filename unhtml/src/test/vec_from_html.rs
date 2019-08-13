use super::super::*;
use scraper::Html;

#[test]
fn test_from_attr() {
    let html = Html::parse_fragment(
        r#"
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
    "#,
    );
    let results = Vec::<u8>::from_attr("#test > a", "href", html.root_element()).unwrap();
    assert_eq!(1u8, results[0]);
    assert_eq!(2u8, results[1]);
    assert_eq!(3u8, results[2]);
}

#[test]
fn test_from_html_ref() {
    let html = Html::parse_fragment(
        r#"
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
    "#,
    );
    let results = Vec::<String>::from_html_ref("#test > a", html.root_element()).unwrap();
    assert_eq!(r#"<a href="1"></a>"#, results[0]);
    assert_eq!(r#"<a href="2"></a>"#, results[1]);
    assert_eq!(r#"<a href="3"></a>"#, results[2]);
}

#[test]
fn test_from_html() {
    let results = Vec::<String>::from_html(
        "#test > a",
        r#"
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
    "#,
    )
    .unwrap();
    assert_eq!(r#"<a href="1"></a>"#, results[0]);
    assert_eq!(r#"<a href="2"></a>"#, results[1]);
    assert_eq!(r#"<a href="3"></a>"#, results[2]);
}

#[test]
fn test_from_inner_text() {
    let html = Html::parse_fragment(
        r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Title</title>
    </head>
    <body>
        <div id="test">
            <a>1</a>
            <a>2</a>
            <a>3</a>
        </div>
    </body>
    </html>
    "#,
    );
    let results = Vec::<u8>::from_inner_text("#test > a", html.root_element()).unwrap();
    assert_eq!(1u8, results[0]);
    assert_eq!(2u8, results[1]);
    assert_eq!(3u8, results[2]);
}

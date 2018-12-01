use super::super::*;
use scraper::Html;
use scraper::Selector;

#[test]
fn test_from_selector_and_attr() {
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
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test").unwrap();
    let result = u8::from_selector_and_attr("a", "href", html.select(&selector).next().unwrap()).unwrap();
    assert_eq!(1u8, result);
}

#[test]
fn test_from_selector_and_inner_text() {
    let html = Html::parse_fragment(r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <a>1</a>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test").unwrap();
    let result = u8::from_selector_and_inner_text("a", html.select(&selector).next().unwrap()).unwrap();
    assert_eq!(1u8, result);
}

#[test]
fn test_from_selector_and_html() {
    let html = Html::parse_fragment(r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <a>1</a>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test").unwrap();
    let result = String::from_selector_and_html("a", html.select(&selector).next().unwrap()).unwrap();
    assert_eq!("<a>1</a>".to_string(), result);
}

#[test]
fn test_from_attr() {
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
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > a").unwrap();
    let result = u8::from_attr("href", html.select(&selector).next().unwrap()).unwrap();
    assert_eq!(1u8, result);
}

#[test]
fn test_from_inner_text() {
    let html = Html::parse_fragment(r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <a>1</a>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > a").unwrap();
    let result = u8::from_inner_text(html.select(&selector).next().unwrap()).unwrap();
    assert_eq!(1u8, result);
}

#[test]
fn test_from_html_ref() {
    let html = Html::parse_fragment(r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <a>1</a>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > a").unwrap();
    let result = String::from_html_ref(html.select(&selector).next().unwrap()).unwrap();
    assert_eq!("<a>1</a>".to_string(), result);
}
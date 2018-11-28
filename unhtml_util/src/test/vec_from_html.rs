use super::super::*;
use scraper::Html;
use scraper::Selector;

#[test]
fn test_vec_by_selector_and_attr() {
    let html = Html::parse_fragment(r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <div>
            <a href="1"></a>
        </div>
        <div>
            <a href="2"></a>
        </div>
        <div>
            <a href="3"></a>
        </div>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > div").unwrap();
    let results = u8::vec_by_selector_and_attr("a", "href")(html.select(&selector)).unwrap();
    assert_eq!(1u8, results[0]);
    assert_eq!(2u8, results[1]);
    assert_eq!(3u8, results[2]);
}

#[test]
fn test_vec_by_selector_and_inner_text() {
    let html = Html::parse_fragment(r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <div>
            <a>1</a>
        </div>
        <div>
            <a>2</a>
        </div>
        <div>
            <a>3</a>
        </div>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > div").unwrap();
    let results = u8::vec_by_selector_and_inner_text("a")(html.select(&selector)).unwrap();
    assert_eq!(1u8, results[0]);
    assert_eq!(2u8, results[1]);
    assert_eq!(3u8, results[2]);
}

#[test]
fn test_vec_by_selector_and_html() {
    let html = Html::parse_fragment(r#"
    <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <div>
            <a href="1"></a>
        </div>
        <div>
            <a href="2"></a>
        </div>
        <div>
            <a href="3"></a>
        </div>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > div").unwrap();
    let results = String::vec_by_selector_and_html("a")(html.select(&selector)).unwrap();
    assert_eq!(r#"<a href="1"></a>"#, results[0]);
    assert_eq!(r#"<a href="2"></a>"#, results[1]);
    assert_eq!(r#"<a href="3"></a>"#, results[2]);
}

#[test]
fn test_vec_by_attr() {
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
    let selector = Selector::parse("#test > a").unwrap();
    let results = u8::vec_by_attr("href")(html.select(&selector)).unwrap();
    assert_eq!(1u8, results[0]);
    assert_eq!(2u8, results[1]);
    assert_eq!(3u8, results[2]);
}

#[test]
fn test_vec_by_inner_text() {
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
        <a>2</a>
        <a>3</a>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > a").unwrap();
    let results = u8::vec_by_inner_text(html.select(&selector)).unwrap();
    assert_eq!(1u8, results[0]);
    assert_eq!(2u8, results[1]);
    assert_eq!(3u8, results[2]);
}

#[test]
fn test_vec_by_html() {
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
        <a>2</a>
        <a>3</a>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > a").unwrap();
    let results = String::vec_by_html(html.select(&selector)).unwrap();
    assert_eq!("<a>1</a>", results[0]);
    assert_eq!("<a>2</a>", results[1]);
    assert_eq!("<a>3</a>", results[2]);
}

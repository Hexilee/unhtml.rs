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
        <a href="1"></a>
        <a href="2"></a>
        <a href="3"></a>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test").unwrap();
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
        <a>1</a>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test").unwrap();
    let result = u8::get_elem_by_selector_and_inner_text("a")(html.select(&selector).next().unwrap()).unwrap();
    assert_eq!(1u8, result);
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
        <a>1</a>
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test").unwrap();
    let result = String::get_elem_by_selector_and_html("a")(html.select(&selector).next().unwrap()).unwrap();
    assert_eq!("<a>1</a>".to_string(), result);
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
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > a").unwrap();
    let result = u8::get_elem_by_attr("href")(html.select(&selector).next().unwrap()).unwrap();
    assert_eq!(1u8, result);
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
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > a").unwrap();
    let result = u8::get_elem_by_inner_text(html.select(&selector).next().unwrap()).unwrap();
    assert_eq!(1u8, result);
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
    </div>
</body>
</html>
    "#);
    let selector = Selector::parse("#test > a").unwrap();
    let result = String::get_elem_by_html(html.select(&selector).next().unwrap()).unwrap();
    assert_eq!("<a>1</a>".to_string(), result);
}

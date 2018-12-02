use super::{DefaultUser, SingleUser, TestUsers, Link};
use unhtml::{self, FromHtml, VecFromHtml};

#[test]
fn test_default_value() {
    let user = DefaultUser::from_html("<p></p>").unwrap();
    assert_eq!("Hexilee".to_string(), user.name);
    assert_eq!(20, user.age);
    assert_eq!(-1000, user.assets);
    assert!(user.like_lemon);
}

#[test]
fn test_single_user() {
    let user = SingleUser::from_html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <div>
            <p>Hexilee</p>
            <p>20</p>
            <p>true</p>
        </div>
    </div>
</body>
</html>"#).unwrap();
    assert_eq!("Hexilee".to_string(), user.name);
    assert_eq!(20, user.age);
    assert!(user.like_lemon);
}

#[test]
fn test_link() {
    let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
    assert_eq!("https://github.com".to_string(), link.href);
    assert_eq!("Github".to_string(), link.value);
}

#[test]
fn test_users_parse() {
    let users = TestUsers::from_html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Title</title>
</head>
<body>
    <div id="test">
        <div>
            <p>Hexilee</p>
            <p>20</p>
            <p>true</p>
        </div>
        <div>
            <p>BigBrother</p>
            <p>21</p>
            <p>false</p>
        </div>
    </div>
</body>
</html>"#).unwrap();
    let hexilee = &users.users[0];
    let big_brother = &users.users[1];
    assert_eq!("Hexilee".to_string(), hexilee.name);
    assert_eq!(20, hexilee.age);
    assert!(hexilee.like_lemon);
    assert_eq!("BigBrother".to_string(), big_brother.name);
    assert_eq!(21, big_brother.age);
    assert!(!big_brother.like_lemon);
}
#[cfg(test)]
mod basic_usage {
    use unhtml::{self, FromHtml};

    #[derive(FromHtml)]
    #[html(selector = "#test")]
    struct SingleUser {
        #[html(selector = "p:nth-child(1)", attr = "value")]
        name: String,

        #[html(selector = "p:nth-child(2)", attr = "value")]
        age: u8,

        #[html(selector = "p:nth-child(3)", attr = "value")]
        like_lemon: bool,
    }

    #[test]
    fn test_basic_usage() {
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
        assert_eq!("Hexilee", &user.name);
        assert_eq!(20, user.age);
        assert!(user.like_lemon);
    }
}

#[cfg(test)]
mod html_valid {
    use unhtml::{self};

    #[derive(FromHtml)]
    struct SingleString {
        _value: String,
    }
}

#[cfg(test)]
mod select_first {
    use unhtml::{self, FromHtml};

    #[derive(FromHtml)]
    #[html(selector = "a")]
    struct Link {
        #[html(attr = "href")]
        href: String,

        #[html(attr = "value")]
        value: String,
    }

    #[test]
    fn test_select_first() {
        let link = Link::from_html(r#"
<a href="https://github.com">Github</a>
<a href="https://google.com">Google</a>
"#).unwrap();
        assert_eq!("https://github.com", &link.href);
        assert_eq!("Github", &link.value);
    }
}

#[cfg(test)]
mod selector_default_behavior {
    use unhtml::{self, FromHtml};

    #[derive(FromHtml)]
    struct Link {
        #[html(attr = "href")]
        href: String,

        #[html(attr = "value")]
        value: String,
    }

    #[test]
    fn test_selector_default_behavior() {
        let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
        assert_eq!("https://github.com", &link.href);
        assert_eq!("Github", &link.value);
    }
}

#[cfg(test)]
mod attr_default_behavior {
    use unhtml::{self, FromHtml};

    #[derive(FromHtml)]
    struct Link {
        #[html(attr = "href")]
        href: String,

        #[html(attr = "value")]
        value: String,

        source: String,
    }

    #[test]
    fn test_attr_default_behavior() {
        let link = Link::from_html(r#"<a href="https://github.com">Github</a>"#).unwrap();
        assert_eq!("https://github.com", &link.href);
        assert_eq!("Github", &link.value);
        assert_eq!(r#"<a href="https://github.com">Github</a>"#, &link.source);
    }
}

#[cfg(test)]
mod string_default_value {
    use unhtml::{self, FromHtml};

    #[derive(FromHtml)]
    struct Link {
        #[html(attr = "href")]
        href: String,

        #[html(attr = "value")]
        value: String,
    }

    #[derive(FromHtml)]
    struct Website {
        #[html(default = "10")]
        age: u8,

        #[html(default = "<a href='https://github.com'>Github</a>")]
        link: Link,
    }

    #[test]
    fn test_string_default_value() {
        let website = Website::from_html("<p></p>").unwrap();
        let link = website.link;
        assert_eq!(10u8, website.age);
        assert_eq!("https://github.com", &link.href);
        assert_eq!("Github", &link.value);
    }
}

#[cfg(test)]
mod get_vec_straightly {
    use unhtml::{self, VecFromHtml};

    #[derive(FromHtml)]
    struct TestUser {
        #[html(selector = "p:nth-child(1)", attr = "value")]
        name: String,

        #[html(selector = "p:nth-child(2)", attr = "value")]
        age: u8,

        #[html(selector = "p:nth-child(3)", attr = "value")]
        like_lemon: bool,
    }

    #[test]
    fn test_get_vec_straightly() {
        let users = Vec::<TestUser>::from_html("#test > div", r#"<!DOCTYPE html>
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
        let hexilee = &users[0];
        let big_brother = &users[1];
        assert_eq!("Hexilee", &hexilee.name);
        assert_eq!(20, hexilee.age);
        assert!(hexilee.like_lemon);
        assert_eq!("BigBrother", &big_brother.name);
        assert_eq!(21, big_brother.age);
        assert!(!big_brother.like_lemon);
    }
}
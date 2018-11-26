use unhtml::traits::*;
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[derive(UnHtml)]
struct User {
    name: String
}
use super::User;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn from_str_works() {
    let user: User = User::from_str("").unwrap();
    println!("{}", user.name)
}
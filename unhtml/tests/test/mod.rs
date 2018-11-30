use super::DefaultUser;
use unhtml_util::*;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_default_value() {
    let user: DefaultUser = DefaultUser::from_str("").unwrap();
    assert_eq!("Hexilee".to_string(), user.name);
    assert_eq!(20, user.age);
    assert_eq!(-1000, user.assets);
    assert!(user.like_lemon);
}
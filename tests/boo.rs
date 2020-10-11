#[macro_use]
extern crate shoogah;

#[test]
fn basic_boo() {
    let t = 1;
    let f = 0;
    assert_eq!(true, boo! { t });
    assert_eq!(false, boo! { f });

    let t = vec![1, 2, 3];
    let f: Vec<i32> = vec![];
    assert_eq!(true, boo! { t });
    assert_eq!(false, boo! { f });

    let t = hml![1: 1, 2: 2, 3: 3];
    let f: std::collections::HashMap<i32, i32> = hml![:];
    assert_eq!(true, boo! { t });
    assert_eq!(false, boo! { f });
}

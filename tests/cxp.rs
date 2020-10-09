#[macro_use]
extern crate shoogah;

#[test]
fn simple_cxp() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = cxp! { (a + b > c) ? (a) : (b) };
    assert_eq!(d, 2);

    let a = 1;
    let b = 2;
    let c = 3;
    let d = cxp! { !(a + b > c) ? (a) : (b) };
    assert_eq!(d, 1);
}

#[test]
fn simple_elv() {
    let a = vec![1, 2, 3];
    let b = vec![];
    let d = elv! { (a) ?: (b) };
    assert_eq!(d.len(), 3);

    let a = vec![1, 2, 3];
    let b = vec![];
    let d = elv! { !(a) ?: (b) };
    assert_eq!(d.len(), 0);
}

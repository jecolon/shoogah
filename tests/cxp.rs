#[macro_use]
extern crate shoogah;

#[test]
fn basic_cxp() {
    let input = "José";
    let username = cxp! { (input) ? (input) : ("Unknown") };
    assert_eq!(username, input);

    let input = "José";
    let username = cxp! { !(input) ? (input) : ("Unknown") };
    assert_eq!(username, "Unknown");

    let input = "";
    let username = cxp! { (input) ? (input) : ("Unknown") };
    assert_eq!(username, "Unknown");
}

#[test]
fn elvis() {
    let a = vec![1, 2, 3];
    let b = vec![4, 56];
    let d = elv! { (a) ?: (b) };
    assert_eq!(d.len(), 3);

    let a = vec![1, 2, 3];
    let b = vec![];
    let d = elv! { !(a) ?: (b) };
    assert_eq!(d.len(), 0);

    let a = vec![];
    let b = vec![1, 2, 3];
    let d = elv! { (a) ?: (b) };
    assert_eq!(d.len(), 3);
}

#[test]
fn elvis_assign() {
    let mut a = vec![1, 2, 3];
    let b = vec![4, 5];
    ela! { a ?= b };
    assert_eq!(a.len(), 3);

    let mut a = vec![1, 2, 3];
    let b = vec![4, 5];
    ela! { !a ?= b };
    assert_eq!(a.len(), 2);

    let mut a = vec![];
    let b = vec![4, 56];
    ela! { a ?= b };
    assert_eq!(a.len(), 2);
}

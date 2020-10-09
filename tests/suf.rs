#[macro_use]
extern crate shoogah;

#[test]
fn inc_dec() {
    let mut x = 1;
    assert_eq!(2, suf! { x++ });
    assert_eq!(1, suf! { x-- });
}

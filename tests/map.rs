#[macro_use]
extern crate shoogah;

#[test]
fn empty_map() {
    let mut m = map![:];
    m.insert("a", 1);
    assert_eq!(m.len(), 1);
}

#[test]
fn map_with_entries() {
    let m = map!["a": 1, "b": 2, "c": 3];
    assert_eq!(m.len(), 3);
}

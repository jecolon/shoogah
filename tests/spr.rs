#[macro_use]
extern crate shoogah;

#[derive(Clone)]
struct Struct {
    x: u8,
}

#[test]
fn basic_spread() {
    let a = Struct { x: 1 };
    let b = Struct { x: 2 };
    let c = Struct { x: 3 };
    let v = vec![a, b, c];
    let r: Vec<u8> = spr! { (v)*.x };
    assert_eq!(vec![1, 2, 3], r);
}

#[derive(Clone)]
struct Address<'a> {
    country: &'a str,
}

#[derive(Clone)]
struct Customer<'c> {
    name: &'c str,
    address: Address<'c>,
}

#[test]
fn spread_chain() {
    let customers = vec![
        Customer{ name: "Carlos", address: Address{ country: "Spain" }},
        Customer{ name: "Johnathan", address: Address{ country: "United Kingdom" }},
        Customer{ name: "Enzo", address: Address{ country: "Italy" }},
    ];
    let countries: Vec<_> = spr! { (customers)*.address*.country };
    assert_eq!(vec!["Spain", "United Kingdom", "Italy"], countries);
}

#[test]
fn literal_spread() {
    let r: Vec<_> = spr!{
        (vec![
            Struct { x: 1 },
            Struct { x: 2 },
            Struct { x: 3 },
        ])*.x
    };
    assert_eq!(vec![1, 2, 3], r);
}

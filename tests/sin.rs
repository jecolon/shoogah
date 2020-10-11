#[macro_use]
extern crate shoogah;

#[test]
fn interpolate() {
    let what = "World!";
    let msg = sin!("Hello, ${what}");
    assert_eq!("Hello, World!", msg);
}

#[test]
fn empty_interpolate() {
    let msg = sin!("");
    assert_eq!("", msg);
}

#[test]
fn multi_interpolate() {
    let what = "World";
    let when = "Now!";
    let msg = sin!("Hello, ${what} ${when}");
    assert_eq!("Hello, World Now!", msg);
}

#[test]
fn dollar_interpolate() {
    let what = "World!";
    let msg = sin!("Hello, $100 $${what}");
    assert_eq!("Hello, $100 $World!", msg);
}

#[test]
fn expr_interpolate() {
    let msg = sin!("Hello, ${ 1 + 1 }");
    assert_eq!("Hello, 2", msg);
}

struct Foo();
impl Foo {
    fn bar() -> &'static str {
        "Bar!"
    }
}

#[test]
fn method_interpolate() {
    let msg = sin!("Hello, ${ Foo::bar() }");
    assert_eq!("Hello, Bar!", msg);
}

#[test]
fn raw_interpolate() {
    let msg = sin!(r#"Hello, ${ "hello" }"#);
    assert_eq!("Hello, hello", msg);
}

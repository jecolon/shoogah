# shoogah
A crate with all sorts of syntactic sugar for Rust. Many of the
items are inspired from the goodness of other languages, especially *Groovy*.
Some operations require an expanded notion of what is *true* and what is *false*.
In these cases, we make use of the `AsBool` trait. Any type that implements
`AsBool`, will work with `shoogah`.

# Easy HashMap literals with the hml! macro
Define a `std::collections::HashMap` via a simple literal.
```
    #[macro_use] extern crate shoogah;
    let my_map = hml! [
        "a": 1,
        "b": 2,
        "c": 1 + 2,
    ];
```
In this example, `my_map` is of type `std::collections::HashMap<&str, i32>`.
To create an empty map:
```
    #[macro_use] extern crate shoogah;
    let mut my_map = hml![:];
    my_map.insert("a", 1);
```
Note that in the case of an empty map declaration like this one, only after
you insert an entry will the map have its type inferred. So if you try to use
the empty map before inserting any entries, you'll get a compiler error. If
your use case requires the empty map, add type annotations to the left hand
side like this:
```
    #[macro_use] extern crate shoogah;
    use std::collections::HashMap;
    let mut my_map: HashMap<&str, u8> = hml![:];
```
Map keys can be identifiers (variable names) or lietrals like `1` or `"Hello"`.
Map values can be any type of expression.

# Compact conditional expressions with the cxp! macro
```
    #[macro_use] extern crate shoogah;
    let x = "";
    let username = cxp!{ (x) ? (x) : ("Bytor") }; // username assigned "Bytor"
```
Given how complex expressions can be, the parentheses are required.

# Elvis says: "Don't Repeat Yourself"; elv! macro
```
    #[macro_use] extern crate shoogah;
    let x = "Cygnus";
    let username = elv!{ (x) ?: ("Bytor") }; // username remains "Cygnus"
```

# Elvis says: "Don't Repeat Yourself... again"; ela! macro
If the assigned-to variable is the condition being tested, the Elvis
assignment macro (ela!) is for you.
```
    #[macro_use] extern crate shoogah;
    let mut username = "";
    ela!{ username ?= "Bytor" }; // username is now "Bytor"
```

# Simple increment and decrement with the suf! macro
```
    #[macro_use] extern crate shoogah;
    let mut x = 1;
    assert_eq!(2, suf!{ x++ });
    assert_eq!(1, suf!{ x-- });
```

# Collect common field values from an `Iterator` with the spr! macro
```
    #[macro_use] extern crate shoogah;
    #[derive(Clone)]
    struct Address<'a> {
        country: &'a str,
    }

    #[derive(Clone)]
    struct Customer<'c> {
        name: &'c str,
        address: Address<'c>,
    }

   let customers = vec![
       Customer{ name: "Carlos", address: Address{ country: "Spain" }},
       Customer{ name: "Johnathan", address: Address{ country: "United Kingdom" }},
       Customer{ name: "Enzo", address: Address{ country: "Italy" }},
   ];
   let countries: Vec<_> = spr! { (customers)*.address*.country };
   assert_eq!(vec!["Spain", "United Kingdom", "Italy"], countries);
```
Note that the operation requires collections that implement `Iterator` and 
items that implement `Clone`, given they are moved out of the original. Also
note that parentheses are required for the first expression, allowing for 
chaining and literals as the initial collection.

# It's all still Rust under the hood
All these macros expand into normal Rust code, so the usual syntax and type
requirements will apply to variable names, literals, and expressions that you
use.

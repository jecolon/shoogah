//! shoogah is a crate with all sorts of syntactic sugar for Rust. Many of the
//! items are inspired from the goodness of other languages, especially *Groovy*.
//! Some operations require an expanded notion of what is *true* and what is *false*.
//! In these cases, we make use of the `AsBool` trait. Any type that implements
//! `AsBool`, will work with `shoogah`.
//!
//! # Easy HashMap literals with the hml! macro
//! Define a `std::collections::HashMap` via a simple literal.
//! ```
//!     # #[macro_use] extern crate shoogah;
//!     let my_map = hml! [
//!         "a": 1,
//!         "b": 2,
//!         "c": 1 + 2,
//!     ];
//! ```
//! In this example, `my_map` is of type `std::collections::HashMap<&str, i32>`.
//! To create an empty map:
//! ```
//!     # #[macro_use] extern crate shoogah;
//!     let mut my_map = hml![:];
//!     my_map.insert("a", 1);
//! ```
//! Map keys can be identifiers (variable names) or lietral expressions like `1`
//! or `"Hello"`. Map values are expressions.
//!
//! # Compact conditional expressions with the cxp! macro
//! ```
//!     # #[macro_use] extern crate shoogah;
//!     let x = "";
//!     let username = cxp!{ (x) ? (x) : ("Bytor") }; // username assigned "Bytor"
//! ```
//! Given how complex expressions can be, the parentheses are required.
//!
//! # Elvis says: "Don't Repeat Yourself"; elv! macro
//! ```
//!     # #[macro_use] extern crate shoogah;
//!     let x = "Cygnus";
//!     let username = elv!{ (x) ?: ("Bytor") }; // username remains "Cygnus"
//! ```
//!
//! # Elvis says: "Don't Repeat Yourself... again"; ela! macro
//! If the assigned-to variable is the condition being tested, the Elvis
//! assignment macro (ela!) is for you.
//! ```
//!     # #[macro_use] extern crate shoogah;
//!     let mut username = "";
//!     ela!{ (username) ?= ("Bytor") }; // username is now "Bytor"
//! ```
//!
//! # Simple increment and decrement with the suf! macro
//! ```
//!     # #[macro_use] extern crate shoogah;
//!     let mut x = 1;
//!     assert_eq!(2, suf!{ x++ });
//!     assert_eq!(1, suf!{ x-- });
//! ```
//!
//! # It's all still Rust under the hood
//! All these macros expand into normal Rust code, so the usual syntax and type
//! requirements will apply to variable names, literals, and expressions that you
//! use.

pub use as_bool::AsBool;
pub use shoogah_macros::*;

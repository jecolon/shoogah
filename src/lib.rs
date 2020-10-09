//! shoogah is a crate with all sorts of syntactic sugar for Rust. Many of the
//! items are inspired from the goodness of other languages, especially *Groovy*.
//! Some operations require an expanded notion of what is *true* and what is *false*.
//! In these cases, we make use of the `AsBool` trait from the `as_bool` crate.
//! Any type that implements `AsBool`, will work with the macros in `shoogah`
//! that require such *truthiness*.

mod cxp;
mod map;
mod suf;

use cxp::{cxp_impl, ela_impl, elv_impl};
use map::map_impl;
use proc_macro::TokenStream;
use suf::suf_impl;

/// map lets you define a `std::collections::HashMap` via a simple literal.
///
/// For example:
/// ```ignore
///     let my_map = map! [
///         "a": 1,
///         "b": 2,
///         "c": 1 + 2,
///     ];
/// ```
/// In this example, `my_map` is of type `std::collections::HashMap<&str, i32>`.
/// To create an empty map:
/// ```ignore
///     let my_map = map![:];
/// ```
/// Map keys can be identifiers (variable names) or lietral expressions like `1`
/// or `"Hello"`. Map values are expressions. The returned value is a
/// `std::collections::HashMap` and thus all normal type checking rules and
/// trait requirements apply.
#[proc_macro]
pub fn map(input: TokenStream) -> TokenStream {
    map_impl(input)
}

/// cxp lets you express an if/else in a shorthand manner. This is sometimes
/// called the *ternary* operator in other languages.
///
/// For example:
/// ```ignore
///     let username = cxp!{ (3 > 4) ? ("a") : ("b") };
/// ```
/// This expands to a normal Rust if/else expression, so Rust syntax and type
/// rules apply to the expressions. Given how complex expressions can be, the
/// parentheses are required.
#[proc_macro]
pub fn cxp(input: TokenStream) -> TokenStream {
    cxp_impl(input)
}

/// elv (Elvis!) lets you express an if/else in a manner shorter than the cxp
/// macro when the result is the same as the condition.
///
/// For example:
/// ```ignore
///     let username = elv!{ ("José") ?: ("Unknown") };
/// ```
/// This expands to a normal Rust if/else expression, so Rust syntax and type
/// rules apply to the expressions. Given how complex expressions can be, the
/// parentheses are required.
#[proc_macro]
pub fn elv(input: TokenStream) -> TokenStream {
    elv_impl(input)
}

/// ela lets you express an if/else in a manner shorter than the cxp and elv
/// macros when the result and assigned-to variable is the same as the condition.
/// In other words, assignment only occurs if the left hand side evaluates to false.
///
/// For example:
/// ```ignore
///     let mut username = "";
///     ela!{ (username) ?= ("José")) }; // username == "José"
/// ```
/// This expands to a normal Rust if/else expression, so Rust syntax and type
/// rules apply to the expressions. Given how complex expressions can be, the
/// parentheses are required.
#[proc_macro]
pub fn ela(input: TokenStream) -> TokenStream {
    ela_impl(input)
}

/// suf is shorthand for incrementing or decrementing a number by 1.
///
/// For example:
/// ```ignore
///     let mut x = 1;
///     assert_eq!(2, suf!{ x++ });
///     assert_eq!(1, suf!{ x-- });
/// ```
/// This expands to a normal Rust expresion like x += 1, so normal type rules apply.
#[proc_macro]
pub fn suf(input: TokenStream) -> TokenStream {
    suf_impl(input)
}

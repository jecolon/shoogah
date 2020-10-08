//! shoogah is a crate with all sorts of syntactic sugar for Rust. Many of the
//! items are inspired from the goodness of other languages, especially *Groovy*.

mod map;

use map::map_impl;
use proc_macro::TokenStream;

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

//! shoogah is a crate with all sorts of syntactic sugar for Rust. Many of the
//! items are inspired from the goodness of other languages, especially *Groovy*.
//! Some operations require an expanded notion of what is *true* and what is *false*.
//! In these cases, we make use of the `AsBool` trait from the `as_bool` crate.
//! Any type that implements `AsBool`, will work with the macros in `shoogah`
//! that require such *truthiness*.

pub use as_bool::AsBool;
pub use shoogah_macros::*;

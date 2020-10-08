//! shoogah is a crate with all sorts of syntactic sugar for Rust. Many of the
//! items are inspired from the goodness of other languages, especially *Groovy*.

mod map;

use map::{Key, MapEntry, MapLiteral};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

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
    let MapLiteral { entries } = parse_macro_input!(input as MapLiteral);

    if let Some(entries) = entries {
        // Map with entries.
        let mut inserts = vec![];
        for MapEntry { key, value } in entries {
            match key {
                Key::Variable(ident) => {
                    inserts.push(quote! {
                        temp_map.insert(#ident, #value);
                    });
                }
                Key::Literal(lit) => {
                    inserts.push(quote! {
                        temp_map.insert(#lit, #value);
                    });
                }
            }
        }

        let capacity = inserts.len();
        TokenStream::from(quote! {
            {
                let mut temp_map = std::collections::HashMap::with_capacity(#capacity);
                #(#inserts)*
                temp_map
            }
        })
    } else {
        // Empty map.
        TokenStream::from(quote! {
            std::collections::HashMap::new()
        })
    }
}

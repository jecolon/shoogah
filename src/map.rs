use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Expr, Ident, Lit, Token};

// `Key` can only be an identifier or literal.
enum Key {
    Variable(Ident),
    Literal(Lit),
}

impl Parse for Key {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Ident) {
            input.parse().map(Key::Variable)
        } else if input.peek(Lit) {
            input.parse().map(Key::Literal)
        } else {
            Err(input.error("expected identifier or literal"))
        }
    }
}

// `MapEntry` has a key (type `Key`) and a value (any expression) separated by
// a colon.
struct MapEntry {
    key: Key,
    value: Expr,
}

impl Parse for MapEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Key = input.parse()?;
        input.parse::<Token![:]>()?;
        let value: Expr = input.parse()?;
        Ok(MapEntry { key, value })
    }
}

// MapLiteral is a sequence of `MapEntry` separated by commas. `[:]` is an empty
// map.
struct MapLiteral {
    entries: Option<Punctuated<MapEntry, Token![,]>>,
}

impl Parse for MapLiteral {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut entries: Option<Punctuated<MapEntry, Token![,]>> = None;
        if input.peek(Token![:]) {
            // Empty map.
            input.parse::<Token![:]>()?;
        } else {
            entries = Some(input.parse_terminated(MapEntry::parse)?);
        }
        Ok(MapLiteral { entries })
    }
}

// map macro implementation.
pub fn map_impl(input: TokenStream) -> TokenStream {
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

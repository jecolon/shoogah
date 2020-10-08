use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, Lit, Token};

// `Key` can only be an identifier or literal.
pub enum Key {
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
pub struct MapEntry {
    pub key: Key,
    pub value: Expr,
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
pub struct MapLiteral {
    pub entries: Option<Punctuated<MapEntry, Token![,]>>,
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

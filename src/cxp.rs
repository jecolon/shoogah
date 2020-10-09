use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{custom_punctuation, parenthesized, parse_macro_input, Expr, Token};

// CondExpr is a shorthand form of the if/else expresion.
struct CondExpr {
    condition: Expr,
    result: Expr,
    alternative: Expr,
    negated: bool,
}

impl Parse for CondExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut negated = false;
        if input.peek(Token![!]) {
            negated = true;
            input.parse::<Token![!]>()?;
        }
        let mut content;
        parenthesized!(content in input);
        let condition: Expr = content.parse()?;
        input.parse::<Token![?]>()?;
        parenthesized!(content in input);
        let result: Expr = content.parse()?;
        input.parse::<Token![:]>()?;
        parenthesized!(content in input);
        let alternative: Expr = content.parse()?;

        Ok(CondExpr {
            condition,
            result,
            alternative,
            negated,
        })
    }
}

// cxp macro implementation.
pub fn cxp_impl(input: TokenStream) -> TokenStream {
    let CondExpr {
        condition,
        result,
        alternative,
        negated,
    } = parse_macro_input!(input as CondExpr);

    if negated {
        TokenStream::from(quote! {{
           use as_bool::AsBool;

           if !((#condition).as_bool()) {
               #result
           } else {
               #alternative
           }
        }})
    } else {
        TokenStream::from(quote! {{
           use as_bool::AsBool;

           if (#condition).as_bool() {
               #result
           } else {
               #alternative
           }
        }})
    }
}

// Ladies and gentelmen, Elvis has entered the building!
custom_punctuation!(Elvis, ?:);
custom_punctuation!(ElvisAssignment, ?=);

// ElvisExpr is a shorthand form of the if/else expresion where the condition and
// result are the same.
struct ElvisExpr {
    condition: Expr,
    alternative: Expr,
    negated: bool,
}

impl Parse for ElvisExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut negated = false;
        if input.peek(Token![!]) {
            negated = true;
            input.parse::<Token![!]>()?;
        }
        let mut content;
        parenthesized!(content in input);
        let condition: Expr = content.parse()?;
        input.parse::<Elvis>()?;
        parenthesized!(content in input);
        let alternative: Expr = content.parse()?;

        Ok(ElvisExpr {
            condition,
            alternative,
            negated,
        })
    }
}

// elv macro implementation.
pub fn elv_impl(input: TokenStream) -> TokenStream {
    let ElvisExpr {
        condition,
        alternative,
        negated,
    } = parse_macro_input!(input as ElvisExpr);

    if negated {
        TokenStream::from(quote! {{
           use as_bool::AsBool;

           if !((#condition).as_bool()) {
               #condition
           } else {
               #alternative
           }
        }})
    } else {
        TokenStream::from(quote! {{
           use as_bool::AsBool;

           if (#condition).as_bool() {
               #condition
           } else {
               #alternative
           }
        }})
    }
}

// ElvisAssign is a shorthand form of the if/else expresion where the condition,
// result, and assigned-to variable are the same. In other words, assignment only
// occurs if the left hand side evaluates to false.
struct ElvisAssign {
    condition: Expr,
    alternative: Expr,
    negated: bool,
}

impl Parse for ElvisAssign {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut negated = false;
        if input.peek(Token![!]) {
            negated = true;
            input.parse::<Token![!]>()?;
        }
        let mut content;
        parenthesized!(content in input);
        let condition: Expr = content.parse()?;
        input.parse::<ElvisAssignment>()?;
        parenthesized!(content in input);
        let alternative: Expr = content.parse()?;

        Ok(ElvisAssign {
            condition,
            alternative,
            negated,
        })
    }
}

// ela macro implementation.
pub fn ela_impl(input: TokenStream) -> TokenStream {
    let ElvisAssign {
        condition,
        alternative,
        negated,
    } = parse_macro_input!(input as ElvisAssign);

    if negated {
        TokenStream::from(quote! {{
           use as_bool::AsBool;

           if (#condition).as_bool() {
               #condition = #alternative;
           }
        }})
    } else {
        TokenStream::from(quote! {{
           use as_bool::AsBool;

           if !((#condition).as_bool()) {
               #condition = #alternative;
           }
        }})
    }
}

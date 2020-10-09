use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{custom_punctuation, parse_macro_input, Ident};

custom_punctuation!(Increment, ++);
custom_punctuation!(Decrement, --);

#[derive(PartialEq)]
enum Op {
    Inc,
    Dec,
}

// IncDec is a shorthand form of incrementing or decrementing a number by 1.
struct IncDec {
    name: Ident,
    op: Op,
}

impl Parse for IncDec {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let mut op = Op::Inc;
        if input.peek(Increment) {
            input.parse::<Increment>()?;
        } else {
            op = Op::Dec;
            input.parse::<Decrement>()?;
        }

        Ok(IncDec { name, op })
    }
}

// suf macro implementation.
pub fn suf_impl(input: TokenStream) -> TokenStream {
    let IncDec { name, op } = parse_macro_input!(input as IncDec);

    if op == Op::Inc {
        TokenStream::from(quote! {{
            #name += 1;
            #name
        }})
    } else {
        TokenStream::from(quote! {{
            #name -= 1;
            #name
        }})
    }
}

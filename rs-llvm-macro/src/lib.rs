#![allow(dead_code)]
extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use syn::{parse_macro_input, Item, LitInt};
use quote::quote;
use syn::parse::{Parse, ParseStream};

static LLVM_VERSION: [&str; 17] = [
    "",
    "",
    "",
    "llvm-4",
    "llvm-5",
    "llvm-6",
    "llvm-7",
    "llvm-8",
    "llvm-9",
    "llvm-10",
    "llvm-11",
    "llvm-12",
    "llvm-13",
    "llvm-14",
    "llvm-15",
    "llvm-16",
    "llvm-17",
];



#[derive(Debug)]
enum FeatureSet {
    Range(syn::LitInt, syn::LitInt),
    InclusiveRange(syn::LitInt, syn::LitInt),
    Latest,
    RangeToLatest(syn::LitInt),
    Single(syn::LitInt),
}

impl Parse for FeatureSet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // syntax is llvm_version(<start>..[=<end>] | latest | <start>.. | <start>)
        let lookahead = input.lookahead1();
        if lookahead.peek(LitInt) {
            let start = input.parse::<syn::LitInt>()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::token::DotDot) {
                input.parse::<syn::Token![..]>()?;
                let lookahead = input.lookahead1();
                if lookahead.peek(syn::Lit) {
                    let end = input.parse::<syn::LitInt>()?;
                    Ok(FeatureSet::Range(start, end))
                } else if lookahead.peek(syn::token::Eq) {
                    input.parse::<syn::Token![=]>()?;
                    let lookahead = input.lookahead1();
                    if lookahead.peek(syn::Lit) {
                        let end = input.parse::<syn::LitInt>()?;
                        Ok(FeatureSet::InclusiveRange(start, end))
                    } else {
                        Err(lookahead.error())
                    }
                } else {
                    Err(lookahead.error())
                }
            } else {
                Ok(FeatureSet::Single(start))
            }
        }
        else if lookahead.peek(syn::Ident) {
            let ident = input.parse::<syn::Ident>()?;
            if ident == "latest" {
                Ok(FeatureSet::Latest)
            } else {
                Err(syn::Error::new(ident.span(), "Expected `latest`"))
            }
        } else {
            Err(lookahead.error())
        }

    }

}


#[proc_macro_attribute]
pub fn llvm_version(attr: TokenStream, input: TokenStream) -> TokenStream {
    let feature_set = parse_macro_input!(attr as FeatureSet);
    let input = parse_macro_input!(input as Item);
    // one cfg with all attr
    let features_condition = match feature_set {
        FeatureSet::Latest => {
            let feature = LLVM_VERSION[LLVM_VERSION.len() - 1];
            quote! { feature =  #feature }
        },
        FeatureSet::Range(start, end) => {
            let start = start.base10_parse::<usize>().unwrap() - 1;
            let end = end.base10_parse::<usize>().unwrap() - 1;
            let mut features_condition = TokenStream2::new();
            for i in start..=end {
                let feature = LLVM_VERSION[i];
                features_condition.extend(quote! { feature = #feature });
                if i != end {
                    features_condition.extend(quote! { , });
                }
            }
            features_condition
        },
        FeatureSet::InclusiveRange(start, end) => {
            let start = start.base10_parse::<usize>().unwrap() - 1;
            let end = end.base10_parse::<usize>().unwrap() - 1;
            let mut features_condition = TokenStream2::new();
            for i in start..=end {
                let feature = LLVM_VERSION[i];
                features_condition.extend(quote! { feature = #feature });
                if i != end {
                    features_condition.extend(quote! { , });
                }
            }
            features_condition
        },
        FeatureSet::RangeToLatest(start) => {
            let start = start.base10_parse::<usize>().unwrap() - 1;
            let mut features_condition = TokenStream2::new();
            for i in start..LLVM_VERSION.len() {
                let feature = LLVM_VERSION[i];
                features_condition.extend(quote! { feature = #feature });
                if i != LLVM_VERSION.len() - 1 {
                    features_condition.extend(quote! { , });
                }
            }
            features_condition
        },

        FeatureSet::Single(start) => {
            let start = start.base10_parse::<usize>().unwrap() - 1;
            let feature = LLVM_VERSION[start];
            quote! { feature = #feature }
        },

    };

    let output = quote! {
        #[cfg(any(#features_condition))]
        #input
    };


    output.into()


}

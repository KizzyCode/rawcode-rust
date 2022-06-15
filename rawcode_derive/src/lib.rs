#![recursion_limit = "128"]

#[macro_use]
extern crate quote;

mod derive;

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Field, Fields, Ident};

/// Implements `rawcode::coding::RawcodeConstSize` and
/// `rawcode::coding::RawcodeEncode` + `rawcode::coding::RawcodeDecode`
#[proc_macro_derive(Rawcode)]
pub fn rawcode_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = syn::parse(input).expect("Failed to parse the input token stream");

    // Unwrap the structure from an input
    let ty_struct = match input.data {
        Data::Struct(ty_struct) => ty_struct,
        _ => panic!("Rawcode supports named structs only"),
    };

    // Digest struct
    let (fields, implementor): (_, fn(&Ident, &[Field]) -> TokenStream) = match ty_struct.fields {
        Fields::Named(fields) => (fields.named, derive::named::impl_all),
        Fields::Unnamed(fields) => (fields.unnamed, derive::unnamed::impl_all),
        _ => panic!("Rawcode supports non-unit fields only"),
    };

    // Derive impl
    let fields: Vec<_> = fields.into_iter().collect();
    implementor(&input.ident, &fields)
}

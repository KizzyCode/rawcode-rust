use proc_macro::TokenStream;
use syn::{Field, Generics, Ident, Index};

/// Implements `RawcodeConstSize` for `ty`
fn impl_const_size(ty: &Ident, generics: &Generics, fields: &[Field]) -> TokenStream {
    let fields_ty = fields.iter().map(|field| &field.ty);
    let where_clause = generics.where_clause.as_ref();

    let implementation = quote! {
        impl #generics ::rawcode::coding::RawcodeConstSize for #ty #generics #where_clause {
            const SIZE: usize = 0 #( + <#fields_ty>::SIZE )*;
        }
    };
    TokenStream::from(implementation)
}

/// Implementors for named structs
pub mod named {
    use super::*;

    /// Implements `RawcodeDecode` for `ty` where `ty` is a named struct
    fn impl_decode(ty: &Ident, generics: &Generics, fields: &[Field]) -> TokenStream {
        let fields_name = fields.iter().map(|field| &field.ident);
        let where_clause = generics.where_clause.as_ref();

        let implementation = quote! {
            impl #generics ::rawcode::coding::RawcodeDecode for #ty #generics #where_clause {
                fn decode(buf: &[u8]) -> ::std::result::Result<Self, ::rawcode::error::Error> {
                    // Decode all fields
                    let mut pos = 0;
                    let this = Self { #( #fields_name: ::rawcode::coding::from_slice_at(buf, &mut pos)?, )* };
                    Ok(this)
                }
            }
        };
        TokenStream::from(implementation)
    }

    /// Implements `RawcodeEncode` for `ty` where `ty` is a tuple struct
    fn impl_encode(ty: &Ident, generics: &Generics, fields: &[Field]) -> TokenStream {
        let fields_name = fields.iter().map(|field| &field.ident);
        let where_clause = generics.where_clause.as_ref();

        let implementation = quote! {
            impl #generics ::rawcode::coding::RawcodeEncode for #ty #generics #where_clause {
                fn encode(&self, buf: &mut [u8]) -> ::std::result::Result<(), ::rawcode::error::Error> {
                    let mut pos = 0;
                    #( ::rawcode::coding::to_slice_at(&self.#fields_name, buf, &mut pos)?; )*
                    Ok(())
                }
            }
        };
        TokenStream::from(implementation)
    }

    /// Implements the `Rawcode` traits for `ty` where `ty` is a named struct
    pub fn impl_all(ty: &Ident, generics: &Generics, fields: &[Field]) -> TokenStream {
        let const_sized = impl_const_size(ty, generics, fields);
        let decode = impl_decode(ty, generics, fields);
        let encode = impl_encode(ty, generics, fields);
        TokenStream::from_iter([const_sized, decode, encode])
    }
}

/// Implementors for unnamed structs
pub mod unnamed {
    use super::*;

    /// Implements `RawcodeDecode` for `ty` where `ty` is a tuple struct
    fn impl_decode(ty: &Ident, generics: &Generics, fields: &[Field]) -> TokenStream {
        let fields_ty = fields.iter().map(|field| &field.ty);
        let where_clause = generics.where_clause.as_ref();

        let implementation = quote! {
            impl #generics ::rawcode::coding::RawcodeDecode for #ty #generics #where_clause {
                fn decode(buf: &[u8]) -> ::std::result::Result<Self, ::rawcode::error::Error> {
                    // Decode all fields
                    let mut pos = 0;
                    let this = Self( #( ::rawcode::coding::from_slice_at::<#fields_ty>(buf, &mut pos)?, )* );
                    Ok(this)
                }
            }
        };
        TokenStream::from(implementation)
    }

    /// Implements `RawcodeEncode` for `ty` where `ty` is a tuple struct
    fn impl_encode(ty: &Ident, generics: &Generics, fields: &[Field]) -> TokenStream {
        let fields_index = fields.iter().enumerate().map(|(index, _)| Index::from(index));
        let where_clause = generics.where_clause.as_ref();

        let implementation = quote! {
            impl #generics ::rawcode::coding::RawcodeEncode for #ty #generics #where_clause {
                fn encode(&self, buf: &mut [u8]) -> ::std::result::Result<(), ::rawcode::error::Error> {
                    let mut pos = 0;
                    #( ::rawcode::coding::to_slice_at(&self.#fields_index, buf, &mut pos)?; )*
                    Ok(())
                }
            }
        };
        TokenStream::from(implementation)
    }

    /// Implements the `Rawcode` traits for `ty` where `ty` is an unnamed struct
    pub fn impl_all(ty: &Ident, generics: &Generics, fields: &[Field]) -> TokenStream {
        let const_sized = impl_const_size(ty, generics, fields);
        let decode = impl_decode(ty, generics, fields);
        let encode = impl_encode(ty, generics, fields);
        TokenStream::from_iter([const_sized, decode, encode])
    }
}

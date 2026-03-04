use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Fields, Type, Variant, parse_macro_input, punctuated::Punctuated,
    token::Comma,
};

#[proc_macro_derive(ByteCodec)]
pub fn derive_byte_codec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = &input.ident;

    let expanded = match &input.data {
        Data::Enum(data_enum) => {
            let variants = &data_enum.variants;
            let max_size_impl = build_enum_max_serialized_size(variants);
            let serialize_impl = build_serialize(type_name, variants);
            let deserialize_impl = build_deserialize(type_name, variants);

            quote! {
                impl liquidcan_rust_macros::byte_codec::ByteCodec for #type_name {
                    #max_size_impl
                    #serialize_impl
                    #deserialize_impl
                }
            }
        }
        Data::Struct(data_struct) => {
            let max_size_impl = build_struct_max_serialized_size(&data_struct.fields);
            let serialize_impl = build_struct_serialize(type_name, &data_struct.fields);
            let deserialize_impl = build_struct_deserialize(type_name, &data_struct.fields);

            quote! {
                impl liquidcan_rust_macros::byte_codec::ByteCodec for #type_name {
                    #max_size_impl
                    #serialize_impl
                    #deserialize_impl
                }
            }
        }
        Data::Union(_) => panic!("ByteCodec cannot be derived for unions"),
    };

    TokenStream::from(expanded)
}

fn build_serialize(
    enum_name: &syn::Ident,
    variants: &Punctuated<Variant, Comma>,
) -> proc_macro2::TokenStream {
    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let (_, variant_discriminant) = variant
            .discriminant
            .as_ref()
            .expect("Must explicitly specify discriminant");

        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

                quote! {
                    #enum_name::#variant_name { #(#field_names),* } => {
                        out.push(#variant_discriminant);
                        #( #field_names.serialize(out); )*
                    }
                }
            }
            Fields::Unnamed(fields) => {
                // Generate dummy identifiers for the tuple fields (e.g., f0, f1, f2)
                let field_idents: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| format_ident!("f{}", i))
                    .collect();

                quote! {
                    #enum_name::#variant_name( #(#field_idents),* ) => {
                        out.push(#variant_discriminant);
                        #( #field_idents.serialize(out); )*
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    #enum_name::#variant_name => {
                        out.push(#variant_discriminant);
                    }
                }
            }
        }
    });

    let expanded = quote! { fn serialize(&self, out: &mut Vec<u8>) {
        let out_len_before = out.len();
        match self {
            #(#match_arms)*
        }
        assert!(out.len() - out_len_before <= Self::MAX_SERIALIZED_SIZE, "Serialized data exceeds MAX_SERIALIZED_SIZE");
    }};

    return expanded;
}

fn max_size_for_type(ty: &Type) -> proc_macro2::TokenStream {
    quote! {
        <#ty as liquidcan_rust_macros::byte_codec::ByteCodec>::MAX_SERIALIZED_SIZE
    }
}

fn sum_max_sizes(types: Vec<&Type>) -> proc_macro2::TokenStream {
    let field_sizes: Vec<_> = types.into_iter().map(max_size_for_type).collect();
    quote! {
        0usize #( + #field_sizes )*
    }
}

fn build_struct_max_serialized_size(fields: &Fields) -> proc_macro2::TokenStream {
    let payload_size = match fields {
        Fields::Named(named) => {
            let types: Vec<_> = named.named.iter().map(|f| &f.ty).collect();
            sum_max_sizes(types)
        }
        Fields::Unnamed(unnamed) => {
            let types: Vec<_> = unnamed.unnamed.iter().map(|f| &f.ty).collect();
            sum_max_sizes(types)
        }
        Fields::Unit => quote! { 0usize },
    };

    quote! {
        const MAX_SERIALIZED_SIZE: usize = #payload_size;
    }
}

fn build_enum_max_serialized_size(
    variants: &Punctuated<Variant, Comma>,
) -> proc_macro2::TokenStream {
    let variant_sizes: Vec<_> = variants
        .iter()
        .map(|variant| {
            let payload_size = match &variant.fields {
                Fields::Named(named) => {
                    let types: Vec<_> = named.named.iter().map(|f| &f.ty).collect();
                    sum_max_sizes(types)
                }
                Fields::Unnamed(unnamed) => {
                    let types: Vec<_> = unnamed.unnamed.iter().map(|f| &f.ty).collect();
                    sum_max_sizes(types)
                }
                Fields::Unit => quote! { 0usize },
            };

            quote! {
                1usize + (#payload_size)
            }
        })
        .collect();

    let mut max_expr = variant_sizes
        .first()
        .cloned()
        .expect("ByteCodec cannot be derived for enums without variants");

    for size_expr in variant_sizes.iter().skip(1) {
        max_expr = quote! {
            {
                let a = #max_expr;
                let b = #size_expr;
                if a > b { a } else { b }
            }
        };
    }

    quote! {
        const MAX_SERIALIZED_SIZE: usize = #max_expr;
    }
}

fn build_deserialize(
    enum_name: &syn::Ident,
    variants: &Punctuated<Variant, Comma>,
) -> proc_macro2::TokenStream {
    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let (_, variant_discriminant) = variant
            .discriminant
            .as_ref()
            .expect("Must explicitly specify discriminant");

        match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

                quote! {
                    #variant_discriminant => {
                        #(
                            let (#field_names, input) = liquidcan_rust_macros::byte_codec::ByteCodec::deserialize(input)?;
                        )*
                        Ok((#enum_name::#variant_name { #( #field_names ),* }, input))
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_idents: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| format_ident!("f{}", i))
                    .collect();

                quote! {
                    #variant_discriminant => {
                        #(
                            let (#field_idents, input) = liquidcan_rust_macros::byte_codec::ByteCodec::deserialize(input)?;
                        )*
                        Ok((#enum_name::#variant_name( #( #field_idents ),* ), input))
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    #variant_discriminant => Ok((#enum_name::#variant_name, input))
                }
            }
        }
    });

    let expanded = quote! { fn deserialize(input: &[u8]) -> Result<(Self, &[u8]), liquidcan_rust_macros::byte_codec::DeserializationError> {
        let (discriminant, input) = input
            .split_first()
            .ok_or(liquidcan_rust_macros::byte_codec::DeserializationError::NotEnoughData)?;

        match discriminant {
            #(#match_arms,)*
            _ => Err(liquidcan_rust_macros::byte_codec::DeserializationError::InvalidDiscriminant(*discriminant)),
        }
    }};

    return expanded;
}

fn build_struct_serialize(type_name: &syn::Ident, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields) => {
            let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
            quote! {
                fn serialize(&self, out: &mut Vec<u8>) {
                    let out_len_before = out.len();
                    let #type_name { #( #field_names ),* } = self;
                    #( #field_names.serialize(out); )*
                    assert!(out.len() - out_len_before <= Self::MAX_SERIALIZED_SIZE, "Serialized data exceeds MAX_SERIALIZED_SIZE");
                }
            }
        }
        Fields::Unnamed(fields) => {
            let field_idents: Vec<_> = (0..fields.unnamed.len())
                .map(|i| format_ident!("f{}", i))
                .collect();

            quote! {
                fn serialize(&self, out: &mut Vec<u8>) {
                    let out_len_before = out.len();
                    let #type_name( #( #field_idents ),* ) = self;
                    #( #field_idents.serialize(out); )*
                    assert!(out.len() - out_len_before <= Self::MAX_SERIALIZED_SIZE, "Serialized data exceeds MAX_SERIALIZED_SIZE");
                }
            }
        }
        Fields::Unit => {
            quote! {
                fn serialize(&self, _out: &mut Vec<u8>) {
                }
            }
        }
    }
}

fn build_struct_deserialize(type_name: &syn::Ident, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields) => {
            let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

            quote! {
                fn deserialize(input: &[u8]) -> Result<(Self, &[u8]), liquidcan_rust_macros::byte_codec::DeserializationError> {
                    #(
                        let (#field_names, input) = liquidcan_rust_macros::byte_codec::ByteCodec::deserialize(input)?;
                    )*
                    Ok((#type_name { #( #field_names ),* }, input))
                }
            }
        }
        Fields::Unnamed(fields) => {
            let field_idents: Vec<_> = (0..fields.unnamed.len())
                .map(|i| format_ident!("f{}", i))
                .collect();

            quote! {
                fn deserialize(input: &[u8]) -> Result<(Self, &[u8]), liquidcan_rust_macros::byte_codec::DeserializationError> {
                    #(
                        let (#field_idents, input) = liquidcan_rust_macros::byte_codec::ByteCodec::deserialize(input)?;
                    )*
                    Ok((#type_name( #( #field_idents ),* ), input))
                }
            }
        }
        Fields::Unit => {
            quote! {
                fn deserialize(input: &[u8]) -> Result<(Self, &[u8]), liquidcan_rust_macros::byte_codec::DeserializationError> {
                    Ok((#type_name, input))
                }
            }
        }
    }
}

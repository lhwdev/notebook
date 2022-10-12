use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

/// Create a conversion into super enum: impl From<SelfEnum> for InheritedEnum
/// TODO: conversion into child: impl TryFrom<InheritedEnum> for SelfEnum
pub fn inherit_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target: Path = parse2(attr).expect("Expected target enum as attribute");
    let self_enum: ItemEnum = parse2(item.clone()).expect("Expected valid enum");

    inherit_enum_with(self_enum, item, target)
}

pub fn inherit_enum_with(
    self_enum: ItemEnum,
    self_enum_tokens: TokenStream,
    target: Path,
) -> TokenStream {
    let self_enum_ident = self_enum.ident;

    let mut matches = TokenStream::new();

    for variant in self_enum.variants.iter() {
        let variant_ident = variant.ident.clone();

        let match_variant = match &variant.fields {
            Fields::Named(fields) => {
                let patterns = punctuated::Punctuated::<Ident, Token![,]>::from_iter(
                    fields
                        .named
                        .iter()
                        .map(|field| field.ident.clone().unwrap()),
                );
                quote! {
                    #self_enum_ident::#variant_ident { #patterns } => #target::#variant_ident { #patterns }
                }
            }
            Fields::Unnamed(fields) => {
                let patterns = punctuated::Punctuated::<Ident, Token![,]>::from_iter(
                    fields.unnamed.iter().enumerate().map(|(index, _)| {
                        Ident::new(&format!("__p{}", index), proc_macro2::Span::call_site())
                    }),
                );
                quote! {
                    #self_enum_ident::#variant_ident(#patterns) => #target::#variant_ident(#patterns)
                }
            }
            Fields::Unit => quote! {
                #self_enum_ident::#variant_ident => #target::#variant_ident
            },
        };
        matches.extend(match_variant);
        matches.extend(quote! { , });
    }

    quote! {
        #self_enum_tokens

        impl From<#self_enum_ident> for #target {
            fn from(value: #self_enum_ident) -> Self {
                match value {
                    #matches
                }
            }
        }
    }
}

pub fn deref_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    let target: Path = parse2(attr).expect("Expected target trait as attribute");
    let self_enum: ItemEnum = parse2(item.clone()).expect("Expected valid enum");

    deref_enum_with(self_enum, item, target)
}

pub fn deref_enum_with(
    self_enum: ItemEnum,
    self_enum_tokens: TokenStream,
    target: Path,
) -> TokenStream {
    let self_enum_name = self_enum.ident;
    let mut matches = TokenStream::new();

    for variant in self_enum.variants {
        let variant_name = variant.ident;
        matches.extend(quote! {
            Self::#variant_name(__inner) => __inner,
        });
    }

    quote! {
        #self_enum_tokens

        impl std::ops::Deref for #self_enum_name {
            type Target = dyn #target;

            fn deref(&self) -> &Self::Target {
                match self {
                    #matches
                }
            }
        }
    }
}

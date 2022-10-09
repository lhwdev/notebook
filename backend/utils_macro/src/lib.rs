use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod thin_orm_wrapper;
mod thin_wrapper;
mod enums;
/// thin_wraper

#[proc_macro_derive(ThinWrapper, attributes(thin_wrapper))]
pub fn thin_wrapper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    thin_wrapper::transform_thin_wrapper(input, false).into()
}

#[proc_macro_derive(ThinWrapperSerde, attributes(thin_wrapper))]
pub fn thin_wrapper_serde(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    thin_wrapper::transform_thin_wrapper(input, true).into()
}

#[proc_macro_derive(ThinOrmWrapper, attributes(thin_wrapper))]
pub fn thin_orm_wrapper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    thin_orm_wrapper::transform_thin_orm_wrapper(input).into()
}

// model

#[proc_macro_attribute]
pub fn note_node(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item: proc_macro2::TokenStream = item.into();
    let attr = syn::parse::<syn::Ident>(attr);
    
    let mut result = quote! {
        #[derive(std::cmp::PartialEq, std::fmt::Debug, std::clone::Clone, serde::Serialize, serde::Deserialize)]
        #item
    };

    if let Ok(target) = attr {
        result = quote! {
            #target! {
                #result
            }
        };
    }
    
    result.into()
}

// other utils?

/// Create a conversion into super enum: impl From<SelfEnum> for InheritedEnum
/// TODO: conversion into child: impl TryFrom<InheritedEnum> for SelfEnum
#[proc_macro_attribute]
pub fn inherit_enum(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    enums::inherit_enum(attr.into(), item.into()).into()
}

#[proc_macro_attribute]
pub fn deref_enum(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    enums::deref_enum(attr.into(), item.into()).into()
}

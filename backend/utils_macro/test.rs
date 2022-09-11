#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use darling::FromMeta;
use proc_macro2::{Ident, Literal, TokenStream, TokenTree};
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, ExprPath, Path, PathArguments, Type,
};

#[darling(default)]
struct ThinWrapperData {
    #[darling(default = "a")]
    constructor: bool,
    field: Option<Ident>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for ThinWrapperData {
    #[inline]
    fn default() -> ThinWrapperData {
        ThinWrapperData {
            constructor: ::core::default::Default::default(),
            field: ::core::default::Default::default(),
        }
    }
}
impl ::darling::FromMeta for ThinWrapperData {
    fn from_list(__items: &[::syn::NestedMeta]) -> ::darling::Result<Self> {
        let mut constructor: (bool, ::darling::export::Option<bool>) =
            (false, None);
        let mut field: (bool, ::darling::export::Option<Option<Ident>>) =
            (false, None);
        let mut __errors = ::darling::Error::accumulator();
        for __item in __items {
            if let ::syn::NestedMeta::Meta(ref __inner) = *__item {
                    let __name =
                        ::darling::util::path_to_string(__inner.path());
                    match __name.as_str() {
                        "constructor" => {
                            if !constructor.0 {
                                    constructor =
                                        (true,
                                            __errors.handle(::darling::FromMeta::from_meta(__inner).map_err(|e|
                                                        e.with_span(&__inner).at("constructor"))));
                                } else {
                                   __errors.push(::darling::Error::duplicate_field("constructor").with_span(&__inner));
                               }
                        }
                        "field" => {
                            if !field.0 {
                                    field =
                                        (true,
                                            __errors.handle(::darling::FromMeta::from_meta(__inner).map_err(|e|
                                                        e.with_span(&__inner).at("field"))));
                                } else {
                                   __errors.push(::darling::Error::duplicate_field("field").with_span(&__inner));
                               }
                        }
                        __other => {
                            __errors.push(::darling::Error::unknown_field_with_alts(__other,
                                        &["constructor", "field"]).with_span(__inner));
                        }
                    }
                }
        }
        __errors.finish()?;
        let __default: Self = ::darling::export::Default::default();
        ::darling::export::Ok(Self {
                constructor: match constructor.1 {
                    ::darling::export::Some(__val) => __val,
                    ::darling::export::None => a(),
                },
                field: match field.1 {
                    ::darling::export::Some(__val) => __val,
                    ::darling::export::None => __default.field,
                },
            })
    }
}

fn a() -> bool { true }


#[proc_macro_derive(ThinWrapper, attributes(thin_wrapper))]
pub fn thin_wrapper(input: proc_macro::TokenStream)
    -> proc_macro::TokenStream {
    let input =





        // Type is inferred here / Type<Argument>::deserialize is wrong grammer













        match ::syn::parse_macro_input::parse::<DeriveInput>(input) {
            ::syn::__private::Ok(data) => data,
            ::syn::__private::Err(err) => {
                return ::syn::__private::TokenStream::from(err.to_compile_error());
            }
        };
    transform_thin_wrapper_main(input, false).into()
}
#[proc_macro_derive(ThinWrapperSerde, attributes(thin_wrapper))]
pub fn thin_wrapper_serde(input: proc_macro::TokenStream)
    -> proc_macro::TokenStream {
    let input =
        match ::syn::parse_macro_input::parse::<DeriveInput>(input) {
            ::syn::__private::Ok(data) => data,
            ::syn::__private::Err(err) => {
                return ::syn::__private::TokenStream::from(err.to_compile_error());
            }
        };
    transform_thin_wrapper_main(input, true).into()
}
fn transform_thin_wrapper_main(input: DeriveInput, serde: bool)
    -> TokenStream {
    let item =
        if let Data::Struct(inner) = &input.data {
                inner
            } else {
               ::core::panicking::panic("internal error: entered unreachable code")
           };
    let ident = &input.ident;
    let inner_type = &item.fields.iter().next().expect("No field").ty;
    let raw_attr =
        input.attrs.iter().find(|attr| is_ident(&attr.path, "thin_wrapper"));
    let attr: ThinWrapperData =
        if let Some(raw_attr) = raw_attr {
                let meta = raw_attr.parse_meta().unwrap();
                ThinWrapperData::from_meta(&meta).expect("Wrong metadata content of thin_wrapper")
            } else { ThinWrapperData::default() };
    let field =
        attr.field.map(|ident|
                    TokenTree::Ident(ident)).unwrap_or_else(||
                TokenTree::Literal(Literal::i32_unsuffixed(0)));
    let inner_type_mapped =
        if let Type::Path(path) = inner_type {
                let mut path = path.clone();
                let segments = &mut path.path.segments;
                for segment in segments {
                    if let PathArguments::AngleBracketed(_) = &segment.arguments
                            {
                            segment.arguments = PathArguments::None;
                        }
                }
                ExprPath {
                    attrs: ::alloc::vec::Vec::new(),
                    qself: path.qself,
                    path: path.path,
                }
            } else {
               ::core::panicking::panic("internal error: entered unreachable code");
           };
    let mut after =
        {
            let mut _s = ::quote::__private::TokenStream::new();
            ;
            ;
            ;
            ::quote::__private::push_ident(&mut _s, "impl");
            ;
            ::quote::ToTokens::to_tokens(&ident, &mut _s);
            ;
            ;
            ::quote::__private::push_group(&mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ;
                    ;
                    ;
                    ::quote::__private::push_ident(&mut _s, "fn");
                    ;
                    ::quote::__private::push_ident(&mut _s, "into_inner");
                    ;
                    ::quote::__private::push_group(&mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "self");
                            ;
                            _s
                        });
                    ;
                    ::quote::__private::push_rarrow(&mut _s);
                    ;
                    ::quote::ToTokens::to_tokens(&inner_type, &mut _s);
                    ;
                    ;
                    ::quote::__private::push_group(&mut _s,
                        ::quote::__private::Delimiter::Brace,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ;
                            ;
                            ;
                            ::quote::__private::push_ident(&mut _s, "self");
                            ;
                            ::quote::__private::push_dot(&mut _s);
                            ;
                            ::quote::ToTokens::to_tokens(&field, &mut _s);
                            ;
                            ;
                            ;
                            ;
                            ;
                            _s
                        });
                    ;
                    ;
                    ;
                    ;
                    _s
                });
            ;
            ::quote::__private::push_ident(&mut _s, "impl");
            ;
            ::quote::__private::push_ident(&mut _s, "std");
            ;
            ::quote::__private::push_colon2(&mut _s);
            ;
            ::quote::__private::push_ident(&mut _s, "ops");
            ;
            ::quote::__private::push_colon2(&mut _s);
            ;
            ::quote::__private::push_ident(&mut _s, "Deref");
            ;
            ::quote::__private::push_ident(&mut _s, "for");
            ;
            ::quote::ToTokens::to_tokens(&ident, &mut _s);
            ;
            ;
            ::quote::__private::push_group(&mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ;
                    ;
                    ;
                    ::quote::__private::push_ident(&mut _s, "type");
                    ;
                    ::quote::__private::push_ident(&mut _s, "Target");
                    ;
                    ::quote::__private::push_eq(&mut _s);
                    ;
                    ::quote::ToTokens::to_tokens(&inner_type, &mut _s);
                    ;
                    ;
                    ::quote::__private::push_semi(&mut _s);
                    ;
                    ::quote::__private::push_ident(&mut _s, "fn");
                    ;
                    ::quote::__private::push_ident(&mut _s, "deref");
                    ;
                    ::quote::__private::push_group(&mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_and(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "self");
                            ;
                            _s
                        });
                    ;
                    ::quote::__private::push_rarrow(&mut _s);
                    ;
                    ::quote::__private::push_and(&mut _s);
                    ;
                    ::quote::__private::push_ident(&mut _s, "Self");
                    ;
                    ::quote::__private::push_colon2(&mut _s);
                    ;
                    ::quote::__private::push_ident(&mut _s, "Target");
                    ;
                    ::quote::__private::push_group(&mut _s,
                        ::quote::__private::Delimiter::Brace,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ;
                            ;
                            ;
                            ::quote::__private::push_and(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "self");
                            ;
                            ::quote::__private::push_dot(&mut _s);
                            ;
                            ::quote::ToTokens::to_tokens(&field, &mut _s);
                            ;
                            ;
                            ;
                            ;
                            ;
                            _s
                        });
                    ;
                    ;
                    ;
                    ;
                    _s
                });
            ;
            ::quote::__private::push_ident(&mut _s, "impl");
            ;
            ::quote::__private::push_ident(&mut _s, "std");
            ;
            ::quote::__private::push_colon2(&mut _s);
            ;
            ::quote::__private::push_ident(&mut _s, "ops");
            ;
            ::quote::__private::push_colon2(&mut _s);
            ;
            ::quote::__private::push_ident(&mut _s, "DerefMut");
            ;
            ::quote::__private::push_ident(&mut _s, "for");
            ;
            ::quote::ToTokens::to_tokens(&ident, &mut _s);
            ;
            ;
            ::quote::__private::push_group(&mut _s,
                ::quote::__private::Delimiter::Brace,
                {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ;
                    ;
                    ;
                    ::quote::__private::push_ident(&mut _s, "fn");
                    ;
                    ::quote::__private::push_ident(&mut _s, "deref_mut");
                    ;
                    ::quote::__private::push_group(&mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ;
                            ;
                            ;
                            ::quote::__private::push_and(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "mut");
                            ;
                            ::quote::__private::push_ident(&mut _s, "self");
                            ;
                            ;
                            ;
                            ;
                            _s
                        });
                    ;
                    ::quote::__private::push_rarrow(&mut _s);
                    ;
                    ::quote::__private::push_and(&mut _s);
                    ;
                    ::quote::__private::push_ident(&mut _s, "mut");
                    ;
                    ::quote::__private::push_ident(&mut _s, "Self");
                    ;
                    ::quote::__private::push_colon2(&mut _s);
                    ;
                    ::quote::__private::push_ident(&mut _s, "Target");
                    ;
                    ::quote::__private::push_group(&mut _s,
                        ::quote::__private::Delimiter::Brace,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ;
                            ;
                            ;
                            ::quote::__private::push_and(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "mut");
                            ;
                            ::quote::__private::push_ident(&mut _s, "self");
                            ;
                            ::quote::__private::push_dot(&mut _s);
                            ;
                            ::quote::ToTokens::to_tokens(&field, &mut _s);
                            ;
                            ;
                            ;
                            ;
                            ;
                            _s
                        });
                    ;
                    ;
                    ;
                    ;
                    _s
                });
            ;
            ;
            ;
            ;
            _s
        };
    if serde {
            after.extend({
                    let mut _s = ::quote::__private::TokenStream::new();
                    ;
                    ;
                    ;
                    ::quote::__private::push_ident(&mut _s, "impl");
                    ;
                    ::quote::__private::push_ident(&mut _s, "serde");
                    ;
                    ::quote::__private::push_colon2(&mut _s);
                    ;
                    ::quote::__private::push_ident(&mut _s, "Serialize");
                    ;
                    ::quote::__private::push_ident(&mut _s, "for");
                    ;
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ;
                    ;
                    ::quote::__private::push_group(&mut _s,
                        ::quote::__private::Delimiter::Brace,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ;
                            ;
                            ;
                            ::quote::__private::push_ident(&mut _s, "fn");
                            ;
                            ::quote::__private::push_ident(&mut _s, "serialize");
                            ;
                            ::quote::__private::push_lt(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "S");
                            ;
                            ::quote::__private::push_gt(&mut _s);
                            ;
                            ::quote::__private::push_group(&mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ;
                                    ;
                                    ;
                                    ::quote::__private::push_and(&mut _s);
                                    ;
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ;
                                    ::quote::__private::push_comma(&mut _s);
                                    ;
                                    ::quote::__private::push_ident(&mut _s, "serializer");
                                    ;
                                    ::quote::__private::push_colon(&mut _s);
                                    ;
                                    ::quote::__private::push_ident(&mut _s, "S");
                                    ;
                                    ;
                                    ;
                                    ;
                                    _s
                                });
                            ;
                            ::quote::__private::push_rarrow(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "Result");
                            ;
                            ::quote::__private::push_lt(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "S");
                            ;
                            ::quote::__private::push_colon2(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "Ok");
                            ;
                            ::quote::__private::push_comma(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "S");
                            ;
                            ::quote::__private::push_colon2(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "Error");
                            ;
                            ::quote::__private::push_gt(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "where");
                            ;
                            ::quote::__private::push_ident(&mut _s, "S");
                            ;
                            ::quote::__private::push_colon(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "serde");
                            ;
                            ::quote::__private::push_colon2(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "Serializer");
                            ;
                            ::quote::__private::push_group(&mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ;
                                    ;
                                    ;
                                    ::quote::__private::push_ident(&mut _s, "self");
                                    ;
                                    ::quote::__private::push_dot(&mut _s);
                                    ;
                                    ::quote::ToTokens::to_tokens(&field, &mut _s);
                                    ;
                                    ;
                                    ::quote::__private::push_dot(&mut _s);
                                    ;
                                    ::quote::__private::push_ident(&mut _s, "serialize");
                                    ;
                                    ::quote::__private::push_group(&mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "serializer");
                                            ;
                                            _s
                                        });
                                    ;
                                    ;
                                    ;
                                    ;
                                    _s
                                });
                            ;
                            ;
                            ;
                            ;
                            _s
                        });
                    ;
                    ::quote::__private::push_ident(&mut _s, "impl");
                    ;
                    ::quote::__private::push_lt(&mut _s);
                    ;
                    ::quote::__private::push_lifetime(&mut _s, "\'de");
                    ;
                    ::quote::__private::push_gt(&mut _s);
                    ;
                    ::quote::__private::push_ident(&mut _s, "serde");
                    ;
                    ::quote::__private::push_colon2(&mut _s);
                    ;
                    ::quote::__private::push_ident(&mut _s, "Deserialize");
                    ;
                    ::quote::__private::push_lt(&mut _s);
                    ;
                    ::quote::__private::push_lifetime(&mut _s, "\'de");
                    ;
                    ::quote::__private::push_gt(&mut _s);
                    ;
                    ::quote::__private::push_ident(&mut _s, "for");
                    ;
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ;
                    ;
                    ::quote::__private::push_group(&mut _s,
                        ::quote::__private::Delimiter::Brace,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ;
                            ;
                            ;
                            ::quote::__private::push_ident(&mut _s, "fn");
                            ;
                            ::quote::__private::push_ident(&mut _s, "deserialize");
                            ;
                            ::quote::__private::push_lt(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "D");
                            ;
                            ::quote::__private::push_gt(&mut _s);
                            ;
                            ::quote::__private::push_group(&mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ;
                                    ;
                                    ;
                                    ::quote::__private::push_ident(&mut _s, "deserializer");
                                    ;
                                    ::quote::__private::push_colon(&mut _s);
                                    ;
                                    ::quote::__private::push_ident(&mut _s, "D");
                                    ;
                                    ;
                                    ;
                                    ;
                                    _s
                                });
                            ;
                            ::quote::__private::push_rarrow(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "Result");
                            ;
                            ::quote::__private::push_lt(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "Self");
                            ;
                            ::quote::__private::push_comma(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "D");
                            ;
                            ::quote::__private::push_colon2(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "Error");
                            ;
                            ::quote::__private::push_gt(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "where");
                            ;
                            ::quote::__private::push_ident(&mut _s, "D");
                            ;
                            ::quote::__private::push_colon(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "serde");
                            ;
                            ::quote::__private::push_colon2(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "Deserializer");
                            ;
                            ::quote::__private::push_lt(&mut _s);
                            ;
                            ::quote::__private::push_lifetime(&mut _s, "\'de");
                            ;
                            ::quote::__private::push_gt(&mut _s);
                            ;
                            ::quote::__private::push_group(&mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "Ok");
                                    ;
                                    ::quote::__private::push_group(&mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ;
                                            ;
                                            ;
                                            ::quote::ToTokens::to_tokens(&ident, &mut _s);
                                            ;
                                            ;
                                            ::quote::__private::push_colon2(&mut _s);
                                            ;
                                            ::quote::__private::push_ident(&mut _s, "new");
                                            ;
                                            ::quote::__private::push_group(&mut _s,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ;
                                                    ;
                                                    ;
                                                    ::quote::ToTokens::to_tokens(&inner_type_mapped, &mut _s);
                                                    ;
                                                    ;
                                                    ::quote::__private::push_colon2(&mut _s);
                                                    ;
                                                    ::quote::__private::push_ident(&mut _s, "deserialize");
                                                    ;
                                                    ::quote::__private::push_group(&mut _s,
                                                        ::quote::__private::Delimiter::Parenthesis,
                                                        {
                                                            let mut _s = ::quote::__private::TokenStream::new();
                                                            ::quote::__private::push_ident(&mut _s, "deserializer");
                                                            ;
                                                            _s
                                                        });
                                                    ;
                                                    ::quote::__private::push_question(&mut _s);
                                                    ;
                                                    ;
                                                    ;
                                                    ;
                                                    _s
                                                });
                                            ;
                                            ;
                                            ;
                                            ;
                                            _s
                                        });
                                    ;
                                    _s
                                });
                            ;
                            ;
                            ;
                            ;
                            _s
                        });
                    ;
                    ;
                    ;
                    ;
                    _s
                });
        }
    if attr.constructor {
            after.extend({
                    let mut _s = ::quote::__private::TokenStream::new();
                    ;
                    ;
                    ;
                    ::quote::__private::push_ident(&mut _s, "impl");
                    ;
                    ::quote::ToTokens::to_tokens(&ident, &mut _s);
                    ;
                    ;
                    ::quote::__private::push_group(&mut _s,
                        ::quote::__private::Delimiter::Brace,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ;
                            ;
                            ;
                            ::quote::__private::push_ident(&mut _s, "pub");
                            ;
                            ::quote::__private::push_ident(&mut _s, "fn");
                            ;
                            ::quote::__private::push_ident(&mut _s, "new");
                            ;
                            ::quote::__private::push_group(&mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ;
                                    ;
                                    ;
                                    ::quote::__private::push_ident(&mut _s, "inner");
                                    ;
                                    ::quote::__private::push_colon(&mut _s);
                                    ;
                                    ::quote::ToTokens::to_tokens(&inner_type, &mut _s);
                                    ;
                                    ;
                                    ;
                                    ;
                                    ;
                                    _s
                                });
                            ;
                            ::quote::__private::push_rarrow(&mut _s);
                            ;
                            ::quote::__private::push_ident(&mut _s, "Self");
                            ;
                            ::quote::__private::push_group(&mut _s,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "Self");
                                    ;
                                    ::quote::__private::push_group(&mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "inner");
                                            ;
                                            _s
                                        });
                                    ;
                                    _s
                                });
                            ;
                            ;
                            ;
                            ;
                            _s
                        });
                    ;
                    ;
                    ;
                    ;
                    _s
                });
        }
    after
}
fn is_ident(path: &Path, text: &str) -> bool {
    match path.get_ident() {
        Some(ident) => ident.to_string() == text,
        None => false,
    }
}
const _: () =
    {
        extern crate proc_macro;
        #[rustc_proc_macro_decls]
        #[allow(deprecated)]
        static _DECLS: &[proc_macro::bridge::client::ProcMacro] =
            &[proc_macro::bridge::client::ProcMacro::custom_derive("ThinWrapper",
                            &["thin_wrapper"], thin_wrapper),
                        proc_macro::bridge::client::ProcMacro::custom_derive("ThinWrapperSerde",
                            &["thin_wrapper"], thin_wrapper_serde)];
    };

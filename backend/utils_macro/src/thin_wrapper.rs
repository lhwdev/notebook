use std::str::FromStr;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Path};

#[derive(FromMeta)]
#[darling(default)]
struct ThinWrapperData {
    constructor: bool,
    field: Option<String>,
}

impl Default for ThinWrapperData {
    fn default() -> Self {
        Self {
            constructor: true,
            field: None,
        }
    }
}

pub fn transform_thin_wrapper(input: DeriveInput, serde: bool) -> TokenStream {
    let item = if let Data::Struct(inner) = &input.data {
        inner
    } else {
        unreachable!()
    };

    let ident = &input.ident;
    let inner_type = &item.fields.iter().next().expect("No field").ty;

    let raw_attr = input
        .attrs
        .iter()
        .find(|attr| is_ident(&attr.path, "thin_wrapper"));
    let attr: ThinWrapperData = if let Some(raw_attr) = raw_attr {
        let meta = raw_attr.parse_meta().unwrap();
        ThinWrapperData::from_meta(&meta).expect("Wrong metadata content of thin_wrapper")
    } else {
        ThinWrapperData::default()
    };

    let field = attr
        .field
        .unwrap_or_else(|| "0".to_string());
    let field = TokenStream::from_str(field.as_str()).unwrap();

    let mut after = quote! {
        impl #ident {
            pub fn into_inner(self) -> #inner_type {
                self.#field
            }
        }

        #[automatically_derived]
        impl std::ops::Deref for #ident {
            type Target = #inner_type;

            fn deref(&self) -> &Self::Target {
                &self.#field
            }
        }

        #[automatically_derived]
        impl std::ops::DerefMut for #ident {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#field
            }
        }
    };
    if serde {
        after.extend(quote! {
            #[automatically_derived]
            impl serde::Serialize for #ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer {
                    self.#field.serialize(serializer)
                }
            }

            #[automatically_derived]
            impl <'de> serde::Deserialize<'de> for #ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de> {
                    Ok(#ident::new(<#inner_type>::deserialize(deserializer)?))
                }
            }
        });
    }

    if attr.constructor {
        after.extend(quote! {
            #[automatically_derived]
            impl #ident {
                pub fn new(inner: #inner_type) -> Self {
                    Self(inner)
                }
            }
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

#[cfg(test)]
fn transform_thin_wrapper_serde_test(input: TokenStream) -> TokenStream {
    transform_thin_wrapper(syn::parse2(input).unwrap(), true)
}

#[cfg(test)]
mod test {
    use quote::quote;

    #[test]
    fn test() {
        let code = quote! {
            #[derive(ThinWrapperSerde, PartialEq)]
            pub struct Privileges(Vec<Privilege>);
        };

        let result = super::transform_thin_wrapper_serde_test(code);
        println!("{}", result);
    }
}


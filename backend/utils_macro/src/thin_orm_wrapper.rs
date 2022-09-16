use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn transform_thin_orm_wrapper(input: DeriveInput) -> TokenStream {
    let item = if let Data::Struct(inner) = &input.data {
        inner
    } else {
        unreachable!()
    };

    let ident = &input.ident;
    let inner_type = &item.fields.iter().next().expect("No field").ty;

    let mut result = quote! {
        #[automatically_derived]
        impl std::convert::From<#ident> for sea_orm::Value {
            fn from(source: #ident) -> Self {
                source.into_inner().into()
            }
        }

        #[automatically_derived]
        impl sea_query::ValueType for #ident {
            fn try_from(v: sea_orm::Value) -> Result<Self, sea_query::ValueTypeErr> {
                let result = <#inner_type as sea_query::ValueType>::try_from(v)?;
                Ok(Self::new(result))
            }

            fn type_name() -> String {
                <#inner_type as sea_query::ValueType>::type_name()
            }

            fn column_type() -> sea_query::ColumnType {
                <#inner_type as sea_query::ValueType>::column_type()
            }
        }

        #[automatically_derived]
        impl sea_orm::TryGetable for #ident {
            fn try_get(res: &sea_orm::QueryResult, pre: &str, col: &str) -> Result<Self, sea_orm::TryGetError> {
                let result = <#inner_type as sea_orm::TryGetable>::try_get(res, pre, col)?;
                Ok(Self::new(result))
            }
        }
    };
    result.extend(super::thin_wrapper::transform_thin_wrapper(input, true));
    result
}

use syn::{parse_macro_input, DeriveInput};

mod thin_wrapper;
mod thin_orm_wrapper;
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

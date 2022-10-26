use syn::{parse::Parse, Token};

pub struct EqualsMeta<Inner: Parse> {
    pub equals_token: Token![=],
    pub inner: Inner,
}

impl <Inner : Parse> Parse for EqualsMeta<Inner> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(EqualsMeta {
            equals_token: input.parse()?,
            inner: input.parse()?,
        })
    }
}

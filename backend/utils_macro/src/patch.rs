use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::*;

/*
#[generate_patch]
#[derive(Debug, Serialize, Deserialize)]
struct Model {
    name: String
}

into

#[automatically_generated]
#[derive(Debug, Serialize, Deserialize)]
struct ModelPatch {
    name: Option<String>
}

impl From<Model> for ModelPatch {
    fn from(model: Model) -> Self {
        ModelPatch {
            name: Some(model.name)
        }
    }
}

impl TryFrom<ModelPatch> for Model {
    fn try_from(patch: ModelPatch) -> Option<Self> {
        Model {
            name: patch.name?
        }
    }
}
*/
// TODO: customize macros

#[derive(Debug, FromMeta, Default)]
pub struct PatchParameters {
    #[darling(default)]
    name: Option<String>,
}

pub fn generate_patch(item: TokenStream) -> TokenStream {
    let mut self_struct: ItemStruct = parse2(item).expect("Expected struct");
    let attr_index = self_struct.attrs.iter().position(|attr|
        matches!(attr.path.get_ident(), Some(ident) if ident.to_string() == "generate_patch")
    );

    let parameters = if let Some(index) = attr_index {
        let parameters = PatchParameters::from_meta(
            &self_struct.attrs[index]
                .parse_meta()
                .expect("Could not parse generate_patch parameters: illegal syntax"),
        )
        .expect("Could not parse generate_patch parameters");

        self_struct.attrs.remove(index);
        parameters
    } else {
        PatchParameters::default()
    };

    generate_patch_with(parameters, self_struct)
}

pub fn generate_patch_with(parameters: PatchParameters, self_struct: ItemStruct) -> TokenStream {
    let fields = self_struct.fields;
    let mut fields = if let Fields::Named(named) = fields {
        named
    } else {
        panic!("Struct fields should be named")
    };
    for field in fields.named.iter_mut() {
        let patch_attr_index = field.attrs.iter().position(
            |attr| matches!(attr.path.get_ident(), Some(ident) if ident.to_string() == "patch"),
        );

        let field_type = if let Some(index) = patch_attr_index {
            let patch_attr = field.attrs.remove(index);
            patch_attr
                .parse_args::<Type>()
                .expect("Expected valid Type in `#[patch(Type)]`")
        } else {
            field.ty.clone()
        };
        field.ty = parse_quote! {
            ::std::option::Option<#field_type>
        };
    }
    let patch_struct = ItemStruct {
        ident: Ident::new(
            &parameters
                .name
                .unwrap_or_else(|| format!("{name}Patch", name = self_struct.ident)),
            Span::call_site(),
        ),
        fields: Fields::Named(fields),
        ..self_struct
    };
    quote! {
        #[derive(Default)]
        #patch_struct
    }
}

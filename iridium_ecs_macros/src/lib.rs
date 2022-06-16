use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

// I am sure this will be used at some point.
#[allow(dead_code)]
fn get_struct_fields(ast: &syn::DeriveInput) -> Vec<&syn::Field> {
    match &ast.data {
        syn::Data::Struct(data) => data.fields.iter().collect(),
        _ => panic!("#[derive(Component)] can only be used on structs"),
    }
}

#[proc_macro_derive(ComponentTrait)]
pub fn derive_component_trait(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = &ast.ident;

    let mut idents = vec![];
    let mut types = vec![];

    if let syn::Data::Struct(data) = &ast.data {
        for field in &data.fields {
            idents.push(field.ident.as_ref().unwrap().clone().to_string());
            types.push(field.ty.clone());
        }
    }

    quote! {
        impl iridium_ecs::ComponentTrait for #struct_name {
            fn dyn_type_name(&self) -> &'static str {
                stringify!(#struct_name)
            }
            fn field_types(&self) -> Vec<(&'static str, &'static str)> {
                let mut fields = vec![];
                #(
                    fields.push((#idents, stringify!(#types)));
                )*
                fields
            }
        }
    }.to_string().parse().unwrap()
}

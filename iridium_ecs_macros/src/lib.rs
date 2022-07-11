#![warn(missing_docs)]

//! This is the procedural macros to be used with the `iridium_ecs` crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Derive macro generating an impl of the trait `ComponentTrait`.
#[proc_macro_derive(ComponentTrait, attributes(hidden, drag_speed))]
pub fn derive_component_trait(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = &ast.ident;

    let mut idents = vec![];
    let mut idents_strings = vec![];
    let mut types = vec![];
    let mut attrs_path = vec![];
    let mut attrs_tts = vec![];

    if let syn::Data::Struct(data) = ast.data {
        for field in data.fields {
            if field.attrs.iter().any(|attr| attr.path.is_ident("hidden")) {
                continue;
            }
            let ident = field.ident.unwrap();
            idents_strings.push(ident.to_string());
            idents.push(ident);
            types.push(field.ty);
            attrs_path.push(vec![]);
            attrs_tts.push(vec![]);
            for attr in field.attrs {
                let last = attrs_path.len() - 1;
                attrs_path[last].push(attr.path);
                attrs_tts[last].push(attr.tokens);
            }
        }
    }

    quote! {
        impl iridium_ecs::ComponentTrait for #struct_name {
            fn type_name() -> &'static str {
                stringify!(#struct_name)
            }
            fn dyn_type_name(&self) -> &'static str {
                stringify!(#struct_name)
            }
            fn field_types(&self) -> Vec<(&'static str, &'static str)> {
                let mut fields = vec![];
                #(
                    fields.push((#idents_strings, stringify!(#types)));
                )*
                fields
            }
            fn ui(&mut self, ui: &mut egui::Ui) {
                #(
                    let attributes = {
                        let mut attributes = hashbrown::HashMap::new();
                        #(
                            let tts = stringify!(#attrs_tts);
                            // Remove the first and last chars, which are ( and )
                            let tts = &tts[1..tts.len() - 1];
                            attributes.insert(stringify!(#attrs_path), tts);
                        )*
                        iridium_ecs::ComponentFieldAttributes(attributes)
                    };

                    ui.label(#idents_strings);
                    self.#idents.ui(ui, attributes);
                    ui.end_row();
                )*
            }
        }
    }
    .to_string()
    .parse()
    .unwrap()
}

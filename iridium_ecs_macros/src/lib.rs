#![warn(
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    clippy::nursery,
    future_incompatible
)]
#![allow(clippy::module_name_repetitions)]

//! This is the procedural macros to be used with the `iridium_ecs` crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Type};

/// This macro simplifies the creation of a system.
///
/// # Panics
///
/// Will panic if the input is invalid.
#[proc_macro_attribute]
pub fn system_helper(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::ItemImpl);

    let self_type = if let Type::Path(path) = &*ast.self_ty {
        path
    } else {
        panic!("`system` must be implemented for a struct");
    };

    let state_type = if let Type::Path(path) = parse_macro_input!(attr as syn::Type) {
        path
    } else {
        panic!("The state type must be a path.");
    };

    quote! {
        impl System for #self_type {
            fn name(&self) -> &'static str {
                stringify!(#self_type)
            }

            fn state_type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<#state_type>()
            }

            fn default_state(&self) -> Component {
                Component::new(#state_type::default())
            }

            fn system(&self, state: &Component, entities: &Entities, assets: &iridium_assets::Assets, delta_time: f64) {
                let state = state.get_mut::<#state_type>();
                #self_type::system(state, entities, assets, delta_time);
            }
        }
    }
    .to_string()
    .parse()
    .expect("Failed to parse the generated code.")
}

/// Derive macro generating an impl of the trait `ComponentTrait`.
#[proc_macro_derive(ComponentTrait)]
pub fn derive_component_trait(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = &ast.ident;

    quote! {
        impl iridium_ecs::ComponentTrait for #struct_name {
            fn type_name() -> &'static str {
                stringify!(#struct_name)
            }
            fn dyn_type_name(&self) -> &'static str {
                stringify!(#struct_name)
            }
        }
    }
    .to_string()
    .parse()
    .expect("Failed to parse derive macro output")
}

/// Derive macro generating an impl of the trait `InspectorUi`.
#[proc_macro_derive(InspectorUi, attributes(hidden, drag_speed))]
pub fn derive_inspector_ui(tokens: TokenStream) -> TokenStream {
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
            let ident = field.ident.expect("Fields must have an identifier");
            idents_strings.push(ident.to_string());
            idents.push(ident);
            types.push(field.ty);
            attrs_path.push(vec![]);
            attrs_tts.push(vec![]);
            for attr in field.attrs {
                let end = attrs_path.len() - 1;
                attrs_path[end].push(attr.path);
                attrs_tts[end].push(attr.tokens);
            }
        }
    }

    quote! {
        impl iridium_ecs::ui::InspectorUi for #struct_name {
            fn ui(&mut self, ui: &mut egui::Ui) {
                use iridium_ecs::ui::InspectorUiField;

                #(
                    let attributes = {
                        let mut attributes = hashbrown::HashMap::new();
                        #(
                            let tts = stringify!(#attrs_tts);
                            // Remove the first and last chars, which are ( and )
                            let tts = &tts[1..tts.len() - 1];
                            attributes.insert(stringify!(#attrs_path), tts);
                        )*
                        iridium_ecs::ui::InspectorUiFieldAttributes::from_inner(attributes)
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
    .expect("Failed to parse derive macro output")
}

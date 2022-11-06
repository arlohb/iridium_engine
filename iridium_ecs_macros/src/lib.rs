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

/// Implements the `SystemInputs` trait for everything.
#[proc_macro]
pub fn impl_system_inputs_for_all(_input: TokenStream) -> TokenStream {
    use std::fmt::Write;

    const MAX_INPUTS: u8 = 10;

    let mut impls: Vec<String> = vec![];

    let mut i_all = String::new();

    for i in 0..MAX_INPUTS {
        write!(i_all, "I_{}, ", (b'A' + i) as char).unwrap();

        // Impl for all mutable inputs
        impls.push(format!("impl_system_inputs!({i_all} ;);"));

        // Impl for all immutable inputs
        impls.push(format!("impl_system_inputs!(; {i_all});"));

        let mut j_all = String::new();

        for j in 0..MAX_INPUTS {
            write!(j_all, "J_{}, ", (b'A' + j) as char).unwrap();

            // Here i_all is immutable and j_all is mutable
            impls.push(format!("impl_system_inputs!({i_all}; {j_all});"));
        }
    }

    format!(
        "{}\n{}",
        quote! {
            /// Queries the entities that have a set of components.
            ///
            /// Used as `query(&Entities, [mut Component1, mut Component2 etc ; Component3, Component4 etc])`.
            ///
            /// Returns an iterator of tuples of the form (Component1, Component2 etc).
            ///
            /// # Examples
            ///
            /// ```ignore
            /// for (transform, velocity)
            /// in query!(&entities, [mut Transform; Velocity]) {
            ///    transform.position += velocity.velocity;
            /// }
            /// ```
            macro_rules! query {
                ($entities:expr, [$(mut $mut_type:ty),* $(,)?; $($type:ty),* $(,)?]) => {
                    {
                        let type_ids = [
                            $(
                                &std::any::TypeId::of::<$mut_type>(),
                            )*
                            $(
                                &std::any::TypeId::of::<$type>(),
                            )*
                        ];

                        $entities.query_by_type_id(type_ids).map(|components| {
                            let mut index = 0;
                            (
                                $(
                                    {
                                        #![allow(clippy::mixed_read_write_in_expression)]
                                        index += 1;
                                        components[index - 1].get_mut::<$mut_type>()
                                    },
                                )*
                                $(
                                    {
                                        #![allow(clippy::mixed_read_write_in_expression)]
                                        index += 1;
                                        components[index - 1].get::<$type>()
                                    },
                                )*
                            )
                        }).collect::<Vec<_>>().into_iter()
                    }
                };
            }

            macro_rules! impl_system_inputs {
                ($($generic_mut:ident),* $(,)? ; $($generic_imut:ident),* $(,)?) => {
                    impl<
                        'a,
                        $($generic_mut: $crate::ComponentTrait, )*
                        $($generic_imut: $crate::ComponentTrait, )*
                    > SystemInputs<'a>
                    for (
                        $(&'a mut $generic_mut, )*
                        $(&'a $generic_imut, )*
                    ) {
                        fn from_entities(entities: &'a Entities) -> std::vec::IntoIter<Self> {
                            query!(entities, [$(mut $generic_mut, )*; $($generic_imut, )*])
                        }
                    }
                }
            }
        },
        impls.join("\n")
    )
    .parse()
    .expect("Failed to parse the generated code.")
}

/// Derive macro generating an impl of the trait `ComponentTrait`.
#[proc_macro_derive(ComponentTrait)]
pub fn derive_component_trait(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = &ast.ident;

    let ecs_crate = if std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME env var not found")
        == "iridium_ecs"
    {
        quote! { crate }
    } else {
        quote! { iridium_ecs }
    };

    quote! {
        impl #ecs_crate::ComponentTrait for #struct_name {
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

/// Derive macro generating an impl of the trait `ComponentStorage`.
///
/// For now this is quite limited, it only works for empty structs.
#[proc_macro_derive(ComponentStorage)]
pub fn derive_component_storage(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = &ast.ident;

    let ecs_crate = if std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME env var not found")
        == "iridium_ecs"
    {
        quote! { crate }
    } else {
        quote! { iridium_ecs }
    };

    if let syn::Data::Struct(data) = ast.data {
        match data.fields {
            syn::Fields::Unit => {
                quote! {
                    impl #ecs_crate::storage::ComponentStorage for #struct_name {
                        fn from_stored(
                            _stored: #ecs_crate::storage::StoredComponent,
                            _assets: &iridium_assets::Assets,
                        ) -> Option<Self> {
                            Some(Self)
                        }

                        fn to_stored(&self) -> #ecs_crate::storage::StoredComponent {
                            #ecs_crate::storage::StoredComponent {
                                type_name: "#struct_name".to_string(),
                                fields: hashbrown::HashMap::new(),
                            }
                        }
                    }
                }
            }
            syn::Fields::Named(fields) if fields.named.is_empty() => {
                quote! {
                    impl #ecs_crate::storage::ComponentStorage for #struct_name {
                        fn from_stored(
                            _stored: #ecs_crate::storage::StoredComponent,
                            _assets: &iridium_assets::Assets,
                        ) -> Option<Self> {
                            Some(Self {})
                        }

                        fn to_stored(&self) -> #ecs_crate::storage::StoredComponent {
                            #ecs_crate::storage::StoredComponent {
                                type_name: "#struct_name".to_string(),
                                fields: hashbrown::HashMap::new(),
                            }
                        }
                    }
                }
            }
            syn::Fields::Unnamed(fields) if fields.unnamed.is_empty() => {
                quote! {
                    impl #ecs_crate::storage::ComponentStorage for #struct_name {
                        fn from_stored(
                            _stored: #ecs_crate::storage::StoredComponent,
                            _assets: &iridium_assets::Assets,
                        ) -> Option<Self> {
                            Some(Self())
                        }

                        fn to_stored(&self) -> #ecs_crate::storage::StoredComponent {
                            #ecs_crate::storage::StoredComponent {
                                type_name: "#struct_name".to_string(),
                                fields: hashbrown::HashMap::new(),
                            }
                        }
                    }
                }
            }
            _ => quote! {},
        }
    } else {
        quote! {}
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

    let ecs_crate = if std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME env var not found")
        == "iridium_ecs"
    {
        quote! { crate }
    } else {
        quote! { iridium_ecs }
    };

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
        impl #ecs_crate::ui::InspectorUi for #struct_name {
            fn ui(&mut self, ui: &mut egui::Ui) {
                use #ecs_crate::ui::InspectorUiField;

                #(
                    let attributes = {
                        let mut attributes = hashbrown::HashMap::new();
                        #(
                            let tts = stringify!(#attrs_tts);
                            // Remove the first and last chars, which are ( and )
                            let tts = &tts[1..tts.len() - 1];
                            attributes.insert(stringify!(#attrs_path), tts);
                        )*
                        #ecs_crate::ui::InspectorUiFieldAttributes::from_inner(attributes)
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

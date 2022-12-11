//! This is the procedural macros to be used with the `iridium_ecs` crate.

mod system_helper;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// This macro simplifies the creation of a system.
///
/// # Panics
///
/// Will panic if the input is invalid.
#[allow(clippy::too_many_lines)]
#[proc_macro_attribute]
pub fn system_helper(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(attr as system_helper::Input);
    let ast = parse_macro_input!(item as syn::ItemImpl);

    system_helper::system_helper(input, ast)
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
                                type_name: stringify!(#struct_name).to_string(),
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
                                type_name: stringify!(#struct_name).to_string(),
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
                                type_name: stringify!(#struct_name).to_string(),
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

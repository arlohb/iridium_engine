//! This is the procedural macros to be used with the `iridium_ecs` crate.

mod system_helper;

use std::hash::{Hash, Hasher};

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

fn get_ecs_crate() -> proc_macro2::TokenStream {
    if std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME env var not found") == "iridium_ecs" {
        quote! { crate }
    } else {
        quote! { iridium_ecs }
    }
}

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

/// Derive macro generating an impl of the trait `HasStableTypeId`.
#[proc_macro_derive(HasStableTypeId)]
pub fn derive_has_stable_type_id(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = &ast.ident;

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    struct_name.to_string().hash(&mut hasher);
    // hasher.write(struct_name.to_string().as_bytes());
    // hasher.write_u8(0xFF);
    let stable_type_id = hasher.finish();

    quote! {
        impl iridium_reflect::HasStableTypeId for #struct_name {
            fn stable_type_id() -> iridium_reflect::StableTypeId {
                #stable_type_id
            }

            fn dyn_stable_type_id(&self) -> iridium_reflect::StableTypeId {
                #stable_type_id
            }
        }
    }
    .to_string()
    .parse()
    .expect("Failed to parse derive macro output")
}

/// Derive macro generating an impl of the trait `Component`.
///
/// # Panics
///
/// If the type isn't a struct.
#[proc_macro_derive(Component)]
pub fn derive_component(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = &ast.ident;

    let ecs_crate = get_ecs_crate();

    let syn::Data::Struct(data) = ast.data
    else {
        panic!("This macro only works with structs");
    };

    // The idents of the asset fields
    let asset_fields: Vec<syn::Ident> = data
        .fields
        .into_iter()
        .filter_map(|field| {
            // Get the path type of the field
            let syn::Type::Path(path) = field.ty else {
                // If it's not a path, it can't be an asset
                return None;
            };

            // If `Asset` isn't one of the segments
            if !path
                .path
                .segments
                .into_iter()
                .any(|segment| segment.ident == "AssetBox")
            {
                return None;
            }

            // Returns the field ident
            field.ident
        })
        .collect();

    quote! {
        impl #ecs_crate::Component for #struct_name {
            fn type_name() -> &'static str {
                stringify!(#struct_name)
            }
            fn dyn_type_name(&self) -> &'static str {
                stringify!(#struct_name)
            }
            fn update_assets(&mut self, assets: &iridium_assets::Assets) -> Result<i32, String> {
                let mut updated = 0;

                #({
                    if self.#asset_fields.update_asset(assets)? {
                        updated += 1;
                    }
                };)*

                Ok(updated)
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
///
/// # Panics
///
/// Panics if it's derived on something that isn't a struct,
/// or a non-empty tuple struct.
#[allow(clippy::too_many_lines)]
#[proc_macro_derive(ComponentStorage, attributes(temporary, string))]
pub fn derive_component_storage(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = &ast.ident;

    let ecs_crate = get_ecs_crate();

    let syn::Data::Struct(data) = ast.data else {
        panic!("ComponentStorage can only be derived on a struct")
    };

    let (from_stored, to_stored) = {
        match data.fields {
            syn::Fields::Unit => (
                quote! { Some(Self) },
                quote! {
                    #ecs_crate::storage::StoredComponent {
                        type_name: stringify!(#struct_name).to_string(),
                        fields: std::collections::HashMap::new(),
                    }
                },
            ),
            syn::Fields::Named(fields) => {
                let mut temporary_fields: Vec<(&syn::Ident, proc_macro2::TokenStream)> = Vec::new();

                let (fields1, is_string): (Vec<&syn::Ident>, Vec<bool>) = fields
                    .named
                    .iter()
                    .filter_map(|field| {
                        if let Some(temp_attr) = field
                            .attrs
                            .iter()
                            .find(|attr| attr.path.is_ident("temporary"))
                        {
                            temporary_fields
                                .push((field.ident.as_ref()?, temp_attr.parse_args().ok()?));
                            None
                        } else {
                            Some((
                                field.ident.as_ref()?,
                                field.attrs.iter().any(|attr| attr.path.is_ident("string")),
                            ))
                        }
                    })
                    .unzip();

                let (temp_fields, temp_tokens): (Vec<&syn::Ident>, Vec<proc_macro2::TokenStream>) =
                    temporary_fields.into_iter().unzip();

                let fields2 = fields1.clone();
                let fields3 = fields1.clone();
                let fields4 = fields1.clone();

                (
                    quote! {
                        Some(Self {
                            #(#fields1: stored.get(stringify!(#fields2))?.parse().ok()?,)*
                            #(#temp_fields: #temp_tokens,)*
                        })
                    },
                    quote! {
                        #ecs_crate::storage::StoredComponent {
                            type_name: stringify!(#struct_name).to_string(),
                            fields: {
                                let mut map = std::collections::HashMap::new();
                                #(map.insert(
                                    stringify!(#fields3).to_string(),
                                    #ecs_crate::storage::StoredComponentField::new(
                                        self.#fields4.to_string(),
                                        #is_string,
                                    ),
                                );)*
                                map
                            },
                        }
                    },
                )
            }
            syn::Fields::Unnamed(fields) if fields.unnamed.is_empty() => (
                quote! { Some(Self()) },
                quote! {
                    #ecs_crate::storage::StoredComponent {
                        type_name: stringify!(#struct_name).to_string(),
                        fields: std::collections::HashMap::new(),
                    }
                },
            ),
            syn::Fields::Unnamed(_) => {
                panic!("ComponentStorage cannot be derived on non-empty tuple struct")
            }
        }
    };

    quote! {
        impl #ecs_crate::storage::ComponentStorage for #struct_name {
            fn from_stored(
                mut stored: #ecs_crate::storage::StoredComponent,
                _assets: &iridium_assets::Assets,
            ) -> Option<Self> {
                #from_stored
            }

            fn to_stored(&self) -> #ecs_crate::storage::StoredComponent {
                #to_stored
            }
        }
    }
    .to_string()
    .parse()
    .expect("Failed to parse derive macro output")
}

/// Derive macro generating an impl of the trait `InspectorUi`.
#[proc_macro_derive(InspectorUi, attributes(hidden, drag_speed, id))]
pub fn derive_inspector_ui(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = &ast.ident;

    let ecs_crate = get_ecs_crate();

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
                        let mut attributes = std::collections::HashMap::new();
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

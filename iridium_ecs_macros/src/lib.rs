use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

#[proc_macro_derive(Component)]
pub fn derive_component(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = ast.ident;

    quote! {
        impl iridium_ecs::Component for #struct_name {
            fn get_type(&self) -> &'static str {
                stringify!(#struct_name)
            }
        }
    }.to_string()
        .parse()
        .unwrap()
}

#[proc_macro_derive(System)]
pub fn derive_system(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = ast.ident;

    quote! {
        impl #struct_name {
            pub fn new(activated: bool) -> Self {
                Self {
                    activated,
                }
            }
        }

        impl iridium_ecs::System for #struct_name {
            fn name(&self) -> &'static str {
                stringify!(#struct_name)
            }

            fn get_activated(&self) -> bool {
                self.activated
            }

            fn set_activated(&mut self, activated: bool) {
                self.activated = activated;
            }

            fn run_system(&self, entities: &mut iridium_ecs::Entities) {
                self.run(entities);
            }
        }
    }.to_string()
        .parse()
        .unwrap()
}

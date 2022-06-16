use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

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

    quote! {
        impl iridium_ecs::ComponentTrait for #struct_name {
            fn dyn_type_name(&self) -> &'static str {
                stringify!(#struct_name)
            }
        }
    }.to_string().parse().unwrap()
}

#[proc_macro_derive(System)]
pub fn derive_system(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = ast.ident.clone();

    let fields = get_struct_fields(&ast);

    ((match fields.len() {
        1 => quote! {
            impl #struct_name {
                pub fn new(activated: bool) -> Self {
                    Self {
                        activated,
                    }
                }
            }
        }.to_string(),
        _ => "".to_string()
    }) + &quote! {
        impl iridium_ecs::systems::System for #struct_name {
            fn name(&self) -> &'static str {
                stringify!(#struct_name)
            }

            fn get_activated(&self) -> bool {
                self.activated
            }

            fn set_activated(&mut self, activated: bool) {
                self.activated = activated;
            }

            fn run_system(&mut self, entities: &iridium_ecs::Entities, delta_time: f64) {
                self.run(entities, delta_time);
            }
        }
    }.to_string())
        .parse()
        .unwrap()
}

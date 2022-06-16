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

#[proc_macro_derive(ComponentTrait, attributes(hidden))]
pub fn derive_component_trait(tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(tokens as syn::DeriveInput);
    let struct_name = &ast.ident;

    let mut idents = vec![];
    let mut idents_strings = vec![];
    let mut types = vec![];
    let mut attributes = vec![];

    if let syn::Data::Struct(data) = ast.data {
        for field in data.fields {
            if field.attrs.iter().any(|attr| attr.path.is_ident("hidden")) {
                continue;
            }
            let ident = field.ident.unwrap();
            idents_strings.push(ident.to_string());
            idents.push(ident);
            types.push(field.ty);
            attributes.push(field.attrs);
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
                    fields.push((#idents_strings, stringify!(#types)));
                )*
                fields
            }
            fn ui(&mut self, ui: &mut egui::Ui) {
                #(
                    let attributes = {
                        let mut attributes = hashbrown::HashMap::new();
                        #(
                            attributes.insert(stringify!(#attributes.path), stringify!(#attributes.tts));
                        )*
                        iridium_ecs::ComponentFieldAttributes(attributes)
                    };
                    ui.horizontal(|ui| {
                        ui.label(#idents_strings);
                        self.#idents.ui(ui, attributes);
                    });
                )*
            }
        }
    }.to_string().parse().unwrap()
}

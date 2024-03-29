use quote::quote;
use syn::Type;

/// The input to this macro.
pub struct Input {
    state: Type,
    mode: Mode,
}

/// Parse the input to this macro.
impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse the system type.
        let state: syn::Type = input.parse()?;

        // Pass through the comma.
        input.parse::<syn::Token![,]>()?;

        // Parse the mode.
        // Logic for this is below in the `Mode` enum.
        let mode = input.parse()?;

        Ok(Self { state, mode })
    }
}

/// An input to a system.
struct SystemInput {
    ty: Type,
    mutable: bool,
}

/// Output the input as a string.
///
/// This will be "&mut X" or "&X" depending on whether the input is mutable.
impl std::fmt::Debug for SystemInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "&{}{}",
            if self.mutable { "mut " } else { "" },
            quote::ToTokens::to_token_stream(&self.ty)
        )
    }
}

/// Parse this from a `ParseStream`.
impl syn::parse::Parse for SystemInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Pass through the '&';
        input.parse::<syn::Token![&]>()?;

        // Check if mutable.
        let mutable = if input.peek(syn::Token![mut]) {
            input.parse::<syn::Token![mut]>()?;
            true
        } else {
            false
        };

        // Parse the type.
        let ty: syn::Type = input.parse()?;

        Ok(Self { ty, mutable })
    }
}

/// Turn this into a `TokenStream` for use in `quote!`.
impl quote::ToTokens for SystemInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // Add the '&'.
        tokens.extend(quote! { & });

        // Add the 'mut' if necessary.
        if self.mutable {
            tokens.extend(quote! { mut });
        }

        // Add the type.
        let ty = &self.ty;
        tokens.extend(quote! { #ty });
    }
}

/// The system mode.
///
/// Decides how the system is called.
#[derive(Debug)]
enum Mode {
    /// Run the system function once per frame.
    ///
    /// The system function will be called with the following arguments:
    /// - `&mut State`
    /// - `&Entities`
    /// - `&Assets`
    /// - `f64` (delta time)
    Once,
    /// Run the system function for each entity with the given components.
    ///
    /// The system function will be called with the following arguments:
    /// - `&mut State`
    /// - `&Entities`
    /// - `(&mut Component1, &Component2, ...)`
    /// - `&Assets`
    /// - `f64` (delta time)
    Iter(Vec<SystemInput>),
    /// Run the system function for each entity with the given components.
    ///
    /// The function is run in parallel.
    ///
    /// The system function will be called with the following arguments:
    /// - `&State`
    /// - `&Entities`
    /// - `(&mut Component1, &Component2, ...)`
    /// - `&Assets`
    /// - `f64` (delta time)
    ///
    /// Note that the state is immutable.
    /// This is because the state is shared between all threads.
    /// If you need to mutate the state, use a locking mechanism,
    /// or consider another solution, as if each thread locks the state,
    /// the benefit of parallelism is lost.
    ParIter(Vec<SystemInput>),
}

impl Mode {
    /// Gets the system inputs.
    ///
    /// if the mode is `Once`, this will return an empty vector.
    fn inputs(&self) -> &[SystemInput] {
        match self {
            Self::Once => &[],
            Self::Iter(inputs) | Self::ParIter(inputs) => inputs,
        }
    }

    /// Splits the inputs between mutable and immutable.
    fn split_inputs(&self) -> (Vec<&SystemInput>, Vec<&SystemInput>) {
        let mut mutable = Vec::new();
        let mut immutable = Vec::new();

        for input in self.inputs() {
            if input.mutable {
                mutable.push(input);
            } else {
                immutable.push(input);
            }
        }

        (mutable, immutable)
    }
}

/// Parse the mode from a `ParseStream`.
impl syn::parse::Parse for Mode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Get the mode name.
        let mode = input.parse::<syn::Ident>()?.to_string();

        // If once, return.
        if mode == "once" {
            return Ok(Self::Once);
        }

        // Parse the next comma.
        input.parse::<syn::Token![,]>()?;

        // Get the types of the components.
        let component_types: Vec<SystemInput> = input
            .parse_terminated::<_, syn::Token![,]>(SystemInput::parse)?
            .into_iter()
            .collect();

        // Return the mode.
        match mode.as_str() {
            "iter" => Ok(Self::Iter(component_types)),
            "par_iter" => Ok(Self::ParIter(component_types)),
            _ => panic!("Invalid mode: {mode}"),
        }
    }
}

/// This macro simplifies the creation of a system.
///
/// # Panics
///
/// Will panic if the input is invalid.
#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
pub fn system_helper(Input { state, mode }: Input, ast: syn::ItemImpl) -> proc_macro::TokenStream {
    // Get the name of the system.
    let Type::Path(self_type) = &*ast.self_ty else {
        panic!("`system` must be implemented for a struct");
    };

    // Split the inputs.
    let (mutable_inputs, immutable_inputs) = mode.split_inputs();

    // Get the types of the inputs.
    let mutable_inputs_types = mutable_inputs
        .iter()
        .map(|input| &input.ty)
        .collect::<Vec<_>>();
    let immutable_inputs_types = immutable_inputs
        .iter()
        .map(|input| &input.ty)
        .collect::<Vec<_>>();

    // An expr to get the state.
    let let_state_expr = if quote::ToTokens::to_token_stream(&state).to_string() == "()" {
        quote! { let state = (); }
    } else {
        quote! { let state = state.unwrap_or_else(|| unreachable!()).get_mut::<#state>(); }
    };

    // An expr for default state.
    let default_state_expr = if quote::ToTokens::to_token_stream(&state).to_string() == "()" {
        quote! { None }
    } else {
        quote! { Some(#state::default().into()) }
    };

    // The system function.
    let system_fn = match mode {
        Mode::Once => quote! {
            fn system(
                &self,
                state: Option<&iridium_ecs::ComponentBox>,
                entities: &iridium_ecs::Entities,
                assets: &iridium_assets::Assets,
                delta_time: f64,
            ) {
                // Get the state as its real type.
                #let_state_expr
                // Run the system.
                if let Err(error) = Self::system(state, entities, assets, delta_time) {
                    entities.get::<iridium_core::LogState>().error(error);
                }
            }
        },

        Mode::Iter(_) => quote! {
            fn system(
                &self,
                state: Option<&iridium_ecs::ComponentBox>,
                entities: &iridium_ecs::Entities,
                assets: &iridium_assets::Assets,
                delta_time: f64,
            ) {
                // Get the state as its real type.
                // The system can mutate this.
                #let_state_expr

                // Query the entities.
                iridium_ecs::query!(entities, [
                    #(mut #mutable_inputs_types, )*;
                    #(#immutable_inputs_types, )*
                ])
                    // For each entity with the given components.
                    .for_each(|components| {
                        // Run the system.
                        if let Err(error) = Self::system(
                            state,
                            entities,
                            components,
                            assets,
                            delta_time,
                        ) {
                            entities.get::<iridium_core::LogState>().error(error);
                        }
                    });
            }
        },

        Mode::ParIter(_) => quote! {
            fn system(
                &self,
                state: Option<&iridium_ecs::ComponentBox>,
                entities: &iridium_ecs::Entities,
                assets: &iridium_assets::Assets,
                delta_time: f64,
            ) {
                use rayon::prelude::*;

                // Get the state as its real type.
                // The system can't mutate this directly as it's shared.
                #let_state_expr

                // Query the entities.
                iridium_ecs::query!(entities, [
                    #(mut #mutable_inputs_types, )*;
                    #(#immutable_inputs_types, )*
                ])
                    // Run the query stuff now.
                    .collect::<Vec<_>>()
                    // Create a parallel iterator.
                    .into_par_iter()
                    // For each entity with the given components.
                    .for_each(|components| {
                        // Run the system.
                        if let Err(error) = Self::system(
                            state,
                            entities,
                            components,
                            assets,
                            delta_time,
                        ) {
                            entities.get::<iridium_core::LogState>().error(error);
                        }
                    });
            }
        },
    };

    quote! {
        impl iridium_ecs::systems::System for #self_type {
            fn name(&self) -> &'static str {
                stringify!(#self_type)
            }

            fn state_type_id(&self) -> iridium_reflect::StableTypeId {
                <#state as iridium_reflect::HasStableTypeId>::stable_type_id()
            }

            fn default_state(&self) -> Option<iridium_ecs::ComponentBox> {
                #default_state_expr
            }

            fn required_components(&self) -> [Vec<iridium_reflect::StableTypeId>; 2] {
                [
                    vec![#(
                        <#mutable_inputs_types as iridium_reflect::HasStableTypeId>::stable_type_id(),
                    )*],
                    vec![#(
                        <#immutable_inputs_types as iridium_reflect::HasStableTypeId>::stable_type_id(),
                    )*],
                ]
            }

            #system_fn
        }
    }
    .to_string()
    .parse()
    .expect("Failed to parse the generated code.")
}

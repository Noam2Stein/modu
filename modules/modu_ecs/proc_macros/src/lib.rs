use proc_macro2::TokenStream;
use quote::quote;
use venial::{Function, Item};

#[proc_macro_derive(Component)]
pub fn derive_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_marker_trait(input, quote! { modu::ecs::Component })
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_marker_trait(input, quote! { modu::ecs::Resource })
}

#[proc_macro_derive(Parameter)]
pub fn derive_parameter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_marker_trait(input, quote! { modu::ecs::Parameter })
}

#[proc_macro_attribute]
pub fn system(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let Ok(Item::Function(item)) = venial::parse_item(item.into()) else {
        return quote! { compile_error!("expected a function declaration") }.into();
    };

    if !attr.is_empty() {
        return quote! { compile_error!("expected an empty attribute") }.into();
    }

    if item.return_ty.is_some() {
        return quote! { compile_error!("systems cannot return a value") }.into();
    }

    if item.qualifiers.extern_abi.is_some()
        || item.qualifiers.tk_default.is_some()
        || item.qualifiers.tk_extern.is_some()
    {
        return quote! { compile_error!("invalid function qualifiers") }.into();
    }

    if item.qualifiers.tk_unsafe.is_some() {
        return quote! { compile_error!("systems cannot be unsafe") }.into();
    }

    if item.qualifiers.tk_async.is_some() {
        return quote! { compile_error!("systems cannot be async") }.into();
    }

    let Function {
        attributes,
        vis_marker,
        name,
        generic_params,
        params,
        where_clause,
        body,
        ..
    } = &item;

    quote! {
        #[allow(non_snake_case)]
        #vis_marker fn #name #generic_params() -> impl modu::ecs::Systems #where_clause {
            #(#attributes)*
            fn _f #generic_params(#params) #where_clause #body

            modu::ecs::System {
                f: _f,
                marker: std::marker::PhantomData
            }
        }
    }
    .into()
}

fn derive_marker_trait(
    input: proc_macro::TokenStream,
    marker_trait: TokenStream,
) -> proc_macro::TokenStream {
    let item = venial::parse_item(input.into()).unwrap();

    match &item {
        Item::Struct(item) => {
            let generic_params = &item.generic_params;
            let name = &item.name;
            let inline_generic_args = &item.get_inline_generic_args();
            let where_clause = &item.where_clause;

            quote! {
                impl #generic_params #marker_trait for #name #inline_generic_args #where_clause {}
            }
        }
        Item::Enum(item) => {
            let generic_params = &item.generic_params;
            let name = &item.name;
            let inline_generic_args = &item.get_inline_generic_args();
            let where_clause = &item.where_clause;

            quote! {
                impl #generic_params #marker_trait for #name #inline_generic_args #where_clause {}
            }
        }
        Item::Union(item) => {
            let generic_params = &item.generic_params;
            let name = &item.name;
            let inline_generic_args = &item.get_inline_generic_args();
            let where_clause = &item.where_clause;

            quote! {
                impl #generic_params #marker_trait for #name #inline_generic_args #where_clause {}
            }
        }
        _ => quote! { compile_error!("unexpected item kind") },
    }
    .into()
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident, Result, Token, Type, Visibility, braced, parse::Parse, parse::ParseStream,
    parse_macro_input,
};

struct EventStructInput {
    visibility: Visibility,
    name: Ident,
    world_ty: Type,
    resources_ty: Type,
    events: Vec<(Ident, Type)>,
}

impl Parse for EventStructInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility = input.parse()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let world_ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let resources_ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;

        let content;
        braced!(content in input);

        let mut events = Vec::new();
        while !content.is_empty() {
            let field: Ident = content.parse()?;
            content.parse::<Token![:]>()?;
            let ty: Type = content.parse()?;
            events.push((field, ty));
            let _ = content.parse::<Token![,]>();
        }

        Ok(EventStructInput {
            visibility,
            name,
            world_ty,
            resources_ty,
            events,
        })
    }
}

pub fn create_event_structs(input: TokenStream) -> TokenStream {
    let EventStructInput {
        visibility,
        name,
        world_ty,
        resources_ty,
        events,
    } = parse_macro_input!(input as EventStructInput);

    let queue_struct = Ident::new(&format!("{}Queues", name), name.span());
    let handler_struct = Ident::new(&format!("{}Handlers", name), name.span());

    let queue_fields = events.iter().map(|(field, ty)| {
        quote! { pub #field: EventQueue<#ty>, }
    });

    let queue_defaults = events.iter().map(|(field, ty)| {
        quote! { #field: EventQueue::<#ty>::new(), }
    });

    let handler_fields = events.iter().map(|(field, ty)| {
        quote! { pub #field: EventHandlers<#ty, #world_ty, #resources_ty>, }
    });

    let dispatch_calls = events.iter().map(|(field, _)| {
        quote! {
            for event in resources.event_queues.#field.drain() {
                self.#field.dispatch(event, world, resources);
            }
        }
    });

    let expanded = quote! {
        #[derive(Default)]
        #visibility struct #handler_struct {
            #(#handler_fields)*
        }

        impl #handler_struct {
            pub fn dispatch_all(
                &self,
                world: &mut #world_ty,
                resources: &mut #resources_ty,
            ) {
                #(#dispatch_calls)*
            }
        }

        #visibility struct #queue_struct {
            #(#queue_fields)*
        }

        impl Default for #queue_struct {
            fn default() -> Self {
                Self {
                    #(#queue_defaults)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

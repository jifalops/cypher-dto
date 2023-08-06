use crate::{
    derive::{derive_serde, EntityType},
    stamps::Stamps,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

pub fn cypher_entity_impl(attr: TokenStream, input: TokenStream, typ: EntityType) -> TokenStream {
    let orig_attr = match typ {
        EntityType::Node => "node",
        EntityType::Relation => "relation",
    };
    let input = parse_macro_input!(input as DeriveInput);
    for attr in input.attrs.iter() {
        if attr.path().is_ident("stamps") {
            panic!("Use {}(stamps = \"...\") instead.", orig_attr);
        } else if attr.path().is_ident("name") {
            panic!("Use {}(name = \"...\") instead.", orig_attr);
        }
    }
    let args = parse_macro_input!(attr as EntityArgs);
    let stamps = args.stamps.map(|s| s.into_attribute()).unwrap_or(quote!());
    let name = args
        .name
        .map(|name| quote!(#[name = #name]))
        .unwrap_or(quote!());

    let serde = derive_serde();
    let derive = match typ {
        EntityType::Node => quote!(::cypher_dto::CypherNode),
        EntityType::Relation => quote!(::cypher_dto::CypherRelation),
    };
    quote! {
        #stamps
        #[derive(Clone, Debug, PartialEq, #derive, #serde)]
        #name
        #input
    }
    .into()
}
struct EntityArgs {
    stamps: Option<Stamps>,
    name: Option<String>,
}
impl syn::parse::Parse for EntityArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut stamps: Option<Stamps> = None;
        let mut name: Option<String> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                if ident == "stamps" {
                    // Check if there's an equals sign after "stamps"
                    if input.peek(syn::Token![=]) {
                        let _ = input.parse::<syn::Token![=]>()?;
                        stamps = Some(input.parse()?);
                    } else {
                        // If no value is provided, use the default value for "stamps"
                        stamps = Some(Stamps::Full);
                    }
                } else if ident == "name" {
                    let _ = input.parse::<syn::Token![=]>()?;
                    name = Some(input.parse::<LitStr>()?.value());
                } else {
                    return Err(lookahead.error());
                }

                // If there's a comma after this argument, parse it
                let _ = input.parse::<syn::Token![,]>().ok();
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(EntityArgs { stamps, name })
    }
}

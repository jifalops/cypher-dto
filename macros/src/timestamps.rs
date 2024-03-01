use std::error::Error;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, DeriveInput, Fields, Ident, LitStr, Result as SynResult, Type,
};

pub fn stamps_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let stamps = parse_macro_input!(args as Stamps);

    let name = &input.ident;
    let data = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("#[timestamps] can only be used with structs"),
    };
    let input_attrs = input.attrs;
    let input_vis = input.vis;

    let fields = match data {
        Fields::Named(fields) => fields.named.iter().map(|f| {
            let name = &f.ident;
            let ty = &f.ty;
            let vis = &f.vis;
            let attrs = &f.attrs;
            quote! {
                #(#attrs)*
                #vis #name: #ty,
            }
        }),
        _ => panic!("#[timestamps] can only be used on structs with named fields"),
    };
    let (stamp_idents, stamp_types) = stamps.into_fields();

    let gen = quote! {
        #(#input_attrs)*
        #input_vis struct #name {
            #(#fields)*
            #(#stamp_idents: #stamp_types,)*
        }
    };
    gen.into()
}

pub enum Stamps {
    /// created_at, updated_at
    Full,
    /// created, updated
    Short,
    CreatedAt,
    UpdatedAt,
    Created,
    Updated,
}
impl Parse for Stamps {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let arg = input
            .parse::<LitStr>()
            .ok()
            .map(|v| v.value())
            .unwrap_or("full".to_owned());
        Stamps::try_from(arg.as_ref()).map_err(|e| syn::Error::new(input.span(), e))
    }
}
impl TryFrom<&str> for Stamps {
    type Error = Box<dyn Error>;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "created" => Ok(Stamps::Created),
            "updated" => Ok(Stamps::Updated),
            "created_at" => Ok(Stamps::CreatedAt),
            "updated_at" => Ok(Stamps::UpdatedAt),
            "full" => Ok(Stamps::Full),
            "short" => Ok(Stamps::Short),
            _ => Err(format!("Invalid `#[timestamps]` argument: \"{}\". Allowed: full, short, created, updated, created_at, updated_at.", s).into()),
        }
    }
}
impl Stamps {
    fn into_fields(self) -> (Vec<Ident>, Vec<Type>) {
        let mut created: Option<Ident> = None;
        let mut updated: Option<Ident> = None;

        match self {
            Stamps::Created => created = Some(format_ident!("created")),
            Stamps::Updated => updated = Some(format_ident!("updated")),
            Stamps::CreatedAt => created = Some(format_ident!("created_at")),
            Stamps::UpdatedAt => updated = Some(format_ident!("updated_at")),
            Stamps::Full => {
                created = Some(format_ident!("created_at"));
                updated = Some(format_ident!("updated_at"));
            }
            Stamps::Short => {
                created = Some(format_ident!("created"));
                updated = Some(format_ident!("updated"));
            }
        }

        let mut idents = Vec::new();
        let mut types = Vec::new();

        if let Some(created) = created {
            idents.push(created);
            types.push(parse_quote!(Option<::chrono::DateTime<::chrono::Utc>>));
        }
        if let Some(updated) = updated {
            idents.push(updated);
            types.push(parse_quote!(Option<::chrono::DateTime<::chrono::Utc>>));
        }

        (idents, types)
    }
}

mod entity;
mod node;
mod relation;

pub use node::Node;
pub use relation::Relation;

use quote::{__private::TokenStream, quote};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, LitStr, Meta, Token,
};

#[cfg(feature = "serde")]
pub fn derive_serde() -> TokenStream {
    quote!(::serde::Serialize, ::serde::Deserialize)
}

#[cfg(not(feature = "serde"))]
pub fn derive_serde() -> TokenStream {
    quote!()
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EntityType {
    Node,
    Relation,
}

/// Get the value as in `#[name("Foo")]`
pub fn parse_name(attr: &Attribute) -> String {
    parse_name_meta(&attr.meta)
        .unwrap_or_else(|| panic!("Expected #[name = \"...\"] or #[name(\"...\")]."))
}

pub fn parse_name_meta(meta: &Meta) -> Option<String> {
    match meta {
        // Parse #[name("Foo")].
        Meta::List(list) => {
            if list.path.is_ident("name") {
                syn::parse2::<LitStr>(list.tokens.clone())
                    .map(|lit| lit.value())
                    .ok()
            } else {
                None
            }
        }
        // Parse #[name = "Foo"].
        Meta::NameValue(name_value) => {
            if name_value.path.is_ident("name") {
                let expr = &name_value.value;
                syn::parse2::<LitStr>(quote!(#expr))
                    .map(|lit| lit.value())
                    .ok()
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Get the labels as in `#[labels("Foo", "Bar")]`
pub fn parse_labels(attr: &Attribute) -> Vec<String> {
    parse_labels_meta(&attr.meta).unwrap_or_else(|| panic!("Expected #[labels(\"...\", \"...\")]."))
}

pub fn parse_labels_meta(meta: &Meta) -> Option<Vec<String>> {
    match meta {
        // Parse #[labels("Foo", "Bar")].
        Meta::List(list) => {
            if list.path.is_ident("labels") {
                match list.parse_args::<Labels>() {
                    Ok(labels) => Some(labels.0),
                    Err(e) => panic!("Expected a list of labels: {}", e),
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

struct Labels(Vec<String>);

impl Parse for Labels {
    fn parse(input: ParseStream) -> syn::Result<Labels> {
        let mut values = Vec::new();
        while !input.is_empty() {
            let lit_str = input.parse::<LitStr>()?;
            values.push(lit_str.value());
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(Labels(values))
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_parse_name() {
        let attr: Attribute = parse_quote!(#[name("Foo")]);
        assert_eq!(parse_name_meta(&attr.meta), Some("Foo".to_owned()));
        let attr: Attribute = parse_quote!(#[name = "Foo"]);
        assert_eq!(parse_name_meta(&attr.meta), Some("Foo".to_owned()));
    }
}

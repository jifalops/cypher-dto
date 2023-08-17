mod entity;
mod node;
mod relation;

pub use node::Node;
pub use relation::Relation;

use quote::{__private::TokenStream, quote};
use syn::{Attribute, LitStr, Meta};

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

use proc_macro::TokenStream;

mod derive;
mod timestamps;

use derive::{Node, Relation};
use syn::{parse_macro_input, DeriveInput};

/// Derives the [NodeEntity](::cypher_dto::NodeEntity) and related traits.
#[proc_macro_derive(Node, attributes(name, id))]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    Node::new(input).to_token_stream()
}

/// Derives the [RelationEntity](::cypher_dto::RelationEntity) and related traits.
#[proc_macro_derive(Relation, attributes(name, id))]
pub fn derive_relation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    Relation::new(input).to_token_stream()
}

/// Adds created/updated timestamp fields to a struct, using [`Option<DateTime<Utc>>`] as the type.
///
/// The default field names are `created_at` and `updated_at`, but can be changed
/// by passing a string to the `#[timestamps]` attribute.
///
/// On structs marked with `#[derive(Node)]` or `#[derive(Relation)]`,
/// their `::new()` implementation already checks for Optional timestamp fields will set them to `None`.
#[proc_macro_attribute]
pub fn timestamps(args: TokenStream, input: TokenStream) -> TokenStream {
    timestamps::stamps_impl(args, input)
}

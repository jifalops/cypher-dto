use proc_macro::TokenStream;

mod derive;
mod node_relation;
mod stamps;

use derive::{Node, Relation};
use syn::{parse_macro_input, DeriveInput};

/// Derives the [NodeEntity](cypher_dto::NodeEntity) and related traits.
#[proc_macro_derive(Node, attributes(name, id))]
pub fn derive_cypher_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    Node::new(input).to_token_stream()
}

/// Derives the [RelationEntity](cypher_dto::RelationEntity) and related traits.
#[proc_macro_derive(Relation, attributes(name, id))]
pub fn derive_cypher_relation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    Relation::new(input).to_token_stream()
}

/// Adds created/updated timestamp fields to a struct, using [`Option<DateTime<Utc>>`] as the type.
///
/// The default field names are `created_at` and `updated_at`, but can be changed by passing a string to the `#[stamps]` attribute.
///
/// On structs marked with `#[derive(Node)]` or `#[derive(Relation)]`,
/// their `::new()` implementation already checks for Optional timestamp fields will set them to `None`.
#[proc_macro_attribute]
pub fn stamps(args: TokenStream, input: TokenStream) -> TokenStream {
    stamps::stamps_impl(args, input)
}

/// Shorthand for `#[derive(Node)]`, `#[stamps]`, and other common derive implementations.
///
/// The default derives are `Clone`, `Debug`, `PartialEq`, and if the `serde` feature is enabled: `Serialize`, and `Deserialize`.
#[proc_macro_attribute]
pub fn node(args: TokenStream, input: TokenStream) -> TokenStream {
    node_relation::cypher_entity_impl(args, input, derive::EntityType::Node)
}

/// Shorthand for `#[derive(Relation)]`, `#[stamps]`, and other common derive implementations.
///
/// The default derives are `Clone`, `Debug`, `PartialEq`, and if the `serde` feature is enabled: `Serialize`, and `Deserialize`.
#[proc_macro_attribute]
pub fn relation(args: TokenStream, input: TokenStream) -> TokenStream {
    node_relation::cypher_entity_impl(args, input, derive::EntityType::Relation)
}

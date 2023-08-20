//! A low-level abstraction for working with serialization, storage, and IPC.
//! [Data Transfer Object](https://en.wikipedia.org/wiki/Data_transfer_object)
//!
//! It works with key-value pairs; only structs with named fields are supported.
//!
//! This library introduces the Identifier concept, that DTOs can be identified
//! by a subset of their fields. A [dto::WithId] is an "ordinary" DTO that has
//! an [dto::Identifier]. Identifiers are also DTOs, but they cannot have an
//! Identifier themselves.
//!
//! The `derive` macro will implement [dto::WithId] on the attributed struct,
//! and create a [dto::Identifier] struct named e.g. `FooId`. The ID struct will use the
//! same [typename()] as the attributed struct, and include any fields that are
//! marked `#[id]` on it.
//!
//! If no fields are marked `#[id]`, then the id field(s) are inferred:
//! - If there is a field named `id`, that will be the singular ID field in e.g. `FooId`.
//! - Otherwise, *all* fields from the attributed struct will also be on the ID struct.
//!
//! Dynamically added methods:
//! 1. `fn into_values(self)` - returns a tuple of all the values in the struct.
#![warn(missing_docs)]

mod entity;
mod error;
mod format;
mod node;
mod relationship;
mod stamps;

#[cfg(feature = "macros")]
pub use cypher_dto_macros::*;

pub use entity::{FieldSet, StampMode};
pub use error::Error;
pub use format::{format_param, format_query_fields};
pub use node::{NodeEntity, NodeId};
pub use relationship::{RelationBound, RelationEntity, RelationId};
pub use stamps::{Neo4jMap, Stamps};

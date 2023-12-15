use chrono::{DateTime, Utc};
use cypher_dto::Node;

/// Has a multi-valued ID, required timestamps, and public fields.
#[derive(Clone, Debug, PartialEq, Node)]
pub struct Company {
    #[id]
    pub name: String,
    #[id]
    pub state: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

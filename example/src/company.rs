use chrono::{DateTime, Utc};
use cypher_dto::CypherNode;

/// Has a multi-valued ID and required timestamps.
#[derive(Clone, Debug, PartialEq, CypherNode)]
pub struct Company {
    #[id]
    pub name: String,
    #[id]
    pub state: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

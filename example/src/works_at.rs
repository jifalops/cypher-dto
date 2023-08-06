use cypher_dto::CypherRelation;

/// A fieldless relation.
#[derive(Clone, Debug, PartialEq, CypherRelation)]
pub struct WorksAt {}

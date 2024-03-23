use cypher_dto::{Error, FieldSet, RelationEntity, RelationId, StampMode};
use neo4rs::{Query, Relation, Row, UnboundedRelation};

/// A fieldless relation.
#[derive(Clone, Debug, PartialEq)]
pub struct WorksAt {}
impl FieldSet for WorksAt {
    fn typename() -> &'static str {
        "WORKS_AT"
    }

    fn labels() -> &'static [&'static str] {
        &["WORKS_AT"]
    }

    fn field_names() -> &'static [&'static str] {
        &[]
    }
    fn as_query_fields() -> &'static str {
        ""
    }
    fn as_query_obj() -> &'static str {
        "WORKS_AT"
    }

    fn add_values_to_params(&self, query: Query, _: Option<&str>, _: StampMode) -> Query {
        query
    }
}
impl TryFrom<Row> for WorksAt {
    type Error = Error;
    fn try_from(_: Row) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
impl RelationEntity for WorksAt {
    type Id = WorksAtId;
    fn identifier(&self) -> Self::Id {
        WorksAtId {}
    }
}
impl TryFrom<Relation> for WorksAt {
    type Error = Error;
    fn try_from(_value: Relation) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
impl TryFrom<UnboundedRelation> for WorksAt {
    type Error = Error;
    fn try_from(_value: UnboundedRelation) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

//
// WorksAtId
//
pub struct WorksAtId {}
impl RelationId for WorksAtId {
    type T = WorksAt;
}
impl From<WorksAt> for WorksAtId {
    fn from(_value: WorksAt) -> Self {
        WorksAtId {}
    }
}
impl TryFrom<Relation> for WorksAtId {
    type Error = Error;
    fn try_from(_value: Relation) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
impl TryFrom<UnboundedRelation> for WorksAtId {
    type Error = Error;
    fn try_from(_value: UnboundedRelation) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
impl FieldSet for WorksAtId {
    fn typename() -> &'static str {
        WorksAt::typename()
    }

    fn labels() -> &'static [&'static str] {
        WorksAt::labels()
    }

    fn field_names() -> &'static [&'static str] {
        &[]
    }
    fn as_query_fields() -> &'static str {
        ""
    }
    fn as_query_obj() -> &'static str {
        "WORKS_AT"
    }
    fn add_values_to_params(&self, query: Query, _prefix: Option<&str>, _mode: StampMode) -> Query {
        query
    }
}
impl TryFrom<Row> for WorksAtId {
    type Error = Error;
    fn try_from(_value: Row) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

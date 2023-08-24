use chrono::{DateTime, Utc};
use cypher_dto::{format_param, Error, FieldSet, Neo4jMap, RelationEntity, RelationId, StampMode};
use neo4rs::{Query, Relation, Row, UnboundedRelation};

/// A relation with an ID field.
///
/// Can be used for multiple relations of the same type between the same nodes.
#[derive(Clone, Debug, PartialEq)]
pub struct WorkedAt {
    pub until: DateTime<Utc>,
}
impl FieldSet for WorkedAt {
    fn typename() -> &'static str {
        "WORKED_AT"
    }

    fn field_names() -> &'static [&'static str] {
        &["until"]
    }
    fn as_query_fields() -> &'static str {
        "until: $until"
    }
    fn as_query_obj() -> &'static str {
        "WORKED_AT { until: $until }"
    }

    fn add_values_to_params(&self, q: Query, prefix: Option<&str>, _mode: StampMode) -> Query {
        q.param(&format_param("until", prefix), self.until.fixed_offset())
    }
}
impl TryFrom<Row> for WorkedAt {
    type Error = Error;
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            until: Neo4jMap::Row(&value).get_timestamp("until")?,
        })
    }
}
impl RelationEntity for WorkedAt {
    type Id = WorkedAtId;
    fn identifier(&self) -> Self::Id {
        WorkedAtId { until: self.until }
    }
}
impl TryFrom<Relation> for WorkedAt {
    type Error = Error;
    fn try_from(value: Relation) -> Result<Self, Self::Error> {
        Ok(Self {
            until: Neo4jMap::Relation(&value).get_timestamp("until")?,
        })
    }
}
impl TryFrom<UnboundedRelation> for WorkedAt {
    type Error = Error;
    fn try_from(value: UnboundedRelation) -> Result<Self, Self::Error> {
        Ok(Self {
            until: Neo4jMap::UnboundedRelation(&value).get_timestamp("until")?,
        })
    }
}

//
// WorkedAtId
//

#[derive(Clone, Debug, PartialEq)]
pub struct WorkedAtId {
    until: DateTime<Utc>,
}
impl RelationId for WorkedAtId {
    type T = WorkedAt;
}
impl From<WorkedAt> for WorkedAtId {
    fn from(value: WorkedAt) -> Self {
        WorkedAtId { until: value.until }
    }
}
impl TryFrom<Relation> for WorkedAtId {
    type Error = Error;
    fn try_from(value: Relation) -> Result<Self, Self::Error> {
        Ok(Self {
            until: Neo4jMap::Relation(&value).get_timestamp("until")?,
        })
    }
}
impl TryFrom<UnboundedRelation> for WorkedAtId {
    type Error = Error;
    fn try_from(value: UnboundedRelation) -> Result<Self, Self::Error> {
        Ok(Self {
            until: Neo4jMap::UnboundedRelation(&value).get_timestamp("until")?,
        })
    }
}
impl FieldSet for WorkedAtId {
    fn typename() -> &'static str {
        WorkedAt::typename()
    }

    fn field_names() -> &'static [&'static str] {
        &["until"]
    }
    fn as_query_fields() -> &'static str {
        "until: $until"
    }
    fn as_query_obj() -> &'static str {
        "WORKED_AT { until: $until }"
    }
    fn add_values_to_params(&self, q: Query, prefix: Option<&str>, _mode: StampMode) -> Query {
        q.param(&format_param("until", prefix), self.until.fixed_offset())
    }
}
impl TryFrom<Row> for WorkedAtId {
    type Error = Error;
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            until: Neo4jMap::Row(&value).get_timestamp("until")?,
        })
    }
}

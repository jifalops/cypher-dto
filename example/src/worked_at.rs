use chrono::{DateTime, Utc};
use cypher_dto::CypherRelation;

/// A relation with an ID field.
///
/// Can be used for multiple relations of the same type between the same nodes.
#[derive(Clone, Debug, PartialEq, CypherRelation)]
pub struct WorkedAt {
    #[id]
    #[name("foo")]
    pub until: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use cypher_dto::{QueryFields, StampMode};

    #[test]
    fn rename() {
        assert_eq!(
            WorkedAt::as_query_fields(None, StampMode::Read),
            "foo: $foo"
        );
    }
}

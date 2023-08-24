use crate::{FieldSet, NodeEntity, NodeId, StampMode};
use neo4rs::{Query, Relation, UnboundedRelation};

/// A relationship entity.
pub trait RelationEntity: FieldSet + TryFrom<Relation> + TryFrom<UnboundedRelation> {
    type Id: RelationId<T = Self>;

    /// Get the [RelationId] for this entity.
    ///
    /// This is less efficient than using self.into(), but is useful when you
    /// don't want to consume the entity.
    ///
    /// The implementation in derive will clone the individual ID fields as
    /// necessary.
    fn identifier(&self) -> Self::Id;

    /// Convenience method for `self.into()`.
    fn into_identifier(self) -> Self::Id {
        self.into()
    }

    fn create<S: NodeEntity, E: NodeEntity>(
        &self,
        start: RelationBound<S>,
        end: RelationBound<E>,
    ) -> Query {
        let q = format!(
            r###"
          {}
          {}
          CREATE (s)-[:{}]->(e)
          "###,
            start.to_query_clause("s"),
            end.to_query_clause("e"),
            Self::to_query_obj(None, StampMode::Create)
        );
        // trace!("creating relation: {}", q);
        let mut q = Query::new(q);
        q = start.add_params(q, "s");
        q = end.add_params(q, "e");
        self.add_values_to_params(q, None, StampMode::Create)
    }

    /// Use only for relations that have one or more ID fields, otherwise use the other `update_` methods.
    ///
    /// This will update all relations of the same type if [FieldSet::field_names()] is empty.
    ///
    /// Treats the current values as the desired values and does a merge update (`SET r += ...`).
    ///
    /// NOTE: Does not support changing the identifier fields.
    fn update(&self) -> Query {
        assert!(!Self::Id::field_names().is_empty());
        let q = Query::new(format!(
            "MATCH ()-[r:{}]-()
             SET r += {{ {} }}",
            Self::Id::to_query_obj(None, StampMode::Read),
            Self::to_query_fields(None, StampMode::Update),
        ));
        self.add_values_to_params(q, None, StampMode::Update)
    }

    /// Treats the current values as the desired values and does a merge update (`SET r += ...`).
    ///
    /// NOTE: Does not support changing the identifier fields.
    fn update_from<T: NodeId>(&self, from: &T) -> Query {
        let mut q = Query::new(format!(
            "MATCH (n:{})-[r:{}]-()
             SET r += {{ {} }}",
            T::to_query_obj(Some("n"), StampMode::Read),
            Self::Id::to_query_obj(None, StampMode::Read),
            Self::to_query_fields(None, StampMode::Update),
        ));
        q = from.add_values_to_params(q, Some("n"), StampMode::Read);
        self.add_values_to_params(q, None, StampMode::Update)
    }

    /// Treats the current values as the desired values and does a merge update (`SET r += ...`).
    ///
    /// NOTE: Does not support changing the identifier fields.
    fn update_between<S: NodeId, E: NodeId>(&self, start: &S, end: &E) -> Query {
        let mut q = Query::new(format!(
            "MATCH (s:{})-[r:{}]-(e:{})
             SET r += {{ {} }}",
            S::to_query_obj(Some("s"), StampMode::Read),
            Self::Id::to_query_obj(None, StampMode::Read),
            E::to_query_obj(Some("e"), StampMode::Read),
            Self::to_query_fields(None, StampMode::Update),
        ));
        q = start.add_values_to_params(q, Some("s"), StampMode::Read);
        q = end.add_values_to_params(q, Some("e"), StampMode::Read);
        self.add_values_to_params(q, None, StampMode::Update)
    }
}

/// The identifying fields of a [RelationEntity].
pub trait RelationId:
    FieldSet + From<Self::T> + TryFrom<Relation> + TryFrom<UnboundedRelation>
{
    type T: RelationEntity<Id = Self>;

    /// Use only for relations that have one or more ID fields, otherwise use the other `read_` methods.
    ///
    /// This will read all relations of the same type if [FieldSet::field_names()] is empty.
    fn read(&self) -> Query {
        assert!(!Self::field_names().is_empty());
        let q = Query::new(format!(
            "MATCH [r:{}] RETURN r",
            Self::to_query_obj(None, StampMode::Read)
        ));
        self.add_values_to_params(q, None, StampMode::Read)
    }
    /// Reads relationship(s) connected to a specific node.
    fn read_from<T: NodeId>(&self, from: &T) -> Query {
        let mut q = Query::new(format!(
            "MATCH (n:{})-[r:{}]-()
             RETURN r",
            T::to_query_obj(Some("n"), StampMode::Read),
            Self::to_query_obj(None, StampMode::Read)
        ));
        q = from.add_values_to_params(q, Some("n"), StampMode::Read);
        self.add_values_to_params(q, None, StampMode::Read)
    }
    /// Reads relationship(s) connected between two specific nodes.
    fn read_between<S: NodeId, E: NodeId>(&self, start: &S, end: &E) -> Query {
        let mut q = Query::new(format!(
            "MATCH (s:{})-[r:{}]-(e:{})
             RETURN r",
            S::to_query_obj(Some("s"), StampMode::Read),
            Self::to_query_obj(None, StampMode::Read),
            E::to_query_obj(Some("e"), StampMode::Read),
        ));
        q = start.add_values_to_params(q, Some("s"), StampMode::Read);
        q = end.add_values_to_params(q, Some("e"), StampMode::Read);
        self.add_values_to_params(q, None, StampMode::Read)
    }
    /// Use only for relations that have one or more ID fields, otherwise use the other `delete_` methods.
    ///
    /// This will delete all relations of the same type if [FieldSet::field_names()] is empty.
    fn delete(&self) -> Query {
        assert!(!Self::field_names().is_empty());
        let q = Query::new(format!(
            "MATCH [r:{}] DELETE r",
            Self::to_query_obj(None, StampMode::Read)
        ));
        self.add_values_to_params(q, None, StampMode::Read)
    }
    /// Deletes relationship(s) connected to a specific node.
    fn delete_from<T: NodeId>(&self, from: &T) -> Query {
        let mut q = Query::new(format!(
            "MATCH (n:{})-[r:{}]-()
             DELETE r",
            T::to_query_obj(Some("n"), StampMode::Read),
            Self::to_query_obj(None, StampMode::Read)
        ));
        q = from.add_values_to_params(q, Some("n"), StampMode::Read);
        self.add_values_to_params(q, None, StampMode::Read)
    }
    /// Deletes relationship(s) connected between two specific nodes.
    fn delete_between<S: NodeId, E: NodeId>(&self, start: &S, end: &E) -> Query {
        let mut q = Query::new(format!(
            "MATCH (s:{})-[r:{}]-(e:{})
             DELETE r",
            S::to_query_obj(Some("s"), StampMode::Read),
            Self::to_query_obj(None, StampMode::Read),
            E::to_query_obj(Some("e"), StampMode::Read),
        ));
        q = start.add_values_to_params(q, Some("s"), StampMode::Read);
        q = end.add_values_to_params(q, Some("e"), StampMode::Read);
        self.add_values_to_params(q, None, StampMode::Read)
    }
}

/// When creating a relationship, the query has to MATCH, CREATE, or MERGE the
/// start and end nodes.
pub enum RelationBound<'a, T: NodeEntity> {
    Create(&'a T),
    Match(&'a T::Id),
    // TODO Cannot complete implementation due to add_params_to_query not having enough info.
    // Merge(T),
}
impl<'a, T: NodeEntity> RelationBound<'a, T> {
    /// Returns a CREATE (node:...) or MATCH (node:...) clause for this variant.
    pub fn to_query_clause(&self, prefix: &str) -> String {
        match self {
            RelationBound::Create(_) => format!(
                "CREATE ({}:{})",
                prefix,
                T::to_query_obj(Some(prefix), StampMode::Create)
            ),
            RelationBound::Match(_) => format!(
                "MATCH ({}:{})",
                prefix,
                T::Id::to_query_obj(Some(prefix), StampMode::Read)
            ),
            // RelationBound::Merge(_) => {
            //     format!(
            //         r###"
            //         MERGE ({prefix})
            //         ON CREATE
            //             SET {prefix} = {}
            //         ON MATCH
            //             SET {prefix} += {}
            //         "###,
            //         T::as_query_obj(Some(prefix), StampMode::Create),
            //         T::as_query_obj(Some(prefix), StampMode::Update),
            //     )
            // }
        }
    }
    pub fn add_params(&self, q: Query, prefix: &str) -> Query {
        match self {
            RelationBound::Create(t) => t.add_values_to_params(q, Some(prefix), StampMode::Create),
            RelationBound::Match(id) => id.add_values_to_params(q, Some(prefix), StampMode::Read),
            // TODO cannot know how to add params due to ON CREATE vs ON MATCH.
            // RelationBound::Merge(t) => {
            //     t.add_values_to_params(q, Some(prefix) /*not enough info */)
            // }
        }
    }
}

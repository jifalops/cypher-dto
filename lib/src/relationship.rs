use crate::{Entity, NodeEntity, NodeId, StampMode};
use neo4rs::{Query, Relation, UnboundedRelation};

/// A relationship [Entity].
pub trait RelationEntity: Entity + TryFrom<Relation> + TryFrom<UnboundedRelation> {
    type Id: RelationId<T = Self>;

    fn identifier(&self) -> Self::Id;
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
            start.to_line("s"),
            end.to_line("e"),
            Self::as_query_obj(None, StampMode::Create)
        );
        // trace!("creating relation: {}", q);
        let mut q = Query::new(q);
        q = start.add_params(q, "s");
        q = end.add_params(q, "e");
        self.add_values_to_params(q, None, StampMode::Create)
    }

    /// Use only for relations that have one or more ID fields, otherwise use the other `update_` methods.
    ///
    /// This will update all relations of the same type if [Id::field_names()] is empty.
    ///
    /// Treats the current values as the desired values and does a merge update (`SET r += ...`).
    ///
    /// NOTE: Does not support changing the identifier fields.
    fn update(&self) -> Query {
        assert!(!Self::Id::field_names().is_empty());
        let q = Query::new(format!(
            "MATCH ()-[r:{}]-()
             SET r += {{ {} }}",
            Self::Id::as_query_obj(None, StampMode::Read),
            Self::as_query_fields(None, StampMode::Update),
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
            T::as_query_obj(Some("n"), StampMode::Read),
            Self::Id::as_query_obj(None, StampMode::Read),
            Self::as_query_fields(None, StampMode::Update),
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
            S::as_query_obj(Some("s"), StampMode::Read),
            Self::Id::as_query_obj(None, StampMode::Read),
            E::as_query_obj(Some("e"), StampMode::Read),
            Self::as_query_fields(None, StampMode::Update),
        ));
        q = start.add_values_to_params(q, Some("s"), StampMode::Read);
        q = end.add_values_to_params(q, Some("e"), StampMode::Read);
        self.add_values_to_params(q, None, StampMode::Update)
    }
}

/// The identifying fields of a [RelationEntity].
pub trait RelationId:
    Entity + From<Self::T> + TryFrom<Relation> + TryFrom<UnboundedRelation>
{
    type T: RelationEntity;

    /// Use only for relations that have one or more ID fields, otherwise use the other `read_` methods.
    ///
    /// This will read all relations of the same type if [field_names()] is empty.
    fn read(&self) -> Query {
        assert!(!Self::field_names().is_empty());
        let q = Query::new(format!(
            "MATCH [r:{}] RETURN r",
            Self::as_query_obj(None, StampMode::Read)
        ));
        self.add_values_to_params(q, None, StampMode::Read)
    }
    /// Reads relationship(s) connected to a specific node.
    fn read_from<T: NodeId>(&self, from: &T) -> Query {
        let mut q = Query::new(format!(
            "MATCH (n:{})-[r:{}]-()
             RETURN r",
            T::as_query_obj(Some("n"), StampMode::Read),
            Self::as_query_obj(None, StampMode::Read)
        ));
        q = from.add_values_to_params(q, Some("n"), StampMode::Read);
        self.add_values_to_params(q, None, StampMode::Read)
    }
    /// Reads relationship(s) connected between two specific nodes.
    fn read_between<S: NodeId, E: NodeId>(&self, start: &S, end: &E) -> Query {
        let mut q = Query::new(format!(
            "MATCH (s:{})-[r:{}]-(e:{})
             RETURN r",
            S::as_query_obj(Some("s"), StampMode::Read),
            Self::as_query_obj(None, StampMode::Read),
            E::as_query_obj(Some("e"), StampMode::Read),
        ));
        q = start.add_values_to_params(q, Some("s"), StampMode::Read);
        q = end.add_values_to_params(q, Some("e"), StampMode::Read);
        self.add_values_to_params(q, None, StampMode::Read)
    }
    /// Use only for relations that have one or more ID fields, otherwise use the other `delete_` methods.
    ///
    /// This will delete all relations of the same type if [field_names()] is empty.
    fn delete(&self) -> Query {
        assert!(!Self::field_names().is_empty());
        let q = Query::new(format!(
            "MATCH [r:{}] DELETE r",
            Self::as_query_obj(None, StampMode::Read)
        ));
        self.add_values_to_params(q, None, StampMode::Read)
    }
    /// Deletes relationship(s) connected to a specific node.
    fn delete_from<T: NodeId>(&self, from: &T) -> Query {
        let mut q = Query::new(format!(
            "MATCH (n:{})-[r:{}]-()
             DELETE r",
            T::as_query_obj(Some("n"), StampMode::Read),
            Self::as_query_obj(None, StampMode::Read)
        ));
        q = from.add_values_to_params(q, Some("n"), StampMode::Read);
        self.add_values_to_params(q, None, StampMode::Read)
    }
    /// Deletes relationship(s) connected between two specific nodes.
    fn delete_between<S: NodeId, E: NodeId>(&self, start: &S, end: &E) -> Query {
        let mut q = Query::new(format!(
            "MATCH (s:{})-[r:{}]-(e:{})
             DELETE r",
            S::as_query_obj(Some("s"), StampMode::Read),
            Self::as_query_obj(None, StampMode::Read),
            E::as_query_obj(Some("e"), StampMode::Read),
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
    pub fn to_line(&self, prefix: &str) -> String {
        match self {
            RelationBound::Create(_) => format!(
                "CREATE ({}:{})",
                prefix,
                T::as_query_obj(Some(prefix), StampMode::Create)
            ),
            RelationBound::Match(_) => format!(
                "MATCH ({}:{})",
                prefix,
                T::Id::as_query_obj(Some(prefix), StampMode::Read)
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

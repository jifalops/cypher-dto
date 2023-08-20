use crate::{FieldSet, StampMode};
use neo4rs::{Node, Query};

/// A node [Entity].
pub trait NodeEntity: FieldSet + TryFrom<Node> {
    type Id: NodeId<T = Self>;

    /// Get the [NodeId] for this entity.
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

    fn create(&self) -> Query {
        let q = Query::new(format!(
            "CREATE (n:{})",
            Self::as_query_obj(None, StampMode::Create),
        ));
        self.add_values_to_params(q, None, StampMode::Create)
    }

    /// Treats the current values as the desired values and does a merge update (`SET n += ...`).
    ///
    /// NOTE: Does not support changing the identifier fields.
    fn update(&self) -> Query {
        let q = Query::new(format!(
            "MATCH (n:{}) SET n += {{ {} }}",
            Self::Id::as_query_obj(None, StampMode::Read),
            Self::as_query_fields(None, StampMode::Update),
        ));
        self.add_values_to_params(q, None, StampMode::Update)
    }
}

/// The identifying fields of a [NodeEntity].
pub trait NodeId: FieldSet + From<Self::T> + TryFrom<Node> {
    type T: NodeEntity<Id = Self>;

    /// Read a [NodeEntity] by its id, using "n" as the variable for the node.
    fn read(&self) -> Query {
        let q = Query::new(format!(
            "MATCH (n:{}) RETURN n",
            Self::as_query_obj(None, StampMode::Read)
        ));
        self.add_values_to_params(q, None, StampMode::Read)
    }

    /// Delete a [NodeEntity] by its id, using "n" as the variable for the node.
    fn delete(&self) -> Query {
        let q = Query::new(format!(
            "MATCH (n:{}) DETACH DELETE n",
            Self::as_query_obj(None, StampMode::Read)
        ));
        self.add_values_to_params(q, None, StampMode::Read)
    }
}

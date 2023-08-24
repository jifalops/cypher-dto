use chrono::{DateTime, Utc};
use cypher_dto::{format_param, Error, FieldSet, Neo4jMap, NodeEntity, NodeId, StampMode};
use neo4rs::{Node, Query, Row};

/// Has a multi-valued ID and required timestamps.
#[derive(Clone, Debug, PartialEq)]
pub struct Company {
    pub name: String,
    pub state: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
impl FieldSet for Company {
    fn typename() -> &'static str {
        "Company"
    }

    fn field_names() -> &'static [&'static str] {
        &["name", "state", "created", "updated"]
    }

    fn as_query_fields() -> &'static str {
        "name: $name, state: $state, created: $created, updated: $updated"
    }

    fn as_query_obj() -> &'static str {
        "Company { name: $name, state: $state, created: $created, updated: $updated }"
    }

    fn add_values_to_params(&self, mut q: Query, prefix: Option<&str>, mode: StampMode) -> Query {
        q = q.param(&format_param("name", prefix), self.name.clone());
        q = q.param(&format_param("state", prefix), self.state.clone());
        match mode {
            StampMode::Create => q,
            StampMode::Read => q
                .param(
                    &format_param("created", prefix),
                    self.created.fixed_offset(),
                )
                .param(
                    &format_param("updated", prefix),
                    self.updated.fixed_offset(),
                ),
            StampMode::Update => q.param(
                &format_param("created", prefix),
                self.created.fixed_offset(),
            ),
        }
    }
}
impl TryFrom<Row> for Company {
    type Error = Error;
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let map = Neo4jMap::Row(&value);
        Ok(Self {
            name: value
                .get("name")
                .ok_or(Error::MissingField("name".to_owned()))?,
            state: value
                .get("state")
                .ok_or(Error::MissingField("state".to_owned()))?,
            created: map.get_timestamp("created")?,
            updated: map.get_timestamp("updated")?,
        })
    }
}
impl NodeEntity for Company {
    type Id = CompanyId;
    fn identifier(&self) -> Self::Id {
        CompanyId {
            name: self.name.clone(),
            state: self.state.clone(),
        }
    }
}
impl TryFrom<Node> for Company {
    type Error = Error;
    fn try_from(value: Node) -> Result<Self, Self::Error> {
        let map = Neo4jMap::Node(&value);
        Ok(Self {
            name: value
                .get("name")
                .ok_or(Error::MissingField("name".to_owned()))?,
            state: value
                .get("state")
                .ok_or(Error::MissingField("state".to_owned()))?,
            created: map.get_timestamp("created")?,
            updated: map.get_timestamp("updated")?,
        })
    }
}

//
// CompanyId
//
#[derive(Clone, Debug, PartialEq)]
pub struct CompanyId {
    pub name: String,
    pub state: String,
}
impl NodeId for CompanyId {
    type T = Company;
}
impl From<Company> for CompanyId {
    fn from(value: Company) -> Self {
        CompanyId {
            name: value.name,
            state: value.state,
        }
    }
}
impl TryFrom<Node> for CompanyId {
    type Error = Error;
    fn try_from(value: Node) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value
                .get("name")
                .ok_or(Error::MissingField("name".to_owned()))?,
            state: value
                .get("state")
                .ok_or(Error::MissingField("state".to_owned()))?,
        })
    }
}
impl FieldSet for CompanyId {
    fn typename() -> &'static str {
        Company::typename()
    }

    fn field_names() -> &'static [&'static str] {
        &["name", "state"]
    }
    fn as_query_fields() -> &'static str {
        "name: $name, state: $state"
    }
    fn as_query_obj() -> &'static str {
        "Company { name: $name, state: $state }"
    }
    fn add_values_to_params(&self, query: Query, prefix: Option<&str>, _: StampMode) -> Query {
        query
            .param(&format_param("name", prefix), self.name.clone())
            .param(&format_param("state", prefix), self.state.clone())
    }
}
impl TryFrom<Row> for CompanyId {
    type Error = Error;
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value
                .get("name")
                .ok_or(Error::MissingField("name".to_owned()))?,
            state: value
                .get("state")
                .ok_or(Error::MissingField("state".to_owned()))?,
        })
    }
}

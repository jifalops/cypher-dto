use chrono::{DateTime, Utc};
use cypher_dto::{format_param, Error, FieldSet, Neo4jMap, NodeEntity, NodeId, StampMode};
use neo4rs::{Node, Query, Row};

/// Single ID field and optional timestamps. Has example of `new()` and `into_builder()` methods.
#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    id: String,
    name: String,
    age: Option<u8>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}
impl Person {
    pub fn new(id: &str, name: &str, age: Option<u8>) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
            age,
            created_at: None,
            updated_at: None,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn age(&self) -> Option<u8> {
        self.age
    }
    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at
    }
    pub fn updated_at(&self) -> Option<DateTime<Utc>> {
        self.updated_at
    }
    pub fn into_builder(self) -> PersonBuilder {
        self.into()
    }
}
impl FieldSet for Person {
    fn typename() -> &'static str {
        "Person"
    }

    fn field_names() -> &'static [&'static str] {
        &["id", "name", "age", "created_at", "updated_at"]
    }
    fn as_query_fields() -> &'static str {
        "id: $id, name: $name, age: $age, created_at: $created_at, updated_at: $updated_at"
    }
    fn as_query_obj() -> &'static str {
        "Person { id: $id, name: $name, age: $age, created_at: $created_at, updated_at: $updated_at }"
    }
    fn add_values_to_params(&self, mut q: Query, prefix: Option<&str>, mode: StampMode) -> Query {
        q = q.param(&format_param("id", prefix), self.id.clone());
        q = q.param(&format_param("name", prefix), self.name.clone());
        q = q.param(&format_param("age", prefix), self.age.map(u16::from));
        match mode {
            StampMode::Create => q,
            StampMode::Read => q
                .param(
                    &format_param("created_at", prefix),
                    self.created_at.map(|v| v.fixed_offset()),
                )
                .param(
                    &format_param("updated_at", prefix),
                    self.updated_at.map(|v| v.fixed_offset()),
                ),
            StampMode::Update => q.param(
                &format_param("created_at", prefix),
                self.created_at.map(|v| v.fixed_offset()),
            ),
        }
    }
}

impl TryFrom<Row> for Person {
    type Error = Error;
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let map = Neo4jMap::Row(&value);
        Ok(Self {
            id: value
                .get("id")
                .map_err(|_e| Error::MissingField("id".to_owned()))?,
            name: value
                .get("name")
                .map_err(|_e| Error::MissingField("name".to_owned()))?,
            age: match value.get::<i64>("age") {
                Ok(age) => Some(
                    age.try_into()
                        .map_err(|_| Error::TypeMismatch("age".to_owned()))?,
                ),
                Err(_) => None,
            },
            created_at: Some(map.get_timestamp("created_at")?),
            updated_at: Some(map.get_timestamp("updated_at")?),
        })
    }
}
impl NodeEntity for Person {
    type Id = PersonId;
    fn identifier(&self) -> Self::Id {
        PersonId {
            id: self.id.clone(),
        }
    }
}
impl TryFrom<Node> for Person {
    type Error = Error;
    fn try_from(value: Node) -> Result<Self, Self::Error> {
        let map = Neo4jMap::Node(&value);
        Ok(Self {
            id: value
                .get("id")
                .map_err(|_e| Error::MissingField("id".to_owned()))?,
            name: value
                .get("name")
                .map_err(|_e| Error::MissingField("name".to_owned()))?,
            age: match value.get::<i64>("age") {
                Ok(age) => Some(
                    age.try_into()
                        .map_err(|_| Error::TypeMismatch("age".to_owned()))?,
                ),
                Err(_) => None,
            },
            created_at: Some(map.get_timestamp("created_at")?),
            updated_at: Some(map.get_timestamp("updated_at")?),
        })
    }
}

//
// PersonId
//
#[derive(Clone, Debug, PartialEq)]
pub struct PersonId {
    pub id: String,
}
impl NodeId for PersonId {
    type T = Person;
}
impl From<Person> for PersonId {
    fn from(value: Person) -> Self {
        PersonId { id: value.id }
    }
}
impl TryFrom<Node> for PersonId {
    type Error = Error;
    fn try_from(value: Node) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value
                .get("id")
                .map_err(|_e| Error::MissingField("id".to_owned()))?,
        })
    }
}
impl FieldSet for PersonId {
    fn typename() -> &'static str {
        Person::typename()
    }

    fn field_names() -> &'static [&'static str] {
        &["id"]
    }
    fn as_query_fields() -> &'static str {
        "id: $id"
    }
    fn as_query_obj() -> &'static str {
        "Person { id: $id }"
    }
    fn add_values_to_params(&self, query: Query, prefix: Option<&str>, _: StampMode) -> Query {
        query.param(&format_param("id", prefix), self.id.clone())
    }
}
impl TryFrom<Row> for PersonId {
    type Error = Error;
    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value
                .get("id")
                .map_err(|_e| Error::MissingField("id".to_owned()))?,
        })
    }
}

//
// PersonBuilder
//
pub struct PersonBuilder {
    id: Option<String>,
    name: Option<String>,
    age: Option<u8>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}
impl PersonBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            age: None,
            created_at: None,
            updated_at: None,
        }
    }
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }
    pub fn created_at(mut self, created_at: Option<DateTime<Utc>>) -> Self {
        self.created_at = created_at;
        self
    }
    pub fn updated_at(mut self, updated_at: Option<DateTime<Utc>>) -> Self {
        self.updated_at = updated_at;
        self
    }
    pub fn build(self) -> Result<Person, Error> {
        self.try_into()
    }
}
impl Default for PersonBuilder {
    fn default() -> Self {
        Self::new()
    }
}
impl From<Person> for PersonBuilder {
    fn from(value: Person) -> Self {
        Self {
            id: Some(value.id),
            name: Some(value.name),
            age: value.age,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
impl TryFrom<PersonBuilder> for Person {
    type Error = Error;
    fn try_from(value: PersonBuilder) -> Result<Self, Self::Error> {
        Ok(Person {
            id: value
                .id
                .ok_or(Error::BuilderError("Person".to_owned(), "id".to_owned()))?,
            name: value
                .name
                .ok_or(Error::BuilderError("Person".to_owned(), "id".to_owned()))?,
            age: value.age,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}

# cypher-dto

A collection of traits and macros for working Data Transfer Objects (DTOs) in Cypher.

This library introduces an identifier concept to structs that represent
a node or relation in Neo4j.

It can help with translating structs (or the `<Foo>Id` counterpart) into Cypher queries.
However, it takes the stance that non trivial queries should be managed by hand.
So while it does provide basic CRUD queries ([`neo4rs::Query`]), for nodes and relations,
it also provides the building block methods for constructing your own queries more cleanly
(i.e. with `format!()`).

## Usage

```sh
cargo add cypher-dto
```

```rust
use cypher_dto::Node;

#[derive(Node)]
struct Person {
    id: String,       // Inferred as an #[id] field
    name: String,
    age: u8,
}

assert_eq!(Person::field_names(), &["id", "name", "age"]);
assert_eq!(Person::as_query_fields(), "id: $id, name: $name, age: $age");
assert_eq!(Person::as_query_obj(), "Person { id: $id, name: $name, age: $age }");

let alice = Person { id: "p1".to_owned(), name: "Alice".to_string(), age: 30 };

// Basic CRUD queries are built-in for node and relation structs.
let q: neo4rs::Query = alice.create();

// or manually...
let mut q2 =  Query::new(format!("CREATE (n:{})", Person::as_query_obj()));
q2 = alice.add_values_to_params(q2);

assert_eq!(q, q2);


#[derive(Relation)]
struct WorksAt {
    since: u32,
}

/// Multi-valued keys are supported.
#[derive(Node)]
struct Company {
  #[id]
  name: String,
  #[id]
  state: String,
  #[name = "zip_code"]
  zip: String,
}
assert_eq!(Company::as_query_obj(), "Company { name: $name, state: $state, zip_code: $zip_code }");

let acme = Company { name: "Acme LLC".to_owned(), state: "CA".to_owned(), zip: "90210".to_owned() };
let rel = WorksAt { since: 2010 };

let query = rel.create(RelationBound::Match(alice.into(), RelationBound::Create(acme)));
// MATCH (s:Person { id: $s_id })
// CREATE (e:Company { name: $e_name, state: $e_state, zip_code: $e_zip_code })
// CREATE (s)-[r:WORKS_AT { since: $since }]->(e)
```

An extra struct, `<StructName>Id`, is created for each derived `Node` or `Relation`.
It represents the zero-to-many valued key for uniquely identifying an entity.

- Nodes have one or more ID fields, and will default to using all fields if none are marked with `#[id]` or named `id`.
- Relations can have zero or more ID fields. Zero is the normal case where the relation is uniquely identified by its type and the nodes it connects. But keys may be added to relations as well. Relations default to no id fields, and simply use the `typename()`.

Lets load a user from their ID:

```rust
#[derive(Node)]
struct User {
  id: String,
  email: String,
}

let id = UserId { id: "u1".to_owned() };
let query = id.read();
// MATCH (n:User { id: $id })
// RETURN n
let stream = db.execute(query).await?;
let row = result.next().await?;
let node: neo4rs::Node = row.get("n").ok_or(...)?;
let user = User::try_from(node)?; // `neo4rs::{Row, Node, Relation, UnboundedRelation}` are supported.

let query = id.delete();
// MATCH (n:User { id: $id })
// DETACH DELETE n
```

### Updating: Merge and Replace

```rust
#[node]
struct User {
  id: String,
  email: String,
  phone: Option<String>
}
let user = User::new("u1".to_owned(), "abc@example.com".to_owned(), None);
let user2 = user.builder().email("foo@example.com").build();

let query = user2.update(UpdateMode::Merge);
// MATCH (n:User { id: $id })
// SET n += { id: $id, email: $email, phone: $phone }
let query = user2.update(UpdateMode::Replace);
// MATCH (n:User { id: $id })
// SET n = { id: $id, email: $email, phone: $phone }

// Note: Merge and Replace have the same effect in this example.
```

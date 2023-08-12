# cypher-dto

A collection of traits and macros for working Data Transfer Objects (DTOs) in Neo4j.

```sh
cargo add cypher-dto #--features serde
```

## Examples

### Basic usage

```rust
#[node]
struct Person {
    id: String,     // Inferred to be the only #[id] field.
    name: String,
    #[name = "zip_code"]
    zip: String,
}
assert_eq!(Person::typename(), "Person");
assert_eq!(Person::field_names(), &["id", "name", "zip_code"]);

// For building parameterized cypher queries...
assert_eq!(Person::as_query_fields(), "id: $id, name: $name, zip_code: $zip_code");
assert_eq!(Person::as_query_obj(), "Person { id: $id, name: $name, zip_code: $zip_code }");

let person = Person::new("123", "John", "12345");

// Unitary CRUD operations are provided for convenience.
let query: neo4rs::Query = person.create();

// Equivalent to:
let mut query = Query::new(format!(
    "CREATE (n:{})",
    Person::as_query_obj()
));
query = person.add_values_to_params(query);
```
```rust
#[relation]
struct Knows;
assert_eq!(Knows::typename(), "KNOWS");
```

### Multi valued identifiers

```rust
#[node]
struct Company {
  #[id]
  name: String,
  #[id]
  state: String,
  phone: String,
}
let company = Company::new("Acme", "CA", "555-555-5555");
let id = company.identifier();
assert_eq!(id.name(), "Acme");
assert_eq!(id.state(), "CA");
assert_eq!(id, CompanyId::new("Acme", "CA"));

assert_eq!(CompanyId::typename(), "Company");
assert_eq!(CompanyId::field_names(), &["name", "state"]);

let query: neo4rs::Query = id.read();
// Equivalent to:
let mut query = Query::new(format!(
    "MATCH (n:{}) RETURN n",
    CompanyId::as_query_obj()
));
query = id.add_values_to_params(query);
```

### Builder, new, and getters

* The generated `::new()` method will accept `&str` for `String` fields, and `&[T]` for `Vec<T>` fields.

* Doc comments are copied to the getters for the struct, the getter(s) on the `FooId` struct, and the methods on the `FooBuilder` struct.

```rust
#[node]
struct Person {
  /// This comment is copied to the getter, the Id getter, and the builder method.
  name: String,
}
let p = Person::new("John");
let p = p.into_builder().name("Ferris").build();
assert_eq!(p.name(), "Ferris");
```

### Using derive instead of `#[node]` or `#[relation]`

```rust
#[derive(Relation, Clone, Debug, PartialEq)]
struct Knows;

// Equivalent to:
#[relation]
struct Knows;

// Or, if the `serde` feature is enabled:
#[derive(Relation, Clone, Debug, PartialEq, Serialize, Deserialize)]
```

For more details about the macro variations, see the [cypher-dto-macros](https://crates.io/crates/cypher-dto-macros) crate.

### Timestamps

There's built-in support for special timestamp fields: `created_at` and `updated_at`, `created` and `updated`, or any single one of those four.

```rust
#[node(stamps)]
struct Person {
  name: String,
}
// Adds two fields:
//   created_at: Option<DateTime<Utc>>,
//   updated_at: Option<DateTime<Utc>>,

#[node(stamps = "short")]
struct Person {
  name: String,
}
// Adds two fields:
//   created: Option<DateTime<Utc>>,
//   updated: Option<DateTime<Utc>>,

#[node(stamps = "updated_at")]
struct Person {
  name: String,
}
// Adds one field:
//   updated_at: Option<DateTime<Utc>>,
```

The timestamp fields are treated a little bit differently than other fields:

* They are not parameters in the generated `::new()` method.
* They sometimes have hardcoded values in `::as_query_fields()`.
  * Calling `as_query_fields()` with `StampMode::Create` will use `datetime()` in the query instead of `$created_at` for example.

`Option<DateTime<Utc>>` is used instead of `DateTime<Utc>` so that the fields can be `None` when creating a new instance, before it exists in the database.

> NOTE: Support for Option params in the `neo4rs` crate is in master, but not yet released.
>
> You are welcome to depend on the master branch of this library to enable full support for timestamps.
>
> ```toml
> [dependencies]
> cypher-dto = { git = "https://github.com/jifalops/cypher-dto.git" }
> ```

### Unitary CRUD operations

This library takes the point of view that non-trivial queries should be managed by hand, but it does provide basic CRUD operations for convenience. Hopefully what you've seen so far shows how it can help create more readable complex queries.

`#[node]` and `#[relation]` structs get `create()` and `update()` methods, while the corresponding `FooId` structs get `read()` and `delete()` methods, all of which return a `neo4rs::Query`.

None of those methods even take any arguments, with the exception of creating a relation, which needs to know if the start and end nodes it's between need created or already exist.

```rust
#[node]
Person {
  name: String,
}

#[relation]
struct Knows;

let alice = Person::new("Alice");
let bob = Person::new("Bob");
let knows = Knows; // Relations can have fields and ids too of course.

let query = knows.create(RelationBound::Create(&alice), RelationBound::Create(&bob));
```

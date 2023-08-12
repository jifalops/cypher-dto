# cypher-dto-macros

Macros for working with the [cypher-dto](https://github.com/jifalops/cypher-dto/tree/main/lib) crate.

There are three types of macros contained:

1. Derive macros, which do most of the work.

    ```rust
    #[derive(Node)]
    struct Person {
      id: String,
      name: String,
      #[name = "zip_code"]
      zip: String,
    }

    #[derive(Relation)]
    struct Knows;
    ```

    These will implement `cypher_dto::{Entity, QueryFields, NodeEntity, RelationEntity}` for the structs.

    There are two helper attributes that come into scope when using the derive macros, `#[id]`, and `#[name]`.

    `#[id]` marks a field as being part of the entity's uniquely identifying set of fields. Multi-valued identifiers are supported this way. If none are specified and a field named `id` exists, that will be inferred to be the only id field. The set of id fields is given its own struct, e.g. `PersonId`, that can be obtained via `person.identifier()` or `person.into()`.

    `#[name]` allows the property name in the database to be different than on the struct. `#[name = "..."]` and `#[name("...")]` are supported, and can be applied to the struct as well.

    A builder is also generated for each struct, e.g. `PersonBuilder`. It can be obtained via `person.into_builder()`.

2. The `#[stamps]` macro, which will add one or two timestamp fields to the struct. By default it adds two fields, `created_at` and `updated_at`, with the type `Option<DateTime<Utc>>`. Optional timestamp fields let the `Person::new()` implementation skip having them as arguments, which is how you would create a DTO in application code before it is created in the database.

    You can control which fields it adds by specifying certain values as a string (`#[stamps = "..."]` and `#[stamps("...")]` are supported). The values must be ONE of `full` (the default), `short` (created, updated), `created_at`, `updated_at`, `created`, or `updated`.

3. The `#[node]` and `#[relation]` macros, which are a shorthand for the derive and stamps macros. The following are equivalent:

    ```rust
    #[node]
    struct Person {}

    #[derive(Node, Clone, Debug, PartialEq)]
    struct Person {}

    // Or, if using the `serde` feature:
    #[derive(Node, Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Person {}
    ```

    ```rust
    #[relation(stamps = "short")]
    struct Knows {}

    #[stamps = "short"]
    #[derive(Relation, Clone, Debug, PartialEq)]
    struct Knows {}
    ```

    ```rust
    #node[stamps, name = "Foo"]
    struct Person {}

    #[stamps]
    #[derive(Node, Clone, Debug, PartialEq)]
    #[name = "Foo"]
    struct Person {}
    ```

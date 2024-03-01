# Change log

## v0.3.0

- Upgrade to neo4rs 0.7.1
- Add a .devcontainer for easy development and contribution.
- Breaking: remove #[node] and #[relation] attributes
- Breaking: renamed the #[stamps] attribute to #[timestamps]
- Breaking: rename derive macros to `Node` and `Relation`.
- Breaking: renamed `as_query_fields()` to `to_query_fields()` and added a parameterless `as_query_fields()` that returns a static string.
- Breaking: renamed `as_query_obj()` to `to_query_obj()` and added a parameterless `as_query_obj()` that returns a static string.
- Breaking: the generated builder struct is now infallible. They no longer have `new()` or `default()` methods.
  This is a better fit for the intended use case, modifying an existing entity.
- `Entity` and `QueryFields` have been combined into one struct named `FieldSet`. This is not breaking for macros/derive, but is breaking if you were implementing them manually.
- Share some tests between docs and code.

## v0.2.0

- Fix handling of Option types in getters and parameters (#2)

## v0.1.1

- Update documentation

## v0.1.0

- First release

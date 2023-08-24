{
  #[derive(Node)]
  struct Person {
    name: String,
  }

  // Build a query string:
  let query = format!("CREATE (n:{})", Person::as_query_obj());
  assert_eq!(query, "CREATE (n:Person { name: $name })");

  // Use it in a [neo4rs::Query]:
  let mut query = neo4rs::Query::new(query);
  let person = Person::new("Alice");
  person.add_values_to_params(query);

  // A shorter way to do the same thing:
  let query = person.create();
}

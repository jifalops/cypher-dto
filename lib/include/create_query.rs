{
  #[derive(Node)]
  struct Person {
    name: String,
  }

  let person = Person::new("Alice");
  let query: neor4s::Query = person.create();

  // Or to build the same query manually:
  let query = format!("CREATE (n:{})", Person::as_query_obj());
  assert_eq!(query, "CREATE (n:Person { name: $name })");

  let mut query = neo4rs::Query::new(query);
  person.add_values_to_params(query);
}

{
  #[derive(Node)]
  struct Person {
    id: String,
    name: String,
  }

  // Query string:
  let query = format!("MATCH (n:{}) RETURN n", PersonId::as_query_obj());
  assert_eq!(query, "MATCH (n:Person { id: $id }) RETURN n");

  // Using [neo4rs::Query] manually:
  let mut query = neo4rs::Query::new(query);
  let id = PersonId::new("1234");
  id.add_values_to_params(query);

  // A shorter way to do the same thing:
  let query = id.read();
}

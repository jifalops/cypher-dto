{
  #[derive(Node)]
  struct Person {
    id: String,
    name: String,
  }

  let id = PersonId::new("1234");
  let query = id.read();

  // Or, to build the query manually:
  let query = format!("MATCH (n:{}) RETURN n", PersonId::as_query_obj());
  assert_eq!(query, "MATCH (n:Person { id: $id }) RETURN n");

  let mut query = neo4rs::Query::new(query);
  id.add_values_to_params(query);
}

{
  #[derive(Node)]
  struct Person {
    id: String,
    name: String,
  }

  let person = Person::new("1234", "Alice");

  // CREATE (n:Person { id: $id, name: $name })
  //
  // $id: "1234"
  // $name: "Alice"
  graph.run(person.create()).await.unwrap();

  // Find an existing person by id.
  let id = PersonId::new("1234");

  // MATCH (n:Person { id: $id }) RETURN n
  //
  // $id: "1234"
  let mut stream = graph.execute(id.read()).await.unwrap();

  let row = stream.next().await.unwrap().unwrap();
  let node: neo4rs::Node = row.get("n").unwrap();
  let person = Person::try_from(node).unwrap();
  assert_eq!(person.name(), "Alice");
}

{
  #[derive(Relation)]
  struct Knows {
    since: u16,
  }

  #[derive(Node)]
  struct Person {
    name: String,
  }

  let alice = Person::new("Alice");
  let knows = Knows::new(2017);
  let bob = Person::new("Bob");

  // CREATE (s:Person { name: $s_name })
  // CREATE (e:Person { name: $e_name })
  // CREATE (s)-[r:KNOWS { since: $since }]->(e)
  //
  // $s_name: "Alice"
  // $e_name: "Bob"
  // $since: 2017
  let query = knows.create(RelationBound::Create(&alice), RelationBound::Create(&bob));
  graph.run(query).await.unwrap();

  // Find the relationship just created.
  let id = KnowsId::new();

  // MATCH (s:Person { name: $s_name })-[r:KNOWS]-(e:Person { name: $e_name }) RETURN r
  //
  // $s_name: "Alice"
  // $e_name: "Bob"
  let query = id.read_between(&alice.into(), &bob.into());
  let mut stream = graph.execute(id.read()).await.unwrap();

  let row = stream.next().await.unwrap().unwrap();
  let relation: neo4rs::UnboundedRelation = row.get("r").unwrap();
  let knows = Knows::try_from(relation).unwrap();
  assert_eq!(knows.since(), 2017);
}

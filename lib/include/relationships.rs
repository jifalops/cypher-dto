{
  #[derive(Relation)]
  struct Knows {
    since: u16,
  }
  assert_eq!(Knows::typename(), "KNOWS");
  assert_eq!(Knows::field_names(), ["since"]);
  assert_eq!(Knows::as_query_fields(), "since: $since");
  assert_eq!(Knows::as_query_obj(), "KNOWS { since: $since }");

  #[derive(Node)]
  struct Person {
    name: String,
  }

  let alice = Person::new("Alice");
  let knows = Knows::new(2017);
  let bob = Person::new("Bob");

  // Create all three at once:
  let query = knows.create(RelationBound::Create(&alice), RelationBound::Create(&bob));

  // Or, manually build the same query:
  let query = format!(
    "CREATE (s:{}) \
    CREATE (e:{}) \
    CREATE (s)-[r:{}]->(e)",
    Person::to_query_obj(Some("s"), StampMode::Create),
    Knows::to_query_obj(None, StampMode::Create),
    Person::to_query_obj(Some("e"), StampMode::Create),
  );
  assert_eq!(
    query,
    "CREATE (s:Person { name: $s_name }) \
    CREATE (e:Person { name: $e_name }) \
    CREATE (s)-[:KNOWS { since: $since }]->(e)"
  );

  // Use it in a [neo4rs::Query]:
  let mut query = neo4rs::Query::new(query);
  alice.add_values_to_params(query, Some("s"), StampMode::Create); // Adds "s_name" to params
  knows.add_values_to_params(query, None, StampMode::Create);      // Adds "since" to params
  bob.add_values_to_params(query, Some("e"), StampMode::Create);   // Adds "e_name" to params
}

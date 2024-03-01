{
  #[derive(Node)]
  struct Person {
    id: String,
    name: String,
  }
  assert_eq!(Person::as_query_obj(), "Person { id: $id, name: $name }");
  assert_eq!(PersonId::as_query_obj(), "Person { id: $id }");
}

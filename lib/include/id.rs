{
  #[derive(Node)]
  struct Person {
    #[id]
    ssn: String,
    name: String,
  }
  assert_eq!(Person::as_query_obj(), "Person { ssn: $ssn, name: $name }");
  assert_eq!(PersonId::as_query_obj(), "Person { ssn: $ssn }");
}

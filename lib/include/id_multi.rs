{
  #[derive(Node)]
  struct Person {
    #[id]
    name: String,
    #[id]
    address: String,
  }

  assert_eq!(PersonId::as_query_obj(), "Person { name: $name, address: $address }");
}

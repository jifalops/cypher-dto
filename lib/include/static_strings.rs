{
  #[derive(Node)]
  struct Person {
    name: String,
  }
  assert_eq!(Person::typename(), "Person");
  assert_eq!(Person::field_names(), ["name"]);
  assert_eq!(Person::as_query_fields(), "name: $name");
  assert_eq!(Person::as_query_obj(), "Person { name: $name }");
}

{
  #[derive(Node)]
  #[name = "Person2"]
  struct Person {
    #[name = "name2"]
    name: String,
  }
  assert_eq!(Person::typename(), "Person2");
  assert_eq!(Person::field_names(), ["name2"]);
  assert_eq!(Person::as_query_fields(), "name2: $name2");
  assert_eq!(Person::as_query_obj(), "Person2 { name2: $name2 }");
}

use cypher_dto::{timestamps, Node, Relation};

/// Single ID field and optional timestamps. Has example of `new()` and `into_builder()` methods.
#[timestamps]
#[derive(Node, Clone)]
// #[name = "Person2"]
#[labels("Person2", "PersonExtraLabel")]
pub struct Person {
    id: String,
    #[name = "name2"]
    name: String,
    age: Option<u8>,
    /// Favorite colors
    colors: Vec<String>,
    photo_url: Option<String>,
}

#[derive(Relation)]
struct Knows;

#[cfg(test)]
mod tests {
    use cypher_dto::{FieldSet, NodeEntity, RelationBound, RelationEntity, StampMode};

    use super::*;

    #[test]
    fn person() {
        assert_eq!(Person::typename(), "Person2");
        assert_eq!(Person::labels(), &["Person2, PersonExtraLabel"]);
        assert_eq!(
            Person::field_names(),
            [
                "id",
                "name2",
                "age",
                "colors",
                "photo_url",
                "created_at",
                "updated_at"
            ]
        );
        assert_eq!(
            Person::as_query_fields(),
            "id: $id, name2: $name2, age: $age, colors: $colors, photo_url: $photo_url, created_at: $created_at, updated_at: $updated_at"
        );
        assert_eq!(
            Person::as_query_obj(),
            format!(
                "{} {{ {} }}",
                Person::labels().join(":"),
                Person::as_query_fields()
            )
        );
        assert_eq!(
            Person::as_query_obj(),
            Person::to_query_obj(None, StampMode::Read)
        );
        let p = Person::new(
            "id",
            "name",
            Some(42),
            &["red".to_owned(), "blue".to_owned()],
            None,
        );
        assert_eq!(p.id(), "id");
        let p = p.into_builder().name("name2").build();
        assert_eq!(p.name(), "name2");
        assert_eq!(p.colors(), &["red", "blue"]);
        assert_eq!(p.age(), Some(42));
        let now = chrono::Utc::now();

        assert_eq!(
            p.clone()
                .into_builder()
                .created_at(Some(now))
                .build()
                .created_at(),
            Some(&now),
        );
        let id: PersonId = p.identifier();
        let _ = Knows.create(
            RelationBound::Create(&p),
            RelationBound::Match::<Person>(&id),
        );
    }
}

use cypher_dto::{timestamps, Node, Relation};

/// Single ID field and optional timestamps. Has example of `new()` and `into_builder()` methods.
#[timestamps]
#[derive(Node, Clone)]
#[name = "Person2"]
pub struct Person {
    id: String,
    #[name = "name2"]
    name: String,
    age: Option<u8>,
    /// Favorite colors
    colors: Vec<String>,
}

#[derive(Relation)]
struct Knows;

#[cfg(test)]
mod tests {
    use cypher_dto::{FieldSet, NodeEntity, RelationBound, RelationEntity};

    use super::*;

    #[test]
    fn person() {
        assert_eq!(Person::typename(), "Person2");
        assert_eq!(
            Person::field_names(),
            vec!["id", "name2", "age", "colors", "created_at", "updated_at"]
        );
        let p = Person::new(
            "id",
            "name",
            Some(42),
            &["red".to_owned(), "blue".to_owned()],
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

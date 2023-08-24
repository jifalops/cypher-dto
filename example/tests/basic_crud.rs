mod common;

use chrono::Utc;
use common::*;
use cypher_dto::*;
use neo4rs::{Node, Query};

#[tokio::test]
async fn basic_crud() {
    let neo4j = Neo4jContainer::new().await;
    let graph = neo4j.graph();

    let alice = Person::new(&uuid(), "Alice", None, &[]);
    assert!(alice.created_at().is_none());
    assert!(alice.updated_at().is_none());

    let acme = Company {
        name: "Acme".to_owned(),
        state: "CA".to_owned(),
        created: Utc::now(),
        updated: Utc::now(),
    };
    let works_at = WorksAt {};

    // Create a node
    let bob = Person::new(
        &uuid(),
        "Bob",
        Some(42),
        &["red".to_owned(), "blue".to_owned()],
    );
    graph.run(bob.create()).await.unwrap();

    // Create a relationship and its nodes.
    let query: neo4rs::Query =
        works_at.create(RelationBound::Create(&alice), RelationBound::Create(&acme));
    graph.run(query).await.unwrap();

    // Read data back from the graph.
    let alice_id = alice.identifier();
    let mut stream = graph.execute(alice_id.read()).await.unwrap();
    let row = stream.next().await.unwrap().unwrap();
    // Nodes use "n" as a default variable name.
    let n: Node = row.get("n").unwrap();
    let alice = Person::try_from(n).unwrap();
    assert_eq!(alice.identifier(), alice_id);

    // Update Alice's name
    let alice = alice.into_builder().name("Allison").build();
    graph.run(alice.update()).await.unwrap();

    let mut stream = graph.execute(alice_id.read()).await.unwrap();
    let row = stream.next().await.unwrap().unwrap();
    let n: Node = row.get("n").unwrap();
    let alice = Person::try_from(n).unwrap();
    assert_eq!(alice.name(), "Allison");

    // Delete
    graph.run(acme.identifier().delete()).await.unwrap();

    let mut stream = graph
        .execute(Query::new(format!("MATCH (n:Company) RETURN n")))
        .await
        .unwrap();
    let row = stream.next().await.unwrap();
    assert!(row.is_none());
}

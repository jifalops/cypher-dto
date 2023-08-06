mod common;

use chrono::{TimeZone, Utc};
use common::*;
use cypher_dto::*;
use neo4rs::{Node, Query, Relation};

#[tokio::test]
async fn create_all_at_once() {
    let neo4j = Neo4jContainer::new().await;
    let graph = neo4j.graph();

    let alice = Person::new(&uuid(), "Alice", None, &[]);
    let bob = Person::new(
        &uuid(),
        "Bob",
        Some(42),
        &["red".to_owned(), "blue".to_owned()],
    );
    let acme = Company {
        name: "Acme".to_owned(),
        state: "CA".to_owned(),
        created: Utc::now(),
        updated: Utc::now(),
    };
    let worked_at = WorkedAt {
        until: Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap(),
    };
    let worked_at2 = WorkedAt {
        until: Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap(),
    };

    let mut q = Query::new(format!(
        "CREATE (alice:{})
        CREATE (bob:{})
        CREATE (acme:{})
        CREATE (alice)-[:WORKS_AT]->(acme)
        CREATE (bob)-[w:{}]->(acme)
        CREATE (bob)-[w2:{}]->(acme)
        RETURN alice, bob, acme, w, w2
        ",
        Person::as_query_obj(Some("alice"), StampMode::Create),
        Person::as_query_obj(Some("bob"), StampMode::Create),
        Company::as_query_obj(Some("acme"), StampMode::Create),
        WorkedAt::as_query_obj(Some("w"), StampMode::Create),
        WorkedAt::as_query_obj(Some("w2"), StampMode::Create),
    ));
    q = alice.add_values_to_params(q, Some("alice"), StampMode::Create);
    q = bob.add_values_to_params(q, Some("bob"), StampMode::Create);
    q = acme.add_values_to_params(q, Some("acme"), StampMode::Create);
    q = worked_at.add_values_to_params(q, Some("w"), StampMode::Create);
    q = worked_at2.add_values_to_params(q, Some("w2"), StampMode::Create);

    let mut stream = graph.execute(q).await.unwrap();
    let row = stream.next().await.unwrap().unwrap();
    assert!(stream.next().await.unwrap().is_none());

    let alice_db: Person = row.get::<Node>("alice").unwrap().try_into().unwrap();
    assert_eq!(alice.id(), alice_db.id());
    assert_eq!(alice.name(), alice_db.name());
    assert_eq!(alice.age(), alice_db.age());

    let bob_db: Person = row.get::<Node>("bob").unwrap().try_into().unwrap();
    assert_eq!(bob.id(), bob_db.id());
    assert_eq!(bob.name(), bob_db.name());
    assert_eq!(bob.age(), bob_db.age());

    let acme_db: Company = row.get::<Node>("acme").unwrap().try_into().unwrap();
    assert_eq!(acme.identifier(), acme_db.identifier());
    assert_ne!(acme.created, acme_db.created);

    let worked_at_db: WorkedAt = row.get::<Relation>("w").unwrap().try_into().unwrap();
    assert_eq!(worked_at, worked_at_db);

    let worked_at2_db: WorkedAt = row.get::<Relation>("w2").unwrap().try_into().unwrap();
    assert_eq!(worked_at2, worked_at2_db);
}

// #[tokio::test]
// async fn manual() {
//     let neo4j = Neo4jContainer::new().await;
//     let graph = neo4j.graph();

//     let alice = Person::new(&uuid(), "Alice", None);
//     let bob = Person::new(&uuid(), "Bob", Some(42));

//     // Built-in support for [Option<DateTime<Utc>>] created/updated fields.
//     assert!(alice.created_at().is_none());
//     assert!(alice.updated_at().is_none());

//     let acme = Company {
//         name: "Acme".to_owned(),
//         state: "CA".to_owned(),
//         created: Utc::now(),
//         updated: Utc::now(),
//     };
//     let works_at = WorksAt;
//     let worked_at = WorkedAt {
//         until: DateTime::from_str("2023").unwrap(),
//     };

//     // Create all three at once.
//     // Creating a relation is actually the most complex built-in query.
//     let query: neo4rs::Query =
//         works_at.create(RelationBound::Create(&alice), RelationBound::Create(&acme));
//     graph.run(query).await.unwrap();

//     // Create individually.
//     graph.run(bob.create()).await.unwrap();
//     let q = works_at.create(
//         RelationBound::Match::<Person>(&bob.identifier()),
//         RelationBound::Match::<Company>(&acme.identifier()),
//     );
//     graph.run(q).await.unwrap();

//     // Read data back from the graph.
//     let mut q = Query::new(format!(
//         "MATCH (p:{})-[w:WORKS_AT]->(c:{}) RETURN p, w, c",
//         Person::as_query_obj(Some("p"), StampMode::Read),
//         Company::as_query_obj(Some("c"), StampMode::Read)
//     ));
//     q = alice.add_values_to_params(q, Some("p"), StampMode::Read);
//     q = acme.add_values_to_params(q, Some("c"), StampMode::Read);

//     // Save for later
//     let alice_id = alice.identifier();
//     let acme_id = acme.identifier();

//     let mut stream = graph.execute(q).await.unwrap();
//     let row = stream.next().await.unwrap().unwrap();
//     let p = row.get::<Node>("p").unwrap();
//     let w = row.get::<Relation>("w").unwrap();
//     let c = row.get::<Node>("c").unwrap();
//     let alice = Person::try_from(p).unwrap();
//     let works_at = WorksAt::try_from(w).unwrap();
//     let acme = Company::try_from(c).unwrap();

//     // The correct entities were retrieved.
//     assert_eq!(alice.identifier(), alice_id);
//     assert_eq!(acme.identifier(), acme_id);
//     assert_eq!(works_at, WorksAt);
//     assert!(alice.created_at().is_some());
//     assert!(alice.updated_at().is_some());

//     // Custom queries are easy to read and write.
//     let mut q = Query::new(format!(
//         "MATCH (alice:{})
//         MATCH (bob:{})
//         MATCH (acme:{})
//         CREATE (alice)-[w:{}]->(acme)
//         CREATE (bob)-[w2:{}]->(acme)
//         RETURN alice, bob, acme, w.until AS history1, w2.until AS history2
//         ",
//         Person::as_query_obj(Some("alice"), StampMode::Read),
//         Person::as_query_obj(Some("bob"), StampMode::Read),
//         Company::as_query_obj(Some("acme"), StampMode::Read),
//         WorkedAt::as_query_obj(Some("w"), StampMode::Create),
//         WorkedAt::as_query_obj(Some("w2"), StampMode::Create),
//     ));
//     // It requires filling in the parameters though.
//     q = alice
//         .identifier()
//         .add_values_to_params(q, Some("alice"), StampMode::Read);
//     q = bob
//         .identifier()
//         .add_values_to_params(q, Some("bob"), StampMode::Read);
//     q = acme
//         .identifier()
//         .add_values_to_params(q, Some("acme"), StampMode::Read);
//     q = worked_at.add_values_to_params(q, Some("w"), StampMode::Create);
//     let worked_at2 = WorkedAt {
//         until: DateTime::from_str("2022").unwrap(),
//     };
//     q.param("w2_until", worked_at2.until.fixed_offset());

//     //
//     // Create nodes and relations individually.
//     //
//     let bob = Person::new(&uuid(), "Bob", Some(42));

//     // Use a custom query to read the relation and other node.
//     let mut q = Query::new(format!(
//         "MATCH (a:{})-[r:{}]->(b:{}) RETURN r, b",
//         // MATCH uses the <Entity>Id struct.
//         PersonId::as_query_obj(Some("a"), StampMode::Read),
//         WorksAtId::as_query_obj(None, StampMode::Read), // "WORKS_AT", but also works if there were ID field(s)
//         CompanyId::as_query_obj(Some("b"), StampMode::Read)
//     ));
//     q = alice_id.add_values_to_params(q, Some("a"), StampMode::Read);
//     q = acme_id.add_values_to_params(q, Some("b"), StampMode::Read);
//     // =================================================
//     // Equivalent to:
//     let id = works_at.into_identifier();
//     let _ = id.read_between(&alice_id, &acme_id);
//     // Except, this only uses "RETURN r" instead of "RETURN r, b".
//     // =================================================
//     let mut stream = graph.execute(q).await.unwrap();
//     let row = stream.next().await.unwrap().unwrap();
//     let relation = row.get::<Relation>("r").unwrap();
//     let node = row.get::<Node>("b").unwrap();
//     let acme = Company::try_from(node).unwrap();
//     let works_at = WorksAt::try_from(relation).unwrap();
//     assert_eq!(acme.name, "Acme");
//     assert_eq!(acme.state, "CA");
//     assert_eq!(works_at, WorksAt);

//     // assert_eq!(alice, alice2);
//     // assert_eq!(bob, bob2);
//     // assert_ne!(alice, bob);
//     // let q = knows.create(
//     //     RelationBound::Create(alice.clone()),
//     //     RelationBound::Create(bob.clone()),
//     // );
//     // graph.run(q).await.unwrap();
//     // let q = likes.create(
//     //     RelationBound::Match::<Person>(alice2.into()),
//     //     RelationBound::Match::<Person>(bob2.into()),
//     // );
//     // graph.run(q).await.unwrap();

//     // let mut q = Query::new(format!(
//     //     "MATCH (a:{})-[r]->(b:{}) RETURN a, r, b",
//     //     Person::as_query_obj(Some("a"), StampMode::Read),
//     //     Person::as_query_obj(Some("b"), StampMode::Read)
//     // ));
//     // q = alice.add_values_to_params(q, Some("a"), StampMode::Read);
//     // q = bob.add_values_to_params(q, Some("b"), StampMode::Read);
//     // let mut res = graph.execute(q).await.unwrap();
//     // while let Some(row) = res.next().await.unwrap() {
//     //     let a: Node = row.get("a").unwrap();
//     //     let b: Node = row.get("b").unwrap();
//     //     let r: Relation = row.get("r").unwrap();
//     //     let r2: UnboundedRelation = row.get("r").unwrap();
//     //     let a = Person::try_from(a).unwrap();
//     //     let b = Person::try_from(b).unwrap();
//     //     let r = Knows::try_from(r).unwrap();
//     //     assert_eq!(a, alice);
//     //     assert_eq!(b, bob);
//     //     assert_eq!(r, knows);
//     //     assert_eq!(Knows::try_from(r2).unwrap(), knows);
//     // }
// }

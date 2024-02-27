# Test on a running Neo4j instance instead of spinning up a container for each test.
export NEO4J_TEST_URI="bolt://db:7687"
export NEO4J_TEST_USER="neo4j"
export NEO4J_TEST_PASS="developer"
cargo test

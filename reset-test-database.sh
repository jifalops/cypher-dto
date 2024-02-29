curl -H accept:application/json -H content-type:application/json \
  "http://$NEO4J_TEST_USER:$NEO4J_TEST_PASS@db:7474/db/neo4j/tx/commit" \
  -d '{"statements": [{"statement": "MATCH (n) DETACH DELETE n;"}]}'
echo

mod container;
mod fixtures;

pub use container::Neo4jContainer;
pub use fixtures::*;

use uuid::Uuid;

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}

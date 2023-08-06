mod container;

pub use container::Neo4jContainer;
pub use example::*;

use uuid::Uuid;

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}

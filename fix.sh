set -e
cargo sort
cargo sort lib
cargo sort macros
cargo fmt
cargo fix --allow-dirty
cargo clippy --fix --allow-dirty

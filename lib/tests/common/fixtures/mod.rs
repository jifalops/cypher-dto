#![allow(dead_code)]
#![allow(unused_imports)]

//! Hand implementations for what the macros would generate.
mod company;
mod person;
mod worked_at;
mod works_at;

pub use company::*;
pub use person::*;
pub use worked_at::*;
pub use works_at::*;

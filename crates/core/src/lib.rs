#![allow(
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value,
    clippy::new_without_default,
    clippy::missing_const_for_fn,
    clippy::use_self
)]

pub mod domain;
pub mod errors;
pub mod ports;
pub mod templates;
pub mod usecases;
pub mod validation;

pub use domain::*;
pub use errors::DomainError;
pub use ports::*;
pub use usecases::*;
pub use validation::validate_schema;

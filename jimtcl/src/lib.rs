//! Rust API for embedding Jim TCL.

pub use jimtcl_sys as sys;
pub mod error;
pub mod object;
pub mod interp;
pub mod prelude;

pub use error::JimResult;

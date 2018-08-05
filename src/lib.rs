//! # Escargot: A Cargo API
//!
//! ## Why escargot
//!
//! Compared to depending on `cargo`:
//! - Faster compile times.
//! - Simpler API.
//! - Better interop with projects relying on other cargo versions.
//! - Probably slower execution, especially on platforms without an optimized `fork` (e.g. Windows).
//!
//! ## Relevant crates
//!
//! Other related crates:
//! * [cargo](https://crates.io/crates/cargo) for the real thing
//! * [cargo-metadata](https://crates.io/crates/cargo_metadata) for a similar project specifically geared to the `metadata` subcommand.
//!
//! # Example
//!
//! ```rust
//! escargot::CargoBuild::new()
//!     .bin("bin_fixture")
//!     .current_release()
//!     .current_target()
//!     .exec()
//!     .unwrap();
//! ```

#![warn(missing_docs)]

#[macro_use]
extern crate serde;
extern crate serde_json;

mod cargo;
pub use cargo::*;
mod build;
pub use build::*;
mod run;
pub use run::*;
mod test;
pub use test::*;
mod msg;
pub use msg::*;
mod error;
pub use error::*;

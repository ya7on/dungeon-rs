#![warn(clippy::all)]
#![warn(clippy::pedantic)]
// #![warn(clippy::nursery)] TODO: update code with this linting rule
#![warn(missing_docs)]
// #![warn(clippy::cargo)] Update Cargo.toml
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![forbid(unsafe_code)]

//! This is a library for transporting data.

mod adapter;
mod client;
mod error;
#[cfg(feature = "local")]
mod local;

pub use client::Transport;
pub use error::{TransportError, TransportResult};
#[cfg(feature = "local")]
pub use local::transport::{LocalState, LocalTransport};

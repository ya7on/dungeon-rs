#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![forbid(unsafe_code)]

//! Backend service for the dungeon game.
//!
//! This crate provides HTTP API endpoints for game management including
//! creating new games, applying player moves, and retrieving game state.

/// API endpoint handlers
pub mod api;

/// Application state management
pub mod state;

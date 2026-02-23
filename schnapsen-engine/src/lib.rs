//! Schnapsen game engine.
//!
//! Provides move validation, game state transitions, and AI strategies.

mod engine;

pub use engine::{apply_move, get_valid_moves, pick_random_move};

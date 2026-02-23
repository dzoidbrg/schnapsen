//! # Schnapsen Engine
//!
//! Game logic for Dreierschnapsen: move validation, application, and AI strategies.

mod engine;
mod moves;
mod validation;

pub use engine::{GameEngine, RandomEngine};
pub use moves::GameMove;
pub use validation::valid_moves;

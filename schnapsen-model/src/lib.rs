//! # Schnapsen Model
//!
//! Data structures for representing the Dreierschnapsen (Three-player Schnapsen) card game.
//!
//! This crate provides the core types for cards, suits, ranks, game variants, and game state
//! without any game logic.

mod card;
mod game;
mod game_type;

pub use card::{Card, Rank, Suit};
pub use game::{Bid, GameConfig, GamePhase, GameState, Hand, PlayerId, RoundState, Trick};
pub use game_type::{CardOrdering, GameType, GameTypeInfo};

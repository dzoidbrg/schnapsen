//! # Schnapsen Model
//!
//! Data structures representing Dreierschnapsen (three-player Schnapsen).
//! Based on the rules from the German Wikipedia.
//!
//! ## Quick Start
//!
//! ```rust
//! use schnapsen_model::{create_deck, Card};
//!
//! let deck = create_deck();
//! let cards: Vec<Card> = deck.into_iter().collect();
//! assert_eq!(cards.len(), 20);
//! ```

mod card;
mod game;
mod game_type;
mod player;
mod trick;

pub use card::{create_deck, Card, Rank, Suit};
pub use game::{deal_round, setup_round, GameConfig, GameError, GamePhase, GameState, Move};
pub use game_type::{FleckenLevel, GameType, GameTypePoints};
pub use player::Player;
pub use trick::Trick;


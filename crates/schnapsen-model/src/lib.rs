//! Core data structures for Dreierschnapsen.
//!
//! This crate intentionally focuses on **representation**:
//! - cards, players, declarations, and scoring metadata
//! - round/trick state containers
//! - legality helpers for card-play moves under Farb- and Stichzwang
//!
//! Full game-flow automation is left to `schnapsen-engine`.

pub mod card;
pub mod player;
pub mod scoring;
pub mod state;
pub mod variant;

pub use card::{compare_rank, full_deck, Card, Rank, RankOrder, Suit};
pub use player::PlayerId;
pub use scoring::{
    normal_game_base_points, score_round, BummerlMark, MatchScore, MatchScoringMode, PointAward,
    RoundScoringInput, RoundWinnerSide, Spritzen, SpritzenAction,
};
pub use state::{
    card_beats, Hand, MarriageAnnouncement, MarriageKind, PlayedCard, PlayerMove, RoundError,
    RoundPhase, RoundState, Trick,
};
pub use variant::{
    DeclarationInfo, DeclarerConstraint, GameDeclaration, GangTieBreak, RegionalOptions,
    RoundObjective, TrumpRule,
};

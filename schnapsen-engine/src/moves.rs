//! Game move types.

use schnapsen_model::{Card, GameType};

/// A legal action a player can take.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMove {
    /// Call trump suit (during TrumpCalling phase)
    CallTrump(schnapsen_model::Suit),
    /// Pass during bidding
    Pass,
    /// Declare a game type
    DeclareGame(GameType),
    /// Play a card
    PlayCard(Card),
    /// Flecken (double) - only when allowed and applicable
    Flecken,
    /// Declare Zwanziger (20 pts) - König+Ober of same suit
    AnnounceZwanziger,
    /// Declare Vierziger (40 pts) - König+Ober of trump
    AnnounceVierziger,
}

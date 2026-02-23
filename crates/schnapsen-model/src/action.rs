use crate::card::{Card, Suit};
use crate::game_type::GameType;

/// Actions a player can take during different phases of the game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Call a trump suit (Rufer's first action after seeing initial 3 cards).
    CallTrump(Suit),

    /// Flip a card to determine trump (when Rufer can't/won't choose from first 3).
    FlipForTrump(usize),

    /// Announce a game type during the bidding phase.
    AnnounceGame(GameAnnouncement),

    /// Pass during bidding (don't announce a game).
    Pass,

    /// Pick up the talon cards.
    TakeTalon,

    /// Discard two cards after picking up talon.
    DiscardToTalon(Card, Card),

    /// Play a card during a trick.
    PlayCard(Card),

    /// Announce a marriage (Zwanziger or Vierziger) before playing a card.
    AnnounceMarriage(Suit),

    /// Spritzen (double the stakes).
    Spritzen,

    /// Accept the Spritzen (continue playing at doubled stakes).
    AcceptSpritzen,

    /// Request a redeal (when dealt three Jacks).
    RequestRedeal,
}

/// A game announcement with optional "without talon" modifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameAnnouncement {
    pub game_type: GameType,
    pub without_talon: bool,
}

impl GameAnnouncement {
    pub fn new(game_type: GameType, without_talon: bool) -> Self {
        Self {
            game_type,
            without_talon,
        }
    }
}

/// The result of completing a single deal.
#[derive(Debug, Clone)]
pub struct DealResult {
    pub game_type: GameType,
    pub winner: Winner,
    pub points_awarded: u32,
}

/// Who won the deal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Winner {
    Announcer(crate::player::PlayerId),
    Defenders(crate::player::PlayerId, crate::player::PlayerId),
}

/// Outcome details for a normal game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalGameOutcome {
    /// Opponent got >= 33 points: winner gets 1 point
    OnePoint,
    /// Opponent got < 33 points: winner gets 2 points (Schneider)
    TwoPoints,
    /// Opponent got no tricks: winner gets 3 points
    ThreePoints,
}

impl NormalGameOutcome {
    pub fn points(&self) -> u32 {
        match self {
            NormalGameOutcome::OnePoint => 1,
            NormalGameOutcome::TwoPoints => 2,
            NormalGameOutcome::ThreePoints => 3,
        }
    }

    pub fn from_defender_points(defender_card_points: u32, defender_has_trick: bool) -> Self {
        if !defender_has_trick {
            NormalGameOutcome::ThreePoints
        } else if defender_card_points < 33 {
            NormalGameOutcome::TwoPoints
        } else {
            NormalGameOutcome::OnePoint
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_outcome_from_points() {
        assert_eq!(
            NormalGameOutcome::from_defender_points(0, false),
            NormalGameOutcome::ThreePoints
        );
        assert_eq!(
            NormalGameOutcome::from_defender_points(20, true),
            NormalGameOutcome::TwoPoints
        );
        assert_eq!(
            NormalGameOutcome::from_defender_points(33, true),
            NormalGameOutcome::OnePoint
        );
        assert_eq!(
            NormalGameOutcome::from_defender_points(50, true),
            NormalGameOutcome::OnePoint
        );
    }

    #[test]
    fn game_announcement() {
        let ann = GameAnnouncement::new(GameType::Gang, false);
        assert_eq!(ann.game_type, GameType::Gang);
        assert!(!ann.without_talon);
    }
}

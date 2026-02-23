//! A trick (Stich) - the cards played in one round.

use crate::card::{Card, Suit};
use crate::game_type::GameType;
use crate::player::Player;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A single trick: up to 3 cards (one from each player).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trick {
    /// Cards played, in order. Entry 0 = first player to play, etc.
    plays: Vec<(Player, Card)>,
}

impl Trick {
    /// Create an empty trick.
    pub fn new() -> Self {
        Self {
            plays: Vec::with_capacity(3),
        }
    }

    /// Add a card to the trick.
    pub fn add(&mut self, player: Player, card: Card) {
        self.plays.push((player, card));
    }

    /// Number of cards played so far.
    pub fn len(&self) -> usize {
        self.plays.len()
    }

    /// Whether the trick has no cards yet.
    pub fn is_empty(&self) -> bool {
        self.plays.is_empty()
    }

    /// Whether the trick is complete (all 3 played).
    pub fn is_complete(&self) -> bool {
        self.plays.len() == 3
    }

    /// All plays in the trick (player and card).
    pub fn plays(&self) -> &[(Player, Card)] {
        &self.plays
    }

    /// Cards in the trick (without player info).
    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.plays.iter().map(|(_, c)| c)
    }

    /// The lead suit (suit of first card).
    pub fn lead_suit(&self) -> Option<Suit> {
        self.plays.first().map(|(_, c)| c.suit)
    }

    /// Determine who won the trick given trump and game type.
    pub fn winner(&self, trump: Option<Suit>, game_type: GameType) -> Option<Player> {
        if self.plays.len() != 3 {
            return None;
        }
        let (leader, lead_card) = &self.plays[0];
        let mut winning_player = *leader;
        let mut winning_card = lead_card;

        for (player, card) in &self.plays[1..] {
            if card.beats(winning_card, trump, game_type) {
                winning_player = *player;
                winning_card = card;
            }
        }
        Some(winning_player)
    }

    /// Total points in the trick.
    pub fn points(&self) -> u16 {
        self.plays.iter().map(|(_, c)| c.points() as u16).sum()
    }
}

impl Default for Trick {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Trick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (_, card)) in self.plays.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", card)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn trick_points() {
        let mut trick = Trick::new();
        trick.add(Player::Rufer, Card::new(Suit::Hearts, Rank::Ass));
        trick.add(Player::LeftOpponent, Card::new(Suit::Hearts, Rank::Unter));
        trick.add(Player::RightOpponent, Card::new(Suit::Hearts, Rank::Ober));
        assert_eq!(trick.points(), 11 + 2 + 3);
    }
}

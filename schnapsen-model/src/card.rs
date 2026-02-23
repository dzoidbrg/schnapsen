//! Card representation for Schnapsen.
//!
//! Schnapsen uses 20 cards: 4 suits × 5 ranks.
//! Point values: Ass=11, Zehner=10, König=4, Ober=3, Unter=2.

use crate::game_type::GameType;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The four suits in Schnapsen.
/// French names: Herz (Hearts), Karo (Diamonds), Pik (Spades), Kreuz (Clubs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Suit {
    /// Hearts (Herz)
    Hearts,
    /// Diamonds (Karo, Schelle)
    Diamonds,
    /// Spades (Pik, Blatt, Grün, Laub)
    Spades,
    /// Clubs (Kreuz, Eichel)
    Clubs,
}

impl Suit {
    /// Returns all suits in standard order.
    pub const fn all() -> [Suit; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs]
    }

    /// Unicode symbol for display.
    pub const fn symbol(&self) -> char {
        match self {
            Suit::Hearts => '♥',
            Suit::Diamonds => '♦',
            Suit::Spades => '♠',
            Suit::Clubs => '♣',
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// The five ranks in each suit with their point values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rank {
    /// Unter/Bube (Jack) - 2 points
    Unter,
    /// Ober/Dame (Queen) - 3 points
    Ober,
    /// König (King) - 4 points
    Konig,
    /// Zehner (10) - 10 points
    Zehner,
    /// Ass (Ace) - 11 points
    Ass,
}

impl Rank {
    /// Point value of the card for scoring tricks.
    pub const fn points(&self) -> u8 {
        match self {
            Rank::Unter => 2,
            Rank::Ober => 3,
            Rank::Konig => 4,
            Rank::Zehner => 10,
            Rank::Ass => 11,
        }
    }

    /// All ranks in standard (low to high) order for normal games.
    pub const fn all() -> [Rank; 5] {
        [
            Rank::Unter,
            Rank::Ober,
            Rank::Konig,
            Rank::Zehner,
            Rank::Ass,
        ]
    }

    /// Display abbreviation.
    pub const fn short_name(&self) -> &'static str {
        match self {
            Rank::Unter => "U",
            Rank::Ober => "O",
            Rank::Konig => "K",
            Rank::Zehner => "10",
            Rank::Ass => "A",
        }
    }

    /// Relative rank for comparison in a specific game type.
    /// Returns 0 (lowest) to 4 (highest) within a suit.
    pub fn order_in_game(&self, game_type: GameType) -> u8 {
        use GameType::*;
        use Rank::*;
        match game_type {
            NormalesSpiel | Schnapser | Kontraschnapser | Bauernschnapser
            | Kontrabauernschnapser | Herrenschnapser | Farbringerl | Flecken
            | Bauernloch => {
                // Normal order: U < O < K < 10 < A
                match self {
                    Unter => 0,
                    Ober => 1,
                    Konig => 2,
                    Zehner => 3,
                    Ass => 4,
                }
            }
            Assenbettler | AssBettler | Zehnergang => {
                // Ass lowest: 10 > K > O > U > A
                match self {
                    Ass => 0,
                    Unter => 1,
                    Ober => 2,
                    Konig => 3,
                    Zehner => 4,
                }
            }
            Koenigsgang => {
                // Zehner lowest: K > O > U > A > 10
                match self {
                    Zehner => 0,
                    Ass => 1,
                    Unter => 2,
                    Ober => 3,
                    Konig => 4,
                }
            }
            Damengang | Gang | Bettler => {
                // König lowest: O > U > A > 10 > K
                match self {
                    Konig => 0,
                    Zehner => 1,
                    Ass => 2,
                    Unter => 3,
                    Ober => 4,
                }
            }
            Plauderer => {
                // Same as normal
                match self {
                    Unter => 0,
                    Ober => 1,
                    Konig => 2,
                    Zehner => 3,
                    Ass => 4,
                }
            }
        }
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order_in_game(GameType::NormalesSpiel)
            .cmp(&other.order_in_game(GameType::NormalesSpiel))
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

/// A Schnapsen card: suit + rank.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub const fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    /// Point value for trick scoring.
    pub const fn points(&self) -> u8 {
        self.rank.points()
    }

    /// Whether this card beats another in a trick, given trump and game type.
    pub fn beats(&self, other: &Card, trump: Option<Suit>, game_type: GameType) -> bool {
        if self.suit != other.suit {
            // Different suits: only trump matters (in games with trump)
            match trump {
                Some(trump_suit) => {
                    let self_trump = self.suit == trump_suit;
                    let other_trump = other.suit == trump_suit;
                    match (self_trump, other_trump) {
                        (true, false) => true,
                        (false, true) => false,
                        (true, true) => self.rank.order_in_game(game_type)
                            > other.rank.order_in_game(game_type),
                        (false, false) => false, // Non-trump cannot beat different suit
                    }
                }
                None => false, // No trump = must follow suit, so this shouldn't occur
            }
        } else {
            // Same suit: higher rank wins
            self.rank.order_in_game(game_type) > other.rank.order_in_game(game_type)
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

/// Create a full Schnapsen deck of 20 cards.
pub fn create_deck() -> [Card; 20] {
    use Rank::*;
    use Suit::*;
    [
        Card::new(Hearts, Unter),
        Card::new(Hearts, Ober),
        Card::new(Hearts, Konig),
        Card::new(Hearts, Zehner),
        Card::new(Hearts, Ass),
        Card::new(Diamonds, Unter),
        Card::new(Diamonds, Ober),
        Card::new(Diamonds, Konig),
        Card::new(Diamonds, Zehner),
        Card::new(Diamonds, Ass),
        Card::new(Spades, Unter),
        Card::new(Spades, Ober),
        Card::new(Spades, Konig),
        Card::new(Spades, Zehner),
        Card::new(Spades, Ass),
        Card::new(Clubs, Unter),
        Card::new(Clubs, Ober),
        Card::new(Clubs, Konig),
        Card::new(Clubs, Zehner),
        Card::new(Clubs, Ass),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GameType;

    #[test]
    fn deck_has_20_cards() {
        let deck = create_deck();
        assert_eq!(deck.len(), 20);
        let unique: std::collections::HashSet<_> = deck.iter().collect();
        assert_eq!(unique.len(), 20);
    }

    #[test]
    fn card_points() {
        assert_eq!(Rank::Ass.points(), 11);
        assert_eq!(Rank::Zehner.points(), 10);
        assert_eq!(Rank::Konig.points(), 4);
        assert_eq!(Rank::Ober.points(), 3);
        assert_eq!(Rank::Unter.points(), 2);
    }

    #[test]
    fn same_suit_beats() {
        let higher = Card::new(Suit::Hearts, Rank::Ass);
        let lower = Card::new(Suit::Hearts, Rank::Unter);
        assert!(higher.beats(
            &lower,
            Some(Suit::Spades),
            GameType::NormalesSpiel
        ));
        assert!(!lower.beats(
            &higher,
            Some(Suit::Spades),
            GameType::NormalesSpiel
        ));
    }

    #[test]
    fn trump_beats_non_trump() {
        let trump = Card::new(Suit::Hearts, Rank::Unter);
        let non_trump = Card::new(Suit::Spades, Rank::Ass);
        assert!(trump.beats(
            &non_trump,
            Some(Suit::Hearts),
            GameType::NormalesSpiel
        ));
    }
}

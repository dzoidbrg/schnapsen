//! Card, Suit, and Rank types for the Schnapsen deck.
//!
//! The game uses a 20-card deck: 4 suits × 5 ranks.

use std::fmt;

/// The four suits in a Schnapsen deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    /// Herz (Hearts) ♥
    Herz,
    /// Karo (Diamonds) ♦
    Karo,
    /// Pik (Spades) ♠
    Pik,
    /// Kreuz (Clubs) ♣
    Kreuz,
}

impl Suit {
    /// All four suits in canonical order.
    pub fn all() -> [Suit; 4] {
        [Suit::Herz, Suit::Karo, Suit::Pik, Suit::Kreuz]
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Herz => write!(f, "♥"),
            Suit::Karo => write!(f, "♦"),
            Suit::Pik => write!(f, "♠"),
            Suit::Kreuz => write!(f, "♣"),
        }
    }
}

/// The five ranks in each suit, with point values for scoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    /// Ass (Ace) – 11 points
    Ass,
    /// Zehner (Ten) – 10 points
    Zehner,
    /// König (King) – 4 points
    Koenig,
    /// Ober (Queen) – 3 points
    Ober,
    /// Unter (Jack) – 2 points
    Unter,
}

impl Rank {
    /// All five ranks in canonical order (highest to lowest for normal play).
    pub fn all() -> [Rank; 5] {
        [
            Rank::Ass,
            Rank::Zehner,
            Rank::Koenig,
            Rank::Ober,
            Rank::Unter,
        ]
    }

    /// Point value of this rank (Ass=11, Zehner=10, König=4, Ober=3, Unter=2).
    pub fn points(self) -> u8 {
        match self {
            Rank::Ass => 11,
            Rank::Zehner => 10,
            Rank::Koenig => 4,
            Rank::Ober => 3,
            Rank::Unter => 2,
        }
    }

    /// Ordinal for normal card ordering (0 = highest).
    /// Ass=0, Zehner=1, König=2, Ober=3, Unter=4
    pub fn ordinal_normal(self) -> u8 {
        match self {
            Rank::Ass => 0,
            Rank::Zehner => 1,
            Rank::Koenig => 2,
            Rank::Ober => 3,
            Rank::Unter => 4,
        }
    }

    /// Ordinal when Ass is lowest (Assenbettler, Zehnergang, Bauernloch).
    /// Zehner=0, König=1, Ober=2, Unter=3, Ass=4
    pub fn ordinal_ass_lowest(self) -> u8 {
        match self {
            Rank::Zehner => 0,
            Rank::Koenig => 1,
            Rank::Ober => 2,
            Rank::Unter => 3,
            Rank::Ass => 4,
        }
    }

    /// Ordinal when Zehner is lowest (Königsgang).
    /// König=0, Ober=1, Unter=2, Ass=3, Zehner=4
    pub fn ordinal_zehner_lowest(self) -> u8 {
        match self {
            Rank::Koenig => 0,
            Rank::Ober => 1,
            Rank::Unter => 2,
            Rank::Ass => 3,
            Rank::Zehner => 4,
        }
    }

    /// Ordinal when König is lowest (Damengang).
    /// Ober=0, Unter=1, Ass=2, Zehner=3, König=4
    pub fn ordinal_koenig_lowest(self) -> u8 {
        match self {
            Rank::Ober => 0,
            Rank::Unter => 1,
            Rank::Ass => 2,
            Rank::Zehner => 3,
            Rank::Koenig => 4,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Ass => write!(f, "A"),
            Rank::Zehner => write!(f, "10"),
            Rank::Koenig => write!(f, "K"),
            Rank::Ober => write!(f, "O"),
            Rank::Unter => write!(f, "U"),
        }
    }
}

/// A single playing card (suit + rank).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    /// Creates a new card.
    pub const fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    /// Point value of this card.
    pub fn points(self) -> u8 {
        self.rank.points()
    }

    /// Full 20-card Schnapsen deck.
    pub fn full_deck() -> Vec<Card> {
        let mut deck = Vec::with_capacity(20);
        for suit in Suit::all() {
            for rank in Rank::all() {
                deck.push(Card::new(suit, rank));
            }
        }
        deck
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_has_20_cards() {
        assert_eq!(Card::full_deck().len(), 20);
    }

    #[test]
    fn all_cards_unique() {
        let deck = Card::full_deck();
        for (i, a) in deck.iter().enumerate() {
            for (j, b) in deck.iter().enumerate() {
                if i != j {
                    assert!(a != b, "Duplicate card: {}", a);
                }
            }
        }
    }

    #[test]
    fn points_sum() {
        let deck = Card::full_deck();
        let total: u32 = deck.iter().map(|c| c.points() as u32).sum();
        assert_eq!(total, 120); // 4 * (11+10+4+3+2) = 4 * 30
    }
}

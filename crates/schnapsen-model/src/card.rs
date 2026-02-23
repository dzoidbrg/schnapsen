use std::fmt;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs]
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Spades => "♠",
            Suit::Clubs => "♣",
        }
    }

    pub fn german_name(&self) -> &'static str {
        match self {
            Suit::Hearts => "Herz",
            Suit::Diamonds => "Karo",
            Suit::Spades => "Pik",
            Suit::Clubs => "Kreuz",
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Rank {
    Ace,
    Ten,
    King,
    Queen,
    Jack,
}

impl Rank {
    pub fn all() -> [Rank; 5] {
        [Rank::Ace, Rank::Ten, Rank::King, Rank::Queen, Rank::Jack]
    }

    pub fn point_value(&self) -> u32 {
        match self {
            Rank::Ace => 11,
            Rank::Ten => 10,
            Rank::King => 4,
            Rank::Queen => 3,
            Rank::Jack => 2,
        }
    }

    /// Standard ordering strength (higher = stronger).
    pub fn standard_strength(&self) -> u8 {
        match self {
            Rank::Ace => 5,
            Rank::Ten => 4,
            Rank::King => 3,
            Rank::Queen => 2,
            Rank::Jack => 1,
        }
    }

    pub fn german_name(&self) -> &'static str {
        match self {
            Rank::Ace => "Ass",
            Rank::Ten => "Zehner",
            Rank::King => "König",
            Rank::Queen => "Ober",
            Rank::Jack => "Unter",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Ten => "10",
            Rank::King => "K",
            Rank::Queen => "Q",
            Rank::Jack => "J",
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn point_value(&self) -> u32 {
        self.rank.point_value()
    }

    /// Returns the strength of this card given a rank ordering.
    pub fn strength(&self, ordering: &RankOrdering) -> u8 {
        ordering.strength(self.rank)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

/// Determines rank ordering for different game types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankOrdering {
    /// Ass > Zehner > König > Ober > Unter
    Standard,
    /// Zehner > König > Ober > Unter > Ass (Assenbettler, Zehnergang, Bauernloch)
    AceLow,
    /// König > Ober > Unter > Ass > Zehner (Königsgang)
    TenLow,
    /// Ober > Unter > Ass > Zehner > König (Damengang)
    KingLow,
}

impl RankOrdering {
    pub fn strength(&self, rank: Rank) -> u8 {
        match self {
            RankOrdering::Standard => rank.standard_strength(),
            RankOrdering::AceLow => match rank {
                Rank::Ten => 5,
                Rank::King => 4,
                Rank::Queen => 3,
                Rank::Jack => 2,
                Rank::Ace => 1,
            },
            RankOrdering::TenLow => match rank {
                Rank::King => 5,
                Rank::Queen => 4,
                Rank::Jack => 3,
                Rank::Ace => 2,
                Rank::Ten => 1,
            },
            RankOrdering::KingLow => match rank {
                Rank::Queen => 5,
                Rank::Jack => 4,
                Rank::Ace => 3,
                Rank::Ten => 2,
                Rank::King => 1,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_point_values() {
        assert_eq!(Card::new(Suit::Hearts, Rank::Ace).point_value(), 11);
        assert_eq!(Card::new(Suit::Spades, Rank::Ten).point_value(), 10);
        assert_eq!(Card::new(Suit::Diamonds, Rank::King).point_value(), 4);
        assert_eq!(Card::new(Suit::Clubs, Rank::Queen).point_value(), 3);
        assert_eq!(Card::new(Suit::Hearts, Rank::Jack).point_value(), 2);
    }

    #[test]
    fn total_deck_points_is_120() {
        let total: u32 = Suit::all()
            .iter()
            .flat_map(|s| {
                let s = *s;
                Rank::all().into_iter().map(move |r| Card::new(s, r).point_value())
            })
            .sum();
        assert_eq!(total, 120);
    }

    #[test]
    fn standard_ordering() {
        let ord = RankOrdering::Standard;
        assert!(ord.strength(Rank::Ace) > ord.strength(Rank::Ten));
        assert!(ord.strength(Rank::Ten) > ord.strength(Rank::King));
        assert!(ord.strength(Rank::King) > ord.strength(Rank::Queen));
        assert!(ord.strength(Rank::Queen) > ord.strength(Rank::Jack));
    }

    #[test]
    fn ace_low_ordering() {
        let ord = RankOrdering::AceLow;
        assert!(ord.strength(Rank::Ten) > ord.strength(Rank::King));
        assert!(ord.strength(Rank::Jack) > ord.strength(Rank::Ace));
    }

    #[test]
    fn ten_low_ordering() {
        let ord = RankOrdering::TenLow;
        assert!(ord.strength(Rank::King) > ord.strength(Rank::Queen));
        assert!(ord.strength(Rank::Ace) > ord.strength(Rank::Ten));
    }

    #[test]
    fn king_low_ordering() {
        let ord = RankOrdering::KingLow;
        assert!(ord.strength(Rank::Queen) > ord.strength(Rank::Jack));
        assert!(ord.strength(Rank::Ten) > ord.strength(Rank::King));
    }

    #[test]
    fn card_display() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(format!("{card}"), "A♥");
    }
}

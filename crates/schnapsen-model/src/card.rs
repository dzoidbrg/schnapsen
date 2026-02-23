use core::cmp::Ordering;
use core::fmt;

/// The four suits used by Dreierschnapsen.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];

    pub const fn short_name(self) -> &'static str {
        match self {
            Suit::Hearts => "H",
            Suit::Diamonds => "D",
            Suit::Spades => "S",
            Suit::Clubs => "C",
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.short_name())
    }
}

/// The five ranks in a 20-card Schnapsen deck.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Rank {
    Ace,
    Ten,
    King,
    Ober,
    Unter,
}

impl Rank {
    pub const ALL: [Rank; 5] = [Rank::Ace, Rank::Ten, Rank::King, Rank::Ober, Rank::Unter];

    /// Card-point value used for 66-point calculations.
    pub const fn card_points(self) -> u8 {
        match self {
            Rank::Ace => 11,
            Rank::Ten => 10,
            Rank::King => 4,
            Rank::Ober => 3,
            Rank::Unter => 2,
        }
    }

    pub const fn short_name(self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Ten => "10",
            Rank::King => "K",
            Rank::Ober => "O",
            Rank::Unter => "U",
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.short_name())
    }
}

/// One Schnapsen card.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub const fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

/// Returns the full 20-card Schnapsen deck.
pub fn full_deck() -> Vec<Card> {
    let mut cards = Vec::with_capacity(20);
    for suit in Suit::ALL {
        for rank in Rank::ALL {
            cards.push(Card::new(suit, rank));
        }
    }
    cards
}

/// Variant-specific card strength ordering.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RankOrder {
    /// A > 10 > K > O > U
    Standard,
    /// 10 > K > O > U > A
    AceLow,
    /// K > O > U > A > 10
    Koenigsgang,
    /// O > U > A > 10 > K
    Damengang,
}

impl RankOrder {
    const fn precedence(self, rank: Rank) -> u8 {
        match (self, rank) {
            (RankOrder::Standard, Rank::Ace) => 5,
            (RankOrder::Standard, Rank::Ten) => 4,
            (RankOrder::Standard, Rank::King) => 3,
            (RankOrder::Standard, Rank::Ober) => 2,
            (RankOrder::Standard, Rank::Unter) => 1,

            (RankOrder::AceLow, Rank::Ten) => 5,
            (RankOrder::AceLow, Rank::King) => 4,
            (RankOrder::AceLow, Rank::Ober) => 3,
            (RankOrder::AceLow, Rank::Unter) => 2,
            (RankOrder::AceLow, Rank::Ace) => 1,

            (RankOrder::Koenigsgang, Rank::King) => 5,
            (RankOrder::Koenigsgang, Rank::Ober) => 4,
            (RankOrder::Koenigsgang, Rank::Unter) => 3,
            (RankOrder::Koenigsgang, Rank::Ace) => 2,
            (RankOrder::Koenigsgang, Rank::Ten) => 1,

            (RankOrder::Damengang, Rank::Ober) => 5,
            (RankOrder::Damengang, Rank::Unter) => 4,
            (RankOrder::Damengang, Rank::Ace) => 3,
            (RankOrder::Damengang, Rank::Ten) => 2,
            (RankOrder::Damengang, Rank::King) => 1,
        }
    }

    pub fn compare(self, left: Rank, right: Rank) -> Ordering {
        self.precedence(left).cmp(&self.precedence(right))
    }
}

/// Compares two ranks according to a selected order.
pub fn compare_rank(order: RankOrder, left: Rank, right: Rank) -> Ordering {
    order.compare(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn deck_contains_twenty_unique_cards() {
        let deck = full_deck();
        assert_eq!(deck.len(), 20);

        let unique: HashSet<Card> = deck.iter().copied().collect();
        assert_eq!(unique.len(), 20);
    }

    #[test]
    fn standard_order_places_ace_above_ten() {
        assert_eq!(
            compare_rank(RankOrder::Standard, Rank::Ace, Rank::Ten),
            Ordering::Greater
        );
    }

    #[test]
    fn ace_low_order_places_ace_below_unter() {
        assert_eq!(
            compare_rank(RankOrder::AceLow, Rank::Ace, Rank::Unter),
            Ordering::Less
        );
    }
}

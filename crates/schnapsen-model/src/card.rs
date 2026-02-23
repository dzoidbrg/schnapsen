/// The four suits in a Schnapsen deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,   // Herz    ♥
    Diamonds, // Karo    ♦
    Spades,   // Pik     ♠
    Clubs,    // Kreuz   ♣
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];

    pub fn symbol(self) -> &'static str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Spades => "♠",
            Suit::Clubs => "♣",
        }
    }

    pub fn name_de(self) -> &'static str {
        match self {
            Suit::Hearts => "Herz",
            Suit::Diamonds => "Karo",
            Suit::Spades => "Pik",
            Suit::Clubs => "Kreuz",
        }
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// The five ranks in a Schnapsen deck, ordered by standard rank strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Jack,  // Unter/Bauer  — 2 points
    Queen, // Ober/Dame    — 3 points
    King,  //              — 4 points
    Ten,   //              — 10 points
    Ace,   // Ass/Sau      — 11 points
}

impl Rank {
    pub const ALL: [Rank; 5] = [Rank::Jack, Rank::Queen, Rank::King, Rank::Ten, Rank::Ace];

    pub fn points(self) -> u8 {
        match self {
            Rank::Jack => 2,
            Rank::Queen => 3,
            Rank::King => 4,
            Rank::Ten => 10,
            Rank::Ace => 11,
        }
    }

    /// Standard strength ordering (0 = weakest). Used for trick comparison.
    pub fn standard_strength(self) -> u8 {
        match self {
            Rank::Jack => 0,
            Rank::Queen => 1,
            Rank::King => 2,
            Rank::Ten => 3,
            Rank::Ace => 4,
        }
    }

    /// Strength when Aces are lowest (Assenbettler, Zehnergang).
    /// Order: Ten > King > Queen > Jack > Ace
    pub fn ace_low_strength(self) -> u8 {
        match self {
            Rank::Ace => 0,
            Rank::Jack => 1,
            Rank::Queen => 2,
            Rank::King => 3,
            Rank::Ten => 4,
        }
    }

    /// Strength for Königsgang (Ten lowest).
    /// Order: King > Queen > Jack > Ace > Ten
    pub fn ten_low_strength(self) -> u8 {
        match self {
            Rank::Ten => 0,
            Rank::Ace => 1,
            Rank::Jack => 2,
            Rank::Queen => 3,
            Rank::King => 4,
        }
    }

    /// Strength for Damengang (King lowest).
    /// Order: Queen > Jack > Ace > Ten > King
    pub fn king_low_strength(self) -> u8 {
        match self {
            Rank::King => 0,
            Rank::Ten => 1,
            Rank::Ace => 2,
            Rank::Jack => 3,
            Rank::Queen => 4,
        }
    }

    pub fn short_name(self) -> &'static str {
        match self {
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ten => "10",
            Rank::Ace => "A",
        }
    }

    pub fn name_de(self) -> &'static str {
        match self {
            Rank::Jack => "Unter",
            Rank::Queen => "Ober",
            Rank::King => "König",
            Rank::Ten => "Zehner",
            Rank::Ace => "Ass",
        }
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

/// A single playing card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub const fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub fn points(self) -> u8 {
        self.rank.points()
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_points() {
        assert_eq!(Card::new(Suit::Hearts, Rank::Ace).points(), 11);
        assert_eq!(Card::new(Suit::Spades, Rank::Ten).points(), 10);
        assert_eq!(Card::new(Suit::Clubs, Rank::King).points(), 4);
        assert_eq!(Card::new(Suit::Diamonds, Rank::Queen).points(), 3);
        assert_eq!(Card::new(Suit::Hearts, Rank::Jack).points(), 2);
    }

    #[test]
    fn total_deck_points() {
        let total: u8 = Rank::ALL.iter().map(|r| r.points()).sum::<u8>() * 4;
        // Each suit has 2+3+4+10+11 = 30 points, 4 suits = 120
        assert_eq!(total, 120);
    }

    #[test]
    fn standard_strength_ordering() {
        assert!(Rank::Ace.standard_strength() > Rank::Ten.standard_strength());
        assert!(Rank::Ten.standard_strength() > Rank::King.standard_strength());
        assert!(Rank::King.standard_strength() > Rank::Queen.standard_strength());
        assert!(Rank::Queen.standard_strength() > Rank::Jack.standard_strength());
    }

    #[test]
    fn ace_low_strength_ordering() {
        assert!(Rank::Ten.ace_low_strength() > Rank::King.ace_low_strength());
        assert!(Rank::King.ace_low_strength() > Rank::Queen.ace_low_strength());
        assert!(Rank::Queen.ace_low_strength() > Rank::Jack.ace_low_strength());
        assert!(Rank::Jack.ace_low_strength() > Rank::Ace.ace_low_strength());
    }

    #[test]
    fn ten_low_strength_ordering() {
        assert!(Rank::King.ten_low_strength() > Rank::Queen.ten_low_strength());
        assert!(Rank::Queen.ten_low_strength() > Rank::Jack.ten_low_strength());
        assert!(Rank::Jack.ten_low_strength() > Rank::Ace.ten_low_strength());
        assert!(Rank::Ace.ten_low_strength() > Rank::Ten.ten_low_strength());
    }

    #[test]
    fn king_low_strength_ordering() {
        assert!(Rank::Queen.king_low_strength() > Rank::Jack.king_low_strength());
        assert!(Rank::Jack.king_low_strength() > Rank::Ace.king_low_strength());
        assert!(Rank::Ace.king_low_strength() > Rank::Ten.king_low_strength());
        assert!(Rank::Ten.king_low_strength() > Rank::King.king_low_strength());
    }

    #[test]
    fn card_display() {
        let c = Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(format!("{c}"), "A♥");
    }

    #[test]
    fn suit_symbols() {
        assert_eq!(Suit::Hearts.symbol(), "♥");
        assert_eq!(Suit::Diamonds.symbol(), "♦");
        assert_eq!(Suit::Spades.symbol(), "♠");
        assert_eq!(Suit::Clubs.symbol(), "♣");
    }
}

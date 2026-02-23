use crate::card::{Card, Suit};
use std::fmt;

/// Identifies a player at the table by seat position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerId {
    Player0,
    Player1,
    Player2,
}

impl PlayerId {
    pub fn all() -> [PlayerId; 3] {
        [PlayerId::Player0, PlayerId::Player1, PlayerId::Player2]
    }

    /// Next player in clockwise order.
    pub fn next(self) -> PlayerId {
        match self {
            PlayerId::Player0 => PlayerId::Player1,
            PlayerId::Player1 => PlayerId::Player2,
            PlayerId::Player2 => PlayerId::Player0,
        }
    }

    /// Previous player (counter-clockwise).
    pub fn prev(self) -> PlayerId {
        match self {
            PlayerId::Player0 => PlayerId::Player2,
            PlayerId::Player1 => PlayerId::Player0,
            PlayerId::Player2 => PlayerId::Player1,
        }
    }

    pub fn index(self) -> usize {
        match self {
            PlayerId::Player0 => 0,
            PlayerId::Player1 => 1,
            PlayerId::Player2 => 2,
        }
    }

    pub fn from_index(i: usize) -> PlayerId {
        match i % 3 {
            0 => PlayerId::Player0,
            1 => PlayerId::Player1,
            _ => PlayerId::Player2,
        }
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player {}", self.index())
    }
}

/// Role assigned for the current deal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Dealer,
    Caller,
    Third,
}

/// A player's hand of cards.
#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn from_cards(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_many(&mut self, cards: &[Card]) {
        self.cards.extend_from_slice(cards);
    }

    pub fn remove(&mut self, card: &Card) -> Option<Card> {
        if let Some(pos) = self.cards.iter().position(|c| c == card) {
            Some(self.cards.remove(pos))
        } else {
            None
        }
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn cards_of_suit(&self, suit: Suit) -> Vec<Card> {
        self.cards.iter().filter(|c| c.suit == suit).copied().collect()
    }

    /// Check if hand contains both King and Queen of the given suit (marriage pair).
    pub fn has_marriage(&self, suit: Suit) -> bool {
        let has_king = self.cards.iter().any(|c| c.suit == suit && c.rank == crate::card::Rank::King);
        let has_queen = self.cards.iter().any(|c| c.suit == suit && c.rank == crate::card::Rank::Queen);
        has_king && has_queen
    }

    /// Returns suits for which the player holds a complete marriage (King + Queen).
    pub fn marriages(&self) -> Vec<Suit> {
        Suit::all()
            .iter()
            .filter(|s| self.has_marriage(**s))
            .copied()
            .collect()
    }

    /// Check if the hand contains all 5 cards of a given suit.
    pub fn has_full_suit(&self, suit: Suit) -> bool {
        self.cards_of_suit(suit).len() == 5
    }

    pub fn sort_by_suit_and_rank(&mut self) {
        self.cards.sort_by(|a, b| {
            let suit_ord = (a.suit as u8).cmp(&(b.suit as u8));
            if suit_ord == std::cmp::Ordering::Equal {
                b.rank.standard_strength().cmp(&a.rank.standard_strength())
            } else {
                suit_ord
            }
        });
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

/// Full player state for a deal.
#[derive(Debug, Clone)]
pub struct PlayerState {
    pub id: PlayerId,
    pub role: Role,
    pub hand: Hand,
    pub tricks_won: Vec<Vec<Card>>,
    pub game_points: u32,
    pub bummerl: u32,
}

impl PlayerState {
    pub fn new(id: PlayerId, role: Role) -> Self {
        Self {
            id,
            role,
            hand: Hand::new(),
            tricks_won: Vec::new(),
            game_points: 0,
            bummerl: 0,
        }
    }

    pub fn card_points(&self) -> u32 {
        self.tricks_won
            .iter()
            .flat_map(|trick| trick.iter())
            .map(|card| card.point_value())
            .sum()
    }

    pub fn trick_count(&self) -> usize {
        self.tricks_won.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn player_id_rotation() {
        assert_eq!(PlayerId::Player0.next(), PlayerId::Player1);
        assert_eq!(PlayerId::Player1.next(), PlayerId::Player2);
        assert_eq!(PlayerId::Player2.next(), PlayerId::Player0);
        assert_eq!(PlayerId::Player0.prev(), PlayerId::Player2);
    }

    #[test]
    fn hand_operations() {
        let mut hand = Hand::new();
        let card = Card::new(Suit::Hearts, Rank::Ace);
        hand.add(card);
        assert_eq!(hand.len(), 1);
        assert!(hand.contains(&card));

        let removed = hand.remove(&card);
        assert_eq!(removed, Some(card));
        assert!(hand.is_empty());
    }

    #[test]
    fn hand_marriage_detection() {
        let hand = Hand::from_cards(vec![
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Spades, Rank::Ace),
        ]);
        assert!(hand.has_marriage(Suit::Hearts));
        assert!(!hand.has_marriage(Suit::Spades));
        assert_eq!(hand.marriages(), vec![Suit::Hearts]);
    }

    #[test]
    fn hand_full_suit() {
        let hand = Hand::from_cards(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Hearts, Rank::Ten),
            Card::new(Suit::Hearts, Rank::King),
            Card::new(Suit::Hearts, Rank::Queen),
            Card::new(Suit::Hearts, Rank::Jack),
        ]);
        assert!(hand.has_full_suit(Suit::Hearts));
        assert!(!hand.has_full_suit(Suit::Spades));
    }

    #[test]
    fn player_card_points() {
        let mut player = PlayerState::new(PlayerId::Player0, Role::Caller);
        player.tricks_won.push(vec![
            Card::new(Suit::Hearts, Rank::Ace),
            Card::new(Suit::Spades, Rank::Ten),
            Card::new(Suit::Diamonds, Rank::Jack),
        ]);
        assert_eq!(player.card_points(), 11 + 10 + 2);
    }
}

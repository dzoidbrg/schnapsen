use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::Rng;

/// A standard 20-card Schnapsen deck.
#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(20);
        for suit in Suit::all() {
            for rank in Rank::all() {
                cards.push(Card::new(suit, rank));
            }
        }
        Self { cards }
    }

    pub fn shuffle(&mut self, rng: &mut impl Rng) {
        self.cards.shuffle(rng);
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Deal cards for Dreierschnapsen:
    /// Returns (first_three_for_each_player, talon, second_three_for_each_player).
    ///
    /// Layout: 3 cards to each of 3 players, 2 to talon, 3 more to each player.
    pub fn deal(&self) -> DealResult {
        assert_eq!(self.cards.len(), 20, "Deck must have exactly 20 cards");

        let first = [
            [self.cards[0], self.cards[1], self.cards[2]],
            [self.cards[3], self.cards[4], self.cards[5]],
            [self.cards[6], self.cards[7], self.cards[8]],
        ];

        let talon = [self.cards[9], self.cards[10]];

        let second = [
            [self.cards[11], self.cards[12], self.cards[13]],
            [self.cards[14], self.cards[15], self.cards[16]],
            [self.cards[17], self.cards[18], self.cards[19]],
        ];

        DealResult {
            first_three: first,
            talon,
            second_three: second,
        }
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of dealing cards to 3 players.
#[derive(Debug, Clone)]
pub struct DealResult {
    /// First 3 cards for each player (caller gets index 0).
    pub first_three: [[Card; 3]; 3],
    /// The 2 talon cards.
    pub talon: [Card; 2],
    /// Second 3 cards for each player.
    pub second_three: [[Card; 3]; 3],
}

impl DealResult {
    /// Get all 6 cards for a given player index (0=caller, 1=next, 2=dealer).
    pub fn full_hand(&self, player_index: usize) -> Vec<Card> {
        let mut cards = Vec::with_capacity(6);
        cards.extend_from_slice(&self.first_three[player_index]);
        cards.extend_from_slice(&self.second_three[player_index]);
        cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_has_20_cards() {
        let deck = Deck::new();
        assert_eq!(deck.cards().len(), 20);
    }

    #[test]
    fn deck_has_unique_cards() {
        let deck = Deck::new();
        let mut seen = std::collections::HashSet::new();
        for card in deck.cards() {
            assert!(seen.insert(*card), "Duplicate card: {card}");
        }
    }

    #[test]
    fn deal_distributes_all_cards() {
        let deck = Deck::new();
        let deal = deck.deal();

        let mut all_cards: Vec<Card> = Vec::new();
        for i in 0..3 {
            all_cards.extend_from_slice(&deal.first_three[i]);
            all_cards.extend_from_slice(&deal.second_three[i]);
        }
        all_cards.extend_from_slice(&deal.talon);
        assert_eq!(all_cards.len(), 20);

        let unique: std::collections::HashSet<_> = all_cards.iter().collect();
        assert_eq!(unique.len(), 20);
    }

    #[test]
    fn full_hand_has_six_cards() {
        let deck = Deck::new();
        let deal = deck.deal();
        for i in 0..3 {
            assert_eq!(deal.full_hand(i).len(), 6);
        }
    }

    #[test]
    fn shuffle_changes_order() {
        let mut deck = Deck::new();
        let original = deck.cards().to_vec();
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);
        // Extremely unlikely (1/20! chance) that shuffle produces same order
        assert_ne!(deck.cards(), original.as_slice());
    }
}

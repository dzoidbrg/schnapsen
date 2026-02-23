use rand::seq::SliceRandom;

use crate::card::{Card, Rank, Suit};

/// A 20-card Schnapsen deck.
#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(20);
        for &suit in &Suit::ALL {
            for &rank in &Rank::ALL {
                cards.push(Card::new(suit, rank));
            }
        }
        Self { cards }
    }

    pub fn shuffle(&mut self, rng: &mut impl rand::Rng) {
        self.cards.shuffle(rng);
    }

    /// Draw the top card. Returns `None` if the deck is empty.
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Draw `n` cards from the top.
    pub fn draw_n(&mut self, n: usize) -> Vec<Card> {
        let start = self.cards.len().saturating_sub(n);
        self.cards.split_off(start)
    }

    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn new_deck_has_20_cards() {
        let deck = Deck::new();
        assert_eq!(deck.remaining(), 20);
    }

    #[test]
    fn all_cards_unique() {
        let deck = Deck::new();
        let set: HashSet<_> = deck.cards.iter().collect();
        assert_eq!(set.len(), 20);
    }

    #[test]
    fn draw_reduces_count() {
        let mut deck = Deck::new();
        let _ = deck.draw();
        assert_eq!(deck.remaining(), 19);
    }

    #[test]
    fn draw_n_returns_correct_count() {
        let mut deck = Deck::new();
        let hand = deck.draw_n(6);
        assert_eq!(hand.len(), 6);
        assert_eq!(deck.remaining(), 14);
    }

    #[test]
    fn shuffle_preserves_all_cards() {
        let mut deck = Deck::new();
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);
        assert_eq!(deck.remaining(), 20);

        let set: HashSet<_> = deck.cards.iter().collect();
        assert_eq!(set.len(), 20);
    }

    #[test]
    fn draw_all_empties_deck() {
        let mut deck = Deck::new();
        for _ in 0..20 {
            assert!(deck.draw().is_some());
        }
        assert!(deck.is_empty());
        assert!(deck.draw().is_none());
    }
}

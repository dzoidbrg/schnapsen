//! Application state and logic.

use schnapsen_engine::{GameEngine, GameMove, RandomEngine, valid_moves};
use rand::seq::SliceRandom;
use schnapsen_model::{
    Card, GameConfig, GamePhase, GameState, Hand, PlayerId, Suit,
};

/// Human player is always player 0.
const HUMAN_PLAYER: PlayerId = 0;

pub struct App {
    pub state: GameState,
    pub selected_card_index: usize,
    pub message: String,
    engine: RandomEngine,
}

impl App {
    pub fn new() -> Self {
        let config = GameConfig::default();
        let mut state = GameState::with_round(0, config);
        // Simple init: deal 6 random cards to each player for demo
        Self::deal_demo_hands(&mut state);
        App {
            state,
            selected_card_index: 0,
            message: "Use ←/→ to select, Enter to play, Space for AI move, q to quit".to_string(),
            engine: RandomEngine::new(),
        }
    }

    fn deal_demo_hands(state: &mut GameState) {
        let mut deck = Card::full_deck();
        deck.shuffle(&mut rand::thread_rng());
        if let Some(ref mut round) = state.round {
            round.phase = GamePhase::Playing;
            round.trump = Some(Suit::Herz);
            round.game_type = Some(schnapsen_model::GameType::NormalesSpiel);
            round.declarer = Some(0);
            round.next_to_play = 0;
            for i in 0..3 {
                let start = i * 6;
                for c in deck[start..start + 6].iter() {
                    round.hands[i].add(*c);
                }
            }
        }
    }

    pub fn hand(&self) -> &Hand {
        &self.state.round.as_ref().unwrap().hands[HUMAN_PLAYER as usize]
    }

    pub fn selected_card(&self) -> Option<Card> {
        let hand = self.hand();
        hand.cards().get(self.selected_card_index).copied()
    }

    pub fn select_prev_card(&mut self) {
        let len = self.hand().len();
        if len > 0 {
            self.selected_card_index = (self.selected_card_index + len - 1) % len;
        }
    }

    pub fn select_next_card(&mut self) {
        let len = self.hand().len();
        if len > 0 {
            self.selected_card_index = (self.selected_card_index + 1) % len;
        }
    }

    pub fn play_selected_card(&mut self) {
        if let Some(card) = self.selected_card() {
            if let Some(ref mut round) = self.state.round {
                if round.next_to_play != HUMAN_PLAYER {
                    self.message = "Not your turn".to_string();
                    return;
                }
                let moves = valid_moves(round);
                if moves.iter().any(|m| matches!(m, GameMove::PlayCard(c) if *c == card)) {
                    self.apply_play(card);
                } else {
                    self.message = "Invalid move".to_string();
                }
            }
        }
    }

    pub fn play_ai_move(&mut self) {
        if let Some(ref round) = self.state.round {
            if round.next_to_play == HUMAN_PLAYER {
                self.message = "Your turn - play a card".to_string();
                return;
            }
        }
        if let Some(mv) = self.engine.pick_move(&self.state) {
            match mv {
                GameMove::PlayCard(c) => self.apply_play(c),
                _ => {}
            }
        }
    }

    fn apply_play(&mut self, card: Card) {
        if let Some(ref mut round) = self.state.round {
            let player = round.next_to_play;
            round.hands[player as usize].remove(&card);
            round.current_trick.add(player, card);
            if round.current_trick.is_complete() {
                // TODO: determine winner, update next_to_play
                let winner = round.rufer; // Simplified
                let trick = std::mem::take(&mut round.current_trick);
                round.completed_tricks.push((winner, trick));
                round.next_to_play = winner;
                if round.hands.iter().all(|h| h.is_empty()) {
                    round.phase = GamePhase::RoundFinished;
                }
            } else {
                round.next_to_play = (round.next_to_play + 1) % 3;
            }
            self.message = format!("Played {}", card);
            if self.selected_card_index >= self.hand().len() && self.hand().len() > 0 {
                self.selected_card_index = self.hand().len() - 1;
            }
        }
    }
}

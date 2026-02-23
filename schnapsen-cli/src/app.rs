//! Application state and logic.

use rand::seq::SliceRandom;
use schnapsen_engine::{apply_move, get_valid_moves, pick_random_move};
use schnapsen_model::{GameConfig, GameState, GamePhase, Player};

/// The human player - always the Rufer (first to play).
const HUMAN_PLAYER: Player = Player::Rufer;

pub struct App {
    pub state: GameState,
    pub selected_card: usize,
    pub message: String,
}

impl App {
    pub fn new() -> Self {
        let mut state = GameState::new(GameConfig::default());
        schnapsen_model::setup_round(&mut state, |deck| {
            deck.shuffle(&mut rand::thread_rng());
        });
        Self {
            state,
            selected_card: 0,
            message: String::new(),
        }
    }

    fn shuffle_deck(deck: &mut [schnapsen_model::Card]) {
        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);
    }

    pub fn new_round(&mut self) {
        let mut state = GameState::new(GameConfig::default());
        schnapsen_model::setup_round(&mut state, Self::shuffle_deck);
        self.state = state;
        self.selected_card = 0;
        self.message = "New round! You are the Rufer.".into();
    }

    pub fn prev_card(&mut self) {
        let hand_len = self.state.hand(HUMAN_PLAYER).len();
        if hand_len > 0 {
            self.selected_card = (self.selected_card + hand_len - 1) % hand_len;
        }
    }

    pub fn next_card(&mut self) {
        let hand_len = self.state.hand(HUMAN_PLAYER).len();
        if hand_len > 0 {
            self.selected_card = (self.selected_card + 1) % hand_len;
        }
    }

    pub fn select_card(&mut self, idx: usize) {
        let hand_len = self.state.hand(HUMAN_PLAYER).len();
        if idx < hand_len {
            self.selected_card = idx;
        }
    }

    pub fn play_selected(&mut self) {
        if self.state.phase != GamePhase::Playing {
            return;
        }
        if self.state.current_player != Some(HUMAN_PLAYER) {
            return;
        }
        let hand = self.state.hand(HUMAN_PLAYER).to_vec();
        let Some(card) = hand.get(self.selected_card).copied() else {
            return;
        };
        let mv = schnapsen_model::Move::PlayCard {
            player: HUMAN_PLAYER,
            card,
        };
        let valid = get_valid_moves(&self.state);
        if !valid.iter().any(|m| matches!(m, schnapsen_model::Move::PlayCard { card: c, .. } if *c == card)) {
            self.message = "Invalid move!".into();
            return;
        }
        if let Err(e) = apply_move(&mut self.state, mv) {
            self.message = format!("Error: {e}");
            return;
        }
        self.message.clear();
        if self.selected_card >= hand.len().saturating_sub(1) && self.selected_card > 0 {
            self.selected_card -= 1;
        }
        self.run_ai_turns();
    }

    fn run_ai_turns(&mut self) {
        while self.state.phase == GamePhase::Playing
            && self.state.current_player != Some(HUMAN_PLAYER)
        {
            if let Some(mv) = pick_random_move(&self.state) {
                if let Err(e) = apply_move(&mut self.state, mv) {
                    self.message = format!("AI error: {e}");
                    break;
                }
            } else {
                break;
            }
        }
        if self.state.hand(HUMAN_PLAYER).is_empty() {
            self.message = "Round over! Press 'n' for new round.".into();
        }
    }

    pub fn hand(&self) -> &[schnapsen_model::Card] {
        self.state.hand(HUMAN_PLAYER)
    }

    pub fn selected_card(&self) -> usize {
        self.selected_card
    }

    pub fn is_my_turn(&self) -> bool {
        self.state.current_player == Some(HUMAN_PLAYER)
    }
}

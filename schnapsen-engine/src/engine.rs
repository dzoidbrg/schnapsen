//! Game engine and AI strategies.

use rand::seq::SliceRandom;
use schnapsen_model::GameState;

use crate::moves::GameMove;
use crate::validation::valid_moves;

/// Core game engine interface.
pub trait GameEngine {
    /// Pick a move to play. Returns None if no valid moves.
    fn pick_move(&self, state: &GameState) -> Option<GameMove>;
}

/// Simple engine that picks a random valid move.
#[derive(Default)]
pub struct RandomEngine;

impl RandomEngine {
    pub fn new() -> Self {
        Self
    }
}

impl GameEngine for RandomEngine {
    fn pick_move(&self, state: &GameState) -> Option<GameMove> {
        let round = state.round.as_ref()?;
        let moves = valid_moves(round);
        if moves.is_empty() {
            return None;
        }
        moves.choose(&mut rand::thread_rng()).copied()
    }
}

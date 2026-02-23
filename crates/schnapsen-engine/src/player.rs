use rand::seq::SliceRandom;
use schnapsen_model::game::GameState;
use schnapsen_model::types::Action;

/// Trait for a player implementation that selects an action given the visible game state.
pub trait Player {
    fn choose_action(&mut self, state: &GameState) -> Action;
    fn name(&self) -> &str;
}

/// A player that picks uniformly at random from valid actions.
pub struct RandomPlayer {
    name: String,
    rng: rand::rngs::ThreadRng,
}

impl RandomPlayer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rng: rand::thread_rng(),
        }
    }
}

impl Player for RandomPlayer {
    fn choose_action(&mut self, state: &GameState) -> Action {
        let actions = state.valid_actions();
        assert!(
            !actions.is_empty(),
            "no valid actions; game should be in a non-terminal state"
        );
        actions.choose(&mut self.rng).unwrap().clone()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use schnapsen_model::game::GameState;

    #[test]
    fn random_player_returns_valid_action() {
        let mut rng = rand::thread_rng();
        let state = GameState::deal(0, &mut rng);
        let mut player = RandomPlayer::new("TestBot");
        let action = player.choose_action(&state);
        let valid = state.valid_actions();
        assert!(valid.contains(&action));
    }

    #[test]
    fn random_player_name() {
        let player = RandomPlayer::new("Alice");
        assert_eq!(player.name(), "Alice");
    }
}

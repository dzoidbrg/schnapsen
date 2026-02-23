use schnapsen_model::game::{GameState, Phase};
use schnapsen_model::types::{MatchScore, PlayerId};

use crate::player::Player;

/// Outcome of a single hand.
#[derive(Debug, Clone)]
pub struct HandResult {
    pub declarer: PlayerId,
    pub declarer_won: bool,
    pub points: u8,
    pub game_type: schnapsen_model::types::GameType,
}

/// Runs complete games of Dreierschnapsen.
pub struct GameRunner {
    pub players: [Box<dyn Player>; 3],
    pub score: MatchScore,
    pub geber: PlayerId,
}

impl GameRunner {
    pub fn new(players: [Box<dyn Player>; 3]) -> Self {
        Self {
            players,
            score: MatchScore::new(),
            geber: 0,
        }
    }

    /// Play a single hand and return the result.
    pub fn play_hand(&mut self) -> HandResult {
        let mut rng = rand::thread_rng();
        let mut state = GameState::deal(self.geber, &mut rng);

        let mut safety = 0;
        while !matches!(state.phase, Phase::Finished { .. }) {
            let player_id = state.active_player().expect("active player in non-finished phase");
            let action = self.players[player_id].choose_action(&state);
            state
                .apply_action(player_id, action)
                .expect("player returned invalid action");

            safety += 1;
            if safety > 200 {
                panic!("game loop exceeded 200 iterations — likely a bug");
            }
        }

        let (declarer, game_type, declarer_won, points) = match state.phase {
            Phase::Finished {
                declarer,
                game_type,
                declarer_won,
                points_awarded,
            } => (declarer, game_type, declarer_won, points_awarded),
            _ => unreachable!(),
        };

        if declarer_won {
            self.score.game_points[declarer] =
                self.score.game_points[declarer].saturating_add(points);
        } else {
            for p in 0..3 {
                if p != declarer {
                    self.score.game_points[p] =
                        self.score.game_points[p].saturating_add(points);
                }
            }
        }

        self.geber = (self.geber + 1) % 3;

        HandResult {
            declarer,
            declarer_won,
            points,
            game_type,
        }
    }

    /// Play hands until someone reaches 24 points. Returns the winner's index.
    pub fn play_match(&mut self) -> PlayerId {
        loop {
            self.play_hand();
            if let Some(winner) = self.score.winner() {
                return winner;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::RandomPlayer;

    fn make_runner() -> GameRunner {
        GameRunner::new([
            Box::new(RandomPlayer::new("Bot A")),
            Box::new(RandomPlayer::new("Bot B")),
            Box::new(RandomPlayer::new("Bot C")),
        ])
    }

    #[test]
    fn play_hand_completes() {
        let mut runner = make_runner();
        let result = runner.play_hand();
        assert!(result.points > 0 || result.game_type == schnapsen_model::types::GameType::Normal);
    }

    #[test]
    fn play_match_produces_winner() {
        let mut runner = make_runner();
        let winner = runner.play_match();
        assert!(winner < 3);
        assert!(runner.score.game_points[winner] >= 24);
    }

    #[test]
    fn geber_rotates_after_hand() {
        let mut runner = make_runner();
        assert_eq!(runner.geber, 0);
        runner.play_hand();
        assert_eq!(runner.geber, 1);
        runner.play_hand();
        assert_eq!(runner.geber, 2);
        runner.play_hand();
        assert_eq!(runner.geber, 0);
    }

    #[test]
    fn multiple_matches_complete() {
        for _ in 0..10 {
            let mut runner = make_runner();
            runner.play_match();
        }
    }
}

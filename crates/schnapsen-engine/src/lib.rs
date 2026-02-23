//! Minimal Dreierschnapsen engine helpers.
//!
//! This crate currently provides:
//! - valid move extraction for a player in a given state
//! - random valid-move selection

use rand::prelude::{IndexedRandom, Rng};
use schnapsen_model::{PlayerId, PlayerMove, RoundError, RoundState};

/// Returns all currently legal moves for one player.
pub fn valid_moves(state: &RoundState, player: PlayerId) -> Vec<PlayerMove> {
    state
        .legal_cards_for_player(player)
        .into_iter()
        .map(PlayerMove::PlayCard)
        .collect()
}

/// Picks one random legal move. Returns `None` when no move is legal.
pub fn pick_random_valid_move<R: Rng + ?Sized>(
    state: &RoundState,
    player: PlayerId,
    rng: &mut R,
) -> Option<PlayerMove> {
    let legal = valid_moves(state, player);
    legal.choose(rng).copied()
}

/// Convenience wrapper that uses thread-local RNG.
pub fn pick_random_valid_move_thread_rng(
    state: &RoundState,
    player: PlayerId,
) -> Option<PlayerMove> {
    let mut rng = rand::rng();
    pick_random_valid_move(state, player, &mut rng)
}

/// Applies an engine move to the model state.
pub fn apply_move(
    state: &mut RoundState,
    player: PlayerId,
    mv: PlayerMove,
) -> Result<(), RoundError> {
    state.apply_move(player, mv)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use schnapsen_model::{Card, GameDeclaration, Rank, Suit};

    fn card(suit: Suit, rank: Rank) -> Card {
        Card::new(suit, rank)
    }

    fn sample_state() -> RoundState {
        RoundState::new(
            PlayerId::P0,
            PlayerId::P0,
            GameDeclaration::Normal,
            Some(Suit::Hearts),
            [
                vec![card(Suit::Hearts, Rank::Ace), card(Suit::Clubs, Rank::King)],
                vec![card(Suit::Hearts, Rank::Ten), card(Suit::Spades, Rank::Ace)],
                vec![
                    card(Suit::Diamonds, Rank::Ace),
                    card(Suit::Clubs, Rank::Ace),
                ],
            ],
            vec![],
        )
    }

    #[test]
    fn random_pick_is_always_legal() {
        let state = sample_state();
        let legal = valid_moves(&state, PlayerId::P0);
        assert!(!legal.is_empty());

        let mut rng = StdRng::seed_from_u64(7);
        let picked =
            pick_random_valid_move(&state, PlayerId::P0, &mut rng).expect("must produce move");
        assert!(legal.contains(&picked));
    }

    #[test]
    fn no_legal_move_for_non_active_player() {
        let state = sample_state();
        let mut rng = StdRng::seed_from_u64(99);
        assert_eq!(pick_random_valid_move(&state, PlayerId::P1, &mut rng), None);
    }
}

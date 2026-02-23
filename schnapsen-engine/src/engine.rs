//! Core engine logic: move validation, state updates, and AI.

use rand::seq::SliceRandom;
use schnapsen_model::{Card, GameError, GameState, Move, Trick};

/// Get all valid moves for the current player.
///
/// Applies Farbzwang (follow suit) and Stichzwang (must trump/win when possible).
pub fn get_valid_moves(state: &GameState) -> Vec<Move> {
    let Some(current_player) = state.current_player else {
        return vec![];
    };

    let hand = state.hand(current_player);
    if hand.is_empty() {
        return vec![];
    }

    // If no trick in progress, any card is valid (first play of trick)
    let lead_suit = state.current_trick.lead_suit();
    let trump = state.trump;
    let _game_type = state.effective_game_type();

    if lead_suit.is_none() {
        // Leading: any card valid
        return hand
            .iter()
            .map(|&card| Move::PlayCard {
                player: current_player,
                card,
            })
            .collect();
    }

    let lead_suit = lead_suit.unwrap();
    let has_lead_suit = hand.iter().any(|c| c.suit == lead_suit);
    let has_trump = trump.map(|t| hand.iter().any(|c| c.suit == t)).unwrap_or(false);

    // Filter by rules
    let valid_cards: Vec<Card> = if has_lead_suit {
        // Must follow suit - can play any card of lead suit
        hand.iter().filter(|c| c.suit == lead_suit).copied().collect()
    } else if has_trump && trump.is_some() {
        // Can't follow suit - must trump if possible
        let t = trump.unwrap();
        hand.iter().filter(|c| c.suit == t).copied().collect()
    } else {
        // Can't follow or trump - can play any card (discard)
        hand.to_vec()
    };

    valid_cards
        .into_iter()
        .map(|card| Move::PlayCard {
            player: current_player,
            card,
        })
        .collect()
}

/// Apply a move to the game state.
pub fn apply_move(state: &mut GameState, mv: Move) -> Result<(), GameError> {
    use schnapsen_model::GamePhase;
    if state.phase != GamePhase::Playing {
        return Err(GameError::InvalidMove("Can only play cards in Playing phase".into()));
    }
    let valid = get_valid_moves(state);
    if !valid.contains(&mv) {
        return Err(GameError::InvalidMove(format!("{:?} is not valid", mv)));
    }

    match mv {
        Move::PlayCard { player, card } => {
            // Remove card from hand
            let hand = state.hand_mut(player).ok_or(GameError::InvalidState)?;
            let pos = hand
                .iter()
                .position(|c| *c == card)
                .ok_or_else(|| GameError::InvalidMove("Card not in hand".into()))?;
            hand.remove(pos);

            // Set trick leader if first card
            if state.trick_leader.is_none() {
                state.trick_leader = Some(player);
            }

            // Add to current trick
            state.current_trick.add(player, card);

            if state.current_trick.is_complete() {
                // Resolve trick
                let winner = state
                    .current_trick
                    .winner(state.trump, state.effective_game_type())
                    .ok_or(GameError::InvalidState)?;
                let points = state.current_trick.points();

                state
                    .tricks_won
                    .entry(winner)
                    .or_default()
                    .push(std::mem::take(&mut state.current_trick));
                *state.trick_points.entry(winner).or_insert(0) += points;

                state.trick_leader = None;
                state.current_trick = Trick::new();
                state.current_player = Some(winner);
            } else {
                state.current_player = Some(player.next());
            }
        }
        _ => return Err(GameError::InvalidMove("Only PlayCard supported in Playing phase".into())),
    }

    Ok(())
}

/// Pick a random valid move. Returns None if no valid moves.
pub fn pick_random_move(state: &GameState) -> Option<Move> {
    let moves = get_valid_moves(state);
    moves.into_iter().collect::<Vec<_>>().choose(&mut rand::thread_rng()).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use schnapsen_model::{deal_round, GameConfig, GamePhase, Player, Suit};

    #[test]
    fn leading_any_card_valid() {
        let (hands, talon) = deal_round();
        let mut state = GameState::new(GameConfig::default());
        state.hands = hands;
        state.talon = talon;
        state.phase = GamePhase::Playing;
        state.trump = Some(Suit::Hearts);
        state.game_type = None;
        state.current_player = Some(Player::Rufer);

        let moves = get_valid_moves(&state);
        assert_eq!(moves.len(), 6, "When leading, all 6 cards should be valid");
    }
}

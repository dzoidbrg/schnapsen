#![forbid(unsafe_code)]

//! Game-state transition helpers and a baseline random-move engine.

use rand::seq::SliceRandom;
use rand::Rng;
use schnapsen_model::{
    highest_card_in_suit, schnapsen_deck, trick_winner, Card, CurrentTrick, GameMove, GamePhase,
    GameState, GameVariant, PlayerId, ResolvedTrick, Ruleset, Suit, TrumpSelection, PLAYER_COUNT,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    NotInPlayingPhase,
    NotPlayersTurn { expected: PlayerId, got: PlayerId },
    UnsupportedMove,
    CardNotInHand(Card),
    IllegalCardPlay(Card),
    TrickResolutionFailed,
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::NotInPlayingPhase => write!(f, "game is not in playing phase"),
            EngineError::NotPlayersTurn { expected, got } => {
                write!(f, "expected player {expected}, got {got}")
            }
            EngineError::UnsupportedMove => write!(f, "move type not supported by simple engine"),
            EngineError::CardNotInHand(card) => write!(f, "card {card} is not in player's hand"),
            EngineError::IllegalCardPlay(card) => write!(f, "card {card} is not legal right now"),
            EngineError::TrickResolutionFailed => write!(f, "failed to resolve trick winner"),
        }
    }
}

impl std::error::Error for EngineError {}

#[derive(Debug, Clone)]
pub struct RandomMoveEngine<R: Rng> {
    rng: R,
}

impl<R: Rng> RandomMoveEngine<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }

    pub fn pick_random_valid_move(&mut self, state: &GameState) -> Option<GameMove> {
        let legal = valid_moves(state);
        if legal.is_empty() {
            None
        } else {
            let index = self.rng.random_range(0..legal.len());
            Some(legal[index])
        }
    }
}

impl Default for RandomMoveEngine<rand::rngs::ThreadRng> {
    fn default() -> Self {
        Self::new(rand::rng())
    }
}

pub fn setup_demo_normal_round(rng: &mut impl Rng) -> GameState {
    let mut deck = schnapsen_deck();
    deck.shuffle(rng);
    let mut deck_iter = deck.into_iter();

    let dealer = PlayerId::ALL[rng.random_range(0..PLAYER_COUNT)];
    let mut state = GameState::empty(Ruleset::all_variants(), dealer, GameVariant::Normal);
    let caller = state.caller;
    let next = caller.next_clockwise();
    let third = next.next_clockwise();
    let deal_order = [caller, next, third];

    for _ in 0..3 {
        for player in deal_order {
            state
                .player_mut(player)
                .hand
                .push(deck_iter.next().expect("deck contains enough cards"));
        }
    }

    state.talon = vec![
        deck_iter.next().expect("deck contains first talon card"),
        deck_iter.next().expect("deck contains second talon card"),
    ];

    for _ in 0..3 {
        for player in deal_order {
            state
                .player_mut(player)
                .hand
                .push(deck_iter.next().expect("deck contains enough cards"));
        }
    }

    let called_trump = state
        .player(caller)
        .hand
        .first()
        .map(|card| card.suit)
        .unwrap_or(Suit::Hearts);

    state.phase = GamePhase::Playing;
    state.variant = GameVariant::Normal;
    state.declarer = caller;
    state.active_player = caller;
    state.current_trick = CurrentTrick::new(caller);
    state.trump = Some(called_trump);
    state.trump_selection = Some(TrumpSelection::CalledFromFirstThree(called_trump));
    state
}

pub fn legal_cards_for_player(state: &GameState, player: PlayerId) -> Vec<Card> {
    if !matches!(state.phase, GamePhase::Playing) || state.active_player != player {
        return Vec::new();
    }
    let hand = &state.player(player).hand;
    if hand.is_empty() {
        return Vec::new();
    }
    if state.current_trick.cards.is_empty() {
        return hand.clone();
    }

    let lead_suit = state.current_trick.cards[0].1.suit;
    let rank_order = state.variant.rank_order();
    let same_suit_cards: Vec<Card> = hand
        .iter()
        .copied()
        .filter(|card| card.suit == lead_suit)
        .collect();

    if !same_suit_cards.is_empty() {
        let played_cards: Vec<Card> = state
            .current_trick
            .cards
            .iter()
            .map(|(_, card)| *card)
            .collect();
        let highest_in_lead = highest_card_in_suit(&played_cards, lead_suit, rank_order)
            .expect("leader has already played a lead suit card");
        let stronger_same_suit: Vec<Card> = same_suit_cards
            .iter()
            .copied()
            .filter(|card| card.strength(rank_order) > highest_in_lead.strength(rank_order))
            .collect();
        return if stronger_same_suit.is_empty() {
            same_suit_cards
        } else {
            stronger_same_suit
        };
    }

    if let Some(trump) = state.trump {
        let trump_cards: Vec<Card> = hand
            .iter()
            .copied()
            .filter(|card| card.suit == trump)
            .collect();
        if !trump_cards.is_empty() {
            return trump_cards;
        }
    }

    hand.clone()
}

pub fn valid_moves_for_player(state: &GameState, player: PlayerId) -> Vec<GameMove> {
    legal_cards_for_player(state, player)
        .into_iter()
        .map(|card| GameMove::PlayCard { player, card })
        .collect()
}

pub fn valid_moves(state: &GameState) -> Vec<GameMove> {
    valid_moves_for_player(state, state.active_player)
}

pub fn apply_move(state: &mut GameState, mv: GameMove) -> Result<(), EngineError> {
    match mv {
        GameMove::PlayCard { player, card } => apply_play_card(state, player, card),
        _ => Err(EngineError::UnsupportedMove),
    }
}

fn apply_play_card(state: &mut GameState, player: PlayerId, card: Card) -> Result<(), EngineError> {
    if !matches!(state.phase, GamePhase::Playing) {
        return Err(EngineError::NotInPlayingPhase);
    }
    if state.active_player != player {
        return Err(EngineError::NotPlayersTurn {
            expected: state.active_player,
            got: player,
        });
    }

    let legal_cards = legal_cards_for_player(state, player);
    if !legal_cards.contains(&card) {
        return Err(EngineError::IllegalCardPlay(card));
    }

    let hand = &mut state.player_mut(player).hand;
    let remove_index = hand
        .iter()
        .position(|candidate| *candidate == card)
        .ok_or(EngineError::CardNotInHand(card))?;
    hand.remove(remove_index);
    state.current_trick.cards.push((player, card));

    if state.current_trick.cards.len() < PLAYER_COUNT {
        state.active_player = player.next_clockwise();
        return Ok(());
    }

    let winner = trick_winner(
        &state.current_trick.cards,
        state.trump,
        state.variant.rank_order(),
    )
    .ok_or(EngineError::TrickResolutionFailed)?;

    let trick_cards = state.current_trick.cards.clone();
    for (_, won_card) in &trick_cards {
        state.player_mut(winner).won_cards.push(*won_card);
    }
    state.completed_tricks.push(ResolvedTrick {
        leader: state.current_trick.leader,
        winner,
        cards: trick_cards,
    });
    state.current_trick = CurrentTrick::new(winner);
    state.active_player = winner;

    if PlayerId::ALL
        .iter()
        .all(|candidate| state.player(*candidate).hand.is_empty())
    {
        state.phase = GamePhase::Completed;
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundSummary {
    pub winner: PlayerId,
    pub card_points: [u16; PLAYER_COUNT],
    pub won_tricks: [usize; PLAYER_COUNT],
}

pub fn round_summary(state: &GameState) -> Option<RoundSummary> {
    if !state.is_finished() {
        return None;
    }

    let mut card_points = [0u16; PLAYER_COUNT];
    let mut won_tricks = [0usize; PLAYER_COUNT];
    for player in PlayerId::ALL {
        card_points[player.as_index()] = state.total_points_for_player(player);
    }
    for trick in &state.completed_tricks {
        won_tricks[trick.winner.as_index()] += 1;
    }

    let mut winner = PlayerId::One;
    for player in PlayerId::ALL.into_iter().skip(1) {
        if card_points[player.as_index()] > card_points[winner.as_index()] {
            winner = player;
        }
    }

    Some(RoundSummary {
        winner,
        card_points,
        won_tricks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use schnapsen_model::{Card, GameVariant, PlayerState, Suit};

    fn playing_state_with_hands(
        active_player: PlayerId,
        leader: PlayerId,
        trump: Option<Suit>,
        hands: [Vec<Card>; PLAYER_COUNT],
        trick_cards: Vec<(PlayerId, Card)>,
    ) -> GameState {
        let dealer = PlayerId::Three;
        let mut state = GameState::empty(Ruleset::classic(), dealer, GameVariant::Normal);
        state.phase = GamePhase::Playing;
        state.active_player = active_player;
        state.current_trick = CurrentTrick {
            leader,
            cards: trick_cards,
        };
        state.trump = trump;
        state.variant = GameVariant::Normal;
        state.players = hands.map(|hand| PlayerState {
            hand,
            ..PlayerState::default()
        });
        state
    }

    #[test]
    fn legal_cards_enforce_higher_same_suit_if_possible() {
        let state = playing_state_with_hands(
            PlayerId::Two,
            PlayerId::One,
            Some(Suit::Clubs),
            [
                vec![],
                vec![
                    Card::new(Suit::Hearts, schnapsen_model::Rank::Ace),
                    Card::new(Suit::Hearts, schnapsen_model::Rank::Under),
                    Card::new(Suit::Spades, schnapsen_model::Rank::Ace),
                ],
                vec![],
            ],
            vec![(
                PlayerId::One,
                Card::new(Suit::Hearts, schnapsen_model::Rank::King),
            )],
        );

        let legal = legal_cards_for_player(&state, PlayerId::Two);
        assert_eq!(
            legal,
            vec![Card::new(Suit::Hearts, schnapsen_model::Rank::Ace)]
        );
    }

    #[test]
    fn legal_cards_force_trump_if_no_lead_suit() {
        let state = playing_state_with_hands(
            PlayerId::Two,
            PlayerId::One,
            Some(Suit::Clubs),
            [
                vec![],
                vec![
                    Card::new(Suit::Clubs, schnapsen_model::Rank::Under),
                    Card::new(Suit::Spades, schnapsen_model::Rank::Ace),
                ],
                vec![],
            ],
            vec![(
                PlayerId::One,
                Card::new(Suit::Hearts, schnapsen_model::Rank::King),
            )],
        );

        let legal = legal_cards_for_player(&state, PlayerId::Two);
        assert_eq!(
            legal,
            vec![Card::new(Suit::Clubs, schnapsen_model::Rank::Under)]
        );
    }

    #[test]
    fn apply_move_resolves_trick_and_completes_round() {
        let mut state = playing_state_with_hands(
            PlayerId::One,
            PlayerId::One,
            Some(Suit::Clubs),
            [
                vec![Card::new(Suit::Hearts, schnapsen_model::Rank::King)],
                vec![Card::new(Suit::Hearts, schnapsen_model::Rank::Under)],
                vec![Card::new(Suit::Clubs, schnapsen_model::Rank::Ace)],
            ],
            vec![],
        );

        apply_move(
            &mut state,
            GameMove::PlayCard {
                player: PlayerId::One,
                card: Card::new(Suit::Hearts, schnapsen_model::Rank::King),
            },
        )
        .expect("first card should be legal");
        apply_move(
            &mut state,
            GameMove::PlayCard {
                player: PlayerId::Two,
                card: Card::new(Suit::Hearts, schnapsen_model::Rank::Under),
            },
        )
        .expect("second card should be legal");
        apply_move(
            &mut state,
            GameMove::PlayCard {
                player: PlayerId::Three,
                card: Card::new(Suit::Clubs, schnapsen_model::Rank::Ace),
            },
        )
        .expect("third card should be legal");

        assert!(matches!(state.phase, GamePhase::Completed));
        assert_eq!(state.completed_tricks.len(), 1);
        assert_eq!(state.completed_tricks[0].winner, PlayerId::Three);
        assert_eq!(state.player(PlayerId::Three).won_cards.len(), 3);
    }

    #[test]
    fn random_engine_returns_legal_move() {
        let mut rng = StdRng::seed_from_u64(7);
        let state = setup_demo_normal_round(&mut rng);
        let legal = valid_moves(&state);
        assert!(!legal.is_empty());

        let mut engine = RandomMoveEngine::new(rng);
        let picked = engine
            .pick_random_valid_move(&state)
            .expect("at least one legal move should exist");
        assert!(legal.contains(&picked));
    }
}

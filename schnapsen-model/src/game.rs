//! Game state and phases for Dreierschnapsen.

use crate::card::{create_deck, Card, Suit};
use crate::game_type::{FleckenLevel, GameType};
use crate::player::Player;
use crate::trick::Trick;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Phase of the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GamePhase {
    /// Initial phase, deck being dealt
    Dealing,
    /// Rufer must call trump from first 3 cards
    TrumpCalling,
    /// Players bid for game type (or pass)
    Bidding,
    /// Main play phase - playing tricks
    Playing,
    /// Round complete, scoring
    Scoring,
}

/// A move in the game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Move {
    /// Play a card
    PlayCard { player: Player, card: Card },
    /// Call trump (during TrumpCalling)
    CallTrump { suit: Suit },
    /// Announce game type (during Bidding)
    AnnounceGame { game_type: GameType },
    /// Pass / continue (during Bidding)
    Pass,
    /// Flecken (spritzen) - double the stakes
    Flecken,
}

/// Configuration for game variants (regional rules).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Whether Bettler variants are allowed
    pub allow_bettler: bool,
    /// Whether Zehnergang is allowed
    pub allow_zehnergang: bool,
    /// Whether Königsgang is allowed
    pub allow_koenigsgang: bool,
    /// Whether Damengang is allowed
    pub allow_damengang: bool,
    /// Whether Bauernloch is allowed
    pub allow_bauernloch: bool,
    /// When both Gang and Zehnergang announced: true = play Zehnergang
    pub zehnergang_over_gang: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            allow_bettler: true,
            allow_zehnergang: true,
            allow_koenigsgang: true,
            allow_damengang: true,
            allow_bauernloch: true,
            zehnergang_over_gang: true,
        }
    }
}

/// Errors that can occur during game execution.
#[derive(Debug, Error)]
pub enum GameError {
    #[error("Invalid move: {0}")]
    InvalidMove(String),
    #[error("Wrong phase: expected {expected:?}, got {actual:?}")]
    WrongPhase { expected: GamePhase, actual: GamePhase },
    #[error("Invalid game state")]
    InvalidState,
}

/// Full game state.
#[derive(Debug, Clone)]
pub struct GameState {
    /// Current phase
    pub phase: GamePhase,
    /// Configuration
    pub config: GameConfig,
    /// Each player's hand (6 cards each after dealing)
    pub hands: HashMap<Player, Vec<Card>>,
    /// The talon (2 cards face-down, or empty if taken)
    pub talon: Vec<Card>,
    /// Trump suit (None in no-trump games)
    pub trump: Option<Suit>,
    /// The announced game type (None = normal game)
    pub game_type: Option<GameType>,
    /// Flecken multiplier level
    pub flecken_level: FleckenLevel,
    /// The declarer (player who announced the game, or Rufer for normal)
    pub declarer: Player,
    /// Tricks won by each player
    pub tricks_won: HashMap<Player, Vec<Trick>>,
    /// Current trick being played
    pub current_trick: Trick,
    /// Player who led the current trick (to determine turn order)
    pub trick_leader: Option<Player>,
    /// Whose turn it is
    pub current_player: Option<Player>,
    /// Points collected in tricks by each player this round
    pub trick_points: HashMap<Player, u16>,
    /// Running scores (0-24, game ends at 24)
    pub scores: HashMap<Player, u8>,
}

impl GameState {
    /// Create a new game state for the start of a round.
    pub fn new(config: GameConfig) -> Self {
        Self {
            phase: GamePhase::Dealing,
            config,
            hands: HashMap::new(),
            talon: Vec::new(),
            trump: None,
            game_type: None,
            flecken_level: FleckenLevel::None,
            declarer: Player::Rufer,
            tricks_won: HashMap::new(),
            current_trick: Trick::new(),
            trick_leader: None,
            current_player: None,
            trick_points: HashMap::new(),
            scores: {
                let mut m = HashMap::new();
                for p in Player::all() {
                    m.insert(p, 0);
                }
                m
            },
        }
    }

    /// Get the effective game type (defaults to NormalesSpiel).
    pub fn effective_game_type(&self) -> GameType {
        self.game_type.unwrap_or(GameType::NormalesSpiel)
    }

    /// Get valid cards for the current player to play.
    /// Returns the full hand for simplicity; engine will filter by rules.
    pub fn hand(&self, player: Player) -> &[Card] {
        self.hands
            .get(&player)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get mutable hand.
    pub fn hand_mut(&mut self, player: Player) -> Option<&mut Vec<Card>> {
        self.hands.get_mut(&player)
    }
}

/// Set up a new round ready for play. Deals cards, sets trump from first card after shuffle.
pub fn setup_round(state: &mut GameState, mut shuffle: impl FnMut(&mut [Card])) {
    let mut deck: Vec<Card> = create_deck().into();
    shuffle(&mut deck);

    let mut idx = 0;
    for p in Player::all() {
        state.hands.insert(p, Vec::new());
    }
    for _ in 0..3 {
        for p in Player::all() {
            state.hands.get_mut(&p).unwrap().push(deck[idx]);
            idx += 1;
        }
    }
    state.talon = vec![deck[idx], deck[idx + 1]];
    idx += 2;
    for _ in 0..3 {
        for p in Player::all() {
            state.hands.get_mut(&p).unwrap().push(deck[idx]);
            idx += 1;
        }
    }
    state.trump = Some(deck[0].suit); // Use first card's suit as trump
    state.phase = GamePhase::Playing;
    state.declarer = Player::Rufer;
    state.current_player = Some(Player::Rufer);
    state.trick_leader = None;
    state.current_trick = Trick::new();
    state.tricks_won.clear();
    state.trick_points.clear();
}

/// Deal cards for a new round.
/// 3 cards each, then talon of 2, then 3 more each. Rufer gets first 3 to call trump.
pub fn deal_round() -> (HashMap<Player, Vec<Card>>, Vec<Card>) {
    let deck = create_deck();
    let mut idx = 0;
    let mut hands: HashMap<Player, Vec<Card>> = HashMap::new();
    for p in Player::all() {
        hands.insert(p, Vec::new());
    }

    // First round: 3 cards each (Rufer, Left, Right order)
    for _ in 0..3 {
        for p in Player::all() {
            hands.get_mut(&p).unwrap().push(deck[idx]);
            idx += 1;
        }
    }
    // Talon: 2 cards
    let talon = vec![deck[idx], deck[idx + 1]];
    idx += 2;
    // Second round: 3 more each
    for _ in 0..3 {
        for p in Player::all() {
            hands.get_mut(&p).unwrap().push(deck[idx]);
            idx += 1;
        }
    }

    (hands, talon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_distribution() {
        let (hands, talon) = deal_round();
        assert_eq!(talon.len(), 2);
        for (_, hand) in &hands {
            assert_eq!(hand.len(), 6, "Each player should have 6 cards");
        }
        let total: usize = hands.values().map(|h| h.len()).sum::<usize>() + talon.len();
        assert_eq!(total, 20, "All 20 cards accounted for");
    }
}

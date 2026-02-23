//! Game state, phases, and configuration for Dreierschnapsen.

use crate::card::{Card, Suit};
use crate::game_type::GameType;

/// Player identifier (0, 1, or 2).
pub type PlayerId = u8;

/// Current phase of the game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GamePhase {
    /// Dealing cards
    Dealing,
    /// Rufer must call trump from first 3 cards
    TrumpCalling,
    /// Bidding: who wants to play which game
    Bidding,
    /// Playing tricks
    Playing,
    /// Round finished, scoring
    RoundFinished,
    /// Match finished (someone reached 24)
    MatchFinished,
}

/// A bid (game type declaration) by a player.
#[derive(Debug, Clone)]
pub struct Bid {
    pub player: PlayerId,
    pub game_type: GameType,
}

/// A player's hand of cards.
#[derive(Debug, Clone, Default)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Vec::with_capacity(6),
        }
    }

    pub fn with_capacity(n: usize) -> Self {
        Hand {
            cards: Vec::with_capacity(n),
        }
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove(&mut self, card: &Card) -> bool {
        if let Some(pos) = self.cards.iter().position(|c| c == card) {
            self.cards.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn contains(&self, card: &Card) -> bool {
        self.cards.iter().any(|c| c == card)
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn total_points(&self) -> u32 {
        self.cards.iter().map(|c| c.points() as u32).sum()
    }
}

/// A single trick (cards played in one round).
#[derive(Debug, Clone, Default)]
pub struct Trick {
    /// Cards played: (player_id, card)
    plays: Vec<(PlayerId, Card)>,
}

impl Trick {
    pub fn new() -> Self {
        Trick {
            plays: Vec::with_capacity(3),
        }
    }

    pub fn add(&mut self, player: PlayerId, card: Card) {
        self.plays.push((player, card));
    }

    pub fn plays(&self) -> &[(PlayerId, Card)] {
        &self.plays
    }

    pub fn len(&self) -> usize {
        self.plays.len()
    }

    pub fn is_complete(&self) -> bool {
        self.plays.len() == 3
    }

    pub fn lead_suit(&self) -> Option<Suit> {
        self.plays.first().map(|(_, c)| c.suit)
    }

    /// Total points in this trick.
    pub fn points(&self) -> u32 {
        self.plays.iter().map(|(_, c)| c.points() as u32).sum()
    }
}

/// Configuration for which game variants are allowed (regional rules).
#[derive(Debug, Clone)]
pub struct GameConfig {
    /// Allow Bettler-style games
    pub allow_bettler: bool,
    /// Allow Assenbettler
    pub allow_assenbettler: bool,
    /// Allow Ass-Bettler
    pub allow_ass_bettler: bool,
    /// Allow Zehnergang, Königsgang, Damengang
    pub allow_gang_variants: bool,
    /// Allow Bauernloch
    pub allow_bauernloch: bool,
    /// Allow Flecken (doubling)
    pub allow_flecken: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            allow_bettler: true,
            allow_assenbettler: true,
            allow_ass_bettler: true,
            allow_gang_variants: true,
            allow_bauernloch: true,
            allow_flecken: true,
        }
    }
}

/// State of a single round.
#[derive(Debug, Clone)]
pub struct RoundState {
    pub phase: GamePhase,
    /// Dealer (Geber) for this round
    pub dealer: PlayerId,
    /// Rufer (left of dealer) – calls trump
    pub rufer: PlayerId,
    /// Trump suit (None if not yet called or game has no trump)
    pub trump: Option<Suit>,
    /// Declared game type (None until bidding resolves)
    pub game_type: Option<GameType>,
    /// Player who declared and is playing
    pub declarer: Option<PlayerId>,
    /// Flecken multiplier: 1, 2, 4, or 8
    pub flecken_multiplier: u8,
    /// Hands for each player
    pub hands: [Hand; 3],
    /// Talon (2 cards)
    pub talon: Vec<Card>,
    /// Current trick being played
    pub current_trick: Trick,
    /// Completed tricks: (winner, trick)
    pub completed_tricks: Vec<(PlayerId, Trick)>,
    /// Points won by declarer so far this round
    pub declarer_points: u32,
    /// Points won by opponents so far
    pub opponent_points: u32,
    /// Who plays next
    pub next_to_play: PlayerId,
    /// Bids made during bidding phase
    pub bids: Vec<Bid>,
}

impl RoundState {
    pub fn new(dealer: PlayerId) -> Self {
        let rufer = (dealer + 1) % 3;
        RoundState {
            phase: GamePhase::Dealing,
            dealer,
            rufer,
            trump: None,
            game_type: None,
            declarer: None,
            flecken_multiplier: 1,
            hands: Default::default(),
            talon: Vec::new(),
            current_trick: Trick::new(),
            completed_tricks: Vec::new(),
            declarer_points: 0,
            opponent_points: 0,
            next_to_play: rufer,
            bids: Vec::new(),
        }
    }
}

/// Full game state (match across rounds).
#[derive(Debug, Clone)]
pub struct GameState {
    pub config: GameConfig,
    /// Scores: 0–24 per player
    pub scores: [u8; 3],
    /// Current round (None between rounds or when match is over)
    pub round: Option<RoundState>,
}

impl GameState {
    pub fn new(config: GameConfig) -> Self {
        GameState {
            config,
            scores: [0, 0, 0],
            round: None,
        }
    }

    pub fn with_round(dealer: PlayerId, config: GameConfig) -> Self {
        GameState {
            config,
            scores: [0, 0, 0],
            round: Some(RoundState::new(dealer)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_operations() {
        let mut hand = Hand::new();
        let card = Card::new(Suit::Herz, crate::card::Rank::Ass);
        hand.add(card);
        assert!(hand.contains(&card));
        assert_eq!(hand.len(), 1);
        assert!(hand.remove(&card));
        assert!(!hand.contains(&card));
        assert_eq!(hand.len(), 0);
    }

    #[test]
    fn trick_points() {
        let mut trick = Trick::new();
        trick.add(0, Card::new(Suit::Herz, crate::card::Rank::Ass));
        trick.add(1, Card::new(Suit::Karo, crate::card::Rank::Zehner));
        trick.add(2, Card::new(Suit::Pik, crate::card::Rank::Koenig));
        assert!(trick.is_complete());
        assert_eq!(trick.points(), 11 + 10 + 4);
    }
}

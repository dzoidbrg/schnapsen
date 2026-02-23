#![forbid(unsafe_code)]

//! Core domain model for Dreierschnapsen.
//!
//! This crate focuses on representing game concepts and variant/rules metadata.
//! It intentionally keeps gameplay logic lightweight so `schnapsen-engine` can
//! evolve independently.

use std::cmp::Ordering;
use std::fmt;

pub const PLAYER_COUNT: usize = 3;
pub const HAND_SIZE: usize = 6;
pub const TALON_SIZE: usize = 2;
pub const MATCH_TARGET_POINTS: u16 = 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];

    pub const fn short(self) -> &'static str {
        match self {
            Suit::Hearts => "H",
            Suit::Diamonds => "D",
            Suit::Spades => "S",
            Suit::Clubs => "C",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Ace,
    Ten,
    King,
    Ober,
    Under,
}

impl Rank {
    pub const ALL: [Rank; 5] = [Rank::Ace, Rank::Ten, Rank::King, Rank::Ober, Rank::Under];

    pub const fn short(self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Ten => "10",
            Rank::King => "K",
            Rank::Ober => "O",
            Rank::Under => "U",
        }
    }

    pub const fn card_points(self) -> u8 {
        match self {
            Rank::Ace => 11,
            Rank::Ten => 10,
            Rank::King => 4,
            Rank::Ober => 3,
            Rank::Under => 2,
        }
    }

    pub const fn strength(self, order: RankOrder) -> u8 {
        match order {
            RankOrder::Standard => match self {
                Rank::Ace => 5,
                Rank::Ten => 4,
                Rank::King => 3,
                Rank::Ober => 2,
                Rank::Under => 1,
            },
            RankOrder::AceLow => match self {
                Rank::Ten => 5,
                Rank::King => 4,
                Rank::Ober => 3,
                Rank::Under => 2,
                Rank::Ace => 1,
            },
            RankOrder::TenLow => match self {
                Rank::King => 5,
                Rank::Ober => 4,
                Rank::Under => 3,
                Rank::Ace => 2,
                Rank::Ten => 1,
            },
            RankOrder::KingLow => match self {
                Rank::Ober => 5,
                Rank::Under => 4,
                Rank::Ace => 3,
                Rank::Ten => 2,
                Rank::King => 1,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub const fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }

    pub const fn points(self) -> u8 {
        self.rank.card_points()
    }

    pub const fn strength(self, order: RankOrder) -> u8 {
        self.rank.strength(order)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank.short(), self.suit.short())
    }
}

pub fn schnapsen_deck() -> Vec<Card> {
    Suit::ALL
        .iter()
        .flat_map(|suit| Rank::ALL.iter().map(move |rank| Card::new(*suit, *rank)))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerId {
    One,
    Two,
    Three,
}

impl PlayerId {
    pub const ALL: [PlayerId; PLAYER_COUNT] = [PlayerId::One, PlayerId::Two, PlayerId::Three];

    pub const fn as_index(self) -> usize {
        match self {
            PlayerId::One => 0,
            PlayerId::Two => 1,
            PlayerId::Three => 2,
        }
    }

    pub const fn next_clockwise(self) -> Self {
        match self {
            PlayerId::One => PlayerId::Two,
            PlayerId::Two => PlayerId::Three,
            PlayerId::Three => PlayerId::One,
        }
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            PlayerId::One => "P1",
            PlayerId::Two => "P2",
            PlayerId::Three => "P3",
        };
        write!(f, "{label}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RankOrder {
    Standard,
    AceLow,
    TenLow,
    KingLow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrumpPolicy {
    CalledTrump,
    NoTrump,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclarerRestriction {
    AnyPlayer,
    CallerOnly,
    DefenderOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerRole {
    Caller,
    Defender,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameObjective {
    Reach66OrLastTrick,
    TakeNoTricks,
    TakeAllTricks,
    SchnapserPattern,
    HoldAllCardsOfSingleSuit,
    HoldAllTrumpCards,
    Custom(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameVariant {
    Normal,
    Bettler,
    Assenbettler,
    AssBettler,
    Schnapser,
    Plauderer,
    Damengang,
    Koenigsgang,
    Gang,
    Zehnergang,
    Bauernloch,
    Bauernschnapser,
    Kontraschnapser,
    Farbringerl,
    Kontrabauernschnapser,
    Herrenschnapser,
}

impl GameVariant {
    pub const ALL: [GameVariant; 16] = [
        GameVariant::Normal,
        GameVariant::Bettler,
        GameVariant::Assenbettler,
        GameVariant::AssBettler,
        GameVariant::Schnapser,
        GameVariant::Plauderer,
        GameVariant::Damengang,
        GameVariant::Koenigsgang,
        GameVariant::Gang,
        GameVariant::Zehnergang,
        GameVariant::Bauernloch,
        GameVariant::Bauernschnapser,
        GameVariant::Kontraschnapser,
        GameVariant::Farbringerl,
        GameVariant::Kontrabauernschnapser,
        GameVariant::Herrenschnapser,
    ];

    pub const fn base_points(self) -> u8 {
        match self {
            GameVariant::Normal => 1,
            GameVariant::Bettler => 4,
            GameVariant::Assenbettler => 5,
            GameVariant::AssBettler => 5,
            GameVariant::Schnapser => 6,
            GameVariant::Plauderer => 7,
            GameVariant::Damengang => 7,
            GameVariant::Koenigsgang => 8,
            GameVariant::Gang => 9,
            GameVariant::Zehnergang => 10,
            GameVariant::Bauernloch => 12,
            GameVariant::Bauernschnapser => 12,
            GameVariant::Kontraschnapser => 12,
            GameVariant::Farbringerl => 18,
            GameVariant::Kontrabauernschnapser => 24,
            GameVariant::Herrenschnapser => 24,
        }
    }

    pub const fn trump_policy(self) -> TrumpPolicy {
        match self {
            GameVariant::Bettler
            | GameVariant::Assenbettler
            | GameVariant::AssBettler
            | GameVariant::Gang
            | GameVariant::Zehnergang
            | GameVariant::Koenigsgang
            | GameVariant::Damengang
            | GameVariant::Farbringerl
            | GameVariant::Plauderer => TrumpPolicy::NoTrump,
            GameVariant::Normal
            | GameVariant::Schnapser
            | GameVariant::Bauernloch
            | GameVariant::Bauernschnapser
            | GameVariant::Kontraschnapser
            | GameVariant::Kontrabauernschnapser
            | GameVariant::Herrenschnapser => TrumpPolicy::CalledTrump,
        }
    }

    pub const fn rank_order(self) -> RankOrder {
        match self {
            GameVariant::Assenbettler | GameVariant::Zehnergang | GameVariant::Bauernloch => {
                RankOrder::AceLow
            }
            GameVariant::Koenigsgang => RankOrder::TenLow,
            GameVariant::Damengang => RankOrder::KingLow,
            _ => RankOrder::Standard,
        }
    }

    pub const fn objective(self) -> GameObjective {
        match self {
            GameVariant::Normal => GameObjective::Reach66OrLastTrick,
            GameVariant::Bettler | GameVariant::Assenbettler | GameVariant::AssBettler => {
                GameObjective::TakeNoTricks
            }
            GameVariant::Schnapser | GameVariant::Kontraschnapser => {
                GameObjective::SchnapserPattern
            }
            GameVariant::Gang
            | GameVariant::Zehnergang
            | GameVariant::Koenigsgang
            | GameVariant::Damengang
            | GameVariant::Bauernloch
            | GameVariant::Bauernschnapser
            | GameVariant::Kontrabauernschnapser => GameObjective::TakeAllTricks,
            GameVariant::Farbringerl => GameObjective::HoldAllCardsOfSingleSuit,
            GameVariant::Herrenschnapser => GameObjective::HoldAllTrumpCards,
            GameVariant::Plauderer => GameObjective::Custom("Regional special game"),
        }
    }

    pub const fn declarer_restriction(self) -> DeclarerRestriction {
        match self {
            GameVariant::Schnapser
            | GameVariant::Bauernschnapser
            | GameVariant::Herrenschnapser
            | GameVariant::Bauernloch => DeclarerRestriction::CallerOnly,
            GameVariant::Kontraschnapser | GameVariant::Kontrabauernschnapser => {
                DeclarerRestriction::DefenderOnly
            }
            _ => DeclarerRestriction::AnyPlayer,
        }
    }

    pub const fn requires_ace_in_hand(self) -> bool {
        matches!(self, GameVariant::AssBettler)
    }

    pub const fn can_be_declared_by(self, role: PlayerRole) -> bool {
        match (self.declarer_restriction(), role) {
            (DeclarerRestriction::AnyPlayer, _) => true,
            (DeclarerRestriction::CallerOnly, PlayerRole::Caller) => true,
            (DeclarerRestriction::DefenderOnly, PlayerRole::Defender) => true,
            _ => false,
        }
    }

    pub const fn spec(self) -> GameVariantSpec {
        GameVariantSpec {
            base_points: self.base_points(),
            trump_policy: self.trump_policy(),
            rank_order: self.rank_order(),
            objective: self.objective(),
            declarer_restriction: self.declarer_restriction(),
            requires_ace_in_hand: self.requires_ace_in_hand(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameVariantSpec {
    pub base_points: u8,
    pub trump_policy: TrumpPolicy,
    pub rank_order: RankOrder,
    pub objective: GameObjective,
    pub declarer_restriction: DeclarerRestriction,
    pub requires_ace_in_hand: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GangVsZehnergangRule {
    HigherPointGameWins,
    GangHasPriority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ruleset {
    pub target_points: u16,
    pub countdown_mode: bool,
    pub allowed_variants: Vec<GameVariant>,
    pub allow_flecken: bool,
    pub gang_vs_zehnergang: GangVsZehnergangRule,
}

impl Ruleset {
    pub fn classic() -> Self {
        Self {
            target_points: MATCH_TARGET_POINTS,
            countdown_mode: false,
            allowed_variants: vec![
                GameVariant::Normal,
                GameVariant::Bettler,
                GameVariant::Schnapser,
                GameVariant::Gang,
                GameVariant::Bauernschnapser,
                GameVariant::Kontraschnapser,
                GameVariant::Farbringerl,
                GameVariant::Kontrabauernschnapser,
                GameVariant::Herrenschnapser,
            ],
            allow_flecken: true,
            gang_vs_zehnergang: GangVsZehnergangRule::HigherPointGameWins,
        }
    }

    pub fn all_variants() -> Self {
        Self {
            allowed_variants: GameVariant::ALL.to_vec(),
            ..Self::classic()
        }
    }

    pub fn is_variant_allowed(&self, variant: GameVariant) -> bool {
        self.allowed_variants.contains(&variant)
    }

    fn variant_priority(&self, variant: GameVariant) -> u8 {
        if matches!(
            self.gang_vs_zehnergang,
            GangVsZehnergangRule::GangHasPriority
        ) && variant == GameVariant::Zehnergang
        {
            GameVariant::Gang.base_points().saturating_sub(1)
        } else {
            variant.base_points()
        }
    }

    pub fn compare_announcements(
        &self,
        left: &GameAnnouncement,
        right: &GameAnnouncement,
        caller: PlayerId,
    ) -> Ordering {
        let by_variant = self
            .variant_priority(left.variant)
            .cmp(&self.variant_priority(right.variant));
        if by_variant != Ordering::Equal {
            return by_variant;
        }

        let by_talon = left.without_talon.cmp(&right.without_talon);
        if by_talon != Ordering::Equal {
            return by_talon;
        }

        if left.player == caller && right.player != caller {
            Ordering::Greater
        } else if right.player == caller && left.player != caller {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Default for Ruleset {
    fn default() -> Self {
        Self::classic()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DealMethod {
    Cut,
    Knock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrumpSelection {
    CalledFromFirstThree(Suit),
    ForcedReveal(Suit),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DoublingLevel {
    None,
    Spritzen,
    Retour,
    ReRetour,
}

impl DoublingLevel {
    pub const fn multiplier(self) -> u8 {
        match self {
            DoublingLevel::None => 1,
            DoublingLevel::Spritzen => 2,
            DoublingLevel::Retour => 4,
            DoublingLevel::ReRetour => 8,
        }
    }

    pub const fn next(self) -> Option<Self> {
        match self {
            DoublingLevel::None => Some(DoublingLevel::Spritzen),
            DoublingLevel::Spritzen => Some(DoublingLevel::Retour),
            DoublingLevel::Retour => Some(DoublingLevel::ReRetour),
            DoublingLevel::ReRetour => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DoublingCall {
    Spritzen,
    Retour,
    Re,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DoublingState {
    pub level: DoublingLevel,
}

impl DoublingState {
    pub const fn new() -> Self {
        Self {
            level: DoublingLevel::None,
        }
    }

    pub fn register_call(&mut self, call: DoublingCall) -> bool {
        let expected = match self.level {
            DoublingLevel::None => DoublingCall::Spritzen,
            DoublingLevel::Spritzen => DoublingCall::Retour,
            DoublingLevel::Retour => DoublingCall::Re,
            DoublingLevel::ReRetour => return false,
        };
        if expected != call {
            return false;
        }
        if let Some(next) = self.level.next() {
            self.level = next;
            true
        } else {
            false
        }
    }

    pub const fn multiplier(self) -> u8 {
        self.level.multiplier()
    }
}

impl Default for DoublingState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamePhase {
    DeterminingTrump,
    Bidding {
        next_to_act: PlayerId,
    },
    ExchangingTalon {
        player: PlayerId,
        discards_remaining: usize,
    },
    Playing,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameAnnouncement {
    pub player: PlayerId,
    pub variant: GameVariant,
    pub without_talon: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NormalGameResultTier {
    OpponentsAtLeast33,
    OpponentsBelow33,
    OpponentsNoTrick,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandResult {
    pub variant: GameVariant,
    pub declarer: PlayerId,
    pub winners: Vec<PlayerId>,
    pub points_awarded: u8,
    pub doubled_multiplier: u8,
    pub normal_game_tier: Option<NormalGameResultTier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlayerState {
    pub hand: Vec<Card>,
    pub won_cards: Vec<Card>,
    pub announced_marriages: Vec<Suit>,
}

impl PlayerState {
    pub fn trick_points(&self) -> u16 {
        self.won_cards.iter().map(|card| card.points() as u16).sum()
    }
}

pub type PlayedCard = (PlayerId, Card);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentTrick {
    pub leader: PlayerId,
    pub cards: Vec<PlayedCard>,
}

impl CurrentTrick {
    pub fn new(leader: PlayerId) -> Self {
        Self {
            leader,
            cards: Vec::with_capacity(PLAYER_COUNT),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedTrick {
    pub leader: PlayerId,
    pub winner: PlayerId,
    pub cards: Vec<PlayedCard>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub ruleset: Ruleset,
    pub dealer: PlayerId,
    pub caller: PlayerId,
    pub declarer: PlayerId,
    pub variant: GameVariant,
    pub trump: Option<Suit>,
    pub trump_selection: Option<TrumpSelection>,
    pub talon: Vec<Card>,
    pub active_player: PlayerId,
    pub phase: GamePhase,
    pub doubling: DoublingState,
    pub players: [PlayerState; PLAYER_COUNT],
    pub current_trick: CurrentTrick,
    pub completed_tricks: Vec<ResolvedTrick>,
}

impl GameState {
    pub fn empty(ruleset: Ruleset, dealer: PlayerId, variant: GameVariant) -> Self {
        let caller = dealer.next_clockwise();
        Self {
            ruleset,
            dealer,
            caller,
            declarer: caller,
            variant,
            trump: None,
            trump_selection: None,
            talon: Vec::with_capacity(TALON_SIZE),
            active_player: caller,
            phase: GamePhase::DeterminingTrump,
            doubling: DoublingState::default(),
            players: std::array::from_fn(|_| PlayerState::default()),
            current_trick: CurrentTrick::new(caller),
            completed_tricks: Vec::with_capacity(HAND_SIZE),
        }
    }

    pub fn player(&self, player: PlayerId) -> &PlayerState {
        &self.players[player.as_index()]
    }

    pub fn player_mut(&mut self, player: PlayerId) -> &mut PlayerState {
        &mut self.players[player.as_index()]
    }

    pub fn total_points_for_player(&self, player: PlayerId) -> u16 {
        let p = self.player(player);
        let marriage_points: u16 = p
            .announced_marriages
            .iter()
            .map(|suit| marriage_value(*suit, self.trump))
            .sum();
        p.trick_points() + marriage_points
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.phase, GamePhase::Completed)
    }
}

pub const fn marriage_value(suit: Suit, trump: Option<Suit>) -> u16 {
    if Some(suit) == trump {
        40
    } else {
        20
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameMove {
    AnnounceVariant {
        player: PlayerId,
        variant: GameVariant,
        without_talon: bool,
    },
    Pass {
        player: PlayerId,
    },
    TakeTalon {
        player: PlayerId,
    },
    Discard {
        player: PlayerId,
        cards: [Card; 2],
    },
    PlayCard {
        player: PlayerId,
        card: Card,
    },
    AnnounceMarriage {
        player: PlayerId,
        suit: Suit,
    },
    Doubling {
        player: PlayerId,
        call: DoublingCall,
    },
}

pub fn trick_winner(
    cards: &[(PlayerId, Card)],
    trump: Option<Suit>,
    rank_order: RankOrder,
) -> Option<PlayerId> {
    if cards.is_empty() {
        return None;
    }
    let lead_suit = cards[0].1.suit;
    let deciding_suit = trump
        .filter(|trump_suit| cards.iter().any(|(_, card)| card.suit == *trump_suit))
        .unwrap_or(lead_suit);

    cards
        .iter()
        .filter(|(_, card)| card.suit == deciding_suit)
        .max_by_key(|(_, card)| card.strength(rank_order))
        .map(|(player, _)| *player)
}

pub fn highest_card_in_suit(cards: &[Card], suit: Suit, order: RankOrder) -> Option<Card> {
    cards
        .iter()
        .copied()
        .filter(|card| card.suit == suit)
        .max_by_key(|card| card.strength(order))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn deck_has_20_unique_cards() {
        let deck = schnapsen_deck();
        assert_eq!(deck.len(), 20);
        let unique = deck.into_iter().collect::<HashSet<_>>();
        assert_eq!(unique.len(), 20);
    }

    #[test]
    fn variant_specs_map_expected_properties() {
        let zehnergang = GameVariant::Zehnergang.spec();
        assert_eq!(zehnergang.base_points, 10);
        assert_eq!(zehnergang.trump_policy, TrumpPolicy::NoTrump);
        assert_eq!(zehnergang.rank_order, RankOrder::AceLow);

        let kontra = GameVariant::Kontraschnapser.spec();
        assert_eq!(kontra.base_points, 12);
        assert_eq!(
            kontra.declarer_restriction,
            DeclarerRestriction::DefenderOnly
        );
        assert!(GameVariant::Kontraschnapser.can_be_declared_by(PlayerRole::Defender));
        assert!(!GameVariant::Kontraschnapser.can_be_declared_by(PlayerRole::Caller));
    }

    #[test]
    fn trick_winner_uses_trump_when_present() {
        let cards = vec![
            (PlayerId::One, Card::new(Suit::Hearts, Rank::Ace)),
            (PlayerId::Two, Card::new(Suit::Hearts, Rank::Ten)),
            (PlayerId::Three, Card::new(Suit::Clubs, Rank::Under)),
        ];
        let winner = trick_winner(&cards, Some(Suit::Clubs), RankOrder::Standard);
        assert_eq!(winner, Some(PlayerId::Three));
    }

    #[test]
    fn trick_winner_respects_ace_low_order() {
        let cards = vec![
            (PlayerId::One, Card::new(Suit::Spades, Rank::Ace)),
            (PlayerId::Two, Card::new(Suit::Spades, Rank::Under)),
            (PlayerId::Three, Card::new(Suit::Spades, Rank::Ten)),
        ];
        let winner = trick_winner(&cards, None, RankOrder::AceLow);
        assert_eq!(winner, Some(PlayerId::Three));
    }

    #[test]
    fn doubling_multiplier_progression() {
        let mut state = DoublingState::default();
        assert_eq!(state.multiplier(), 1);
        assert!(state.register_call(DoublingCall::Spritzen));
        assert_eq!(state.multiplier(), 2);
        assert!(state.register_call(DoublingCall::Retour));
        assert_eq!(state.multiplier(), 4);
        assert!(state.register_call(DoublingCall::Re));
        assert_eq!(state.multiplier(), 8);
        assert!(!state.register_call(DoublingCall::Re));
    }
}

use crate::card::{Card, Rank, Suit};

/// Identifies a player at the table (0, 1, 2).
pub type PlayerId = usize;

/// The three seat roles, rotating each hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Dealer — shuffles and deals.
    Geber,
    /// Caller — determines trump; sits left of Geber.
    Rufer,
    /// Third player — sits left of Rufer.
    Third,
}

/// All supported game types in Dreierschnapsen, ordered by ascending point value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameType {
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

impl GameType {
    /// Base point value of this game type (before any Spritzen doubling).
    pub fn base_points(self) -> u8 {
        match self {
            GameType::Normal => 1, // 1, 2, or 3 — handled dynamically
            GameType::Bettler => 4,
            GameType::Assenbettler => 5,
            GameType::AssBettler => 5,
            GameType::Schnapser => 6,
            GameType::Plauderer => 7,
            GameType::Damengang => 7,
            GameType::Koenigsgang => 8,
            GameType::Gang => 9,
            GameType::Zehnergang => 10,
            GameType::Bauernloch => 12,
            GameType::Bauernschnapser => 12,
            GameType::Kontraschnapser => 12,
            GameType::Farbringerl => 18,
            GameType::Kontrabauernschnapser => 24,
            GameType::Herrenschnapser => 24,
        }
    }

    /// Whether trump suit applies in this game type.
    pub fn has_trump(self) -> bool {
        matches!(
            self,
            GameType::Normal
                | GameType::Schnapser
                | GameType::Bauernloch
                | GameType::Bauernschnapser
                | GameType::Kontraschnapser
                | GameType::Kontrabauernschnapser
                | GameType::Herrenschnapser
                | GameType::Plauderer
        )
    }

    /// Whether only the Rufer can announce this game type.
    pub fn rufer_only(self) -> bool {
        matches!(
            self,
            GameType::Schnapser | GameType::Bauernschnapser | GameType::Bauernloch | GameType::Herrenschnapser
        )
    }

    /// Whether only a non-Rufer can announce this game type.
    pub fn non_rufer_only(self) -> bool {
        matches!(
            self,
            GameType::Kontraschnapser | GameType::Kontrabauernschnapser
        )
    }

    /// The rank ordering used for trick comparison in this game type.
    pub fn rank_ordering(self) -> RankOrdering {
        match self {
            GameType::Assenbettler | GameType::Zehnergang | GameType::Bauernloch => {
                RankOrdering::AceLow
            }
            GameType::Koenigsgang => RankOrdering::TenLow,
            GameType::Damengang => RankOrdering::KingLow,
            _ => RankOrdering::Standard,
        }
    }

    /// The goal the announcing player must achieve.
    pub fn goal(self) -> GameGoal {
        match self {
            GameType::Normal => GameGoal::Reach66,
            GameType::Bettler | GameType::Assenbettler | GameType::AssBettler => {
                GameGoal::TakeNoTricks
            }
            GameType::Schnapser | GameType::Kontraschnapser => GameGoal::Reach66,
            GameType::Gang
            | GameType::Zehnergang
            | GameType::Koenigsgang
            | GameType::Damengang
            | GameType::Bauernschnapser
            | GameType::Kontrabauernschnapser
            | GameType::Bauernloch => GameGoal::TakeAllTricks,
            GameType::Farbringerl | GameType::Herrenschnapser => GameGoal::TakeAllTricks,
            GameType::Plauderer => GameGoal::Reach66,
        }
    }

    /// Name of the game type in German.
    pub fn name_de(self) -> &'static str {
        match self {
            GameType::Normal => "Normales Spiel",
            GameType::Bettler => "Bettler",
            GameType::Assenbettler => "Assenbettler",
            GameType::AssBettler => "Ass-Bettler",
            GameType::Schnapser => "Schnapser",
            GameType::Plauderer => "Plauderer",
            GameType::Damengang => "Damengang",
            GameType::Koenigsgang => "Königsgang",
            GameType::Gang => "Gang",
            GameType::Zehnergang => "Zehnergang",
            GameType::Bauernloch => "Bauernloch",
            GameType::Bauernschnapser => "Bauernschnapser",
            GameType::Kontraschnapser => "Kontraschnapser",
            GameType::Farbringerl => "Farbringerl",
            GameType::Kontrabauernschnapser => "Kontrabauernschnapser",
            GameType::Herrenschnapser => "Herrenschnapser",
        }
    }
}

impl std::fmt::Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}pts)", self.name_de(), self.base_points())
    }
}

/// Which rank ordering to use for trick comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankOrdering {
    /// Ace > Ten > King > Queen > Jack
    Standard,
    /// Ten > King > Queen > Jack > Ace
    AceLow,
    /// King > Queen > Jack > Ace > Ten
    TenLow,
    /// Queen > Jack > Ace > Ten > King
    KingLow,
}

impl RankOrdering {
    pub fn strength(self, rank: Rank) -> u8 {
        match self {
            RankOrdering::Standard => rank.standard_strength(),
            RankOrdering::AceLow => rank.ace_low_strength(),
            RankOrdering::TenLow => rank.ten_low_strength(),
            RankOrdering::KingLow => rank.king_low_strength(),
        }
    }
}

/// What the announcing player must achieve to win.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameGoal {
    /// Reach 66 card points (Normal, Schnapser, Kontraschnapser).
    Reach66,
    /// Take zero tricks (Bettler variants).
    TakeNoTricks,
    /// Win every trick (Gang variants, Bauernschnapser, etc.).
    TakeAllTricks,
}

/// Level of Spritzen (doubling) applied to the current game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpritzenLevel {
    None,
    Gespritzt,       // ×2
    Zurueckgespritzt, // ×4
    Re,              // ×8
}

impl SpritzenLevel {
    pub fn multiplier(self) -> u8 {
        match self {
            SpritzenLevel::None => 1,
            SpritzenLevel::Gespritzt => 2,
            SpritzenLevel::Zurueckgespritzt => 4,
            SpritzenLevel::Re => 8,
        }
    }

    pub fn next(self) -> Option<SpritzenLevel> {
        match self {
            SpritzenLevel::None => Some(SpritzenLevel::Gespritzt),
            SpritzenLevel::Gespritzt => Some(SpritzenLevel::Zurueckgespritzt),
            SpritzenLevel::Zurueckgespritzt => Some(SpritzenLevel::Re),
            SpritzenLevel::Re => None,
        }
    }
}

/// A completed trick: the cards played and who won.
#[derive(Debug, Clone)]
pub struct Trick {
    pub cards: Vec<(PlayerId, Card)>,
    pub winner: PlayerId,
}

impl Trick {
    pub fn points(&self) -> u8 {
        self.cards.iter().map(|(_, c)| c.points()).sum()
    }

    pub fn lead_suit(&self) -> Suit {
        self.cards[0].1.suit
    }
}

/// A marriage announcement (Zwanziger or Vierziger).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Marriage {
    pub suit: Suit,
    pub is_trump: bool,
}

impl Marriage {
    pub fn points(self) -> u8 {
        if self.is_trump { 40 } else { 20 }
    }
}

/// An action a player can take during the game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Choose the trump suit from the first 3 cards dealt.
    CallTrump(Suit),
    /// Announce a game type during the bidding phase.
    Bid(GameType),
    /// Pass during bidding.
    Pass,
    /// Choose to play with talon (pick up + discard 2) or without.
    TakeTalon { take: bool },
    /// Discard cards after taking the talon.
    Discard(Card, Card),
    /// Play a card during trick-taking.
    PlayCard(Card),
    /// Announce a marriage (20 or 40) before playing a card.
    AnnounceMarriage(Marriage, Card),
    /// Spritzen / counter-spritzen.
    Spritzen,
    /// Accept the Spritzen without counter.
    AcceptSpritzen,
}

/// Points scored for a Normal game depend on the loser's situation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalGameResult {
    /// Loser has ≥33 points — winner gets 1
    OnePunkt,
    /// Loser has <33 points — winner gets 2 ("Schneider")
    Schneider,
    /// Loser took no tricks — winner gets 3
    ThreePoints,
}

impl NormalGameResult {
    pub fn points(self) -> u8 {
        match self {
            NormalGameResult::OnePunkt => 1,
            NormalGameResult::Schneider => 2,
            NormalGameResult::ThreePoints => 3,
        }
    }

    pub fn from_loser_points(loser_points: u8, loser_has_trick: bool) -> Self {
        if !loser_has_trick {
            NormalGameResult::ThreePoints
        } else if loser_points < 33 {
            NormalGameResult::Schneider
        } else {
            NormalGameResult::OnePunkt
        }
    }
}

/// The overall match scoring (Bummerl tracking).
#[derive(Debug, Clone)]
pub struct MatchScore {
    pub game_points: [u8; 3],
    pub bummerl: [u8; 3],
}

impl MatchScore {
    pub fn new() -> Self {
        Self {
            game_points: [0; 3],
            bummerl: [0; 3],
        }
    }

    pub fn winner(&self) -> Option<PlayerId> {
        self.game_points.iter().position(|&p| p >= 24)
    }
}

impl Default for MatchScore {
    fn default() -> Self {
        Self::new()
    }
}

/// Determines which card wins a trick, given the game's rank ordering and optional trump.
pub fn trick_winner(
    cards: &[(PlayerId, Card)],
    trump: Option<Suit>,
    ordering: RankOrdering,
) -> PlayerId {
    assert!(!cards.is_empty(), "trick must have at least one card");

    let lead_suit = cards[0].1.suit;
    let mut best_idx = 0;
    let mut best_is_trump = trump.map_or(false, |t| cards[0].1.suit == t);
    let mut best_strength = ordering.strength(cards[0].1.rank);

    for (i, &(_, card)) in cards.iter().enumerate().skip(1) {
        let is_trump = trump.map_or(false, |t| card.suit == t);
        let strength = ordering.strength(card.rank);

        let beats_current = if is_trump && !best_is_trump {
            true
        } else if !is_trump && best_is_trump {
            false
        } else if is_trump && best_is_trump {
            strength > best_strength
        } else if card.suit == lead_suit && cards[best_idx].1.suit == lead_suit {
            strength > best_strength
        } else if card.suit == lead_suit {
            true
        } else {
            false
        };

        if beats_current {
            best_idx = i;
            best_is_trump = is_trump;
            best_strength = strength;
        }
    }

    cards[best_idx].0
}

/// Returns the valid cards a player can play given the current trick state.
/// Implements Farb- und Stichzwang (suit-following and trump obligations).
pub fn valid_plays(
    hand: &[Card],
    lead: Option<(Suit, Rank)>,
    trump: Option<Suit>,
    ordering: RankOrdering,
) -> Vec<Card> {
    let Some((lead_suit, lead_rank)) = lead else {
        return hand.to_vec();
    };

    let lead_strength = ordering.strength(lead_rank);

    let same_suit: Vec<Card> = hand.iter().filter(|c| c.suit == lead_suit).copied().collect();
    let higher_same: Vec<Card> = same_suit
        .iter()
        .filter(|c| ordering.strength(c.rank) > lead_strength)
        .copied()
        .collect();

    if !higher_same.is_empty() {
        return higher_same;
    }
    if !same_suit.is_empty() {
        return same_suit;
    }

    if let Some(trump_suit) = trump {
        if trump_suit != lead_suit {
            let trumps: Vec<Card> = hand.iter().filter(|c| c.suit == trump_suit).copied().collect();
            if !trumps.is_empty() {
                return trumps;
            }
        }
    }

    hand.to_vec()
}

/// Detect available marriages in a hand.
pub fn detect_marriages(hand: &[Card], trump: Option<Suit>) -> Vec<Marriage> {
    let mut marriages = Vec::new();
    for &suit in &Suit::ALL {
        let has_king = hand.iter().any(|c| c.suit == suit && c.rank == Rank::King);
        let has_queen = hand.iter().any(|c| c.suit == suit && c.rank == Rank::Queen);
        if has_king && has_queen {
            marriages.push(Marriage {
                suit,
                is_trump: trump.map_or(false, |t| t == suit),
            });
        }
    }
    marriages
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(suit: Suit, rank: Rank) -> Card {
        Card::new(suit, rank)
    }

    #[test]
    fn game_type_ordering_is_consistent() {
        assert!(GameType::Bettler.base_points() < GameType::Gang.base_points());
        assert!(GameType::Gang.base_points() < GameType::Bauernschnapser.base_points());
        assert!(GameType::Bauernschnapser.base_points() < GameType::Farbringerl.base_points());
    }

    #[test]
    fn spritzen_multipliers() {
        assert_eq!(SpritzenLevel::None.multiplier(), 1);
        assert_eq!(SpritzenLevel::Gespritzt.multiplier(), 2);
        assert_eq!(SpritzenLevel::Zurueckgespritzt.multiplier(), 4);
        assert_eq!(SpritzenLevel::Re.multiplier(), 8);
    }

    #[test]
    fn spritzen_chain() {
        let mut level = SpritzenLevel::None;
        level = level.next().unwrap();
        assert_eq!(level, SpritzenLevel::Gespritzt);
        level = level.next().unwrap();
        assert_eq!(level, SpritzenLevel::Zurueckgespritzt);
        level = level.next().unwrap();
        assert_eq!(level, SpritzenLevel::Re);
        assert!(level.next().is_none());
    }

    #[test]
    fn trick_winner_same_suit() {
        let cards = vec![
            (0, c(Suit::Hearts, Rank::King)),
            (1, c(Suit::Hearts, Rank::Ace)),
            (2, c(Suit::Hearts, Rank::Jack)),
        ];
        assert_eq!(trick_winner(&cards, None, RankOrdering::Standard), 1);
    }

    #[test]
    fn trick_winner_trump_wins() {
        let cards = vec![
            (0, c(Suit::Hearts, Rank::Ace)),
            (1, c(Suit::Spades, Rank::Jack)),
            (2, c(Suit::Hearts, Rank::King)),
        ];
        assert_eq!(
            trick_winner(&cards, Some(Suit::Spades), RankOrdering::Standard),
            1
        );
    }

    #[test]
    fn trick_winner_off_suit_no_trump_loses() {
        let cards = vec![
            (0, c(Suit::Hearts, Rank::Jack)),
            (1, c(Suit::Spades, Rank::Ace)),
        ];
        assert_eq!(trick_winner(&cards, None, RankOrdering::Standard), 0);
    }

    #[test]
    fn trick_winner_ace_low_ordering() {
        let cards = vec![
            (0, c(Suit::Hearts, Rank::Ace)),
            (1, c(Suit::Hearts, Rank::Jack)),
        ];
        assert_eq!(trick_winner(&cards, None, RankOrdering::AceLow), 1);
    }

    #[test]
    fn valid_plays_must_follow_suit() {
        let hand = vec![
            c(Suit::Hearts, Rank::Ace),
            c(Suit::Hearts, Rank::Jack),
            c(Suit::Spades, Rank::King),
        ];
        let plays = valid_plays(
            &hand,
            Some((Suit::Hearts, Rank::Queen)),
            None,
            RankOrdering::Standard,
        );
        assert_eq!(plays, vec![c(Suit::Hearts, Rank::Ace)]);
    }

    #[test]
    fn valid_plays_must_trump_if_no_suit() {
        let hand = vec![
            c(Suit::Spades, Rank::Ace),
            c(Suit::Clubs, Rank::King),
        ];
        let plays = valid_plays(
            &hand,
            Some((Suit::Hearts, Rank::Queen)),
            Some(Suit::Clubs),
            RankOrdering::Standard,
        );
        assert_eq!(plays, vec![c(Suit::Clubs, Rank::King)]);
    }

    #[test]
    fn valid_plays_any_card_if_no_suit_no_trump() {
        let hand = vec![
            c(Suit::Spades, Rank::Ace),
            c(Suit::Clubs, Rank::King),
        ];
        let plays = valid_plays(
            &hand,
            Some((Suit::Hearts, Rank::Queen)),
            Some(Suit::Diamonds),
            RankOrdering::Standard,
        );
        assert_eq!(plays, hand);
    }

    #[test]
    fn valid_plays_lead_can_play_anything() {
        let hand = vec![
            c(Suit::Hearts, Rank::Ace),
            c(Suit::Spades, Rank::King),
        ];
        let plays = valid_plays(&hand, None, Some(Suit::Hearts), RankOrdering::Standard);
        assert_eq!(plays, hand);
    }

    #[test]
    fn detect_marriages_basic() {
        let hand = vec![
            c(Suit::Hearts, Rank::King),
            c(Suit::Hearts, Rank::Queen),
            c(Suit::Spades, Rank::Ace),
        ];
        let marriages = detect_marriages(&hand, Some(Suit::Hearts));
        assert_eq!(marriages.len(), 1);
        assert!(marriages[0].is_trump);
        assert_eq!(marriages[0].points(), 40);
    }

    #[test]
    fn detect_marriages_non_trump() {
        let hand = vec![
            c(Suit::Spades, Rank::King),
            c(Suit::Spades, Rank::Queen),
            c(Suit::Hearts, Rank::Ace),
        ];
        let marriages = detect_marriages(&hand, Some(Suit::Hearts));
        assert_eq!(marriages.len(), 1);
        assert!(!marriages[0].is_trump);
        assert_eq!(marriages[0].points(), 20);
    }

    #[test]
    fn normal_game_result_scoring() {
        assert_eq!(
            NormalGameResult::from_loser_points(40, true),
            NormalGameResult::OnePunkt
        );
        assert_eq!(
            NormalGameResult::from_loser_points(20, true),
            NormalGameResult::Schneider
        );
        assert_eq!(
            NormalGameResult::from_loser_points(0, false),
            NormalGameResult::ThreePoints
        );
    }

    #[test]
    fn match_score_winner() {
        let mut score = MatchScore::new();
        assert!(score.winner().is_none());
        score.game_points[1] = 24;
        assert_eq!(score.winner(), Some(1));
    }

    #[test]
    fn trick_points() {
        let trick = Trick {
            cards: vec![
                (0, c(Suit::Hearts, Rank::Ace)),
                (1, c(Suit::Hearts, Rank::Ten)),
                (2, c(Suit::Hearts, Rank::King)),
            ],
            winner: 0,
        };
        assert_eq!(trick.points(), 25); // 11 + 10 + 4
    }
}

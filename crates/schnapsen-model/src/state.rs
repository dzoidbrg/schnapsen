use crate::card::{Card, RankOrdering, Suit};
use crate::game_type::{DoublingLevel, GameType};
use crate::player::{PlayerId, PlayerState, Role};

/// The high-level phase of a deal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phase {
    /// Waiting for the Rufer to call trump.
    CallingTrump,
    /// Players are bidding game types.
    Bidding(BiddingState),
    /// Announcer is exchanging cards with the talon.
    TalonExchange,
    /// Cards are being played in tricks.
    Playing(PlayingState),
    /// The deal is finished.
    Finished(FinishedState),
}

/// State during the bidding phase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiddingState {
    /// Who is currently being asked.
    pub current_bidder: PlayerId,
    /// Bids made so far (player, their announcement).
    pub bids: Vec<(PlayerId, crate::action::GameAnnouncement)>,
    /// Players who have passed.
    pub passed: Vec<PlayerId>,
}

/// State during trick-taking play.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayingState {
    /// The current trick being played.
    pub current_trick: Trick,
    /// How many tricks have been completed.
    pub tricks_completed: usize,
    /// Who leads the current trick.
    pub lead_player: PlayerId,
    /// Cards won by each side (announcer vs defenders).
    pub announcer_tricks: Vec<Vec<Card>>,
    pub defender_tricks: Vec<Vec<Card>>,
    /// Card points collected by announcer.
    pub announcer_card_points: u32,
    /// Card points collected by defenders.
    pub defender_card_points: u32,
    /// Marriages announced this deal (suit, point value).
    pub marriages_announced: Vec<(PlayerId, Suit, u32)>,
    /// Whether a marriage was just announced and needs to be played.
    pub pending_marriage_play: Option<Suit>,
}

/// A single trick in progress.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trick {
    pub cards: Vec<(PlayerId, Card)>,
    pub lead_suit: Option<Suit>,
}

impl Trick {
    pub fn new() -> Self {
        Self {
            cards: Vec::with_capacity(3),
            lead_suit: None,
        }
    }

    pub fn play(&mut self, player: PlayerId, card: Card) {
        if self.cards.is_empty() {
            self.lead_suit = Some(card.suit);
        }
        self.cards.push((player, card));
    }

    pub fn is_complete(&self) -> bool {
        self.cards.len() == 3
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Determine the winner of this trick.
    pub fn winner(&self, trump: Option<Suit>, ordering: &RankOrdering) -> Option<PlayerId> {
        if !self.is_complete() {
            return None;
        }

        let lead_suit = self.lead_suit?;
        let mut best_player = self.cards[0].0;
        let mut best_card = self.cards[0].1;

        for &(player, card) in &self.cards[1..] {
            if beats(&best_card, &card, lead_suit, trump, ordering) {
                best_player = player;
                best_card = card;
            }
        }

        Some(best_player)
    }

    pub fn card_points(&self) -> u32 {
        self.cards.iter().map(|(_, c)| c.point_value()).sum()
    }
}

impl Default for Trick {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns true if `challenger` beats `current` in the given trick context.
fn beats(
    current: &Card,
    challenger: &Card,
    lead_suit: Suit,
    trump: Option<Suit>,
    ordering: &RankOrdering,
) -> bool {
    let current_is_trump = trump.map_or(false, |t| current.suit == t);
    let challenger_is_trump = trump.map_or(false, |t| challenger.suit == t);

    match (current_is_trump, challenger_is_trump) {
        (false, true) => true,
        (true, false) => false,
        _ => {
            if challenger.suit == current.suit {
                ordering.strength(challenger.rank) > ordering.strength(current.rank)
            } else if challenger.suit == lead_suit && current.suit != lead_suit {
                true
            } else if current.suit == lead_suit && challenger.suit != lead_suit {
                false
            } else {
                false
            }
        }
    }
}

/// The result of a finished deal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinishedState {
    pub winner: crate::action::Winner,
    pub points: u32,
}

/// Complete state of one deal of Dreierschnapsen.
#[derive(Debug, Clone)]
pub struct DealState {
    /// Who is the dealer this deal.
    pub dealer: PlayerId,
    /// Who is the caller (Rufer) — always left of dealer.
    pub caller: PlayerId,
    /// The third player.
    pub third: PlayerId,
    /// Player states (indexed by PlayerId).
    pub players: [PlayerState; 3],
    /// Trump suit once called.
    pub trump: Option<Suit>,
    /// The game type being played.
    pub game_type: Option<GameType>,
    /// Who announced the current game.
    pub announcer: Option<PlayerId>,
    /// Current doubling level.
    pub doubling: DoublingLevel,
    /// Who last doubled (for tracking who can re-double).
    pub last_doubler: Option<PlayerId>,
    /// The talon cards.
    pub talon: Option<[Card; 2]>,
    /// Cards discarded to talon by the announcer.
    pub discarded: Vec<Card>,
    /// Current phase of the deal.
    pub phase: Phase,
    /// First tricks of each team for visibility rule.
    pub first_trick_announcer: Option<Vec<Card>>,
    pub first_trick_defenders: Option<Vec<Card>>,
}

impl DealState {
    pub fn new(dealer: PlayerId) -> Self {
        let caller = dealer.next();
        let third = caller.next();

        let players = [
            PlayerState::new(PlayerId::Player0, Self::role_for(PlayerId::Player0, dealer)),
            PlayerState::new(PlayerId::Player1, Self::role_for(PlayerId::Player1, dealer)),
            PlayerState::new(PlayerId::Player2, Self::role_for(PlayerId::Player2, dealer)),
        ];

        Self {
            dealer,
            caller,
            third,
            players,
            trump: None,
            game_type: None,
            announcer: None,
            doubling: DoublingLevel::None,
            last_doubler: None,
            talon: None,
            discarded: Vec::new(),
            phase: Phase::CallingTrump,
            first_trick_announcer: None,
            first_trick_defenders: None,
        }
    }

    fn role_for(player: PlayerId, dealer: PlayerId) -> Role {
        if player == dealer {
            Role::Dealer
        } else if player == dealer.next() {
            Role::Caller
        } else {
            Role::Third
        }
    }

    pub fn player(&self, id: PlayerId) -> &PlayerState {
        &self.players[id.index()]
    }

    pub fn player_mut(&mut self, id: PlayerId) -> &mut PlayerState {
        &mut self.players[id.index()]
    }

    pub fn is_announcer(&self, id: PlayerId) -> bool {
        self.announcer == Some(id)
    }

    pub fn is_defender(&self, id: PlayerId) -> bool {
        self.announcer.map_or(false, |a| a != id)
    }

    pub fn rank_ordering(&self) -> RankOrdering {
        self.game_type
            .map_or(RankOrdering::Standard, |gt| gt.rank_ordering())
    }

    pub fn effective_trump(&self) -> Option<Suit> {
        if self.game_type.map_or(true, |gt| gt.uses_trump()) {
            self.trump
        } else {
            None
        }
    }

    /// Calculate final points for the current deal including doubling.
    pub fn final_points(&self) -> u32 {
        let base = self.game_type.map_or(1, |gt| gt.base_points());
        base * self.doubling.multiplier()
    }
}

/// Overall match state tracking scores across multiple deals.
#[derive(Debug, Clone)]
pub struct MatchState {
    pub scores: [u32; 3],
    pub bummerl: [u32; 3],
    pub target_score: u32,
    pub current_dealer: PlayerId,
    pub current_deal: Option<DealState>,
}

impl MatchState {
    pub fn new() -> Self {
        Self {
            scores: [0; 3],
            bummerl: [0; 3],
            target_score: 24,
            current_dealer: PlayerId::Player0,
            current_deal: None,
        }
    }

    pub fn winner(&self) -> Option<PlayerId> {
        for id in PlayerId::all() {
            if self.scores[id.index()] >= self.target_score {
                return Some(id);
            }
        }
        None
    }

    pub fn advance_dealer(&mut self) {
        self.current_dealer = self.current_dealer.next();
    }
}

impl Default for MatchState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn trick_determines_winner_by_lead_suit() {
        let mut trick = Trick::new();
        trick.play(PlayerId::Player0, Card::new(Suit::Hearts, Rank::Jack));
        trick.play(PlayerId::Player1, Card::new(Suit::Hearts, Rank::Ace));
        trick.play(PlayerId::Player2, Card::new(Suit::Hearts, Rank::Ten));

        let winner = trick.winner(Some(Suit::Spades), &RankOrdering::Standard);
        assert_eq!(winner, Some(PlayerId::Player1));
    }

    #[test]
    fn trick_trump_beats_lead_suit() {
        let mut trick = Trick::new();
        trick.play(PlayerId::Player0, Card::new(Suit::Hearts, Rank::Ace));
        trick.play(PlayerId::Player1, Card::new(Suit::Spades, Rank::Jack));
        trick.play(PlayerId::Player2, Card::new(Suit::Hearts, Rank::Ten));

        let winner = trick.winner(Some(Suit::Spades), &RankOrdering::Standard);
        assert_eq!(winner, Some(PlayerId::Player1));
    }

    #[test]
    fn trick_higher_trump_wins() {
        let mut trick = Trick::new();
        trick.play(PlayerId::Player0, Card::new(Suit::Hearts, Rank::Ace));
        trick.play(PlayerId::Player1, Card::new(Suit::Spades, Rank::Jack));
        trick.play(PlayerId::Player2, Card::new(Suit::Spades, Rank::Ten));

        let winner = trick.winner(Some(Suit::Spades), &RankOrdering::Standard);
        assert_eq!(winner, Some(PlayerId::Player2));
    }

    #[test]
    fn trick_no_trump_off_suit_cannot_win() {
        let mut trick = Trick::new();
        trick.play(PlayerId::Player0, Card::new(Suit::Hearts, Rank::Jack));
        trick.play(PlayerId::Player1, Card::new(Suit::Diamonds, Rank::Ace));
        trick.play(PlayerId::Player2, Card::new(Suit::Clubs, Rank::Ace));

        let winner = trick.winner(None, &RankOrdering::Standard);
        assert_eq!(winner, Some(PlayerId::Player0));
    }

    #[test]
    fn trick_card_points() {
        let mut trick = Trick::new();
        trick.play(PlayerId::Player0, Card::new(Suit::Hearts, Rank::Ace));
        trick.play(PlayerId::Player1, Card::new(Suit::Spades, Rank::Ten));
        trick.play(PlayerId::Player2, Card::new(Suit::Diamonds, Rank::Jack));
        assert_eq!(trick.card_points(), 11 + 10 + 2);
    }

    #[test]
    fn deal_state_roles() {
        let state = DealState::new(PlayerId::Player0);
        assert_eq!(state.player(PlayerId::Player0).role, Role::Dealer);
        assert_eq!(state.player(PlayerId::Player1).role, Role::Caller);
        assert_eq!(state.player(PlayerId::Player2).role, Role::Third);
    }

    #[test]
    fn match_state_no_winner_at_start() {
        let state = MatchState::new();
        assert!(state.winner().is_none());
    }

    #[test]
    fn match_state_detects_winner() {
        let mut state = MatchState::new();
        state.scores[1] = 24;
        assert_eq!(state.winner(), Some(PlayerId::Player1));
    }

    #[test]
    fn ace_low_ordering_in_trick() {
        let mut trick = Trick::new();
        trick.play(PlayerId::Player0, Card::new(Suit::Hearts, Rank::Ace));
        trick.play(PlayerId::Player1, Card::new(Suit::Hearts, Rank::Jack));
        trick.play(PlayerId::Player2, Card::new(Suit::Hearts, Rank::Ten));

        let winner = trick.winner(None, &RankOrdering::AceLow);
        assert_eq!(winner, Some(PlayerId::Player2));
    }
}

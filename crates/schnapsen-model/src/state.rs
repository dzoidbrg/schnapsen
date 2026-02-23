use crate::card::{compare_rank, Card, RankOrder, Suit};
use crate::player::PlayerId;
use crate::scoring::Spritzen;
use crate::variant::{GameDeclaration, TrumpRule};

pub type Hand = Vec<Card>;

/// Marriage announcement category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MarriageKind {
    Twenty,
    Forty,
}

/// A single marriage announcement.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MarriageAnnouncement {
    pub player: PlayerId,
    pub suit: Suit,
    pub kind: MarriageKind,
}

/// A card already played to a trick.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayedCard {
    pub player: PlayerId,
    pub card: Card,
}

/// One in-progress or completed trick.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Trick {
    pub leader: PlayerId,
    pub plays: Vec<PlayedCard>,
}

impl Trick {
    pub fn new(leader: PlayerId) -> Self {
        Self {
            leader,
            plays: Vec::with_capacity(3),
        }
    }

    pub fn led_suit(&self) -> Option<Suit> {
        self.plays.first().map(|play| play.card.suit)
    }

    pub fn is_complete(&self) -> bool {
        self.plays.len() == 3
    }

    pub fn current_winning_play(
        &self,
        trump: Option<Suit>,
        rank_order: RankOrder,
    ) -> Option<PlayedCard> {
        let lead_suit = self.led_suit()?;
        let mut winner = *self.plays.first()?;
        for play in self.plays.iter().skip(1) {
            if card_beats(play.card, winner.card, lead_suit, trump, rank_order) {
                winner = *play;
            }
        }
        Some(winner)
    }
}

/// True if `candidate` outranks `incumbent` in the current trick context.
pub fn card_beats(
    candidate: Card,
    incumbent: Card,
    lead_suit: Suit,
    trump: Option<Suit>,
    rank_order: RankOrder,
) -> bool {
    if candidate.suit == incumbent.suit {
        return compare_rank(rank_order, candidate.rank, incumbent.rank).is_gt();
    }

    if let Some(trump_suit) = trump {
        if candidate.suit == trump_suit && incumbent.suit != trump_suit {
            return true;
        }
        if candidate.suit != trump_suit && incumbent.suit == trump_suit {
            return false;
        }
    }

    candidate.suit == lead_suit && incumbent.suit != lead_suit
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RoundPhase {
    Bidding,
    Playing,
    Finished,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PlayerMove {
    PlayCard(Card),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RoundError {
    NotPlayersTurn,
    CardNotInHand,
    IllegalCard,
    RoundNotPlayable,
}

/// One full 6-card round state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoundState {
    pub caller: PlayerId,
    pub declarer: PlayerId,
    pub declaration: GameDeclaration,
    pub phase: RoundPhase,
    pub active_player: PlayerId,
    pub trump: Option<Suit>,
    pub hands: [Hand; 3],
    pub talon: Vec<Card>,
    pub current_trick: Trick,
    pub completed_tricks: Vec<Trick>,
    pub card_points: [u16; 3],
    pub announcements: Vec<MarriageAnnouncement>,
    pub spritzen: Spritzen,
}

impl RoundState {
    pub fn new(
        caller: PlayerId,
        declarer: PlayerId,
        declaration: GameDeclaration,
        trump: Option<Suit>,
        hands: [Hand; 3],
        talon: Vec<Card>,
    ) -> Self {
        Self {
            caller,
            declarer,
            declaration,
            phase: RoundPhase::Playing,
            active_player: caller,
            trump,
            hands,
            talon,
            current_trick: Trick::new(caller),
            completed_tricks: Vec::new(),
            card_points: [0, 0, 0],
            announcements: Vec::new(),
            spritzen: Spritzen::new(),
        }
    }

    pub fn rank_order(&self) -> RankOrder {
        self.declaration.rank_order()
    }

    pub fn active_trump(&self) -> Option<Suit> {
        match self.declaration.trump_rule() {
            TrumpRule::CalledTrump => self.trump,
            TrumpRule::NoTrump => None,
        }
    }

    pub fn hand(&self, player: PlayerId) -> &[Card] {
        &self.hands[player.index()]
    }

    pub fn is_round_over(&self) -> bool {
        self.phase == RoundPhase::Finished
    }

    pub fn legal_cards_for_player(&self, player: PlayerId) -> Vec<Card> {
        if self.phase != RoundPhase::Playing || player != self.active_player {
            return Vec::new();
        }

        let hand = self.hand(player);
        if hand.is_empty() {
            return Vec::new();
        }
        if self.current_trick.plays.is_empty() {
            return hand.to_vec();
        }

        let lead_suit = match self.current_trick.led_suit() {
            Some(suit) => suit,
            None => return hand.to_vec(),
        };
        let trump = self.active_trump();
        let rank_order = self.rank_order();
        let winning_play = match self.current_trick.current_winning_play(trump, rank_order) {
            Some(play) => play,
            None => return hand.to_vec(),
        };

        let same_suit_cards: Vec<Card> = hand
            .iter()
            .copied()
            .filter(|card| card.suit == lead_suit)
            .collect();
        if !same_suit_cards.is_empty() {
            let beating_same_suit: Vec<Card> = same_suit_cards
                .iter()
                .copied()
                .filter(|card| card_beats(*card, winning_play.card, lead_suit, trump, rank_order))
                .collect();
            if !beating_same_suit.is_empty() {
                return beating_same_suit;
            }
            return same_suit_cards;
        }

        if let Some(trump_suit) = trump {
            let trump_cards: Vec<Card> = hand
                .iter()
                .copied()
                .filter(|card| card.suit == trump_suit)
                .collect();
            if !trump_cards.is_empty() {
                let beating_trumps: Vec<Card> = trump_cards
                    .iter()
                    .copied()
                    .filter(|card| {
                        card_beats(*card, winning_play.card, lead_suit, trump, rank_order)
                    })
                    .collect();
                if !beating_trumps.is_empty() {
                    return beating_trumps;
                }
                return trump_cards;
            }
        }

        hand.to_vec()
    }

    pub fn apply_move(&mut self, player: PlayerId, mv: PlayerMove) -> Result<(), RoundError> {
        match mv {
            PlayerMove::PlayCard(card) => self.apply_play(player, card),
        }
    }

    pub fn apply_play(&mut self, player: PlayerId, card: Card) -> Result<(), RoundError> {
        if self.phase != RoundPhase::Playing {
            return Err(RoundError::RoundNotPlayable);
        }
        if player != self.active_player {
            return Err(RoundError::NotPlayersTurn);
        }

        let legal_cards = self.legal_cards_for_player(player);
        if !legal_cards.contains(&card) {
            return Err(RoundError::IllegalCard);
        }

        let hand = &mut self.hands[player.index()];
        let pos = hand.iter().position(|candidate| *candidate == card);
        let Some(pos) = pos else {
            return Err(RoundError::CardNotInHand);
        };
        hand.remove(pos);
        self.current_trick.plays.push(PlayedCard { player, card });

        if self.current_trick.is_complete() {
            let trick = self.current_trick.clone();
            let trump = self.active_trump();
            let rank_order = self.rank_order();
            let winner = trick
                .current_winning_play(trump, rank_order)
                .expect("complete trick must have winner")
                .player;

            let trick_points: u16 = trick
                .plays
                .iter()
                .map(|play| u16::from(play.card.rank.card_points()))
                .sum();
            self.card_points[winner.index()] += trick_points;
            self.completed_tricks.push(trick);

            let cards_left = self.hands.iter().any(|h| !h.is_empty());
            if cards_left {
                self.active_player = winner;
                self.current_trick = Trick::new(winner);
            } else {
                self.phase = RoundPhase::Finished;
                self.active_player = winner;
                self.current_trick = Trick::new(winner);
            }
        } else {
            self.active_player = player.next_cw();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    fn card(suit: Suit, rank: Rank) -> Card {
        Card::new(suit, rank)
    }

    #[test]
    fn legal_cards_enforce_follow_suit_and_trump() {
        let hands = [
            vec![card(Suit::Hearts, Rank::Ace)],
            vec![
                card(Suit::Hearts, Rank::Ten),
                card(Suit::Clubs, Rank::Ace),
                card(Suit::Spades, Rank::Ace),
            ],
            vec![
                card(Suit::Clubs, Rank::King),
                card(Suit::Diamonds, Rank::Ace),
            ],
        ];
        let mut state = RoundState::new(
            PlayerId::P0,
            PlayerId::P0,
            GameDeclaration::Normal,
            Some(Suit::Clubs),
            hands,
            vec![],
        );

        state
            .apply_play(PlayerId::P0, card(Suit::Hearts, Rank::Ace))
            .expect("lead should be legal");

        let legal_for_p1 = state.legal_cards_for_player(PlayerId::P1);
        assert_eq!(legal_for_p1, vec![card(Suit::Hearts, Rank::Ten)]);
    }

    #[test]
    fn trick_winner_collects_points() {
        let hands = [
            vec![card(Suit::Hearts, Rank::Ace)],
            vec![card(Suit::Hearts, Rank::Ten)],
            vec![card(Suit::Clubs, Rank::Ace)],
        ];
        let mut state = RoundState::new(
            PlayerId::P0,
            PlayerId::P0,
            GameDeclaration::Normal,
            Some(Suit::Clubs),
            hands,
            vec![],
        );

        state
            .apply_play(PlayerId::P0, card(Suit::Hearts, Rank::Ace))
            .expect("lead");
        state
            .apply_play(PlayerId::P1, card(Suit::Hearts, Rank::Ten))
            .expect("follow");
        state
            .apply_play(PlayerId::P2, card(Suit::Clubs, Rank::Ace))
            .expect("trump");

        assert_eq!(state.card_points[PlayerId::P2.index()], 32);
        assert!(state.is_round_over());
    }
}

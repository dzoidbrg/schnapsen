use schnapsen_model::card::{Card, RankOrdering, Suit};
use schnapsen_model::player::{Hand, PlayerId};
use schnapsen_model::state::{DealState, Phase, Trick};

/// Returns the list of cards that a player is legally allowed to play in the current trick.
pub fn valid_cards(deal: &DealState, player: PlayerId) -> Vec<Card> {
    let hand = &deal.player(player).hand;

    let playing = match &deal.phase {
        Phase::Playing(ps) => ps,
        _ => return Vec::new(),
    };

    let trick = &playing.current_trick;
    let trump = deal.effective_trump();
    let ordering = deal.rank_ordering();

    if trick.is_empty() {
        if let Some(suit) = playing.pending_marriage_play {
            return hand
                .cards()
                .iter()
                .filter(|c| c.suit == suit)
                .copied()
                .collect();
        }
        return hand.cards().to_vec();
    }

    legal_follow_cards(hand, trick, trump, &ordering)
}

/// Determine which cards a player may play when following in a trick.
/// Implements Farb- und Stichzwang (suit-following and trick-taking obligation).
fn legal_follow_cards(
    hand: &Hand,
    trick: &Trick,
    trump: Option<Suit>,
    ordering: &RankOrdering,
) -> Vec<Card> {
    let lead_suit = match trick.lead_suit {
        Some(s) => s,
        None => return hand.cards().to_vec(),
    };

    let cards_in_hand = hand.cards();
    let same_suit: Vec<Card> = cards_in_hand.iter().filter(|c| c.suit == lead_suit).copied().collect();

    if !same_suit.is_empty() {
        let current_best = trick_best_of_suit(trick, lead_suit, ordering);

        let higher: Vec<Card> = same_suit
            .iter()
            .filter(|c| ordering.strength(c.rank) > current_best)
            .copied()
            .collect();

        if !higher.is_empty() {
            return higher;
        }
        return same_suit;
    }

    if let Some(trump_suit) = trump {
        if trump_suit != lead_suit {
            let trump_cards: Vec<Card> = cards_in_hand
                .iter()
                .filter(|c| c.suit == trump_suit)
                .copied()
                .collect();

            if !trump_cards.is_empty() {
                let current_trump_best = trick_best_of_suit(trick, trump_suit, ordering);

                let higher_trump: Vec<Card> = trump_cards
                    .iter()
                    .filter(|c| ordering.strength(c.rank) > current_trump_best)
                    .copied()
                    .collect();

                if !higher_trump.is_empty() {
                    return higher_trump;
                }
                return trump_cards;
            }
        }
    }

    cards_in_hand.to_vec()
}

/// Find the highest strength of a given suit played in the current trick.
fn trick_best_of_suit(trick: &Trick, suit: Suit, ordering: &RankOrdering) -> u8 {
    trick
        .cards
        .iter()
        .filter(|(_, c)| c.suit == suit)
        .map(|(_, c)| ordering.strength(c.rank))
        .max()
        .unwrap_or(0)
}

/// Check if a marriage announcement is valid for the given player and suit.
pub fn can_announce_marriage(deal: &DealState, player: PlayerId, suit: Suit) -> bool {
    if !matches!(&deal.phase, Phase::Playing(ps) if ps.current_trick.is_empty()) {
        return false;
    }

    if let Phase::Playing(ps) = &deal.phase {
        if ps.lead_player != player {
            return false;
        }
    }

    let hand = &deal.player(player).hand;
    if !hand.has_marriage(suit) {
        return false;
    }

    let game_type = match deal.game_type {
        Some(gt) => gt,
        None => return false,
    };

    match game_type {
        schnapsen_model::game_type::GameType::Normal
        | schnapsen_model::game_type::GameType::Schnapser
        | schnapsen_model::game_type::GameType::Kontraschnapser => true,
        _ => false,
    }
}

/// Check if Spritzen is valid for the given player.
pub fn can_spritzen(deal: &DealState, player: PlayerId) -> bool {
    if !deal.doubling.can_raise() {
        return false;
    }

    if deal.last_doubler == Some(player) {
        return false;
    }

    let is_announcer = deal.is_announcer(player);

    match deal.doubling {
        schnapsen_model::DoublingLevel::None => !is_announcer,
        schnapsen_model::DoublingLevel::Gespritzt => is_announcer,
        schnapsen_model::DoublingLevel::Zurueckgespritzt => !is_announcer,
        schnapsen_model::DoublingLevel::NochmalZurueckgespritzt => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use schnapsen_model::card::{Card, Rank, Suit};
    use schnapsen_model::game_type::GameType;
    use schnapsen_model::player::PlayerId;
    use schnapsen_model::state::{DealState, Phase, PlayingState, Trick};

    fn setup_playing_deal(
        hand_cards: Vec<Card>,
        trick_cards: Vec<(PlayerId, Card)>,
        trump: Option<Suit>,
        game_type: GameType,
        lead_player: PlayerId,
    ) -> DealState {
        let mut deal = DealState::new(PlayerId::Player2);
        deal.trump = trump;
        deal.game_type = Some(game_type);
        deal.announcer = Some(PlayerId::Player1);

        deal.player_mut(PlayerId::Player1).hand =
            schnapsen_model::player::Hand::from_cards(hand_cards);

        let mut trick = Trick::new();
        for (pid, card) in trick_cards {
            trick.play(pid, card);
        }

        deal.phase = Phase::Playing(PlayingState {
            current_trick: trick,
            tricks_completed: 0,
            lead_player,
            announcer_tricks: Vec::new(),
            defender_tricks: Vec::new(),
            announcer_card_points: 0,
            defender_card_points: 0,
            marriages_announced: Vec::new(),
            pending_marriage_play: None,
        });

        deal
    }

    #[test]
    fn must_follow_suit_with_higher_card() {
        let deal = setup_playing_deal(
            vec![
                Card::new(Suit::Hearts, Rank::King),
                Card::new(Suit::Hearts, Rank::Ace),
                Card::new(Suit::Spades, Rank::Ten),
            ],
            vec![(PlayerId::Player0, Card::new(Suit::Hearts, Rank::Ten))],
            Some(Suit::Spades),
            GameType::Normal,
            PlayerId::Player0,
        );

        let valid = valid_cards(&deal, PlayerId::Player1);
        assert_eq!(valid, vec![Card::new(Suit::Hearts, Rank::Ace)]);
    }

    #[test]
    fn must_follow_suit_with_lower_if_no_higher() {
        let deal = setup_playing_deal(
            vec![
                Card::new(Suit::Hearts, Rank::Jack),
                Card::new(Suit::Spades, Rank::Ten),
            ],
            vec![(PlayerId::Player0, Card::new(Suit::Hearts, Rank::Ace))],
            Some(Suit::Spades),
            GameType::Normal,
            PlayerId::Player0,
        );

        let valid = valid_cards(&deal, PlayerId::Player1);
        assert_eq!(valid, vec![Card::new(Suit::Hearts, Rank::Jack)]);
    }

    #[test]
    fn must_trump_if_no_suit_cards() {
        let deal = setup_playing_deal(
            vec![
                Card::new(Suit::Spades, Rank::Jack),
                Card::new(Suit::Spades, Rank::King),
                Card::new(Suit::Clubs, Rank::Ace),
            ],
            vec![(PlayerId::Player0, Card::new(Suit::Hearts, Rank::Ace))],
            Some(Suit::Spades),
            GameType::Normal,
            PlayerId::Player0,
        );

        let valid = valid_cards(&deal, PlayerId::Player1);
        assert_eq!(valid.len(), 2);
        assert!(valid.iter().all(|c| c.suit == Suit::Spades));
    }

    #[test]
    fn can_play_anything_if_no_suit_no_trump() {
        let deal = setup_playing_deal(
            vec![
                Card::new(Suit::Clubs, Rank::Jack),
                Card::new(Suit::Diamonds, Rank::King),
            ],
            vec![(PlayerId::Player0, Card::new(Suit::Hearts, Rank::Ace))],
            None,
            GameType::Bettler,
            PlayerId::Player0,
        );

        let valid = valid_cards(&deal, PlayerId::Player1);
        assert_eq!(valid.len(), 2);
    }

    #[test]
    fn leading_player_can_play_any_card() {
        let deal = setup_playing_deal(
            vec![
                Card::new(Suit::Hearts, Rank::Ace),
                Card::new(Suit::Spades, Rank::Ten),
                Card::new(Suit::Clubs, Rank::King),
            ],
            vec![],
            Some(Suit::Hearts),
            GameType::Normal,
            PlayerId::Player1,
        );

        let valid = valid_cards(&deal, PlayerId::Player1);
        assert_eq!(valid.len(), 3);
    }

    #[test]
    fn no_cards_outside_playing_phase() {
        let deal = DealState::new(PlayerId::Player0);
        let valid = valid_cards(&deal, PlayerId::Player1);
        assert!(valid.is_empty());
    }
}

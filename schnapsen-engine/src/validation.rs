//! Move validation: compute all legal moves for the current player.

use schnapsen_model::{
    Card, CardOrdering, GamePhase, GameType, RoundState, Suit,
};

use crate::moves::GameMove;

/// Returns all valid moves for the current player in the given round state.
pub fn valid_moves(round: &RoundState) -> Vec<GameMove> {
    match &round.phase {
        GamePhase::TrumpCalling => valid_trump_calls(round),
        GamePhase::Bidding => valid_bids(round),
        GamePhase::Playing => valid_plays(round),
        _ => Vec::new(),
    }
}

fn valid_trump_calls(round: &RoundState) -> Vec<GameMove> {
    let player = round.next_to_play;
    if player != round.rufer {
        return Vec::new();
    }
    Suit::all()
        .iter()
        .map(|&s| GameMove::CallTrump(s))
        .collect()
}

fn valid_bids(round: &RoundState) -> Vec<GameMove> {
    let mut moves = vec![GameMove::Pass];
    let player = round.next_to_play;
    let config = &schnapsen_model::GameConfig::default();

    for gt in GameType::all_standard() {
        if gt == GameType::NormalesSpiel {
            continue;
        }
        let info = gt.info();
        if info.rufer_only && player != round.rufer {
            continue;
        }
        if info.opponents_only {
            let is_opponent = player != round.rufer;
            if !is_opponent {
                continue;
            }
        }
        if !config.allow_bettler && (gt == GameType::Bettler || gt == GameType::Assenbettler || gt == GameType::AssBettler) {
            continue;
        }
        if !config.allow_gang_variants && (gt == GameType::Zehnergang || gt == GameType::Koenigsgang || gt == GameType::Damengang) {
            continue;
        }
        let max_bid = round.bids.iter().map(|b| b.game_type.base_points()).max();
        if let Some(max) = max_bid {
            if gt.base_points() <= max {
                continue;
            }
        }
        moves.push(GameMove::DeclareGame(gt));
    }
    moves
}

fn valid_plays(round: &RoundState) -> Vec<GameMove> {
    let player = round.next_to_play;
    let hand = &round.hands[player as usize];
    let trick = &round.current_trick;

    if hand.is_empty() {
        return Vec::new();
    }

    let cards: Vec<Card> = hand.cards().to_vec();

    if trick.len() == 0 {
        return cards.into_iter().map(GameMove::PlayCard).collect();
    }

    let ordering = round
        .game_type
        .map(|gt| gt.info().card_ordering)
        .unwrap_or(CardOrdering::Normal);
    let trump = round.trump;

    let lead_suit = trick.lead_suit().unwrap();
    let same_suit: Vec<Card> = cards
        .iter()
        .filter(|c| c.suit == lead_suit)
        .cloned()
        .collect();

    let _has_trump = trump.map(|t| cards.iter().any(|c| c.suit == t)).unwrap_or(false);

    if !same_suit.is_empty() {
        let led_cards: Vec<Card> = trick
            .plays()
            .iter()
            .map(|(_, c)| *c)
            .collect();
        let best_led = best_in_trick(&led_cards, lead_suit, trump, ordering);
        let can_beat = same_suit.iter().any(|c| beats(*c, best_led, lead_suit, trump, ordering));
        if can_beat {
            let beating: Vec<Card> = same_suit
                .into_iter()
                .filter(|c| beats(*c, best_led, lead_suit, trump, ordering))
                .collect();
            return beating.into_iter().map(GameMove::PlayCard).collect();
        } else {
            return same_suit.into_iter().map(GameMove::PlayCard).collect();
        }
    }

    if let Some(t) = trump {
        let trump_cards: Vec<Card> = cards.iter().filter(|c| c.suit == t).cloned().collect();
        if !trump_cards.is_empty() {
            let led_cards: Vec<Card> = trick.plays().iter().map(|(_, c)| *c).collect();
            let best_led = best_in_trick(&led_cards, lead_suit, Some(t), ordering);
            let can_beat = trump_cards
                .iter()
                .any(|c| beats(*c, best_led, lead_suit, Some(t), ordering));
            if can_beat {
                let beating: Vec<Card> = trump_cards
                    .into_iter()
                    .filter(|c| beats(*c, best_led, lead_suit, Some(t), ordering))
                    .collect();
                return beating.into_iter().map(GameMove::PlayCard).collect();
            } else {
                return trump_cards.into_iter().map(GameMove::PlayCard).collect();
            }
        }
    }

    cards.into_iter().map(GameMove::PlayCard).collect()
}

fn best_in_trick(
    plays: &[Card],
    led_suit: Suit,
    trump: Option<Suit>,
    ordering: CardOrdering,
) -> Card {
    *plays
        .iter()
        .max_by(|a, b| {
            let ord_a = card_strength(**a, led_suit, trump, ordering);
            let ord_b = card_strength(**b, led_suit, trump, ordering);
            ord_a.cmp(&ord_b)
        })
        .unwrap()
}

fn card_strength(card: Card, _led_suit: Suit, trump: Option<Suit>, ordering: CardOrdering) -> u8 {
    let is_trump = trump.map(|t| card.suit == t).unwrap_or(false);
    let is_led = card.suit == _led_suit;
    let rank_ord = match ordering {
        CardOrdering::Normal => 5 - card.rank.ordinal_normal(),
        CardOrdering::AssLowest => 5 - card.rank.ordinal_ass_lowest(),
        CardOrdering::ZehnerLowest => 5 - card.rank.ordinal_zehner_lowest(),
        CardOrdering::KoenigLowest => 5 - card.rank.ordinal_koenig_lowest(),
    };
    if is_trump && is_led {
        20 + rank_ord
    } else if is_trump {
        10 + rank_ord
    } else if is_led {
        rank_ord
    } else {
        0
    }
}

fn beats(
    card: Card,
    other: Card,
    _led_suit: Suit,
    trump: Option<Suit>,
    ordering: CardOrdering,
) -> bool {
    if card.suit != other.suit {
        if let Some(t) = trump {
            if card.suit == t && other.suit != t {
                return true;
            }
            if card.suit != t && other.suit == t {
                return false;
            }
        }
        return false;
    }
    let ord_c = match ordering {
        CardOrdering::Normal => card.rank.ordinal_normal(),
        CardOrdering::AssLowest => card.rank.ordinal_ass_lowest(),
        CardOrdering::ZehnerLowest => card.rank.ordinal_zehner_lowest(),
        CardOrdering::KoenigLowest => card.rank.ordinal_koenig_lowest(),
    };
    let ord_o = match ordering {
        CardOrdering::Normal => other.rank.ordinal_normal(),
        CardOrdering::AssLowest => other.rank.ordinal_ass_lowest(),
        CardOrdering::ZehnerLowest => other.rank.ordinal_zehner_lowest(),
        CardOrdering::KoenigLowest => other.rank.ordinal_koenig_lowest(),
    };
    ord_c < ord_o
}

#[cfg(test)]
mod tests {
    use super::*;
    use schnapsen_model::{Card, Rank, Suit};

    #[test]
    fn trump_calling_only_for_rufer() {
        let mut round = RoundState::new(0); // dealer=0, rufer=1
        round.phase = GamePhase::TrumpCalling;
        round.next_to_play = 0; // not rufer (rufer is 1)
        let moves = valid_moves(&round);
        assert!(moves.is_empty(), "non-rufer should have no trump calls");

        round.next_to_play = 1; // rufer
        let moves = valid_moves(&round);
        assert_eq!(moves.len(), 4, "rufer can call any of 4 suits");
    }

    #[test]
    fn lead_any_card() {
        let mut round = RoundState::new(0);
        round.phase = GamePhase::Playing;
        round.trump = Some(Suit::Herz);
        round.game_type = Some(GameType::NormalesSpiel);
        round.next_to_play = 0;
        round.hands[0].add(Card::new(Suit::Herz, Rank::Ass));
        round.hands[0].add(Card::new(Suit::Karo, Rank::Zehner));
        let moves = valid_moves(&round);
        assert_eq!(moves.len(), 2);
    }
}

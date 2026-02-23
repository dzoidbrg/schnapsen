use rand::seq::SliceRandom;
use rand::Rng;

use schnapsen_model::action::Action;
use schnapsen_model::card::Suit;
use schnapsen_model::player::PlayerId;
use schnapsen_model::state::{DealState, Phase};

use crate::engine::MoveSelector;
use crate::validator;

/// A move selector that always picks a random valid move.
pub struct RandomSelector<R: Rng> {
    rng: R,
}

impl<R: Rng> RandomSelector<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> MoveSelector for RandomSelector<R> {
    fn select_action(&mut self, deal: &DealState, player: PlayerId) -> Action {
        match &deal.phase {
            Phase::CallingTrump => {
                let hand = &deal.player(player).hand;
                let suits: Vec<Suit> = hand.cards().iter().map(|c| c.suit).collect();
                let suit = suits.choose(&mut self.rng).copied().unwrap_or(Suit::Hearts);
                Action::CallTrump(suit)
            }

            Phase::Bidding(_) => Action::Pass,

            Phase::TalonExchange => {
                let hand = &deal.player(player).hand;
                if hand.len() == 6 {
                    Action::TakeTalon
                } else {
                    let cards: Vec<_> = hand.cards().to_vec();
                    let c1 = cards[0];
                    let c2 = cards[1];
                    Action::DiscardToTalon(c1, c2)
                }
            }

            Phase::Playing(_) => {
                let valid = validator::valid_cards(deal, player);
                if valid.is_empty() {
                    let hand = &deal.player(player).hand;
                    Action::PlayCard(hand.cards()[0])
                } else {
                    let card = valid.choose(&mut self.rng).copied().unwrap();
                    Action::PlayCard(card)
                }
            }

            Phase::Finished(_) => Action::Pass,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::GameEngine;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn random_selector_completes_deal() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut engine = GameEngine::new();

        let mut s0 = RandomSelector::new(StdRng::seed_from_u64(1));
        let mut s1 = RandomSelector::new(StdRng::seed_from_u64(2));
        let mut s2 = RandomSelector::new(StdRng::seed_from_u64(3));

        let result = engine.play_deal(
            &mut rng,
            &mut [&mut s0, &mut s1, &mut s2],
        );

        assert!(result.is_ok(), "Deal should complete without errors: {:?}", result);

        let deal = engine.match_state.current_deal.as_ref().unwrap();
        assert!(matches!(deal.phase, Phase::Finished(_)));
    }

    #[test]
    fn random_selector_multiple_deals() {
        let mut rng = StdRng::seed_from_u64(100);
        let mut engine = GameEngine::new();

        for _ in 0..10 {
            let mut s0 = RandomSelector::new(StdRng::seed_from_u64(rng.gen()));
            let mut s1 = RandomSelector::new(StdRng::seed_from_u64(rng.gen()));
            let mut s2 = RandomSelector::new(StdRng::seed_from_u64(rng.gen()));

            let result = engine.play_deal(
                &mut rng,
                &mut [&mut s0, &mut s1, &mut s2],
            );

            assert!(result.is_ok(), "Deal {} should complete: {:?}", engine.match_state.current_dealer.index(), result);
        }

        let total_score: u32 = engine.match_state.scores.iter().sum();
        assert!(total_score > 0, "Some points should have been scored");
    }
}

use schnapsen_model::action::Action;
use schnapsen_model::deck::Deck;
use schnapsen_model::game_type::GameType;
use schnapsen_model::player::PlayerId;
use schnapsen_model::state::{
    BiddingState, DealState, FinishedState, MatchState, Phase, PlayingState, Trick,
};

use crate::validator;

/// Trait for selecting moves.
pub trait MoveSelector {
    fn select_action(&mut self, deal: &DealState, player: PlayerId) -> Action;
}

/// Manages game flow: dealing, phase transitions, and trick resolution.
pub struct GameEngine {
    pub match_state: MatchState,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            match_state: MatchState::new(),
        }
    }

    /// Start a new deal with the current dealer.
    pub fn start_deal(&mut self, rng: &mut impl rand::Rng) {
        let dealer = self.match_state.current_dealer;
        let mut deal = DealState::new(dealer);

        let mut deck = Deck::new();
        deck.shuffle(rng);
        let deal_result = deck.deal();

        let caller = dealer.next();
        let third = caller.next();
        let player_order = [caller, third, dealer];

        for (i, &pid) in player_order.iter().enumerate() {
            deal.player_mut(pid)
                .hand
                .add_many(&deal_result.first_three[i]);
        }

        deal.talon = Some(deal_result.talon);

        for (i, &pid) in player_order.iter().enumerate() {
            deal.player_mut(pid)
                .hand
                .add_many(&deal_result.second_three[i]);
        }

        deal.phase = Phase::CallingTrump;
        self.match_state.current_deal = Some(deal);
    }

    /// Apply an action to the current deal state.
    pub fn apply_action(&mut self, player: PlayerId, action: Action) -> Result<(), String> {
        let deal = self
            .match_state
            .current_deal
            .as_mut()
            .ok_or("No active deal")?;

        match (&deal.phase, &action) {
            (Phase::CallingTrump, Action::CallTrump(suit)) => {
                if player != deal.caller {
                    return Err("Only the caller can call trump".into());
                }
                deal.trump = Some(*suit);
                deal.phase = Phase::Bidding(BiddingState {
                    current_bidder: deal.caller,
                    bids: Vec::new(),
                    passed: Vec::new(),
                });
                Ok(())
            }

            (Phase::Bidding(_), Action::AnnounceGame(announcement)) => {
                let game_type = announcement.game_type;
                let eligibility = game_type.eligibility();

                match eligibility {
                    schnapsen_model::game_type::Eligibility::CallerOnly => {
                        if player != deal.caller {
                            return Err("Only the caller can announce this game".into());
                        }
                    }
                    schnapsen_model::game_type::Eligibility::OpponentsOnly => {
                        if player == deal.caller {
                            return Err("The caller cannot announce this game".into());
                        }
                    }
                    schnapsen_model::game_type::Eligibility::Anyone => {}
                }

                deal.game_type = Some(game_type);
                deal.announcer = Some(player);

                if announcement.without_talon {
                    self.start_playing(player)?;
                } else {
                    deal.phase = Phase::TalonExchange;
                }
                Ok(())
            }

            (Phase::Bidding(_), Action::Pass) => {
                if let Phase::Bidding(ref mut bs) = deal.phase {
                    bs.passed.push(player);
                    bs.current_bidder = player.next();

                    if bs.passed.len() >= 2 {
                        let caller = deal.caller;
                        deal.announcer = Some(caller);
                        deal.game_type = Some(GameType::Normal);
                        deal.phase = Phase::TalonExchange;
                    }
                }
                Ok(())
            }

            (Phase::TalonExchange, Action::TakeTalon) => {
                let announcer = deal.announcer.ok_or("No announcer set")?;
                if player != announcer {
                    return Err("Only the announcer can take the talon".into());
                }
                if let Some(talon) = deal.talon.take() {
                    deal.player_mut(announcer).hand.add(talon[0]);
                    deal.player_mut(announcer).hand.add(talon[1]);
                }
                Ok(())
            }

            (Phase::TalonExchange, Action::DiscardToTalon(c1, c2)) => {
                let announcer = deal.announcer.ok_or("No announcer set")?;
                if player != announcer {
                    return Err("Only the announcer can discard".into());
                }
                let hand = &mut deal.player_mut(announcer).hand;
                hand.remove(c1).ok_or("Card not in hand")?;
                hand.remove(c2).ok_or("Card not in hand")?;
                deal.discarded = vec![*c1, *c2];

                self.start_playing(announcer)?;
                Ok(())
            }

            (Phase::Playing(_), Action::PlayCard(card)) => {
                self.play_card(player, *card)
            }

            (Phase::Playing(_), Action::AnnounceMarriage(suit)) => {
                if !validator::can_announce_marriage(deal, player, *suit) {
                    return Err("Cannot announce marriage".into());
                }
                let points = if Some(*suit) == deal.trump { 40 } else { 20 };
                let is_announcer = deal.is_announcer(player);

                if let Phase::Playing(ref mut ps) = deal.phase {
                    ps.marriages_announced.push((player, *suit, points));
                    ps.pending_marriage_play = Some(*suit);

                    if is_announcer {
                        ps.announcer_card_points += points;
                    } else {
                        ps.defender_card_points += points;
                    }
                }
                Ok(())
            }

            (Phase::Playing(_), Action::Spritzen) => {
                if !validator::can_spritzen(deal, player) {
                    return Err("Cannot spritzen".into());
                }
                if let Some(new_level) = deal.doubling.raise() {
                    deal.doubling = new_level;
                    deal.last_doubler = Some(player);
                }
                Ok(())
            }

            _ => Err(format!("Invalid action {:?} in current phase", action)),
        }
    }

    fn start_playing(&mut self, lead_player: PlayerId) -> Result<(), String> {
        let deal = self
            .match_state
            .current_deal
            .as_mut()
            .ok_or("No active deal")?;

        let actual_lead = if deal.game_type.map_or(false, |gt| gt.caller_leads()) {
            deal.caller
        } else {
            lead_player
        };

        deal.phase = Phase::Playing(PlayingState {
            current_trick: Trick::new(),
            tricks_completed: 0,
            lead_player: actual_lead,
            announcer_tricks: Vec::new(),
            defender_tricks: Vec::new(),
            announcer_card_points: 0,
            defender_card_points: 0,
            marriages_announced: Vec::new(),
            pending_marriage_play: None,
        });
        Ok(())
    }

    fn play_card(&mut self, player: PlayerId, card: schnapsen_model::Card) -> Result<(), String> {
        let deal = self
            .match_state
            .current_deal
            .as_mut()
            .ok_or("No active deal")?;

        let valid = validator::valid_cards(deal, player);
        if !valid.contains(&card) {
            return Err(format!("Card {} is not a valid play", card));
        }

        deal.player_mut(player)
            .hand
            .remove(&card)
            .ok_or("Card not in hand")?;

        if let Phase::Playing(ref mut ps) = deal.phase {
            ps.pending_marriage_play = None;
            ps.current_trick.play(player, card);

            if ps.current_trick.is_complete() {
                self.resolve_trick()?;
            }
        }

        Ok(())
    }

    fn resolve_trick(&mut self) -> Result<(), String> {
        let deal = self
            .match_state
            .current_deal
            .as_mut()
            .ok_or("No active deal")?;

        let (winner, trick_cards, trick_points, tricks_completed, is_announcer) = {
            let ps = match &deal.phase {
                Phase::Playing(ps) => ps,
                _ => return Err("Not in playing phase".into()),
            };

            let trump = deal.effective_trump();
            let ordering = deal.rank_ordering();
            let winner = ps
                .current_trick
                .winner(trump, &ordering)
                .ok_or("Cannot determine trick winner")?;

            let trick_cards: Vec<schnapsen_model::Card> =
                ps.current_trick.cards.iter().map(|(_, c)| *c).collect();
            let trick_points = ps.current_trick.card_points();
            let tricks_completed = ps.tricks_completed;
            let is_announcer = deal.is_announcer(winner);

            (winner, trick_cards, trick_points, tricks_completed, is_announcer)
        };

        if is_announcer {
            if deal.first_trick_announcer.is_none() {
                deal.first_trick_announcer = Some(trick_cards.clone());
            }
        } else if deal.first_trick_defenders.is_none() {
            deal.first_trick_defenders = Some(trick_cards.clone());
        }

        let should_finish;
        if let Phase::Playing(ref mut ps) = deal.phase {
            if is_announcer {
                ps.announcer_card_points += trick_points;
                ps.announcer_tricks.push(trick_cards);
            } else {
                ps.defender_card_points += trick_points;
                ps.defender_tricks.push(trick_cards);
            }

            ps.tricks_completed = tricks_completed + 1;
            should_finish = ps.tricks_completed >= 5;

            if !should_finish {
                ps.current_trick = Trick::new();
                ps.lead_player = winner;
            }
        } else {
            return Err("Not in playing phase".into());
        }

        if should_finish || self.check_early_win_from_match() {
            self.finish_deal()?;
        }

        Ok(())
    }

    fn check_early_win_from_match(&self) -> bool {
        let deal = match &self.match_state.current_deal {
            Some(d) => d,
            None => return false,
        };

        let ps = match &deal.phase {
            Phase::Playing(ps) => ps,
            _ => return false,
        };

        match deal.game_type {
            Some(GameType::Normal) | Some(GameType::Schnapser) | Some(GameType::Kontraschnapser) => {
                ps.announcer_card_points >= 66 || ps.defender_card_points >= 66
            }
            _ => false,
        }
    }

    fn finish_deal(&mut self) -> Result<(), String> {
        let deal = self
            .match_state
            .current_deal
            .as_mut()
            .ok_or("No active deal")?;

        let ps = match &deal.phase {
            Phase::Playing(ps) => ps,
            _ => return Err("Not in playing phase".into()),
        };

        let announcer = deal.announcer.ok_or("No announcer")?;
        let game_type = deal.game_type.ok_or("No game type")?;

        let announcer_wins = match game_type.goal() {
            schnapsen_model::game_type::GameGoal::Reach66
            | schnapsen_model::game_type::GameGoal::SchnapserReach66 => {
                ps.announcer_card_points >= 66
            }
            schnapsen_model::game_type::GameGoal::WinNoTricks => {
                ps.announcer_tricks.is_empty()
            }
            schnapsen_model::game_type::GameGoal::WinAllTricks
            | schnapsen_model::game_type::GameGoal::WinAllTricksOneSuit
            | schnapsen_model::game_type::GameGoal::WinAllTricksTrump => {
                ps.defender_tricks.is_empty()
            }
        };

        let points = if game_type == GameType::Normal {
            let outcome = schnapsen_model::action::NormalGameOutcome::from_defender_points(
                if announcer_wins {
                    ps.defender_card_points
                } else {
                    ps.announcer_card_points
                },
                if announcer_wins {
                    !ps.defender_tricks.is_empty()
                } else {
                    !ps.announcer_tricks.is_empty()
                },
            );
            outcome.points() * deal.doubling.multiplier()
        } else {
            deal.final_points()
        };

        let winner = if announcer_wins {
            schnapsen_model::action::Winner::Announcer(announcer)
        } else {
            let defenders: Vec<PlayerId> = PlayerId::all()
                .iter()
                .filter(|&&p| p != announcer)
                .copied()
                .collect();
            schnapsen_model::action::Winner::Defenders(defenders[0], defenders[1])
        };

        match &winner {
            schnapsen_model::action::Winner::Announcer(pid) => {
                self.match_state.scores[pid.index()] += points;
            }
            schnapsen_model::action::Winner::Defenders(p1, p2) => {
                self.match_state.scores[p1.index()] += points;
                self.match_state.scores[p2.index()] += points;
            }
        }

        deal.phase = Phase::Finished(FinishedState { winner, points });

        Ok(())
    }

    /// Run a complete deal with the given move selectors for each player.
    pub fn play_deal(
        &mut self,
        rng: &mut impl rand::Rng,
        selectors: &mut [&mut dyn MoveSelector; 3],
    ) -> Result<(), String> {
        self.start_deal(rng);

        loop {
            let deal = self
                .match_state
                .current_deal
                .as_ref()
                .ok_or("No active deal")?;

            if matches!(deal.phase, Phase::Finished(_)) {
                break;
            }

            let current_player = self.current_player()?;
            let deal_ref = self.match_state.current_deal.as_ref().unwrap();
            let action = selectors[current_player.index()].select_action(deal_ref, current_player);
            self.apply_action(current_player, action)?;
        }

        self.match_state.advance_dealer();
        Ok(())
    }

    /// Determine who should act next based on the current phase.
    pub fn current_player(&self) -> Result<PlayerId, String> {
        let deal = self
            .match_state
            .current_deal
            .as_ref()
            .ok_or("No active deal")?;

        match &deal.phase {
            Phase::CallingTrump => Ok(deal.caller),
            Phase::Bidding(bs) => Ok(bs.current_bidder),
            Phase::TalonExchange => deal.announcer.ok_or("No announcer".into()),
            Phase::Playing(ps) => {
                let trick = &ps.current_trick;
                if trick.is_empty() {
                    Ok(ps.lead_player)
                } else {
                    let last_player = trick.cards.last().map(|(p, _)| *p).unwrap();
                    Ok(last_player.next())
                }
            }
            Phase::Finished(_) => Err("Deal is finished".into()),
        }
    }
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use schnapsen_model::card::Suit;

    #[test]
    fn engine_starts_deal() {
        let mut engine = GameEngine::new();
        let mut rng = rand::thread_rng();
        engine.start_deal(&mut rng);

        let deal = engine.match_state.current_deal.as_ref().unwrap();
        assert!(matches!(deal.phase, Phase::CallingTrump));

        for pid in PlayerId::all() {
            assert_eq!(deal.player(pid).hand.len(), 6);
        }
    }

    #[test]
    fn call_trump_transitions_to_bidding() {
        let mut engine = GameEngine::new();
        let mut rng = rand::thread_rng();
        engine.start_deal(&mut rng);

        let caller = engine.match_state.current_deal.as_ref().unwrap().caller;
        engine
            .apply_action(caller, Action::CallTrump(Suit::Hearts))
            .unwrap();

        let deal = engine.match_state.current_deal.as_ref().unwrap();
        assert!(matches!(deal.phase, Phase::Bidding(_)));
        assert_eq!(deal.trump, Some(Suit::Hearts));
    }

    #[test]
    fn all_pass_leads_to_normal_game() {
        let mut engine = GameEngine::new();
        let mut rng = rand::thread_rng();
        engine.start_deal(&mut rng);

        let deal = engine.match_state.current_deal.as_ref().unwrap();
        let caller = deal.caller;
        let p2 = caller.next();
        let _p3 = p2.next();

        engine
            .apply_action(caller, Action::CallTrump(Suit::Hearts))
            .unwrap();
        engine.apply_action(caller, Action::Pass).unwrap();
        engine.apply_action(p2, Action::Pass).unwrap();

        let deal = engine.match_state.current_deal.as_ref().unwrap();
        assert_eq!(deal.game_type, Some(GameType::Normal));
        assert!(matches!(deal.phase, Phase::TalonExchange));
    }

    #[test]
    fn current_player_follows_phase() {
        let mut engine = GameEngine::new();
        let mut rng = rand::thread_rng();
        engine.start_deal(&mut rng);

        let caller = engine.match_state.current_deal.as_ref().unwrap().caller;
        assert_eq!(engine.current_player().unwrap(), caller);
    }
}

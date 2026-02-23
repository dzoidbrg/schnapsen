use crate::card::{Card, Rank, Suit};
use crate::deck::Deck;
use crate::types::*;

/// The current phase of a hand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phase {
    /// Waiting for the Rufer to choose a trump suit from their first 3 cards.
    CallingTrump,
    /// Players are announcing game types (bidding).
    Bidding {
        bids: Vec<(PlayerId, GameType)>,
        passed: Vec<PlayerId>,
        current_bidder: PlayerId,
    },
    /// The winning bidder decides whether to take the talon.
    TalonDecision {
        declarer: PlayerId,
        game_type: GameType,
    },
    /// The declarer must discard 2 cards after picking up the talon.
    Discarding {
        declarer: PlayerId,
        game_type: GameType,
    },
    /// Active trick-taking play.
    Playing {
        declarer: PlayerId,
        game_type: GameType,
        spritzen: SpritzenLevel,
    },
    /// The hand is finished.
    Finished {
        declarer: PlayerId,
        game_type: GameType,
        declarer_won: bool,
        points_awarded: u8,
    },
}

/// Complete state of a single hand (one deal).
#[derive(Debug, Clone)]
pub struct GameState {
    pub phase: Phase,
    pub hands: [Vec<Card>; 3],
    pub trump: Option<Suit>,
    pub talon: Vec<Card>,

    /// Index of the Rufer for this hand.
    pub rufer: PlayerId,
    /// Index of the Geber for this hand.
    pub geber: PlayerId,

    /// Cards in the current (incomplete) trick.
    pub current_trick: Vec<(PlayerId, Card)>,
    /// Who leads the next/current trick.
    pub trick_leader: PlayerId,
    /// Completed tricks.
    pub tricks: Vec<Trick>,
    /// Card points won by each player.
    pub card_points: [u8; 3],
    /// Number of tricks won by each player.
    pub tricks_won: [u8; 3],
    /// Marriages announced during this hand.
    pub marriages: Vec<(PlayerId, Marriage)>,
}

impl GameState {
    /// Deal a new hand. `geber` is the dealer index (0, 1, or 2).
    pub fn deal(geber: PlayerId, rng: &mut impl rand::Rng) -> Self {
        let rufer = (geber + 1) % 3;
        let third = (geber + 2) % 3;

        let mut deck = Deck::new();
        deck.shuffle(rng);

        let mut hands: [Vec<Card>; 3] = [Vec::new(), Vec::new(), Vec::new()];

        // First round: 3 cards each, starting with Rufer
        for &p in &[rufer, third, geber] {
            hands[p] = deck.draw_n(3);
        }

        let talon = deck.draw_n(2);

        // Second round: 3 more cards each
        for &p in &[rufer, third, geber] {
            hands[p].extend(deck.draw_n(3));
        }

        assert!(deck.is_empty(), "all 20 cards should be dealt");

        Self {
            phase: Phase::CallingTrump,
            hands,
            trump: None,
            talon,
            rufer,
            geber,
            current_trick: Vec::new(),
            trick_leader: rufer,
            tricks: Vec::new(),
            card_points: [0; 3],
            tricks_won: [0; 3],
            marriages: Vec::new(),
        }
    }

    /// The player whose turn it is to act.
    pub fn active_player(&self) -> Option<PlayerId> {
        match &self.phase {
            Phase::CallingTrump => Some(self.rufer),
            Phase::Bidding { current_bidder, .. } => Some(*current_bidder),
            Phase::TalonDecision { declarer, .. } => Some(*declarer),
            Phase::Discarding { declarer, .. } => Some(*declarer),
            Phase::Playing { .. } => {
                if self.current_trick.len() >= 3 {
                    None
                } else {
                    let offset = self.current_trick.len();
                    Some((self.trick_leader + offset) % 3)
                }
            }
            Phase::Finished { .. } => None,
        }
    }

    /// Apply an action to advance the game state. Returns `Err` for invalid actions.
    pub fn apply_action(&mut self, player: PlayerId, action: Action) -> Result<(), String> {
        let active = self
            .active_player()
            .ok_or("no active player in current phase")?;
        if player != active {
            return Err(format!("not player {player}'s turn, expected {active}"));
        }

        match (&self.phase, action) {
            (Phase::CallingTrump, Action::CallTrump(suit)) => {
                self.trump = Some(suit);
                let rufer = self.rufer;
                self.phase = Phase::Bidding {
                    bids: Vec::new(),
                    passed: Vec::new(),
                    current_bidder: rufer,
                };
                Ok(())
            }

            (
                Phase::Bidding {
                    bids,
                    passed,
                    current_bidder,
                },
                Action::Bid(game_type),
            ) => {
                let mut bids = bids.clone();
                let passed = passed.clone();
                let bidder = *current_bidder;

                if game_type.rufer_only() && bidder != self.rufer {
                    return Err(format!("{} can only be announced by the Rufer", game_type));
                }
                if game_type.non_rufer_only() && bidder == self.rufer {
                    return Err(format!(
                        "{} can only be announced by a non-Rufer",
                        game_type
                    ));
                }

                bids.push((bidder, game_type));
                self.advance_bidding(bids, passed, bidder);
                Ok(())
            }

            (
                Phase::Bidding {
                    bids,
                    passed,
                    current_bidder,
                },
                Action::Pass,
            ) => {
                let bids = bids.clone();
                let mut passed = passed.clone();
                let bidder = *current_bidder;
                passed.push(bidder);
                self.advance_bidding(bids, passed, bidder);
                Ok(())
            }

            (
                Phase::TalonDecision {
                    declarer,
                    game_type,
                },
                Action::TakeTalon { take },
            ) => {
                let declarer = *declarer;
                let game_type = *game_type;
                if take {
                    self.hands[declarer].extend(self.talon.drain(..));
                    self.phase = Phase::Discarding {
                        declarer,
                        game_type,
                    };
                } else {
                    self.phase = Phase::Playing {
                        declarer,
                        game_type,
                        spritzen: SpritzenLevel::None,
                    };
                    self.trick_leader = self.determine_leader(declarer, game_type);
                }
                Ok(())
            }

            (
                Phase::Discarding {
                    declarer,
                    game_type,
                },
                Action::Discard(c1, c2),
            ) => {
                let declarer = *declarer;
                let game_type = *game_type;
                let hand = &mut self.hands[declarer];

                let pos1 = hand
                    .iter()
                    .position(|c| *c == c1)
                    .ok_or("first discard card not in hand")?;
                hand.remove(pos1);
                let pos2 = hand
                    .iter()
                    .position(|c| *c == c2)
                    .ok_or("second discard card not in hand")?;
                hand.remove(pos2);

                self.talon = vec![c1, c2]; // store discarded cards (face-down)

                self.phase = Phase::Playing {
                    declarer,
                    game_type,
                    spritzen: SpritzenLevel::None,
                };
                self.trick_leader = self.determine_leader(declarer, game_type);
                Ok(())
            }

            (
                Phase::Playing {
                    game_type,
                    ..
                },
                Action::PlayCard(card),
            ) => {
                let game_type = *game_type;
                self.play_card(player, card, game_type)
            }

            (
                Phase::Playing {
                    game_type,
                    ..
                },
                Action::AnnounceMarriage(marriage, card),
            ) => {
                let game_type = *game_type;
                if player != self.trick_leader {
                    return Err("can only announce marriage when leading".into());
                }
                let hand = &self.hands[player];
                let has_king = hand.iter().any(|c| c.suit == marriage.suit && c.rank == Rank::King);
                let has_queen = hand
                    .iter()
                    .any(|c| c.suit == marriage.suit && c.rank == Rank::Queen);
                if !has_king || !has_queen {
                    return Err("missing king or queen for marriage".into());
                }
                if card.suit != marriage.suit
                    || (card.rank != Rank::King && card.rank != Rank::Queen)
                {
                    return Err("must play king or queen of the marriage suit".into());
                }

                self.marriages.push((player, marriage));
                self.play_card(player, card, game_type)
            }

            _ => Err("invalid action for current phase".into()),
        }
    }

    fn advance_bidding(
        &mut self,
        bids: Vec<(PlayerId, GameType)>,
        passed: Vec<PlayerId>,
        last_bidder: PlayerId,
    ) {
        let next = (last_bidder + 1) % 3;

        if passed.len() == 3 {
            let rufer = self.rufer;
            self.phase = Phase::TalonDecision {
                declarer: rufer,
                game_type: GameType::Normal,
            };
            return;
        }

        if passed.len() == 2 && bids.len() >= 1 {
            let (declarer, game_type) = *bids.last().unwrap();
            self.phase = Phase::TalonDecision {
                declarer,
                game_type,
            };
            return;
        }

        let all_acted = bids.len() + passed.len() == 3;
        if all_acted {
            let (declarer, game_type) = bids
                .iter()
                .max_by_key(|(_, gt)| gt.base_points())
                .copied()
                .unwrap();
            self.phase = Phase::TalonDecision {
                declarer,
                game_type,
            };
            return;
        }

        if passed.contains(&next) || bids.iter().any(|(p, _)| *p == next) {
            let third = (next + 1) % 3;
            self.phase = Phase::Bidding {
                bids,
                passed,
                current_bidder: third,
            };
        } else {
            self.phase = Phase::Bidding {
                bids,
                passed,
                current_bidder: next,
            };
        }
    }

    fn determine_leader(&self, declarer: PlayerId, game_type: GameType) -> PlayerId {
        match game_type {
            GameType::Kontraschnapser | GameType::Kontrabauernschnapser => self.rufer,
            _ => declarer,
        }
    }

    fn play_card(
        &mut self,
        player: PlayerId,
        card: Card,
        game_type: GameType,
    ) -> Result<(), String> {
        let hand = &self.hands[player];
        if !hand.contains(&card) {
            return Err("card not in hand".into());
        }

        let trump = if game_type.has_trump() {
            self.trump
        } else {
            None
        };
        let ordering = game_type.rank_ordering();

        let lead = if self.current_trick.is_empty() {
            None
        } else {
            let lead_card = self.current_trick[0].1;
            Some((lead_card.suit, lead_card.rank))
        };

        let valid = valid_plays(hand, lead, trump, ordering);
        if !valid.contains(&card) {
            return Err(format!("{card} is not a valid play"));
        }

        let pos = self.hands[player].iter().position(|c| *c == card).unwrap();
        self.hands[player].remove(pos);
        self.current_trick.push((player, card));

        if self.current_trick.len() == 3 {
            self.resolve_trick(game_type);
        }

        Ok(())
    }

    fn resolve_trick(&mut self, game_type: GameType) {
        let trump = if game_type.has_trump() {
            self.trump
        } else {
            None
        };
        let ordering = game_type.rank_ordering();
        let winner = trick_winner(&self.current_trick, trump, ordering);

        let trick = Trick {
            cards: self.current_trick.clone(),
            winner,
        };
        let pts = trick.points();
        self.card_points[winner] = self.card_points[winner].saturating_add(pts);
        self.tricks_won[winner] += 1;
        self.tricks.push(trick);
        self.current_trick.clear();
        self.trick_leader = winner;

        self.check_hand_end(game_type);
    }

    fn check_hand_end(&mut self, game_type: GameType) {
        let Phase::Playing {
            declarer, spritzen, ..
        } = self.phase
        else {
            return;
        };

        let all_played = self.hands.iter().all(|h| h.is_empty());

        let finished = match game_type.goal() {
            GameGoal::TakeAllTricks => {
                let declarer_lost_trick = self
                    .tricks
                    .iter()
                    .any(|t| t.winner != declarer);
                if declarer_lost_trick {
                    Some(false)
                } else if all_played {
                    Some(true)
                } else {
                    None
                }
            }
            GameGoal::TakeNoTricks => {
                if self.tricks_won[declarer] > 0 {
                    Some(false)
                } else if all_played {
                    Some(true)
                } else {
                    None
                }
            }
            GameGoal::Reach66 => {
                let marriage_points: u8 = self
                    .marriages
                    .iter()
                    .filter(|(p, _)| *p == declarer)
                    .filter(|_| self.tricks_won[declarer] > 0) // marriages only count with at least 1 trick
                    .map(|(_, m)| m.points())
                    .sum();
                let total = self.card_points[declarer] + marriage_points;
                if total >= 66 {
                    Some(true)
                } else if all_played {
                    Some(false)
                } else {
                    None
                }
            }
        };

        if let Some(declarer_won) = finished {
            let base = if game_type == GameType::Normal && declarer_won {
                let opponents: Vec<PlayerId> = (0..3).filter(|&p| p != declarer).collect();
                let opp_points: u8 = opponents.iter().map(|&p| self.card_points[p]).sum();
                let opp_has_trick = opponents.iter().any(|&p| self.tricks_won[p] > 0);
                NormalGameResult::from_loser_points(opp_points, opp_has_trick).points()
            } else if game_type == GameType::Normal && !declarer_won {
                NormalGameResult::from_loser_points(
                    self.card_points[declarer],
                    self.tricks_won[declarer] > 0,
                )
                .points()
            } else {
                game_type.base_points()
            };

            let points_awarded = base * spritzen.multiplier();

            self.phase = Phase::Finished {
                declarer,
                game_type,
                declarer_won,
                points_awarded,
            };
        }
    }

    /// Compute total card+marriage points for a player.
    pub fn total_points_for(&self, player: PlayerId) -> u8 {
        let marriage_pts: u8 = self
            .marriages
            .iter()
            .filter(|(p, _)| *p == player && self.tricks_won[player] > 0)
            .map(|(_, m)| m.points())
            .sum();
        self.card_points[player] + marriage_pts
    }

    /// Returns the list of valid actions for the active player.
    pub fn valid_actions(&self) -> Vec<Action> {
        let Some(player) = self.active_player() else {
            return Vec::new();
        };

        match &self.phase {
            Phase::CallingTrump => {
                let suits: Vec<Suit> = self.hands[player]
                    .iter()
                    .map(|c| c.suit)
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();
                suits.into_iter().map(Action::CallTrump).collect()
            }
            Phase::Bidding { .. } => {
                let mut actions = vec![Action::Pass];
                for gt in all_game_types() {
                    if gt.rufer_only() && player != self.rufer {
                        continue;
                    }
                    if gt.non_rufer_only() && player == self.rufer {
                        continue;
                    }
                    actions.push(Action::Bid(gt));
                }
                actions
            }
            Phase::TalonDecision { .. } => {
                vec![
                    Action::TakeTalon { take: true },
                    Action::TakeTalon { take: false },
                ]
            }
            Phase::Discarding { declarer, .. } => {
                let hand = &self.hands[*declarer];
                let mut discards = Vec::new();
                for i in 0..hand.len() {
                    for j in (i + 1)..hand.len() {
                        discards.push(Action::Discard(hand[i], hand[j]));
                    }
                }
                discards
            }
            Phase::Playing { game_type, .. } => {
                let trump = if game_type.has_trump() {
                    self.trump
                } else {
                    None
                };
                let ordering = game_type.rank_ordering();
                let lead = if self.current_trick.is_empty() {
                    None
                } else {
                    let lc = self.current_trick[0].1;
                    Some((lc.suit, lc.rank))
                };

                let valid = valid_plays(&self.hands[player], lead, trump, ordering);

                let is_leader = player == self.trick_leader && self.current_trick.is_empty();
                let marriages = if is_leader {
                    detect_marriages(&self.hands[player], trump)
                } else {
                    Vec::new()
                };

                let mut actions: Vec<Action> = Vec::new();

                for m in &marriages {
                    for &card in &valid {
                        if card.suit == m.suit
                            && (card.rank == Rank::King || card.rank == Rank::Queen)
                        {
                            actions.push(Action::AnnounceMarriage(*m, card));
                        }
                    }
                }

                for card in valid {
                    actions.push(Action::PlayCard(card));
                }

                actions
            }
            Phase::Finished { .. } => Vec::new(),
        }
    }
}

fn all_game_types() -> Vec<GameType> {
    vec![
        GameType::Normal,
        GameType::Bettler,
        GameType::Assenbettler,
        GameType::AssBettler,
        GameType::Schnapser,
        GameType::Plauderer,
        GameType::Damengang,
        GameType::Koenigsgang,
        GameType::Gang,
        GameType::Zehnergang,
        GameType::Bauernloch,
        GameType::Bauernschnapser,
        GameType::Kontraschnapser,
        GameType::Farbringerl,
        GameType::Kontrabauernschnapser,
        GameType::Herrenschnapser,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_gives_6_cards_each_and_2_talon() {
        let mut rng = rand::thread_rng();
        let state = GameState::deal(0, &mut rng);
        for hand in &state.hands {
            assert_eq!(hand.len(), 6);
        }
        assert_eq!(state.talon.len(), 2);
    }

    #[test]
    fn deal_uses_all_20_cards() {
        let mut rng = rand::thread_rng();
        let state = GameState::deal(0, &mut rng);
        let mut all: Vec<Card> = Vec::new();
        for hand in &state.hands {
            all.extend(hand);
        }
        all.extend(&state.talon);
        assert_eq!(all.len(), 20);

        let set: std::collections::HashSet<Card> = all.into_iter().collect();
        assert_eq!(set.len(), 20);
    }

    #[test]
    fn initial_phase_is_calling_trump() {
        let mut rng = rand::thread_rng();
        let state = GameState::deal(0, &mut rng);
        assert_eq!(state.phase, Phase::CallingTrump);
        assert_eq!(state.active_player(), Some(state.rufer));
    }

    #[test]
    fn calling_trump_advances_to_bidding() {
        let mut rng = rand::thread_rng();
        let mut state = GameState::deal(0, &mut rng);
        let rufer = state.rufer;
        state
            .apply_action(rufer, Action::CallTrump(Suit::Hearts))
            .unwrap();
        assert!(matches!(state.phase, Phase::Bidding { .. }));
        assert_eq!(state.trump, Some(Suit::Hearts));
    }

    #[test]
    fn all_pass_leads_to_normal_game() {
        let mut rng = rand::thread_rng();
        let mut state = GameState::deal(0, &mut rng);
        let rufer = state.rufer;
        state
            .apply_action(rufer, Action::CallTrump(Suit::Hearts))
            .unwrap();

        for _ in 0..3 {
            let p = state.active_player().unwrap();
            state.apply_action(p, Action::Pass).unwrap();
        }

        match &state.phase {
            Phase::TalonDecision {
                declarer,
                game_type,
            } => {
                assert_eq!(*declarer, rufer);
                assert_eq!(*game_type, GameType::Normal);
            }
            other => panic!("expected TalonDecision, got {other:?}"),
        }
    }

    #[test]
    fn valid_actions_always_non_empty_before_finish() {
        let mut rng = rand::thread_rng();
        let state = GameState::deal(0, &mut rng);
        let actions = state.valid_actions();
        assert!(!actions.is_empty());
    }
}

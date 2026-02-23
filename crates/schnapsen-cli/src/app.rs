use rand::rngs::StdRng;
use rand::SeedableRng;

use schnapsen_engine::engine::GameEngine;
use schnapsen_engine::validator;
use schnapsen_model::action::{Action, GameAnnouncement};
use schnapsen_model::card::{Card, Suit};
use schnapsen_model::game_type::GameType;
use schnapsen_model::player::PlayerId;
use schnapsen_model::state::Phase;

/// Which UI screen to show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    TrumpSelection,
    Bidding,
    TalonExchange,
    Playing,
    TrickResult,
    DealResult,
    MatchResult,
}

/// Status messages to show the player.
#[derive(Debug, Clone)]
pub struct StatusMessage {
    pub text: String,
    pub is_error: bool,
}

pub struct App {
    pub engine: GameEngine,
    pub rng: StdRng,
    pub human_player: PlayerId,
    pub selected_card_index: usize,
    pub selected_trump_index: usize,
    pub selected_bid_index: usize,
    pub screen: Screen,
    pub status: Option<StatusMessage>,
    pub should_quit: bool,
    pub last_trick_cards: Vec<(PlayerId, Card)>,
    pub last_trick_winner: Option<PlayerId>,
    pub deal_message: Option<String>,
    pub talon_taken: bool,
    pub discard_selected: Vec<usize>,
}

impl App {
    pub fn new() -> Self {
        let rng = StdRng::from_entropy();
        Self {
            engine: GameEngine::new(),
            rng,
            human_player: PlayerId::Player0,
            selected_card_index: 0,
            selected_trump_index: 0,
            selected_bid_index: 0,
            screen: Screen::TrumpSelection,
            status: None,
            should_quit: false,
            last_trick_cards: Vec::new(),
            last_trick_winner: None,
            deal_message: None,
            talon_taken: false,
            discard_selected: Vec::new(),
        }
    }

    pub fn start_new_deal(&mut self) {
        self.engine.start_deal(&mut self.rng);
        self.selected_card_index = 0;
        self.selected_trump_index = 0;
        self.selected_bid_index = 0;
        self.status = None;
        self.last_trick_cards.clear();
        self.last_trick_winner = None;
        self.deal_message = None;
        self.talon_taken = false;
        self.discard_selected.clear();

        let deal = self.engine.match_state.current_deal.as_ref().unwrap();
        if deal.caller == self.human_player {
            self.screen = Screen::TrumpSelection;
        } else {
            self.ai_call_trump();
            self.ai_bidding_until_human();
        }
    }

    fn ai_call_trump(&mut self) {
        let deal = self.engine.match_state.current_deal.as_ref().unwrap();
        let caller = deal.caller;
        let hand = &deal.player(caller).hand;
        let suit = hand.cards()[0].suit;
        self.engine
            .apply_action(caller, Action::CallTrump(suit))
            .ok();
    }

    fn ai_bidding_until_human(&mut self) {
        loop {
            let deal = self.engine.match_state.current_deal.as_ref().unwrap();
            match &deal.phase {
                Phase::Bidding(bs) => {
                    if bs.current_bidder == self.human_player {
                        self.screen = Screen::Bidding;
                        return;
                    }
                    let bidder = bs.current_bidder;
                    self.engine.apply_action(bidder, Action::Pass).ok();
                }
                Phase::TalonExchange => {
                    let announcer = deal.announcer.unwrap();
                    if announcer == self.human_player {
                        self.screen = Screen::TalonExchange;
                        return;
                    }
                    self.ai_talon_exchange();
                    self.screen = Screen::Playing;
                    self.advance_ai_plays();
                    return;
                }
                _ => {
                    self.screen = Screen::Playing;
                    self.advance_ai_plays();
                    return;
                }
            }
        }
    }

    fn ai_talon_exchange(&mut self) {
        let deal = self.engine.match_state.current_deal.as_ref().unwrap();
        let announcer = deal.announcer.unwrap();
        self.engine
            .apply_action(announcer, Action::TakeTalon)
            .ok();

        let deal = self.engine.match_state.current_deal.as_ref().unwrap();
        let hand = deal.player(announcer).hand.cards().to_vec();
        if hand.len() >= 2 {
            let c1 = hand[0];
            let c2 = hand[1];
            self.engine
                .apply_action(announcer, Action::DiscardToTalon(c1, c2))
                .ok();
        }
    }

    pub fn select_trump(&mut self) {
        let suits = Suit::all();
        let suit = suits[self.selected_trump_index % 4];
        let caller = self.engine.match_state.current_deal.as_ref().unwrap().caller;

        match self.engine.apply_action(caller, Action::CallTrump(suit)) {
            Ok(()) => {
                self.status = Some(StatusMessage {
                    text: format!("Trump: {}", suit.german_name()),
                    is_error: false,
                });
                self.screen = Screen::Bidding;
            }
            Err(e) => {
                self.status = Some(StatusMessage {
                    text: e,
                    is_error: true,
                });
            }
        }
    }

    pub fn bid_pass(&mut self) {
        let result = self
            .engine
            .apply_action(self.human_player, Action::Pass);

        match result {
            Ok(()) => {
                self.ai_bidding_until_human();
                self.check_talon_or_play();
            }
            Err(e) => {
                self.status = Some(StatusMessage {
                    text: e,
                    is_error: true,
                });
            }
        }
    }

    pub fn bid_game(&mut self, game_type: GameType) {
        let announcement = GameAnnouncement::new(game_type, false);
        let result =
            self.engine
                .apply_action(self.human_player, Action::AnnounceGame(announcement));

        match result {
            Ok(()) => {
                self.check_talon_or_play();
            }
            Err(e) => {
                self.status = Some(StatusMessage {
                    text: e,
                    is_error: true,
                });
            }
        }
    }

    fn check_talon_or_play(&mut self) {
        let deal = self.engine.match_state.current_deal.as_ref().unwrap();
        match &deal.phase {
            Phase::TalonExchange => {
                let announcer = deal.announcer.unwrap();
                if announcer == self.human_player {
                    self.screen = Screen::TalonExchange;
                } else {
                    self.ai_talon_exchange();
                    self.screen = Screen::Playing;
                    self.advance_ai_plays();
                }
            }
            Phase::Playing(_) => {
                self.screen = Screen::Playing;
                self.advance_ai_plays();
            }
            Phase::Finished(_) => {
                self.show_deal_result();
            }
            _ => {}
        }
    }

    pub fn take_talon(&mut self) {
        if self.talon_taken {
            return;
        }
        let result = self
            .engine
            .apply_action(self.human_player, Action::TakeTalon);
        if result.is_ok() {
            self.talon_taken = true;
            self.status = Some(StatusMessage {
                text: "Talon aufgenommen! Wähle 2 Karten zum Ablegen.".into(),
                is_error: false,
            });
        }
    }

    pub fn toggle_discard(&mut self, index: usize) {
        if let Some(pos) = self.discard_selected.iter().position(|&i| i == index) {
            self.discard_selected.remove(pos);
        } else if self.discard_selected.len() < 2 {
            self.discard_selected.push(index);
        }
    }

    pub fn confirm_discard(&mut self) {
        if self.discard_selected.len() != 2 {
            self.status = Some(StatusMessage {
                text: "Wähle genau 2 Karten zum Ablegen.".into(),
                is_error: true,
            });
            return;
        }

        let deal = self.engine.match_state.current_deal.as_ref().unwrap();
        let hand = deal.player(self.human_player).hand.cards().to_vec();
        let c1 = hand[self.discard_selected[0]];
        let c2 = hand[self.discard_selected[1]];

        match self
            .engine
            .apply_action(self.human_player, Action::DiscardToTalon(c1, c2))
        {
            Ok(()) => {
                self.discard_selected.clear();
                self.screen = Screen::Playing;
                self.advance_ai_plays();
            }
            Err(e) => {
                self.status = Some(StatusMessage {
                    text: e,
                    is_error: true,
                });
            }
        }
    }

    pub fn play_selected_card(&mut self) {
        let deal = self.engine.match_state.current_deal.as_ref().unwrap();
        let valid = validator::valid_cards(deal, self.human_player);

        if valid.is_empty() {
            self.status = Some(StatusMessage {
                text: "Keine gültigen Karten!".into(),
                is_error: true,
            });
            return;
        }

        let idx = self.selected_card_index % valid.len();
        let card = valid[idx];

        match self
            .engine
            .apply_action(self.human_player, Action::PlayCard(card))
        {
            Ok(()) => {
                self.status = None;
                self.after_play();
            }
            Err(e) => {
                self.status = Some(StatusMessage {
                    text: e,
                    is_error: true,
                });
            }
        }
    }

    fn after_play(&mut self) {
        let deal = self.engine.match_state.current_deal.as_ref().unwrap();
        match &deal.phase {
            Phase::Playing(ps) => {
                if ps.current_trick.is_complete() {
                    // Trick was just completed - shouldn't happen as engine auto-resolves
                    self.advance_ai_plays();
                } else if ps.current_trick.is_empty() {
                    // New trick started (previous was resolved)
                    self.advance_ai_plays();
                } else {
                    self.advance_ai_plays();
                }
            }
            Phase::Finished(_) => {
                self.show_deal_result();
            }
            _ => {}
        }
    }

    fn advance_ai_plays(&mut self) {
        loop {
            let deal = self.engine.match_state.current_deal.as_ref().unwrap();

            match &deal.phase {
                Phase::Finished(_) => {
                    self.show_deal_result();
                    return;
                }
                Phase::Playing(ps) => {
                    let current = if ps.current_trick.is_empty() {
                        ps.lead_player
                    } else {
                        let last = ps.current_trick.cards.last().map(|(p, _)| *p).unwrap();
                        last.next()
                    };

                    if current == self.human_player {
                        self.screen = Screen::Playing;
                        self.selected_card_index = 0;
                        return;
                    }

                    let valid = validator::valid_cards(deal, current);
                    if valid.is_empty() {
                        return;
                    }

                    let card_idx = rand::random::<usize>() % valid.len();
                    let card = valid[card_idx];
                    self.engine
                        .apply_action(current, Action::PlayCard(card))
                        .ok();
                }
                _ => return,
            }
        }
    }

    fn show_deal_result(&mut self) {
        let deal = self.engine.match_state.current_deal.as_ref().unwrap();
        if let Phase::Finished(ref fs) = deal.phase {
            let winner_text = match &fs.winner {
                schnapsen_model::action::Winner::Announcer(pid) => {
                    format!("{} gewinnt", pid)
                }
                schnapsen_model::action::Winner::Defenders(p1, p2) => {
                    format!("{} & {} gewinnen", p1, p2)
                }
            };
            self.deal_message = Some(format!(
                "{} ({} Punkte)",
                winner_text, fs.points
            ));
            self.screen = Screen::DealResult;
        }
    }

    pub fn next_deal(&mut self) {
        if self.engine.match_state.winner().is_some() {
            self.screen = Screen::MatchResult;
        } else {
            self.start_new_deal();
        }
    }

    pub fn new_match(&mut self) {
        self.engine = GameEngine::new();
        self.start_new_deal();
    }

    pub fn human_hand(&self) -> Vec<Card> {
        let deal = match &self.engine.match_state.current_deal {
            Some(d) => d,
            None => return Vec::new(),
        };
        deal.player(self.human_player).hand.cards().to_vec()
    }

    pub fn valid_plays(&self) -> Vec<Card> {
        let deal = match &self.engine.match_state.current_deal {
            Some(d) => d,
            None => return Vec::new(),
        };
        validator::valid_cards(deal, self.human_player)
    }

    pub fn current_trick_cards(&self) -> Vec<(PlayerId, Card)> {
        let deal = match &self.engine.match_state.current_deal {
            Some(d) => d,
            None => return Vec::new(),
        };
        match &deal.phase {
            Phase::Playing(ps) => ps.current_trick.cards.clone(),
            _ => Vec::new(),
        }
    }

    pub fn available_bids(&self) -> Vec<GameType> {
        let deal = match &self.engine.match_state.current_deal {
            Some(d) => d,
            None => return Vec::new(),
        };

        let is_caller = self.human_player == deal.caller;
        GameType::all_by_value()
            .into_iter()
            .filter(|gt| {
                match gt.eligibility() {
                    schnapsen_model::game_type::Eligibility::CallerOnly => is_caller,
                    schnapsen_model::game_type::Eligibility::OpponentsOnly => !is_caller,
                    schnapsen_model::game_type::Eligibility::Anyone => true,
                }
            })
            .collect()
    }
}

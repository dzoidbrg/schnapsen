use schnapsen_engine::player::{Player, RandomPlayer};
use schnapsen_model::game::{GameState, Phase};
use schnapsen_model::types::{Action, MatchScore, PlayerId};

pub const HUMAN_PLAYER: PlayerId = 0;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AppState {
    /// Waiting for the human to pick from a list of options.
    Choosing {
        prompt: String,
        options: Vec<ChoiceOption>,
        selected: usize,
    },
    /// An AI player is acting — we show what happened and wait for input to continue.
    AiActed {
        message: String,
    },
    /// A hand just finished.
    HandOver {
        message: String,
    },
    /// The match is over.
    MatchOver {
        winner: PlayerId,
    },
}

#[derive(Debug, Clone)]
pub struct ChoiceOption {
    pub label: String,
    pub action: Action,
}

pub struct App {
    pub state: AppState,
    pub game: GameState,
    pub score: MatchScore,
    pub geber: PlayerId,
    pub ai: [RandomPlayer; 2],
    pub log: Vec<String>,
    pub last_trick_display: Vec<(PlayerId, schnapsen_model::card::Card)>,
}

impl App {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let geber = 0usize;
        let game = GameState::deal(geber, &mut rng);
        let ai = [RandomPlayer::new("Bot B"), RandomPlayer::new("Bot C")];

        let mut app = Self {
            state: AppState::Choosing {
                prompt: String::new(),
                options: Vec::new(),
                selected: 0,
            },
            game,
            score: MatchScore::new(),
            geber,
            ai,
            log: vec!["Welcome to Dreierschnapsen!".into()],
            last_trick_display: Vec::new(),
        };

        app.advance_to_human();
        app
    }

    pub fn move_selection_left(&mut self) {
        if let AppState::Choosing { selected, options, .. } = &mut self.state {
            if *selected > 0 {
                *selected -= 1;
            } else {
                *selected = options.len().saturating_sub(1);
            }
        }
    }

    pub fn move_selection_right(&mut self) {
        if let AppState::Choosing { selected, options, .. } = &mut self.state {
            *selected = (*selected + 1) % options.len().max(1);
        }
    }

    pub fn confirm_selection(&mut self) {
        match &self.state {
            AppState::Choosing {
                selected, options, ..
            } => {
                if options.is_empty() {
                    return;
                }
                let choice = options[*selected].clone();
                self.log.push(format!("You: {}", choice.label));
                self.game
                    .apply_action(HUMAN_PLAYER, choice.action)
                    .expect("human selected invalid action");
                self.after_human_action();
            }
            AppState::AiActed { .. } | AppState::HandOver { .. } => {
                self.advance_to_human();
            }
            AppState::MatchOver { .. } => {}
        }
    }

    fn after_human_action(&mut self) {
        if self.check_hand_over() {
            return;
        }
        self.advance_to_human();
    }

    /// Run AI actions until it's the human's turn (or the hand ends).
    fn advance_to_human(&mut self) {
        loop {
            if matches!(self.game.phase, Phase::Finished { .. }) {
                self.check_hand_over();
                return;
            }

            let Some(active) = self.game.active_player() else {
                return;
            };

            if active == HUMAN_PLAYER {
                self.present_choices();
                return;
            }

            let ai_idx = if active == 1 { 0 } else { 1 };
            let action = self.ai[ai_idx].choose_action(&self.game);
            let label = format_action(&action);
            self.log
                .push(format!("Player {}: {}", active + 1, label));
            self.game
                .apply_action(active, action)
                .expect("AI selected invalid action");

            if self.game.current_trick.is_empty() && !self.game.tricks.is_empty() {
                if let Some(last) = self.game.tricks.last() {
                    self.last_trick_display = last.cards.clone();
                }
            }
        }
    }

    fn present_choices(&mut self) {
        let actions = self.game.valid_actions();
        if actions.is_empty() {
            return;
        }

        let prompt = match &self.game.phase {
            Phase::CallingTrump => "Choose trump suit:".into(),
            Phase::Bidding { .. } => "Bid or pass:".into(),
            Phase::TalonDecision { .. } => "Take the talon?".into(),
            Phase::Discarding { .. } => "Choose 2 cards to discard:".into(),
            Phase::Playing { game_type, .. } => {
                format!("Play a card ({}): ", game_type.name_de())
            }
            Phase::Finished { .. } => "Hand finished.".into(),
        };

        let options: Vec<ChoiceOption> = actions
            .into_iter()
            .map(|a| ChoiceOption {
                label: format_action(&a),
                action: a,
            })
            .collect();

        self.state = AppState::Choosing {
            prompt,
            options,
            selected: 0,
        };
    }

    fn check_hand_over(&mut self) -> bool {
        let Phase::Finished {
            declarer,
            game_type,
            declarer_won,
            points_awarded,
        } = self.game.phase
        else {
            return false;
        };

        if declarer_won {
            self.score.game_points[declarer] =
                self.score.game_points[declarer].saturating_add(points_awarded);
        } else {
            for p in 0..3 {
                if p != declarer {
                    self.score.game_points[p] =
                        self.score.game_points[p].saturating_add(points_awarded);
                }
            }
        }

        let winner_name = player_name(declarer);
        let result = if declarer_won { "WON" } else { "LOST" };
        let msg = format!(
            "{} {} {} (+{} pts)",
            winner_name,
            result,
            game_type.name_de(),
            points_awarded
        );
        self.log.push(msg.clone());

        if let Some(winner) = self.score.winner() {
            self.state = AppState::MatchOver { winner };
            return true;
        }

        self.geber = (self.geber + 1) % 3;
        let mut rng = rand::thread_rng();
        self.game = GameState::deal(self.geber, &mut rng);
        self.last_trick_display.clear();

        self.state = AppState::HandOver { message: msg };

        true
    }
}

fn player_name(id: PlayerId) -> &'static str {
    match id {
        0 => "You",
        1 => "Bot B",
        2 => "Bot C",
        _ => "???",
    }
}

fn format_action(action: &Action) -> String {
    match action {
        Action::CallTrump(suit) => format!("Trump: {} {}", suit.symbol(), suit.name_de()),
        Action::Bid(gt) => format!("Bid: {}", gt),
        Action::Pass => "Pass".into(),
        Action::TakeTalon { take: true } => "Take talon".into(),
        Action::TakeTalon { take: false } => "Play without talon".into(),
        Action::Discard(c1, c2) => format!("Discard: {} {}", c1, c2),
        Action::PlayCard(card) => format!("{}", card),
        Action::AnnounceMarriage(m, card) => {
            let kind = if m.is_trump { "40er" } else { "20er" };
            format!("{} ({} {})", card, kind, m.suit)
        }
        Action::Spritzen => "Spritzen!".into(),
        Action::AcceptSpritzen => "Accept".into(),
    }
}

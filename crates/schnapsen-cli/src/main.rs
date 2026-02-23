use std::error::Error;
use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Terminal;
use schnapsen_engine::{apply_move, pick_random_valid_move_thread_rng, valid_moves};
use schnapsen_model::{full_deck, Card, GameDeclaration, PlayerId, PlayerMove, RoundState, Suit};

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    let mut app = App::new();

    let run_result = run_app(&mut terminal, &mut app);
    restore_terminal(&mut terminal)?;

    run_result?;
    Ok(())
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    loop {
        if !app.state.is_round_over() && app.state.active_player != app.human_player {
            app.play_bot_turn();
        }

        terminal.draw(|frame| draw_ui(frame, app))?;

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(120))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code);
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
struct App {
    state: RoundState,
    human_player: PlayerId,
    selected_move: usize,
    status: String,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        let state = demo_round_state();
        Self {
            state,
            human_player: PlayerId::P0,
            selected_move: 0,
            status: "Your turn. Select a legal move and press Enter.".to_string(),
            should_quit: false,
        }
    }

    fn legal_moves(&self) -> Vec<PlayerMove> {
        valid_moves(&self.state, self.human_player)
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') => self.select_next_move(),
            KeyCode::Up | KeyCode::Char('k') => self.select_previous_move(),
            KeyCode::Enter => self.play_selected_human_move(),
            KeyCode::Char('r') => self.play_random_human_move(),
            _ => {}
        }
    }

    fn select_next_move(&mut self) {
        let legal_count = self.legal_moves().len();
        if legal_count == 0 {
            self.selected_move = 0;
            return;
        }
        self.selected_move = (self.selected_move + 1) % legal_count;
    }

    fn select_previous_move(&mut self) {
        let legal_count = self.legal_moves().len();
        if legal_count == 0 {
            self.selected_move = 0;
            return;
        }
        self.selected_move = (self.selected_move + legal_count - 1) % legal_count;
    }

    fn play_selected_human_move(&mut self) {
        if self.state.is_round_over() {
            self.status = "Round is finished. Press q to quit.".to_string();
            return;
        }
        if self.state.active_player != self.human_player {
            self.status = "Wait for bot players.".to_string();
            return;
        }

        let legal = self.legal_moves();
        if legal.is_empty() {
            self.status = "No legal move available.".to_string();
            return;
        }
        let idx = self.selected_move.min(legal.len() - 1);
        let mv = legal[idx];
        self.apply_player_move(self.human_player, mv);
    }

    fn play_random_human_move(&mut self) {
        if self.state.is_round_over() {
            self.status = "Round is finished. Press q to quit.".to_string();
            return;
        }
        if self.state.active_player != self.human_player {
            self.status = "Wait for bot players.".to_string();
            return;
        }

        if let Some(mv) = pick_random_valid_move_thread_rng(&self.state, self.human_player) {
            self.apply_player_move(self.human_player, mv);
        } else {
            self.status = "No legal move available.".to_string();
        }
    }

    fn play_bot_turn(&mut self) {
        if self.state.is_round_over() {
            return;
        }
        let bot = self.state.active_player;
        if bot == self.human_player {
            return;
        }

        if let Some(mv) = pick_random_valid_move_thread_rng(&self.state, bot) {
            self.apply_player_move(bot, mv);
        } else {
            self.status = format!("{bot} has no legal move.");
        }
    }

    fn apply_player_move(&mut self, player: PlayerId, mv: PlayerMove) {
        let label = move_label(mv);
        match apply_move(&mut self.state, player, mv) {
            Ok(()) => {
                if self.state.is_round_over() {
                    self.status = self.round_finished_message();
                } else {
                    self.status = format!("{player} played {label}");
                    self.ensure_selection_in_range();
                }
            }
            Err(err) => {
                self.status = format!("Move rejected for {player}: {err:?}");
                self.ensure_selection_in_range();
            }
        }
    }

    fn ensure_selection_in_range(&mut self) {
        let legal = self.legal_moves().len();
        if legal == 0 {
            self.selected_move = 0;
            return;
        }
        if self.selected_move >= legal {
            self.selected_move = legal - 1;
        }
    }

    fn round_finished_message(&self) -> String {
        let mut best_player = PlayerId::P0;
        let mut best_points = self.state.card_points[PlayerId::P0.index()];

        for player in [PlayerId::P1, PlayerId::P2] {
            let points = self.state.card_points[player.index()];
            if points > best_points {
                best_player = player;
                best_points = points;
            }
        }

        format!("Round finished. Trick-points leader: {best_player} with {best_points} points.")
    }
}

fn demo_round_state() -> RoundState {
    let mut deck = full_deck();
    let hands = [
        deck.drain(0..6).collect::<Vec<_>>(),
        deck.drain(0..6).collect::<Vec<_>>(),
        deck.drain(0..6).collect::<Vec<_>>(),
    ];
    let talon = deck;

    RoundState::new(
        PlayerId::P0,
        PlayerId::P0,
        GameDeclaration::Normal,
        Some(Suit::Hearts),
        hands,
        talon,
    )
}

fn move_label(mv: PlayerMove) -> String {
    match mv {
        PlayerMove::PlayCard(card) => card.to_string(),
    }
}

fn draw_ui(frame: &mut Frame, app: &App) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(14),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new(format!(
        "Dreierschnapsen | Declaration: {:?} | Trump: {} | Active: {}",
        app.state.declaration,
        app.state
            .active_trump()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "none".to_string()),
        app.state.active_player
    ))
    .block(Block::default().borders(Borders::ALL).title("Round"))
    .wrap(Wrap { trim: true });
    frame.render_widget(header, root[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(42),
            Constraint::Percentage(30),
            Constraint::Percentage(28),
        ])
        .split(root[1]);

    draw_hand_panel(frame, app, body[0]);
    draw_moves_panel(frame, app, body[1]);
    draw_trick_panel(frame, app, body[2]);

    let footer = Paragraph::new(format!(
        "{}\nControls: j/k or arrows=move, Enter=play selected, r=random, q=quit",
        app.status
    ))
    .block(Block::default().borders(Borders::ALL).title("Status"))
    .wrap(Wrap { trim: true });
    frame.render_widget(footer, root[2]);
}

fn draw_hand_panel(frame: &mut Frame, app: &App, area: Rect) {
    let legal_cards: Vec<Card> = app
        .legal_moves()
        .into_iter()
        .map(|mv| match mv {
            PlayerMove::PlayCard(card) => card,
        })
        .collect();

    let items: Vec<ListItem> = app
        .state
        .hand(app.human_player)
        .iter()
        .copied()
        .map(|card| {
            let is_legal = legal_cards.contains(&card);
            let prefix = if is_legal { "* " } else { "  " };
            let style = suit_style(card.suit);
            ListItem::new(format!("{prefix}{card}")).style(style)
        })
        .collect();

    let hand = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Your Hand (* = legal)"),
    );
    frame.render_widget(hand, area);
}

fn draw_moves_panel(frame: &mut Frame, app: &App, area: Rect) {
    let legal = app.legal_moves();
    let items: Vec<ListItem> = legal
        .iter()
        .copied()
        .map(move_label)
        .map(ListItem::new)
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Legal Moves"))
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    let mut state = ListState::default();
    if !legal.is_empty() {
        state.select(Some(app.selected_move.min(legal.len() - 1)));
    }
    frame.render_stateful_widget(list, area, &mut state);
}

fn draw_trick_panel(frame: &mut Frame, app: &App, area: Rect) {
    let current_trick_lines = if app.state.current_trick.plays.is_empty() {
        vec!["(empty)".to_string()]
    } else {
        app.state
            .current_trick
            .plays
            .iter()
            .map(|play| format!("{}: {}", play.player, play.card))
            .collect()
    };

    let text = format!(
        "Current trick:\n{}\n\nCard points:\nP0: {}\nP1: {}\nP2: {}\nCompleted tricks: {}",
        current_trick_lines.join("\n"),
        app.state.card_points[0],
        app.state.card_points[1],
        app.state.card_points[2],
        app.state.completed_tricks.len()
    );
    let panel =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Table State"));
    frame.render_widget(panel, area);
}

fn suit_style(suit: Suit) -> Style {
    match suit {
        Suit::Hearts | Suit::Diamonds => Style::default().fg(Color::Red),
        Suit::Spades | Suit::Clubs => Style::default().fg(Color::White),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use schnapsen_model::Rank;

    #[test]
    fn selection_wraps_forward_and_backward() {
        let mut app = App::new();
        let legal_count = app.legal_moves().len();
        assert!(legal_count > 0);

        app.selected_move = legal_count - 1;
        app.select_next_move();
        assert_eq!(app.selected_move, 0);

        app.select_previous_move();
        assert_eq!(app.selected_move, legal_count - 1);
    }

    #[test]
    fn move_label_formats_card() {
        let mv = PlayerMove::PlayCard(Card::new(Suit::Hearts, Rank::Ace));
        assert_eq!(move_label(mv), "AH");
    }
}

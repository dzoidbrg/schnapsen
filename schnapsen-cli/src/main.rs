#![forbid(unsafe_code)]

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use schnapsen_engine::{
    apply_move, legal_cards_for_player, round_summary, setup_demo_normal_round, RandomMoveEngine,
};
use schnapsen_model::{Card, GameMove, PlayerId, Rank, Suit};
use std::io::{self, Stdout};
use std::time::Duration;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    app_result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut app = App::new();
    loop {
        app.play_bot_turn_if_needed();
        terminal.draw(|frame| draw_ui(frame, &app))?;

        if event::poll(Duration::from_millis(150))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }
                if app.on_key(key_event.code) {
                    break;
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct App {
    state: schnapsen_model::GameState,
    engine: RandomMoveEngine<rand::rngs::ThreadRng>,
    human: PlayerId,
    selection: usize,
    status_line: String,
}

impl App {
    fn new() -> Self {
        let mut rng = rand::rng();
        let state = setup_demo_normal_round(&mut rng);
        Self {
            state,
            engine: RandomMoveEngine::default(),
            human: PlayerId::One,
            selection: 0,
            status_line: "Arrows to choose card, Enter to play, r restart, q quit.".to_string(),
        }
    }

    fn restart(&mut self) {
        let mut rng = rand::rng();
        self.state = setup_demo_normal_round(&mut rng);
        self.selection = 0;
        self.status_line = "New round started.".to_string();
    }

    fn legal_human_cards(&self) -> Vec<Card> {
        legal_cards_for_player(&self.state, self.human)
    }

    fn play_bot_turn_if_needed(&mut self) {
        if self.state.is_finished() || self.state.active_player == self.human {
            return;
        }
        if let Some(next_move) = self.engine.pick_random_valid_move(&self.state) {
            let status = match next_move {
                GameMove::PlayCard { player, card } => format!("{player} played {card}"),
                _ => "Bot made unsupported move".to_string(),
            };
            if let Err(err) = apply_move(&mut self.state, next_move) {
                self.status_line = format!("engine error: {err}");
                return;
            }
            self.status_line = status;
        }
        if self.state.is_finished() {
            self.status_line = self.finished_message();
        }
    }

    fn play_selected_human_card(&mut self) {
        if self.state.is_finished() {
            self.status_line = "Round already finished. Press r to restart.".to_string();
            return;
        }
        if self.state.active_player != self.human {
            self.status_line = "Wait for bot turns.".to_string();
            return;
        }

        let legal = self.legal_human_cards();
        if legal.is_empty() {
            self.status_line = "No legal move available.".to_string();
            return;
        }
        let clamped_selection = self.selection.min(legal.len() - 1);
        self.selection = clamped_selection;
        let selected_card = legal[clamped_selection];
        let selected_move = GameMove::PlayCard {
            player: self.human,
            card: selected_card,
        };

        match apply_move(&mut self.state, selected_move) {
            Ok(()) => {
                self.status_line = format!("You played {selected_card}");
                if self.state.is_finished() {
                    self.status_line = self.finished_message();
                }
            }
            Err(err) => {
                self.status_line = format!("illegal move: {err}");
            }
        }
    }

    fn on_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') => true,
            KeyCode::Char('r') => {
                self.restart();
                false
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.selection = self.selection.saturating_sub(1);
                false
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let legal_len = self.legal_human_cards().len();
                if legal_len > 0 {
                    self.selection = (self.selection + 1).min(legal_len - 1);
                }
                false
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.play_selected_human_card();
                false
            }
            _ => false,
        }
    }

    fn finished_message(&self) -> String {
        if let Some(summary) = round_summary(&self.state) {
            format!(
                "Round finished. Winner: {} ({} card points). Press r to restart.",
                summary.winner,
                summary.card_points[summary.winner.as_index()]
            )
        } else {
            "Round finished. Press r to restart.".to_string()
        }
    }
}

fn draw_ui(frame: &mut Frame<'_>, app: &App) {
    let root = frame.area();
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(12),
            Constraint::Length(8),
        ])
        .split(root);

    let header = render_header(app);
    frame.render_widget(header, vertical[0]);

    let center_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(vertical[1]);

    frame.render_widget(render_players_panel(app), center_layout[0]);
    frame.render_widget(render_trick_panel(app), center_layout[1]);
    frame.render_widget(render_legal_moves_panel(app), center_layout[2]);
    frame.render_widget(render_hand_panel(app), vertical[2]);
}

fn render_header(app: &App) -> Paragraph<'static> {
    let trump = app
        .state
        .trump
        .map(|s| s.short().to_string())
        .unwrap_or_else(|| "-".to_string());
    let header_text = vec![
        Line::from(format!(
            "Dreierschnapsen CLI | Variant: {:?} | Trump: {} | Active: {}",
            app.state.variant, trump, app.state.active_player
        )),
        Line::from(app.status_line.clone()),
    ];
    Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .wrap(Wrap { trim: true })
}

fn render_players_panel(app: &App) -> Paragraph<'static> {
    let mut lines = vec![Line::from("Players overview:")];
    for player in PlayerId::ALL {
        let you = if player == app.human { " (you)" } else { "" };
        let won_tricks = app
            .state
            .completed_tricks
            .iter()
            .filter(|trick| trick.winner == player)
            .count();
        lines.push(Line::from(format!(
            "{}{} | hand: {} | points: {} | tricks: {}",
            player,
            you,
            app.state.player(player).hand.len(),
            app.state.total_points_for_player(player),
            won_tricks
        )));
    }

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .wrap(Wrap { trim: true })
}

fn render_trick_panel(app: &App) -> Paragraph<'static> {
    let mut lines = vec![
        Line::from(format!(
            "Current trick leader: {}",
            app.state.current_trick.leader
        )),
        Line::from(""),
    ];
    if app.state.current_trick.cards.is_empty() {
        lines.push(Line::from("No cards played in this trick yet."));
    } else {
        for (player, card) in &app.state.current_trick.cards {
            lines.push(Line::from(vec![
                Span::raw(format!("{player}: ")),
                Span::styled(
                    card_label(*card),
                    Style::default()
                        .fg(suit_color(card.suit))
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
        }
    }
    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "Completed tricks: {}",
        app.state.completed_tricks.len()
    )));

    Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Trick"))
        .wrap(Wrap { trim: true })
}

fn render_legal_moves_panel(app: &App) -> Paragraph<'static> {
    let legal_cards = app.legal_human_cards();
    let mut lines = vec![Line::from("Playable cards:")];
    if legal_cards.is_empty() {
        lines.push(Line::from("No legal moves right now."));
    } else {
        let selected = app.selection.min(legal_cards.len() - 1);
        for (index, card) in legal_cards.iter().copied().enumerate() {
            let marker = if index == selected { ">" } else { " " };
            lines.push(Line::from(vec![
                Span::raw(format!("{marker} {:>2}: ", index + 1)),
                Span::styled(
                    card_label(card),
                    Style::default()
                        .fg(suit_color(card.suit))
                        .add_modifier(if index == selected {
                            Modifier::BOLD | Modifier::UNDERLINED
                        } else {
                            Modifier::BOLD
                        }),
                ),
            ]));
        }
    }
    Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Your legal moves"),
        )
        .wrap(Wrap { trim: true })
}

fn render_hand_panel(app: &App) -> Paragraph<'static> {
    let legal_cards = app.legal_human_cards();
    let selected = legal_cards
        .get(app.selection.min(legal_cards.len().saturating_sub(1)))
        .copied();
    let mut spans = vec![Span::raw("Your hand: ")];
    for card in &app.state.player(app.human).hand {
        let is_legal = legal_cards.contains(card);
        let is_selected = selected == Some(*card);
        let base_style = Style::default().fg(suit_color(card.suit));
        let styled = if is_selected {
            base_style
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else if is_legal {
            base_style.add_modifier(Modifier::BOLD)
        } else {
            base_style
        };
        spans.push(Span::styled(format!(" {} ", card_label(*card)), styled));
    }
    let content = vec![
        Line::from(spans),
        Line::from("Controls: Left/Right to choose, Enter to play, r restart, q quit"),
    ];
    Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Hand"))
        .wrap(Wrap { trim: true })
}

fn card_label(card: Card) -> String {
    format!("{}{}", short_rank(card.rank), card.suit.short())
}

fn short_rank(rank: Rank) -> &'static str {
    match rank {
        Rank::Ace => "A",
        Rank::Ten => "10",
        Rank::King => "K",
        Rank::Ober => "O",
        Rank::Under => "U",
    }
}

fn suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Hearts | Suit::Diamonds => Color::Red,
        Suit::Spades => Color::White,
        Suit::Clubs => Color::Green,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_label_is_stable() {
        assert_eq!(card_label(Card::new(Suit::Hearts, Rank::Ace)), "AH");
        assert_eq!(card_label(Card::new(Suit::Clubs, Rank::Under)), "UC");
    }
}

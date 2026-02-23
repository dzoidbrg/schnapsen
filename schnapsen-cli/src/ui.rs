//! Terminal UI rendering.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use schnapsen_model::{Card, Player, Suit};

/// Draw the full UI.
pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(frame.size());

    draw_header(frame, chunks[0], app);
    draw_main(frame, chunks[1], app);
    draw_hand(frame, chunks[2], app);
    draw_footer(frame, chunks[3], app);
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App) {
    let title = format!(
        " Dreierschnapsen │ Trump: {} │ {} ",
        app.state
            .trump
            .map(|s| s.to_string())
            .unwrap_or_else(|| "—".into()),
        if app.is_my_turn() {
            "Your turn"
        } else {
            "Opponents' turn..."
        }
    );
    let block = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .style(Style::default().fg(Color::Cyan));
    let para = Paragraph::new(title).block(block);
    frame.render_widget(para, area);
}

fn draw_main(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(area);

    // Opponents' tricks
    let opp_tricks: Vec<String> = [Player::LeftOpponent, Player::RightOpponent]
        .iter()
        .map(|p| {
            let tricks = app.state.tricks_won.get(p).map(|t| t.len()).unwrap_or(0);
            let pts = app.state.trick_points.get(p).unwrap_or(&0);
            format!("{}: {} tricks, {} pts", p, tricks, pts)
        })
        .collect();
    let opp_block = Block::default()
        .borders(Borders::ALL)
        .title(" Opponents ")
        .border_set(symbols::border::ROUNDED);
    let opp_para = Paragraph::new(opp_tricks.join(" │ "))
        .block(opp_block)
        .wrap(Wrap { trim: true });
    frame.render_widget(opp_para, chunks[0]);

    // Current trick
    let trick_cards: Vec<Span> = app
        .state
        .current_trick
        .plays()
        .iter()
        .map(|(p, c)| {
            Span::styled(
                format!(" {} {} ", p, render_card_compact(c)),
                Style::default().fg(color_for_suit(c.suit)),
            )
        })
        .collect();
    let trick_text = if trick_cards.is_empty() {
        Line::from("Current trick: (empty)")
    } else {
        Line::from("Current trick: ".to_string()).spans(trick_cards)
    };
    let trick_block = Block::default()
        .borders(Borders::ALL)
        .title(" Current Trick ")
        .border_set(symbols::border::ROUNDED);
    let trick_para = Paragraph::new(trick_text).block(trick_block);
    frame.render_widget(trick_para, chunks[1]);
}

fn draw_hand(frame: &mut Frame, area: Rect, app: &App) {
    let hand = app.hand();
    let selected = app.selected_card();

    let card_spans: Vec<Span> = hand
        .iter()
        .enumerate()
        .map(|(i, card)| {
            let style = if i == selected && app.is_my_turn() {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(color_for_suit(card.suit))
            };
            Span::styled(
                format!(" {} ", render_card_compact(card)),
                style,
            )
        })
        .collect();

    let line = Line::from(" Your hand: ").spans(card_spans);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Your Cards ")
        .border_set(symbols::border::ROUNDED);
    let para = Paragraph::new(line).block(block);
    frame.render_widget(para, area);
}

fn draw_footer(frame: &mut Frame, area: Rect, app: &App) {
    let scores: String = schnapsen_model::Player::all()
        .iter()
        .map(|p| {
            let s = app.state.scores.get(p).copied().unwrap_or(0);
            format!("{}: {}", p, s)
        })
        .collect::<Vec<_>>()
        .join("  │  ");
    let help = " ← → select card  Enter play  q quit  n new round ";
    let text = if app.message.is_empty() {
        format!("Scores: {}  │  {}", scores, help)
    } else {
        format!("{}  │  {}", app.message, help)
    };
    let para = Paragraph::new(text).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(para, area);
}

fn render_card_compact(card: &Card) -> String {
    format!("{}{}", card.rank.short_name(), card.suit.symbol())
}

fn color_for_suit(suit: Suit) -> Color {
    match suit {
        Suit::Hearts => Color::LightRed,
        Suit::Diamonds => Color::LightYellow,
        Suit::Spades => Color::White,
        Suit::Clubs => Color::Green,
    }
}

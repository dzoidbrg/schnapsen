//! Ratatui UI layout and rendering.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use schnapsen_model::{Card, Suit};

use crate::app::App;

const CARD_WIDTH: u16 = 5;
const CARD_HEIGHT: u16 = 3;

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(frame.size());

    // Title
    let title = Paragraph::new("Dreierschnapsen — Press q to quit")
        .block(Block::default().borders(Borders::BOTTOM))
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(title, chunks[0]);

    // Main area: trick + hand
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(5)])
        .split(chunks[1]);

    if let Some(ref round) = app.state.round {
        // Current trick
        let trick_block = Block::default()
            .title("Current Trick")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        frame.render_widget(&trick_block, main_chunks[0]);
        let trick_area = trick_block.inner(main_chunks[0]);
        render_trick(frame, trick_area, round);

        // Player hand
        let hand = &round.hands[0];
        let hand_block = Block::default()
            .title("Your Hand")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));
        frame.render_widget(&hand_block, main_chunks[1]);
        let hand_area = hand_block.inner(main_chunks[1]);
        render_hand(frame, hand_area, hand.cards(), app.selected_card_index);
    }

    // Status / message
    let status = Paragraph::new(app.message.as_str())
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .wrap(Wrap { trim: true });
    frame.render_widget(status, chunks[2]);
}

fn render_hand(
    frame: &mut Frame,
    area: Rect,
    cards: &[Card],
    selected: usize,
) {
    if cards.is_empty() {
        return;
    }
    let card_w = CARD_WIDTH + 1;
    let start_x = area.x + area.width.saturating_sub(cards.len() as u16 * card_w) / 2;
    let y = area.y + 1;
    for (i, card) in cards.iter().enumerate() {
        let x = start_x + i as u16 * card_w;
        if x + CARD_WIDTH <= area.x + area.width {
            let style = if i == selected {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            render_card(frame, x, y, card, style);
        }
    }
}

fn render_card(frame: &mut Frame, x: u16, y: u16, card: &Card, style: Style) {
    let label = format!("{}{}", card.rank, suit_symbol(card.suit));
    let para = Paragraph::new(label)
        .style(style)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(para, Rect::new(x, y, CARD_WIDTH, CARD_HEIGHT));
}

fn render_trick(
    frame: &mut Frame,
    area: Rect,
    round: &schnapsen_model::RoundState,
) {
    let plays = round.current_trick.plays();
    if plays.is_empty() {
        let empty = Paragraph::new("(empty)").style(Style::default().fg(Color::DarkGray));
        frame.render_widget(empty, area);
        return;
    }
    let lines: Vec<Line> = plays
        .iter()
        .map(|(pid, c)| {
            Line::from(Span::raw(format!("P{}: {}{}", pid + 1, c.rank, suit_symbol(c.suit))))
        })
        .collect();
    let para = Paragraph::new(lines);
    frame.render_widget(para, area);
}

fn suit_symbol(suit: Suit) -> &'static str {
    match suit {
        Suit::Herz => "♥",
        Suit::Karo => "♦",
        Suit::Pik => "♠",
        Suit::Kreuz => "♣",
    }
}

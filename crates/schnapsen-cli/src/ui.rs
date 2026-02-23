use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

use schnapsen_model::card::{Card, Suit};
use schnapsen_model::player::PlayerId;
use schnapsen_model::state::Phase;

use crate::app::{App, Screen};

fn suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Hearts | Suit::Diamonds => Color::Red,
        Suit::Spades | Suit::Clubs => Color::White,
    }
}

fn card_span(card: &Card) -> Span<'static> {
    let text = format!(" {}{} ", card.rank.short_name(), card.suit.symbol());
    Span::styled(text, Style::default().fg(suit_color(card.suit)).bold())
}

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),   // Main area
            Constraint::Length(3), // Status bar
        ])
        .split(f.area());

    draw_header(f, app, chunks[0]);
    draw_status_bar(f, app, chunks[2]);

    match app.screen {
        Screen::TrumpSelection => draw_trump_selection(f, app, chunks[1]),
        Screen::Bidding => draw_bidding(f, app, chunks[1]),
        Screen::TalonExchange => draw_talon_exchange(f, app, chunks[1]),
        Screen::Playing => draw_playing(f, app, chunks[1]),
        Screen::TrickResult => draw_trick_result(f, app, chunks[1]),
        Screen::DealResult => draw_deal_result(f, app, chunks[1]),
        Screen::MatchResult => draw_match_result(f, app, chunks[1]),
    }
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let scores = &app.engine.match_state.scores;
    let deal = app.engine.match_state.current_deal.as_ref();

    let trump_text = deal
        .and_then(|d| d.trump)
        .map(|s| format!(" | Trumpf: {}{}", s.german_name(), s.symbol()))
        .unwrap_or_default();

    let game_text = deal
        .and_then(|d| d.game_type)
        .map(|gt| format!(" | {}", gt.german_name()))
        .unwrap_or_default();

    let header = Paragraph::new(Line::from(vec![
        Span::styled("Dreierschnapsen", Style::default().fg(Color::Yellow).bold()),
        Span::raw(format!(
            "  |  Du: {}  P1: {}  P2: {}{}{}",
            scores[0], scores[1], scores[2], trump_text, game_text
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(header, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status_text = if let Some(ref msg) = app.status {
        Span::styled(
            &msg.text,
            Style::default().fg(if msg.is_error {
                Color::Red
            } else {
                Color::Green
            }),
        )
    } else {
        Span::raw(match app.screen {
            Screen::TrumpSelection => "←/→: Farbe wählen | Enter: Bestätigen | q: Beenden",
            Screen::Bidding => "↑/↓: Spiel wählen | Enter: Ansagen | p: Passen | q: Beenden",
            Screen::TalonExchange => "t: Talon nehmen | ←/→: Karte wählen | Space: Markieren | Enter: Ablegen | q: Beenden",
            Screen::Playing => "←/→: Karte wählen | Enter: Spielen | q: Beenden",
            Screen::TrickResult | Screen::DealResult => "Enter: Weiter | q: Beenden",
            Screen::MatchResult => "Enter: Neues Spiel | q: Beenden",
        })
    };

    let bar = Paragraph::new(Line::from(status_text)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(bar, area);
}

fn draw_trump_selection(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(7),
        ])
        .split(area);

    let prompt = Paragraph::new("Wähle die Trumpffarbe:")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(prompt, chunks[0]);

    draw_hand(f, app, chunks[2]);

    let suits = Suit::all();
    let suit_spans: Vec<Span> = suits
        .iter()
        .enumerate()
        .flat_map(|(i, s)| {
            let style = if i == app.selected_trump_index % 4 {
                Style::default()
                    .fg(suit_color(*s))
                    .add_modifier(Modifier::REVERSED | Modifier::BOLD)
            } else {
                Style::default().fg(suit_color(*s))
            };
            vec![
                Span::styled(
                    format!("  {} {}  ", s.symbol(), s.german_name()),
                    style,
                ),
                Span::raw("  "),
            ]
        })
        .collect();

    let suit_line = Paragraph::new(Line::from(suit_spans))
        .alignment(Alignment::Center)
        .block(Block::default());
    f.render_widget(suit_line, chunks[1]);
}

fn draw_bidding(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(7),
        ])
        .split(area);

    let prompt = Paragraph::new("Welches Spiel möchtest du ansagen? (p = Passen)")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(prompt, chunks[0]);

    draw_hand(f, app, chunks[2]);

    let bids = app.available_bids();
    let items: Vec<ListItem> = bids
        .iter()
        .enumerate()
        .map(|(i, gt)| {
            let style = if i == app.selected_bid_index % bids.len().max(1) {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::REVERSED)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("  {} ({} Punkte)", gt.german_name(), gt.base_points()))
                .style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" Spiele ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(list, chunks[1]);
}

fn draw_talon_exchange(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(7),
        ])
        .split(area);

    let msg = if app.talon_taken {
        "Wähle 2 Karten zum Ablegen (Space=Markieren, Enter=Bestätigen)"
    } else {
        "Drücke 't' um den Talon aufzunehmen"
    };

    let prompt = Paragraph::new(msg)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(prompt, chunks[0]);

    if !app.talon_taken {
        if let Some(deal) = &app.engine.match_state.current_deal {
            if let Some(talon) = &deal.talon {
                let talon_spans: Vec<Span> = vec![
                    Span::raw("  Talon: "),
                    card_span(&talon[0]),
                    Span::raw("  "),
                    card_span(&talon[1]),
                ];
                let talon_line = Paragraph::new(Line::from(talon_spans))
                    .alignment(Alignment::Center);
                f.render_widget(talon_line, chunks[1]);
            }
        }
    }

    draw_hand_with_selection(f, app, chunks[2]);
}

fn draw_playing(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Opponents
            Constraint::Min(5),   // Trick area
            Constraint::Length(7), // Player hand
        ])
        .split(area);

    draw_opponents(f, app, chunks[0]);
    draw_trick_area(f, app, chunks[1]);
    draw_playable_hand(f, app, chunks[2]);
}

fn draw_opponents(f: &mut Frame, app: &App, area: Rect) {
    let deal = match &app.engine.match_state.current_deal {
        Some(d) => d,
        None => return,
    };

    let opp_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    for (i, &pid) in [PlayerId::Player1, PlayerId::Player2].iter().enumerate() {
        let player = deal.player(pid);
        let card_count = player.hand.len();
        let role = match player.role {
            schnapsen_model::Role::Dealer => "Geber",
            schnapsen_model::Role::Caller => "Rufer",
            schnapsen_model::Role::Third => "Dritter",
        };

        let back_cards = "🂠 ".repeat(card_count);
        let info = format!("{} ({}) - {} Karten", pid, role, card_count);

        let tricks = if let Phase::Playing(ps) = &deal.phase {
            if deal.is_announcer(pid) {
                format!("Stiche: {} | Punkte: {}", ps.announcer_tricks.len(), ps.announcer_card_points)
            } else {
                format!("Stiche: {} | Punkte: {}", ps.defender_tricks.len(), ps.defender_card_points)
            }
        } else {
            String::new()
        };

        let block = Block::default()
            .title(format!(" {} ", info))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let content = Paragraph::new(vec![
            Line::from(Span::styled(back_cards, Style::default().fg(Color::Blue))),
            Line::from(Span::styled(tricks, Style::default().fg(Color::Gray))),
        ])
        .block(block);

        f.render_widget(content, opp_chunks[i]);
    }
}

fn draw_trick_area(f: &mut Frame, app: &App, area: Rect) {
    if app.engine.match_state.current_deal.is_none() {
        return;
    }

    let trick_cards = app.current_trick_cards();

    let mut lines = vec![Line::from("")];

    if trick_cards.is_empty() {
        lines.push(Line::from(Span::styled(
            "Warte auf ersten Stich...",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for (pid, card) in &trick_cards {
            let player_label = if *pid == app.human_player {
                "Du".to_string()
            } else {
                format!("{}", pid)
            };
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {}: ", player_label),
                    Style::default().fg(Color::Gray),
                ),
                card_span(card),
            ]));
        }
    }

    let block = Block::default()
        .title(" Stich ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}

fn draw_playable_hand(f: &mut Frame, app: &App, area: Rect) {
    let valid = app.valid_plays();
    let hand = app.human_hand();

    if valid.is_empty() && hand.is_empty() {
        let block = Block::default()
            .title(" Deine Karten ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        let para = Paragraph::new("Keine Karten").block(block);
        f.render_widget(para, area);
        return;
    }

    let display_cards = if valid.is_empty() { &hand } else { &valid };

    let spans: Vec<Span> = display_cards
        .iter()
        .enumerate()
        .flat_map(|(i, card)| {
            let is_selected = i == app.selected_card_index % display_cards.len().max(1);
            let base_style = Style::default().fg(suit_color(card.suit));
            let style = if is_selected {
                base_style.add_modifier(Modifier::REVERSED | Modifier::BOLD)
            } else {
                base_style
            };
            let text = format!(" {}{} ", card.rank.short_name(), card.suit.symbol());
            vec![Span::styled(text, style), Span::raw(" ")]
        })
        .collect();

    let non_valid: Vec<Card> = hand.iter().filter(|c| !valid.contains(c)).copied().collect();
    let mut all_lines = vec![Line::from(spans)];

    if !non_valid.is_empty() && !valid.is_empty() {
        let dim_spans: Vec<Span> = non_valid
            .iter()
            .flat_map(|card| {
                vec![
                    Span::styled(
                        format!(" {}{} ", card.rank.short_name(), card.suit.symbol()),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::raw(" "),
                ]
            })
            .collect();
        all_lines.push(Line::from(Span::styled(
            "Nicht spielbar:",
            Style::default().fg(Color::DarkGray),
        )));
        all_lines.push(Line::from(dim_spans));
    }

    let block = Block::default()
        .title(" Deine Karten (spielbar) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let para = Paragraph::new(all_lines).block(block);
    f.render_widget(para, area);
}

fn draw_hand(f: &mut Frame, app: &App, area: Rect) {
    let hand = app.human_hand();
    let spans: Vec<Span> = hand
        .iter()
        .flat_map(|card| vec![card_span(card), Span::raw(" ")])
        .collect();

    let block = Block::default()
        .title(" Deine Karten ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let para = Paragraph::new(Line::from(spans)).block(block);
    f.render_widget(para, area);
}

fn draw_hand_with_selection(f: &mut Frame, app: &App, area: Rect) {
    let hand = app.human_hand();
    let spans: Vec<Span> = hand
        .iter()
        .enumerate()
        .flat_map(|(i, card)| {
            let is_selected = i == app.selected_card_index % hand.len().max(1);
            let is_marked = app.discard_selected.contains(&i);
            let base_style = Style::default().fg(suit_color(card.suit));
            let style = if is_marked {
                base_style.bg(Color::DarkGray).add_modifier(Modifier::BOLD)
            } else if is_selected {
                base_style.add_modifier(Modifier::REVERSED | Modifier::BOLD)
            } else {
                base_style
            };
            let marker = if is_marked { "×" } else { " " };
            let text = format!("{}{}{} ", marker, card.rank.short_name(), card.suit.symbol());
            vec![Span::styled(text, style), Span::raw(" ")]
        })
        .collect();

    let block = Block::default()
        .title(" Deine Karten ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let para = Paragraph::new(Line::from(spans)).block(block);
    f.render_widget(para, area);
}

fn draw_trick_result(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Stich beendet!",
            Style::default().fg(Color::Yellow).bold(),
        )),
        Line::from(""),
    ];

    for (pid, card) in &app.last_trick_cards {
        let label = if *pid == app.human_player {
            "Du".to_string()
        } else {
            format!("{}", pid)
        };
        lines.push(Line::from(vec![
            Span::raw(format!("  {}: ", label)),
            card_span(card),
        ]));
    }

    if let Some(winner) = app.last_trick_winner {
        let winner_text = if winner == app.human_player {
            "Du gewinnst den Stich!".to_string()
        } else {
            format!("{} gewinnt den Stich", winner)
        };
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            winner_text,
            Style::default().fg(Color::Green).bold(),
        )));
    }

    let block = Block::default()
        .title(" Stichergebnis ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let para = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(para, area);
}

fn draw_deal_result(f: &mut Frame, app: &App, area: Rect) {
    let deal = match &app.engine.match_state.current_deal {
        Some(d) => d,
        None => return,
    };

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "═══ Spiel beendet! ═══",
            Style::default().fg(Color::Yellow).bold(),
        )),
        Line::from(""),
    ];

    if let Some(ref msg) = app.deal_message {
        lines.push(Line::from(Span::styled(
            msg.clone(),
            Style::default().fg(Color::Green).bold(),
        )));
    }

    if let Some(gt) = deal.game_type {
        lines.push(Line::from(""));
        lines.push(Line::from(format!("Spieltyp: {}", gt.german_name())));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Spielstand:",
        Style::default().fg(Color::Cyan).bold(),
    )));

    let scores = &app.engine.match_state.scores;
    lines.push(Line::from(format!(
        "  Du: {}  |  Player 1: {}  |  Player 2: {}",
        scores[0], scores[1], scores[2]
    )));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Enter = Nächstes Spiel",
        Style::default().fg(Color::DarkGray),
    )));

    let block = Block::default()
        .title(" Ergebnis ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let para = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(para, area);
}

fn draw_match_result(f: &mut Frame, app: &App, area: Rect) {
    let scores = &app.engine.match_state.scores;
    let winner = app.engine.match_state.winner();

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "╔═══════════════════════════╗",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            "║   BUMMERL BEENDET!       ║",
            Style::default().fg(Color::Yellow).bold(),
        )),
        Line::from(Span::styled(
            "╚═══════════════════════════╝",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
    ];

    if let Some(w) = winner {
        let text = if w == app.human_player {
            "Du hast gewonnen! 🎉"
        } else {
            "Du hast verloren."
        };
        lines.push(Line::from(Span::styled(
            text,
            Style::default()
                .fg(if w == app.human_player {
                    Color::Green
                } else {
                    Color::Red
                })
                .bold(),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "Endstand — Du: {}  P1: {}  P2: {}",
        scores[0], scores[1], scores[2]
    )));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Enter = Neues Bummerl | q = Beenden",
        Style::default().fg(Color::DarkGray),
    )));

    let block = Block::default()
        .title(" Endstand ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let para = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center);
    f.render_widget(para, area);
}

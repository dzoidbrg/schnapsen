use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use schnapsen_model::card::{Card, Suit};
use schnapsen_model::game::Phase;

use crate::app::{App, AppState, HUMAN_PLAYER};

fn suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Hearts | Suit::Diamonds => Color::Red,
        Suit::Spades | Suit::Clubs => Color::White,
    }
}

fn render_card_span(card: &Card) -> Span<'static> {
    let color = suit_color(card.suit);
    Span::styled(
        format!(" {}{} ", card.rank.short_name(), card.suit.symbol()),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )
}

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // title
            Constraint::Length(5),  // opponents
            Constraint::Min(8),    // middle (trick area + info)
            Constraint::Length(5),  // player hand
            Constraint::Length(3),  // action bar
        ])
        .split(size);

    draw_title(frame, main_chunks[0], app);
    draw_opponents(frame, main_chunks[1], app);
    draw_middle(frame, main_chunks[2], app);
    draw_hand(frame, main_chunks[3], app);
    draw_action_bar(frame, main_chunks[4], app);
}

fn draw_title(frame: &mut Frame, area: Rect, app: &App) {
    let trump_str = app
        .game
        .trump
        .map(|s| format!("Trump: {} {}", s.symbol(), s.name_de()))
        .unwrap_or_else(|| "No trump yet".into());

    let scores = format!(
        "You: {} | Bot B: {} | Bot C: {}",
        app.score.game_points[0], app.score.game_points[1], app.score.game_points[2]
    );

    let phase_name = match &app.game.phase {
        Phase::CallingTrump => "Calling Trump",
        Phase::Bidding { .. } => "Bidding",
        Phase::TalonDecision { .. } => "Talon Decision",
        Phase::Discarding { .. } => "Discarding",
        Phase::Playing { game_type, .. } => game_type.name_de(),
        Phase::Finished { .. } => "Hand Finished",
    };

    let title = Line::from(vec![
        Span::styled(
            " Dreierschnapsen ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("│ "),
        Span::styled(phase_name, Style::default().fg(Color::Cyan)),
        Span::raw(" │ "),
        Span::styled(trump_str, Style::default().fg(Color::Green)),
        Span::raw(" │ "),
        Span::styled(scores, Style::default().fg(Color::Magenta)),
    ]);

    let block = Block::default().borders(Borders::ALL);
    let p = Paragraph::new(title).block(block).alignment(Alignment::Left);
    frame.render_widget(p, area);
}

fn draw_opponents(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    for (i, &pid) in [1usize, 2].iter().enumerate() {
        let name = if pid == 1 { "Bot B" } else { "Bot C" };
        let card_count = app.game.hands[pid].len();
        let tricks = app.game.tricks_won[pid];
        let pts = app.game.card_points[pid];

        let cards_repr: String = (0..card_count).map(|_| "🂠 ").collect();

        let info = format!("{} cards │ {} tricks │ {} pts", card_count, tricks, pts);

        let content = vec![
            Line::from(Span::styled(
                cards_repr,
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(info, Style::default().fg(Color::Gray))),
        ];

        let block = Block::default()
            .title(format!(" {} ", name))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let p = Paragraph::new(content).block(block);
        frame.render_widget(p, chunks[i]);
    }
}

fn draw_middle(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    draw_trick_area(frame, chunks[0], app);
    draw_log(frame, chunks[1], app);
}

fn draw_trick_area(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Table ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let mut lines: Vec<Line> = Vec::new();

    if !app.game.current_trick.is_empty() {
        lines.push(Line::from(Span::styled(
            "Current trick:",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::UNDERLINED),
        )));
        for (pid, card) in &app.game.current_trick {
            let name = match pid {
                0 => "You   ",
                1 => "Bot B ",
                2 => "Bot C ",
                _ => "???   ",
            };
            let mut spans = vec![Span::styled(
                format!("  {} ", name),
                Style::default().fg(Color::Gray),
            )];
            spans.push(render_card_span(card));
            lines.push(Line::from(spans));
        }
    }

    if !app.last_trick_display.is_empty() && app.game.current_trick.is_empty() {
        lines.push(Line::from(Span::styled(
            "Last trick:",
            Style::default().fg(Color::DarkGray),
        )));
        for (pid, card) in &app.last_trick_display {
            let name = match pid {
                0 => "You   ",
                1 => "Bot B ",
                2 => "Bot C ",
                _ => "???   ",
            };
            let mut spans = vec![Span::styled(
                format!("  {} ", name),
                Style::default().fg(Color::DarkGray),
            )];
            spans.push(Span::styled(
                format!(" {}{} ", card.rank.short_name(), card.suit.symbol()),
                Style::default().fg(Color::DarkGray),
            ));
            lines.push(Line::from(spans));
        }
    }

    if app.game.talon.len() == 2
        && matches!(app.game.phase, Phase::CallingTrump | Phase::Bidding { .. })
    {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Talon: 🂠 🂠",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let your_tricks = app.game.tricks_won[HUMAN_PLAYER];
    let your_pts = app.game.total_points_for(HUMAN_PLAYER);
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("Your tricks: {} │ Your card pts: {}", your_tricks, your_pts),
        Style::default().fg(Color::Cyan),
    )));

    let p = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    frame.render_widget(p, area);
}

fn draw_log(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Log ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let visible_height = area.height.saturating_sub(2) as usize;
    let skip = app.log.len().saturating_sub(visible_height);

    let items: Vec<ListItem> = app.log[skip..]
        .iter()
        .map(|entry| ListItem::new(Span::styled(entry.clone(), Style::default().fg(Color::Gray))))
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_hand(frame: &mut Frame, area: Rect, app: &App) {
    let hand = &app.game.hands[HUMAN_PLAYER];
    let block = Block::default()
        .title(" Your Hand ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    if hand.is_empty() {
        let p = Paragraph::new(Span::styled(
            "  (empty)",
            Style::default().fg(Color::DarkGray),
        ))
        .block(block);
        frame.render_widget(p, area);
        return;
    }

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::raw(" "));
    for (i, card) in hand.iter().enumerate() {
        let color = suit_color(card.suit);
        let label = format!(" {}{} ", card.rank.short_name(), card.suit.symbol());
        let style = Style::default().fg(color).add_modifier(Modifier::BOLD);
        spans.push(Span::styled(label, style));
        if i < hand.len() - 1 {
            spans.push(Span::raw("  "));
        }
    }

    let line = Line::from(spans);

    let mut idx_spans: Vec<Span> = Vec::new();
    idx_spans.push(Span::raw(" "));
    for (i, card) in hand.iter().enumerate() {
        let w = format!(" {}{} ", card.rank.short_name(), card.suit.symbol()).len();
        let idx_str = format!("{:^width$}", i + 1, width = w);
        idx_spans.push(Span::styled(idx_str, Style::default().fg(Color::DarkGray)));
        if i < hand.len() - 1 {
            idx_spans.push(Span::raw("  "));
        }
    }

    let content = vec![Line::from(""), line, Line::from(idx_spans)];
    let p = Paragraph::new(content).block(block);
    frame.render_widget(p, area);
}

fn draw_action_bar(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default().borders(Borders::ALL);

    match &app.state {
        AppState::Choosing {
            prompt,
            options,
            selected,
        } => {
            let mut spans: Vec<Span> = vec![
                Span::styled(
                    format!(" {} ", prompt),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
            ];

            for (i, opt) in options.iter().enumerate() {
                let is_sel = i == *selected;
                let style = if is_sel {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                let prefix = if is_sel { "▸ " } else { "  " };
                spans.push(Span::styled(
                    format!("{}{}  ", prefix, opt.label),
                    style,
                ));
            }

            let line = Line::from(spans);
            let p = Paragraph::new(line).block(block);
            frame.render_widget(p, area);
        }
        AppState::AiActed { message } | AppState::HandOver { message } => {
            let line = Line::from(vec![
                Span::styled(
                    format!(" {} ", message),
                    Style::default().fg(Color::Cyan),
                ),
                Span::styled(
                    " [Enter to continue] ",
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            let p = Paragraph::new(line).block(block);
            frame.render_widget(p, area);
        }
        AppState::MatchOver { winner } => {
            let name = match winner {
                0 => "You",
                1 => "Bot B",
                2 => "Bot C",
                _ => "???",
            };
            let line = Line::from(vec![
                Span::styled(
                    format!(" {} won the match! ", name),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " [n] New game │ [q] Quit ",
                    Style::default().fg(Color::DarkGray),
                ),
            ]);
            let p = Paragraph::new(line).block(block);
            frame.render_widget(p, area);
        }
    }
}
